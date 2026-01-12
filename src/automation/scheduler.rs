use crate::automation::database::Database;
use crate::automation::engine::AutomationEngine;
use crate::automation::types::*;
use leptos::logging::log;
use std::sync::Arc;
use tokio::sync::Mutex;
use tokio_cron_scheduler::{Job, JobScheduler};
use uuid::Uuid;

pub struct AutomationScheduler {
    scheduler: Arc<Mutex<JobScheduler>>,
    database: Arc<Database>,
    engine: AutomationEngine,
    running_jobs: Arc<Mutex<std::collections::HashMap<i64, Uuid>>>,
    execution_timeout_secs: u64,
}

impl AutomationScheduler {
    pub async fn new(database: Arc<Database>, execution_timeout_secs: u64) -> Result<Self, String> {
        let scheduler = JobScheduler::new()
            .await
            .map_err(|e| format!("Failed to create scheduler: {}", e))?;

        Ok(Self {
            scheduler: Arc::new(Mutex::new(scheduler)),
            database,
            engine: AutomationEngine::new(),
            running_jobs: Arc::new(Mutex::new(std::collections::HashMap::new())),
            execution_timeout_secs,
        })
    }

    pub async fn start(&self) -> Result<(), String> {
        log!("Starting automation scheduler...");
        self.load_and_schedule_rules().await?;

        let scheduler = self.scheduler.lock().await;
        scheduler
            .start()
            .await
            .map_err(|e| format!("Failed to start scheduler: {}", e))?;

        log!("Automation scheduler started");
        Ok(())
    }

    pub async fn reload_rules(&self) -> Result<(), String> {
        log!("Reloading automation rules...");

        let mut jobs = self.running_jobs.lock().await;
        let scheduler = self.scheduler.lock().await;

        for (rule_id, job_uuid) in jobs.drain() {
            scheduler
                .remove(&job_uuid)
                .await
                .map_err(|e| format!("Failed to remove job {}: {}", rule_id, e))?;
        }

        drop(scheduler);
        drop(jobs);

        self.load_and_schedule_rules().await?;
        log!("Automation rules reloaded");
        Ok(())
    }

    async fn load_and_schedule_rules(&self) -> Result<(), String> {
        let rules = self.database.get_all_enabled_rules().await?;
        log!("Loading {} automation rules", rules.len());

        let scheduler = self.scheduler.lock().await;
        let mut jobs = self.running_jobs.lock().await;

        for rule in rules {
            if let Some(rule_id) = rule.id {
                let cron_expr = match self.trigger_to_cron(&rule.trigger_config) {
                    Ok(expr) => {
                        log!("Rule {}: Generated cron expression: {}", rule_id, expr);
                        expr
                    }
                    Err(e) => {
                        log!("Rule {}: Failed to generate cron expression: {}", rule_id, e);
                        return Err(format!("Failed to generate cron for rule {}: {}", rule_id, e));
                    }
                };

                let database = self.database.clone();
                let engine = self.engine.clone();
                let rule_name = rule.name.clone();
                let rule_clone = rule.clone();
                let timeout_secs = self.execution_timeout_secs;

                let job = Job::new_async(cron_expr.as_str(), move |_uuid, _l| {
                    let database = database.clone();
                    let engine = engine.clone();
                    let rule = rule_clone.clone();

                    Box::pin(async move {
                        if let Err(e) = Self::execute_rule_task(database, engine, rule.clone(), timeout_secs).await {
                            let error_str = e.to_string();
                            let is_transient = error_str.contains("530") 
                                || error_str.contains("504") 
                                || error_str.contains("502") 
                                || error_str.contains("503")
                                || error_str.contains("Network error");
                            
                            if is_transient {
                                log!("Rule '{}' execution failed due to transient API error (will retry on next schedule)", rule.name);
                            } else {
                                log!("Rule execution error for '{}': {}", rule.name, error_str);
                            }
                        }
                    })
                })
                .map_err(|e| format!("Failed to create job for rule {}: {}", rule_id, e))?;

                let job_uuid = scheduler
                    .add(job)
                    .await
                    .map_err(|e| format!("Failed to add job for rule {}: {}", rule_id, e))?;

                jobs.insert(rule_id, job_uuid);
                log!("Scheduled rule: {} ({})", rule_name, cron_expr);
            }
        }

        Ok(())
    }

    async fn execute_rule_task(
        database: Arc<Database>,
        engine: AutomationEngine,
        rule: AutomationRule,
        _timeout_secs: u64,
    ) -> Result<(), String> {
        let api_key = database.get_api_key(&rule.api_key_hash).await?;

        log!("Executing rule: {}", rule.name);

        let result = engine.execute_rule(&rule, &api_key).await
            .map_err(|e| format!("Rule execution failed: {}", e))?;

        let log_entry = ExecutionLog {
            id: None,
            rule_id: rule.id.unwrap_or(0),
            rule_name: rule.name.clone(),
            api_key_hash: rule.api_key_hash.clone(),
            execution_type: "execution".to_string(),
            items_processed: result.items_processed,
            total_items: Some(result.total_items),
            success: result.success,
            error_message: result.error_message.clone(),
            processed_items: result.processed_items.clone(),
            executed_at: None,
            partial: Some(result.partial),
        };

        database.log_execution(&log_entry).await?;

        if result.success && !result.partial {
            log!("Rule {} executed successfully: {}/{} items processed", rule.name, result.items_processed, result.total_items);
        } else if result.partial {
            log!("Rule {} partially completed: {}/{} items processed", rule.name, result.items_processed, result.total_items);
        } else {
            log!("Rule {} execution had errors: {}", rule.name, result.error_message.as_ref().unwrap_or(&"Unknown error".to_string()));
        }

        Ok(())
    }

    fn trigger_to_cron(&self, trigger: &TriggerConfig) -> Result<String, String> {
        match trigger {
            TriggerConfig::Cron { expression } => {
                let expr = expression.trim();
                if expr.is_empty() {
                    return Err("Cron expression cannot be empty".to_string());
                }
                Ok(expr.to_string())
            },
            TriggerConfig::Interval { minutes } => {
                let minutes = if *minutes < 30 { 30 } else { *minutes };

                if minutes < 60 {
                    Ok(format!("0 */{} * * * *", minutes))
                } else {
                    let hours = minutes / 60;
                    let remaining_minutes = minutes % 60;

                    if remaining_minutes == 0 {
                        if hours == 1 {
                            Ok("0 0 * * * *".to_string())
                        } else {
                            Ok(format!("0 0 */{} * * *", hours))
                        }
                    } else {
                        Ok(format!("0 {} */{} * * *", remaining_minutes, hours))
                    }
                }
            }
        }
    }

    pub async fn get_next_run_time(&self, rule_id: i64) -> Result<Option<chrono::DateTime<chrono::Utc>>, String> {
        let jobs = self.running_jobs.lock().await;
        let job_uuid = jobs.get(&rule_id).copied();
        drop(jobs);
        
        if let Some(job_uuid) = job_uuid {
            let mut scheduler = self.scheduler.lock().await;
            match scheduler.next_tick_for_job(job_uuid).await {
                Ok(Some(next_tick)) => Ok(Some(next_tick)),
                Ok(None) => Ok(None),
                Err(e) => Err(format!("Failed to get next run time: {}", e)),
            }
        } else {
            Ok(None)
        }
    }

    pub async fn shutdown(&self) -> Result<(), String> {
        log!("Shutting down automation scheduler...");

        let mut jobs = self.running_jobs.lock().await;
        let mut scheduler = self.scheduler.lock().await;

        for (rule_id, job_uuid) in jobs.drain() {
            scheduler
                .remove(&job_uuid)
                .await
                .map_err(|e| format!("Failed to remove job {}: {}", rule_id, e))?;
        }

        scheduler
            .shutdown()
            .await
            .map_err(|e| format!("Failed to shutdown scheduler: {}", e))?;

        log!("Automation scheduler shut down");
        Ok(())
    }
}

impl Clone for AutomationEngine {
    fn clone(&self) -> Self {
        AutomationEngine::new()
    }
}
