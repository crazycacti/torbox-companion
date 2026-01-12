use crate::api::{TorboxClient, ApiError};
use crate::api::types::Torrent;
use crate::automation::types::*;
use chrono::{DateTime, Utc};
use leptos::logging::log;
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

        log!("Fetching torrent list for rule: {}", rule.name);
        let torrents = self.fetch_torrents_with_retry(&client, rule.name.as_str(), 3).await?;
        log!("Fetched {} torrents for rule: {}", torrents.len(), rule.name);

        let matching_items = self.evaluate_conditions(rule, &torrents);
        log!("Rule '{}' matched {} items", rule.name, matching_items.len());

        let total_items = matching_items.len() as i32;
        if matching_items.is_empty() {
            return Ok(ExecutionResult {
                items_processed: 0,
                total_items: 0,
                success: true,
                error_message: None,
                processed_items: Some(Vec::new()),
                partial: false,
            });
        }

        let mut error_count = 0;
        let mut errors: Vec<String> = Vec::new();
        let mut processed_items: Vec<ProcessedItem> = Vec::new();

        let action_name = match rule.action_config.action_type {
            ActionType::StopSeeding => "Stop Seeding",
            ActionType::Delete => "Delete",
            ActionType::Stop => "Stop",
            ActionType::Resume => "Resume",
            ActionType::Restart => "Restart",
            ActionType::Reannounce => "Reannounce",
            ActionType::ForceStart => "Force Start",
        }.to_string();

        log!("Processing {} items for rule '{}' with action: {}", matching_items.len(), rule.name, action_name);
        let per_item_timeout = tokio::time::Duration::from_secs(10);
        
        for (idx, item) in matching_items.iter().enumerate() {
            if idx > 0 && idx % 10 == 0 {
                log!("Processed {}/{} items for rule '{}'", idx, matching_items.len(), rule.name);
            }
            
            let action_future = self.execute_action(&rule.action_config, &client, item);
            let result = match tokio::time::timeout(per_item_timeout, action_future).await {
                Ok(Ok(())) => Ok(()),
                Ok(Err(e)) => {
                    if e.contains("429") || e.contains("Rate limit") {
                        log!("Rate limit hit for rule '{}', waiting 2 seconds before retry...", rule.name);
                        tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;
                        let retry_result = self.execute_action(&rule.action_config, &client, item).await;
                        if retry_result.is_err() {
                            log!("Retry failed for rule '{}'", rule.name);
                        }
                        retry_result
                    } else {
                        Err(e)
                    }
                }
                Err(_) => {
                    log!("Action timed out after 10 seconds for rule '{}'", rule.name);
                    Err(format!("Action timed out after 10 seconds"))
                }
            };
            
            let success = result.is_ok();
            let error = result.as_ref().err().map(|e| e.to_string());

            processed_items.push(ProcessedItem {
                id: item.id,
                name: item.name.clone(),
                action: action_name.clone(),
                success,
                error: error.clone(),
            });

            if let Err(e) = result {
                error_count += 1;
                errors.push(e);
                if errors.len() >= 10 {
                    errors.push(format!("... and {} more errors", error_count - 10));
                }
            }
        }

        let items_processed = processed_items.len() as i32;
        let partial = items_processed < total_items;
        
        log!("Completed processing {}/{} items for rule '{}' ({} errors, partial: {})", 
             items_processed, total_items, rule.name, error_count, partial);
        
        Ok(ExecutionResult {
            items_processed,
            total_items,
            success: error_count == 0 && !partial,
            error_message: if error_count > 0 || partial {
                let mut msg_parts = Vec::new();
                if partial {
                    msg_parts.push(format!("Only processed {}/{} items", items_processed, total_items));
                }
                if error_count > 0 {
                    let error_summary = if error_count <= 3 {
                        errors.iter().take(error_count).cloned().collect::<Vec<_>>().join("; ")
                    } else {
                        format!("{} errors occurred (see processed items for details)", error_count)
                    };
                    msg_parts.push(format!("{} of {} actions failed. {}", error_count, items_processed, error_summary));
                }
                Some(msg_parts.join(". "))
            } else {
                None
            },
            processed_items: Some(processed_items),
            partial,
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

    async fn fetch_torrents_with_retry(
        &self,
        client: &TorboxClient,
        rule_name: &str,
        max_retries: u32,
    ) -> Result<Vec<Torrent>, String> {
        let mut last_error = None;
        
        for attempt in 1..=max_retries {
            match client.get_torrent_list(None, Some(true), None, None).await {
                Ok(response) => {
                    if let Some(data) = response.data {
                        if attempt > 1 {
                            log!("Successfully fetched torrent list for rule '{}' on attempt {}", rule_name, attempt);
                        }
                        return Ok(data);
                    } else {
                        return Err("No torrent data returned".to_string());
                    }
                }
                Err(e) => {
                    let error_str = e.to_string();
                    let is_transient = error_str.contains("530") 
                        || error_str.contains("504") 
                        || error_str.contains("502") 
                        || error_str.contains("503")
                        || error_str.contains("Network error");
                    
                    last_error = Some(error_str.clone());
                    
                    if is_transient && attempt < max_retries {
                        let delay_secs = attempt as u64;
                        log!("Transient error fetching torrents for rule '{}' (attempt {}/{}): {}. Retrying in {} seconds...", 
                             rule_name, attempt, max_retries, error_str, delay_secs);
                        tokio::time::sleep(tokio::time::Duration::from_secs(delay_secs)).await;
                        continue;
                    } else {
                        if !is_transient {
                            return Err(format!("Failed to fetch torrents: {}", error_str));
                        }
                    }
                }
            }
        }
        
        Err(format!("Failed to fetch torrents after {} attempts: {}", max_retries, last_error.unwrap_or_else(|| "Unknown error".to_string())))
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ExecutionResult {
    pub items_processed: i32,
    pub total_items: i32,
    pub success: bool,
    pub error_message: Option<String>,
    pub processed_items: Option<Vec<ProcessedItem>>,
    pub partial: bool,
}

impl Default for AutomationEngine {
    fn default() -> Self {
        Self::new()
    }
}
