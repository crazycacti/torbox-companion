use crate::automation::database::Database;
use crate::automation::encryption::EncryptionService;
use crate::automation::engine::ExecutionResult;
use crate::automation::scheduler::AutomationScheduler;
use crate::automation::types::*;
use axum::{
    extract::{Path, Query, State},
    http::{HeaderMap, StatusCode},
    response::Json,
    routing::{get, post, put, delete},
    Router,
};
use leptos::logging::log;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio_cron_scheduler::Job;

#[derive(Clone)]
pub struct AppState {
    pub database: Arc<Database>,
    pub scheduler: Arc<AutomationScheduler>,
    pub max_rules_per_user: i64,
    pub execution_timeout_secs: u64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ApiResponse<T> {
    pub success: bool,
    pub error: Option<String>,
    pub data: Option<T>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RuleLimitInfo {
    pub current_count: i64,
    pub max_rules: i64,
}

fn extract_api_key(headers: &HeaderMap, query: &HashMap<String, String>) -> Option<String> {
    if let Some(auth_header) = headers.get("authorization") {
        if let Ok(auth_str) = auth_header.to_str() {
            if auth_str.starts_with("Bearer ") {
                return Some(auth_str[7..].to_string());
            }
        }
    }

    if let Some(api_key_header) = headers.get("x-api-key") {
        if let Ok(key_str) = api_key_header.to_str() {
            return Some(key_str.to_string());
        }
    }

    query.get("api_key").cloned()
}

async fn get_api_key_hash(api_key: &str) -> String {
    EncryptionService::hash_api_key(api_key)
}

fn validate_rule(rule: &CreateRuleRequest) -> Result<(), String> {
    if rule.name.trim().is_empty() {
        return Err("Rule name cannot be empty".to_string());
    }
    
    if rule.name.len() > 200 {
        return Err("Rule name cannot exceed 200 characters".to_string());
    }
    
    if rule.conditions.is_empty() {
        return Err("At least one condition is required".to_string());
    }
    
    if rule.conditions.len() > 20 {
        return Err("Maximum of 20 conditions allowed".to_string());
    }
    
    for (idx, condition) in rule.conditions.iter().enumerate() {
        if condition.value.is_nan() {
            return Err(format!("Condition {}: value cannot be NaN", idx + 1));
        }
        
        if condition.value.is_infinite() {
            return Err(format!("Condition {}: value cannot be infinite", idx + 1));
        }
        
        if condition.value < -1_000_000_000.0 || condition.value > 1_000_000_000.0 {
            return Err(format!("Condition {}: value out of reasonable range", idx + 1));
        }
    }
    
    match &rule.trigger_config {
        TriggerConfig::Cron { expression } => {
            let expr = expression.trim();
            if expr.is_empty() {
                return Err("Cron expression cannot be empty".to_string());
            }
            
            let parts: Vec<&str> = expr.split_whitespace().collect();
            if parts.len() != 6 {
                return Err("Cron expression must have 6 fields (seconds minutes hours day month day-of-week)".to_string());
            }
            
            if let Err(e) = tokio_cron_scheduler::Job::new_async(expr, move |_, _| {
                Box::pin(async move {})
            }) {
                return Err(format!("Invalid cron expression: {}", e));
            }
        }
        TriggerConfig::Interval { minutes } => {
            if *minutes > 525600 {
                return Err("Interval cannot exceed 525600 minutes (1 year)".to_string());
            }
        }
    }
    
    Ok(())
}

pub fn create_routes(
    database: Arc<Database>,
    scheduler: Arc<AutomationScheduler>,
    max_rules_per_user: i64,
    execution_timeout_secs: u64,
) -> Router {
    let state = AppState { database, scheduler, max_rules_per_user, execution_timeout_secs };
    
    Router::new()
        .route("/api/automation/rules", get({
            let state = state.clone();
            move |headers: HeaderMap, query: Query<HashMap<String, String>>| {
                let state = state.clone();
                async move { get_rules(headers, query, state).await }
            }
        }).post({
            let state = state.clone();
            move |headers: HeaderMap, query: Query<HashMap<String, String>>, payload: Json<CreateRuleRequest>| {
                let state = state.clone();
                async move { create_rule(headers, query, state, payload).await }
            }
        }))
        .route("/api/automation/rules/{id}", get({
            let state = state.clone();
            move |id: Path<i64>, headers: HeaderMap, query: Query<HashMap<String, String>>| {
                let state = state.clone();
                async move { get_rule(id, headers, query, state).await }
            }
        }).put({
            let state = state.clone();
            move |id: Path<i64>, headers: HeaderMap, query: Query<HashMap<String, String>>, payload: Json<CreateRuleRequest>| {
                let state = state.clone();
                async move { update_rule(id, headers, query, state, payload).await }
            }
        }).delete({
            let state = state.clone();
            move |id: Path<i64>, headers: HeaderMap, query: Query<HashMap<String, String>>| {
                let state = state.clone();
                async move { delete_rule(id, headers, query, state).await }
            }
        }))
        .route("/api/automation/rules/{id}/logs", get({
            let state = state.clone();
            move |id: Path<i64>, headers: HeaderMap, query: Query<HashMap<String, String>>| {
                let state = state.clone();
                async move { get_rule_logs(id, headers, query, state).await }
            }
        }))
        .route("/api/automation/rules/{id}/next-run", get({
            let state = state.clone();
            move |id: Path<i64>, headers: HeaderMap, query: Query<HashMap<String, String>>| {
                let state = state.clone();
                async move { get_next_run_time(id, headers, query, state).await }
            }
        }))
        .route("/api/automation/rules/{id}/run", post({
            let state = state.clone();
            move |id: Path<i64>, headers: HeaderMap, query: Query<HashMap<String, String>>| {
                let state = state.clone();
                async move { force_run_rule(id, headers, query, state).await }
            }
        }))
        .route("/api/automation/rules/bulk-delete", post({
            let state = state.clone();
            move |headers: HeaderMap, query: Query<HashMap<String, String>>, payload: Json<serde_json::Value>| {
                let state = state.clone();
                async move { bulk_delete_rules(headers, query, state, payload).await }
            }
        }))
        .route("/api/automation/rules/limit", get({
            let state = state.clone();
            move |headers: HeaderMap, query: Query<HashMap<String, String>>| {
                let state = state.clone();
                async move { get_rule_limit(headers, query, state).await }
            }
        }))
        .route("/api/automation/health", get({
            let state = state.clone();
            move || {
                let state = state.clone();
                async move { health_check(state).await }
            }
        }))
}

async fn get_rules(
    headers: HeaderMap,
    query: Query<HashMap<String, String>>,
    state: AppState,
) -> Result<Json<ApiResponse<Vec<AutomationRule>>>, StatusCode> {
    let api_key = extract_api_key(&headers, &query)
        .ok_or(StatusCode::UNAUTHORIZED)?;

    let api_key_hash = get_api_key_hash(&api_key).await;

    match state.database.get_rules_by_api_key(&api_key_hash).await {
        Ok(rules) => Ok(Json(ApiResponse {
            success: true,
            error: None,
            data: Some(rules),
        })),
        Err(e) => {
            log!("Failed to get rules: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

async fn get_rule(
    id: Path<i64>,
    headers: HeaderMap,
    query: Query<HashMap<String, String>>,
    state: AppState,
) -> Result<Json<ApiResponse<AutomationRule>>, StatusCode> {
    let api_key = extract_api_key(&headers, &query)
        .ok_or(StatusCode::UNAUTHORIZED)?;

    let api_key_hash = get_api_key_hash(&api_key).await;

    match state.database.get_rule_by_id(*id, &api_key_hash).await {
        Ok(Some(rule)) => Ok(Json(ApiResponse {
            success: true,
            error: None,
            data: Some(rule),
        })),
        Ok(None) => Err(StatusCode::NOT_FOUND),
        Err(e) => {
            log!("Failed to get rule: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

#[derive(Deserialize)]
struct CreateRuleRequest {
    name: String,
    enabled: Option<bool>,
    trigger_config: TriggerConfig,
    conditions: Vec<Condition>,
    action_config: ActionConfig,
}

async fn create_rule(
    headers: HeaderMap,
    query: Query<HashMap<String, String>>,
    state: AppState,
    payload: Json<CreateRuleRequest>,
) -> Result<Json<ApiResponse<AutomationRule>>, StatusCode> {
    let api_key = extract_api_key(&headers, &query)
        .ok_or(StatusCode::UNAUTHORIZED)?;

    if let Err(e) = validate_rule(&payload) {
        return Ok(Json(ApiResponse {
            success: false,
            error: Some(e),
            data: None,
        }));
    }

    let api_key_hash = get_api_key_hash(&api_key).await;

    let current_rule_count = state.database.count_rules_by_api_key(&api_key_hash).await
        .map_err(|e| {
            log!("Failed to count rules: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    if current_rule_count >= state.max_rules_per_user {
        return Ok(Json(ApiResponse {
            success: false,
            error: Some(format!("Maximum rule limit ({}) reached for this API key", state.max_rules_per_user)),
            data: None,
        }));
    }

    state.database.save_api_key(&api_key).await
        .map_err(|e| {
            log!("Failed to save API key: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    let mut trigger_config = payload.trigger_config.clone();
    if let TriggerConfig::Interval { ref mut minutes } = trigger_config {
        if *minutes < 30 {
            *minutes = 30;
        }
    }

    let rule = AutomationRule {
        id: None,
        api_key_hash: api_key_hash.clone(),
        name: payload.name.clone(),
        enabled: payload.enabled.unwrap_or(true),
        trigger_config,
        conditions: payload.conditions.clone(),
        action_config: payload.action_config.clone(),
        created_at: None,
        updated_at: None,
    };

    match state.database.save_rule(&rule).await {
        Ok(rule_id) => {
            let mut rule_with_id = rule;
            rule_with_id.id = Some(rule_id);

            state.scheduler.reload_rules().await
                .map_err(|e| {
                    log!("Failed to reload rules: {}", e);
                    StatusCode::INTERNAL_SERVER_ERROR
                })?;

            Ok(Json(ApiResponse {
                success: true,
                error: None,
                data: Some(rule_with_id),
            }))
        }
        Err(e) => {
            log!("Failed to create rule: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

async fn update_rule(
    id: Path<i64>,
    headers: HeaderMap,
    query: Query<HashMap<String, String>>,
    state: AppState,
    payload: Json<CreateRuleRequest>,
) -> Result<Json<ApiResponse<AutomationRule>>, StatusCode> {
    let api_key = extract_api_key(&headers, &query)
        .ok_or(StatusCode::UNAUTHORIZED)?;

    if let Err(e) = validate_rule(&payload) {
        return Ok(Json(ApiResponse {
            success: false,
            error: Some(e),
            data: None,
        }));
    }

    let api_key_hash = get_api_key_hash(&api_key).await;

    let existing_rule = state.database.get_rule_by_id(*id, &api_key_hash).await
        .map_err(|e| {
            log!("Failed to get rule: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    let rule = existing_rule.ok_or(StatusCode::NOT_FOUND)?;

    let mut trigger_config = payload.trigger_config.clone();
    if let TriggerConfig::Interval { ref mut minutes } = trigger_config {
        if *minutes < 30 {
            *minutes = 30;
        }
    }

    let updated_rule = AutomationRule {
        id: Some(*id),
        api_key_hash: api_key_hash.clone(),
        name: payload.name.clone(),
        enabled: payload.enabled.unwrap_or(rule.enabled),
        trigger_config,
        conditions: payload.conditions.clone(),
        action_config: payload.action_config.clone(),
        created_at: rule.created_at,
        updated_at: None,
    };

    match state.database.save_rule(&updated_rule).await {
        Ok(_) => {
            state.scheduler.reload_rules().await
                .map_err(|e| {
                    log!("Failed to reload rules: {}", e);
                    StatusCode::INTERNAL_SERVER_ERROR
                })?;

            Ok(Json(ApiResponse {
                success: true,
                error: None,
                data: Some(updated_rule),
            }))
        }
        Err(e) => {
            log!("Failed to update rule: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

async fn delete_rule(
    id: Path<i64>,
    headers: HeaderMap,
    query: Query<HashMap<String, String>>,
    state: AppState,
) -> Result<Json<ApiResponse<()>>, StatusCode> {
    let api_key = extract_api_key(&headers, &query)
        .ok_or(StatusCode::UNAUTHORIZED)?;

    let api_key_hash = get_api_key_hash(&api_key).await;

    match state.database.delete_rule(*id, &api_key_hash).await {
        Ok(true) => {
            state.scheduler.reload_rules().await
                .map_err(|e| {
                    log!("Failed to reload rules: {}", e);
                    StatusCode::INTERNAL_SERVER_ERROR
                })?;

            Ok(Json(ApiResponse {
                success: true,
                error: None,
                data: None,
            }))
        }
        Ok(false) => Err(StatusCode::NOT_FOUND),
        Err(e) => {
            log!("Failed to delete rule: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

async fn get_rule_logs(
    id: Path<i64>,
    headers: HeaderMap,
    query: Query<HashMap<String, String>>,
    state: AppState,
) -> Result<Json<ApiResponse<Vec<ExecutionLog>>>, StatusCode> {
    let api_key = extract_api_key(&headers, &query)
        .ok_or(StatusCode::UNAUTHORIZED)?;

    let api_key_hash = get_api_key_hash(&api_key).await;

    let limit = query.get("limit")
        .and_then(|s| s.parse::<i32>().ok())
        .unwrap_or(100);

    match state.database.get_execution_logs(Some(*id), &api_key_hash, limit).await {
        Ok(logs) => Ok(Json(ApiResponse {
            success: true,
            error: None,
            data: Some(logs),
        })),
        Err(e) => {
            log!("Failed to get logs: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

async fn get_next_run_time(
    id: Path<i64>,
    headers: HeaderMap,
    query: Query<HashMap<String, String>>,
    state: AppState,
) -> Result<Json<ApiResponse<Option<String>>>, StatusCode> {
    let api_key = extract_api_key(&headers, &query)
        .ok_or(StatusCode::UNAUTHORIZED)?;

    let api_key_hash = get_api_key_hash(&api_key).await;

    let rule = state.database.get_rule_by_id(*id, &api_key_hash).await
        .map_err(|e| {
            log!("Failed to get rule: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?
        .ok_or(StatusCode::NOT_FOUND)?;

    if !rule.enabled {
        return Ok(Json(ApiResponse {
            success: true,
            error: None,
            data: None,
        }));
    }

    match state.scheduler.get_next_run_time(*id).await {
        Ok(Some(next_tick)) => {
            let formatted = next_tick.format("%Y-%m-%dT%H:%M:%S%.3fZ").to_string();
            Ok(Json(ApiResponse {
                success: true,
                error: None,
                data: Some(Some(formatted)),
            }))
        }
        Ok(None) => Ok(Json(ApiResponse {
            success: true,
            error: None,
            data: None,
        })),
        Err(e) => {
            log!("Failed to get next run time: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

async fn force_run_rule(
    id: Path<i64>,
    headers: HeaderMap,
    query: Query<HashMap<String, String>>,
    state: AppState,
) -> Result<Json<ApiResponse<ExecutionResult>>, StatusCode> {
    let api_key = extract_api_key(&headers, &query)
        .ok_or(StatusCode::UNAUTHORIZED)?;

    let api_key_hash = get_api_key_hash(&api_key).await;

    let rule = state.database.get_rule_by_id(*id, &api_key_hash).await
        .map_err(|e| {
            log!("Failed to get rule: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    let rule = rule.ok_or(StatusCode::NOT_FOUND)?;

    log!("Force running rule: {} (ID: {})", rule.name, id.0);

    use crate::automation::engine::AutomationEngine;
    let engine = AutomationEngine::new();
    
    let result = match engine.execute_rule(&rule, &api_key).await {
        Ok(result) => result,
        Err(e) => {
            log!("Failed to execute rule: {}", e);
            return Err(StatusCode::INTERNAL_SERVER_ERROR);
        }
    };

    let log_entry = ExecutionLog {
        id: None,
        rule_id: rule.id.unwrap_or(0),
        rule_name: rule.name.clone(),
        api_key_hash: rule.api_key_hash.clone(),
        execution_type: "manual".to_string(),
        items_processed: result.items_processed,
        total_items: Some(result.total_items),
        success: result.success,
        error_message: result.error_message.clone(),
        processed_items: result.processed_items.clone(),
        executed_at: None,
        partial: Some(result.partial),
    };

    state.database.log_execution(&log_entry).await
        .map_err(|e| {
            log!("Failed to log execution: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    if result.success && !result.partial {
        log!("Rule {} force run successful: {}/{} items processed", rule.name, result.items_processed, result.total_items);
    } else if result.partial {
        log!("Rule {} force run partially completed: {}/{} items processed", rule.name, result.items_processed, result.total_items);
    } else {
        log!("Rule {} force run had errors: {}", rule.name, result.error_message.as_ref().unwrap_or(&"Unknown error".to_string()));
    }

    Ok(Json(ApiResponse {
        success: true,
        error: None,
        data: Some(result),
    }))
}

#[derive(Debug, Deserialize)]
struct BulkDeleteRequest {
    rule_ids: Vec<i64>,
}

async fn bulk_delete_rules(
    headers: HeaderMap,
    query: Query<HashMap<String, String>>,
    state: AppState,
    payload: Json<serde_json::Value>,
) -> Result<Json<ApiResponse<serde_json::Value>>, StatusCode> {
    let api_key = extract_api_key(&headers, &query)
        .ok_or(StatusCode::UNAUTHORIZED)?;

    let api_key_hash = get_api_key_hash(&api_key).await;

    let delete_request: BulkDeleteRequest = serde_json::from_value(payload.0)
        .map_err(|_| StatusCode::BAD_REQUEST)?;

    if delete_request.rule_ids.is_empty() {
        return Ok(Json(ApiResponse {
            success: false,
            error: Some("No rule IDs provided".to_string()),
            data: None,
        }));
    }

    let mut deleted_count = 0;
    let mut errors = Vec::new();

    log!("Bulk delete request: {} rule IDs for API key hash: {}", delete_request.rule_ids.len(), api_key_hash);

    for rule_id in &delete_request.rule_ids {
        match state.database.delete_rule(*rule_id, &api_key_hash).await {
            Ok(true) => {
                deleted_count += 1;
                log!("Successfully deleted rule {}", rule_id);
            }
            Ok(false) => {
                let error_msg = format!("Rule {} not found or access denied", rule_id);
                errors.push(error_msg.clone());
                log!("{}", error_msg);
            }
            Err(e) => {
                let error_msg = format!("Failed to delete rule {}: {}", rule_id, e);
                errors.push(error_msg.clone());
                log!("{}", error_msg);
            }
        }
    }

    log!("Bulk delete completed: {} deleted, {} errors", deleted_count, errors.len());

    if deleted_count > 0 {
        state.scheduler.reload_rules().await
            .map_err(|e| {
                log!("Failed to reload rules after bulk delete: {}", e);
                StatusCode::INTERNAL_SERVER_ERROR
            })?;
    }

    if errors.is_empty() {
        Ok(Json(ApiResponse {
            success: true,
            error: None,
            data: Some(serde_json::json!({
                "deleted_count": deleted_count,
                "total_requested": delete_request.rule_ids.len()
            })),
        }))
    } else {
        Ok(Json(ApiResponse {
            success: deleted_count > 0,
            error: Some(format!("Deleted {} of {} rules. Errors: {}", deleted_count, delete_request.rule_ids.len(), errors.join("; "))),
            data: Some(serde_json::json!({
                "deleted_count": deleted_count,
                "total_requested": delete_request.rule_ids.len(),
                "errors": errors
            })),
        }))
    }
}

async fn get_rule_limit(
    headers: HeaderMap,
    query: Query<HashMap<String, String>>,
    state: AppState,
) -> Result<Json<ApiResponse<RuleLimitInfo>>, StatusCode> {
    let api_key = extract_api_key(&headers, &query)
        .ok_or(StatusCode::UNAUTHORIZED)?;

    let api_key_hash = get_api_key_hash(&api_key).await;

    let current_count = state.database.count_rules_by_api_key(&api_key_hash).await
        .map_err(|e| {
            log!("Failed to count rules: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    Ok(Json(ApiResponse {
        success: true,
        error: None,
        data: Some(RuleLimitInfo {
            current_count,
            max_rules: state.max_rules_per_user,
        }),
    }))
}

async fn health_check(state: AppState) -> Json<ApiResponse<serde_json::Value>> {
    let mut status = serde_json::json!({
        "database": "unknown",
        "scheduler": "unknown"
    });

    match state.database.get_all_api_key_hashes().await {
        Ok(_) => {
            status["database"] = serde_json::json!("ok");
        }
        Err(e) => {
            log!("Database health check failed: {}", e);
            status["database"] = serde_json::json!("error");
        }
    }

    status["scheduler"] = serde_json::json!("running");

    Json(ApiResponse {
        success: status["database"] == "ok",
        error: None,
        data: Some(status),
    })
}
