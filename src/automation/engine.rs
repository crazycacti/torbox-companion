use crate::api::{TorboxClient, ApiError};
use crate::api::types::Torrent;
use crate::automation::types::*;
use chrono::{DateTime, Utc};
use std::time::{SystemTime, UNIX_EPOCH};

pub struct AutomationEngine;

impl AutomationEngine {
    pub fn new() -> Self {
        Self
    }

    pub async fn execute_rule(
        &self,
        rule: &AutomationRule,
        api_key: &str,
    ) -> Result<ExecutionResult, String> {
        let client = TorboxClient::new(api_key.to_string());

        let torrents = client
            .get_torrent_list(None, Some(true), None, None)
            .await
            .map_err(|e| format!("Failed to fetch torrents: {}", e))?;

        let torrents = torrents.data.ok_or("No torrent data returned")?;

        let matching_items = self.evaluate_conditions(rule, &torrents);

        if matching_items.is_empty() {
            return Ok(ExecutionResult {
                items_processed: 0,
                success: true,
                error_message: None,
            });
        }

        let mut error_count = 0;
        let mut errors: Vec<String> = Vec::new();

        for item in &matching_items {
            if let Err(e) = self.execute_action(&rule.action_config, &client, item).await {
                error_count += 1;
                errors.push(format!("Torrent {} ({}): {}", item.id, item.name, e));
                if errors.len() >= 10 {
                    errors.push(format!("... and {} more errors", error_count - 10));
                    break;
                }
            }
        }

        Ok(ExecutionResult {
            items_processed: matching_items.len() as i32,
            success: error_count == 0,
            error_message: if error_count > 0 {
                Some(format!("{} of {} actions failed. {}", error_count, matching_items.len(), errors.join("; ")))
            } else {
                None
            },
        })
    }

    fn evaluate_conditions<'a>(&self, rule: &AutomationRule, items: &'a [Torrent]) -> Vec<&'a Torrent> {
        items
            .iter()
            .filter(|item| {
                rule.conditions.iter().all(|condition| {
                    self.evaluate_condition(condition, item)
                })
            })
            .collect()
    }

    fn evaluate_condition(&self, condition: &Condition, item: &Torrent) -> bool {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs() as i64;

        let condition_value: Option<f64> = match condition.r#type {
            ConditionType::SeedingTime => {
                if !item.active {
                    return false;
                }
                if !item.download_finished {
                    return false;
                }
                if let Some(cached_at) = &item.cached_at {
                    if let Ok(cached_time) = DateTime::parse_from_rfc3339(cached_at) {
                        let elapsed = now - cached_time.timestamp();
                        Some(elapsed as f64 / 3600.0)
                    } else {
                        return false;
                    }
                } else if let Ok(updated_time) = DateTime::parse_from_rfc3339(&item.updated_at) {
                    let elapsed = now - updated_time.timestamp();
                    Some(elapsed as f64 / 3600.0)
                } else {
                    return false;
                }
            }
            ConditionType::SeedingRatio => {
                if !item.active {
                    return false;
                }
                Some(item.ratio as f64)
            }
            ConditionType::StalledTime => {
                let status_lower = item.download_state.to_lowercase();
                
                if status_lower.contains("stalled") {
                    if let Ok(created_time) = DateTime::parse_from_rfc3339(&item.created_at) {
                        let elapsed = now - created_time.timestamp();
                        Some(elapsed as f64 / 3600.0)
                    } else {
                        Some(0.0)
                    }
                } else {
                    let is_downloading_status = status_lower == "downloading" 
                        || status_lower == "active" 
                        || status_lower.contains("downloading");
                    let is_checking = status_lower == "checking";
                    
                    if !is_downloading_status && !is_checking {
                        return false;
                    }
                    
                    let has_no_speed = item.download_speed < 1024;
                    let has_no_seeds = item.seeds == 0;
                    let has_no_peers = item.peers == 0;
                    let is_inactive = !item.active;
                    
                    let is_stalled = if is_checking {
                        if let Ok(updated_time) = DateTime::parse_from_rfc3339(&item.updated_at) {
                            let elapsed = now - updated_time.timestamp();
                            elapsed > 21600
                        } else {
                            false
                        }
                    } else if is_downloading_status {
                        has_no_speed && item.upload_speed == 0 && ((has_no_seeds && has_no_peers) || is_inactive)
                    } else {
                        false
                    };
                    
                    if !is_stalled {
                        return false;
                    }
                    
                    if let Ok(updated_time) = DateTime::parse_from_rfc3339(&item.updated_at) {
                        let elapsed = now - updated_time.timestamp();
                        Some(elapsed as f64 / 3600.0)
                    } else {
                        return false;
                    }
                }
            }
            ConditionType::Age => {
                if let Ok(created_time) = DateTime::parse_from_rfc3339(&item.created_at) {
                    let elapsed = now - created_time.timestamp();
                    Some(elapsed as f64 / 3600.0)
                } else {
                    return false;
                }
            }
            ConditionType::DownloadSpeed => Some(item.download_speed as f64),
            ConditionType::UploadSpeed => Some(item.upload_speed as f64),
            ConditionType::FileSize => Some(item.size as f64 / (1024.0 * 1024.0 * 1024.0)),
            ConditionType::Progress => Some(item.progress as f64),
            ConditionType::Seeds => Some(item.seeds as f64),
            ConditionType::Peers => Some(item.peers as f64),
            ConditionType::TotalUploaded => Some(item.total_uploaded as f64 / (1024.0 * 1024.0 * 1024.0)),
            ConditionType::TotalDownloaded => Some(item.total_downloaded as f64 / (1024.0 * 1024.0 * 1024.0)),
            ConditionType::DownloadState => {
                return match condition.value as i32 {
                    0 => item.download_state == "downloading",
                    1 => item.download_state == "uploading" || item.download_state == "uploading (no peers)",
                    2 => item.download_state == "stopped seeding" || item.download_state == "stopped",
                    3 => item.download_state == "cached",
                    _ => false,
                };
            }
            ConditionType::Inactive => {
                let status_lower = item.download_state.to_lowercase();
                
                if status_lower == "reported missing"
                    || status_lower == "missingfiles"
                    || status_lower == "failed"
                    || status_lower.starts_with("failed")
                    || status_lower == "error" {
                    Some(1.0)
                } else if item.download_finished {
                    Some(0.0)
                } else {
                    let is_stalled = {
                        let is_checking = status_lower == "checking";
                        let is_downloading_status = status_lower == "downloading" 
                            || status_lower == "active" 
                            || status_lower.contains("downloading");
                        
                        if is_checking {
                            if let Ok(updated_time) = DateTime::parse_from_rfc3339(&item.updated_at) {
                                let elapsed = now - updated_time.timestamp();
                                elapsed > 21600
                            } else {
                                false
                            }
                        } else if is_downloading_status {
                            let has_no_speed = item.download_speed < 1024;
                            let has_no_seeds = item.seeds == 0;
                            let has_no_peers = item.peers == 0;
                            let is_inactive = !item.active;
                            
                            has_no_speed && item.upload_speed == 0 && ((has_no_seeds && has_no_peers) || is_inactive)
                        } else {
                            status_lower.contains("stalled")
                        }
                    };
                    
                    if is_stalled && !item.active {
                        Some(1.0)
                    } else {
                        let is_expired = if let Some(expires_at) = &item.expires_at {
                            if let Ok(expires_time) = DateTime::parse_from_rfc3339(expires_at) {
                                expires_time.timestamp() < now
                            } else {
                                false
                            }
                        } else {
                            false
                        };
                        
                        if is_expired || status_lower == "expired" {
                            Some(1.0)
                        } else if !item.active 
                            && status_lower != "expired" 
                            && !status_lower.contains("cached") 
                            && !status_lower.contains("completed") 
                            && !status_lower.contains("uploading") 
                            && !status_lower.contains("seeding")
                            && !status_lower.contains("stalled") {
                            Some(1.0)
                        } else if status_lower == "stopped seeding" 
                            || status_lower == "stopped"
                            || status_lower == "error"
                            || status_lower == "failed" {
                            Some(1.0)
                        } else {
                            Some(0.0)
                        }
                    }
                }
            }
            ConditionType::DownloadFinished => {
                return if item.download_finished { 1.0 } else { 0.0 } == condition.value;
            }
            ConditionType::Cached => {
                return if item.cached { 1.0 } else { 0.0 } == condition.value;
            }
            ConditionType::Private => {
                return if item.private { 1.0 } else { 0.0 } == condition.value;
            }
            ConditionType::LongTermSeeding => {
                return if item.long_term_seeding { 1.0 } else { 0.0 } == condition.value;
            }
            ConditionType::SeedTorrent => {
                return if item.seed_torrent { 1.0 } else { 0.0 } == condition.value;
            }
            ConditionType::ETA => {
                Some(item.eta as f64 / 3600.0)
            }
            ConditionType::Availability => {
                Some(item.availability as f64)
            }
            ConditionType::ExpiresAt => {
                if let Some(expires_at) = &item.expires_at {
                    if let Ok(expires_time) = DateTime::parse_from_rfc3339(expires_at) {
                        let elapsed = expires_time.timestamp() - now;
                        if elapsed > 0 {
                            Some(elapsed as f64 / 3600.0)
                        } else {
                            Some(0.0)
                        }
                    } else {
                        None
                    }
                } else {
                    None
                }
            }
            ConditionType::DownloadPresent => {
                return if item.download_present { 1.0 } else { 0.0 } == condition.value;
            }
            ConditionType::TorrentFile => {
                return if item.torrent_file { 1.0 } else { 0.0 } == condition.value;
            }
            ConditionType::AllowZipped => {
                return if item.allow_zipped { 1.0 } else { 0.0 } == condition.value;
            }
            ConditionType::HasMagnet => {
                return if item.magnet.is_some() { 1.0 } else { 0.0 } == condition.value;
            }
        };

        let condition_value = condition_value.unwrap_or(0.0);
        match condition.operator {
            Operator::GreaterThan => condition_value > condition.value,
            Operator::LessThan => condition_value < condition.value,
            Operator::GreaterThanOrEqual => condition_value >= condition.value,
            Operator::LessThanOrEqual => condition_value <= condition.value,
            Operator::Equal => (condition_value - condition.value).abs() < 0.001,
        }
    }

    async fn execute_action(
        &self,
        action: &ActionConfig,
        client: &TorboxClient,
        item: &Torrent,
    ) -> Result<(), String> {
        match action.action_type {
            ActionType::StopSeeding => {
                client
                    .control_torrent("stop_seeding".to_string(), item.id, false)
                    .await
                    .map_err(|e| format!("Failed to stop seeding: {}", e))?;
            }
            ActionType::Delete => {
                client
                    .control_torrent("delete".to_string(), item.id, false)
                    .await
                    .map_err(|e| format!("Failed to delete: {}", e))?;
            }
            ActionType::Stop => {
                client
                    .control_torrent("stop".to_string(), item.id, false)
                    .await
                    .map_err(|e| format!("Failed to stop: {}", e))?;
            }
            ActionType::Resume => {
                client
                    .control_torrent("resume".to_string(), item.id, false)
                    .await
                    .map_err(|e| format!("Failed to resume: {}", e))?;
            }
            ActionType::Restart => {
                client
                    .control_torrent("restart".to_string(), item.id, false)
                    .await
                    .map_err(|e| format!("Failed to restart: {}", e))?;
            }
            ActionType::Reannounce => {
                client
                    .control_torrent("reannounce".to_string(), item.id, false)
                    .await
                    .map_err(|e| format!("Failed to reannounce: {}", e))?;
            }
            ActionType::ForceStart => {
                client
                    .control_torrent("start".to_string(), item.id, false)
                    .await
                    .map_err(|e| format!("Failed to force start: {}", e))?;
            }
        }
        Ok(())
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ExecutionResult {
    pub items_processed: i32,
    pub success: bool,
    pub error_message: Option<String>,
}

impl Default for AutomationEngine {
    fn default() -> Self {
        Self::new()
    }
}
