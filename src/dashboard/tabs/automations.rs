use leptos::prelude::*;
#[cfg(feature = "hydrate")]
use leptos::task::spawn_local;
#[cfg(feature = "hydrate")]
use web_sys;
#[cfg(feature = "hydrate")]
use wasm_bindgen_futures;
#[cfg(feature = "hydrate")]
use wasm_bindgen::JsCast;
#[cfg(feature = "hydrate")]
use js_sys;
use std::sync::Arc;
use crate::notifications::{use_confirmation, show_confirmation, ConfirmationVariant};
use crate::dashboard::components::loading_spinner::{LoadingSpinner, SpinnerSize, SpinnerVariant};

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct AutomationRule {
    pub id: Option<i64>,
    pub name: String,
    pub enabled: bool,
    pub trigger_config: serde_json::Value,
    pub conditions: Vec<serde_json::Value>,
    pub action_config: serde_json::Value,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ProcessedItem {
    pub id: i32,
    pub name: String,
    pub action: String,
    pub success: bool,
    pub error: Option<String>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ExecutionLog {
    pub id: Option<i64>,
    pub rule_id: i64,
    pub rule_name: String,
    pub api_key_hash: String,
    pub execution_type: String,
    pub items_processed: i32,
    #[serde(default)]
    pub total_items: Option<i32>,
    pub success: bool,
    pub error_message: Option<String>,
    pub processed_items: Option<Vec<ProcessedItem>>,
    pub executed_at: Option<String>,
    #[serde(default)]
    pub partial: Option<bool>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ApiResponse<T> {
    pub success: bool,
    pub error: Option<String>,
    pub data: Option<T>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
struct CreateRuleRequest {
    name: String,
    enabled: Option<bool>,
    trigger_config: serde_json::Value,
    conditions: Vec<serde_json::Value>,
    action_config: serde_json::Value,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct RuleLimitInfo {
    pub current_count: i64,
    pub max_rules: i64,
}

#[component]
pub fn AutomationsTab() -> impl IntoView {
    let rules = RwSignal::new(Vec::<AutomationRule>::new());
    let loading = RwSignal::new(false);
    let error = RwSignal::new(None::<String>);
    let show_create_modal = RwSignal::new(false);
    let editing_rule_id = RwSignal::new(None::<i64>);
    let preset_data = RwSignal::new(None::<(String, u32, String, String, f64, String)>);
    let rule_logs = RwSignal::new(std::collections::HashMap::<i64, Vec<ExecutionLog>>::new());
    let next_run_times = RwSignal::new(std::collections::HashMap::<i64, Option<String>>::new());
    let confirmation_state = use_confirmation();
    let rule_limit = RwSignal::new(None::<RuleLimitInfo>);
    let selected_rules = RwSignal::new(std::collections::HashSet::<i64>::new());
    let running_rules = RwSignal::new(std::collections::HashSet::<i64>::new());
    let expanded_items = RwSignal::new(std::collections::HashSet::<i64>::new());

    let fetch_rules = move || {
        #[cfg(feature = "hydrate")]
        {
            loading.set(true);
            error.set(None);
            spawn_local(async move {
                if let Some(window) = web_sys::window() {
                    if let Ok(Some(storage)) = window.local_storage() {
                        if let Ok(Some(api_key)) = storage.get_item("api_key") {
                            if !api_key.is_empty() {
                                let url = "/api/automation/rules";
                                let headers = web_sys::Headers::new().unwrap();
                                headers.set("Authorization", &format!("Bearer {}", api_key)).unwrap();
                                
                                let init = {
                                    let mut i = web_sys::RequestInit::new();
                                    i.set_method("GET");
                                    i.set_headers(&headers);
                                    i
                                };
                                
                                let promise = window.fetch_with_str_and_init(url, &init);
                                let future = wasm_bindgen_futures::JsFuture::from(promise);
                                
                                if let Ok(response) = future.await {
                                    let resp: web_sys::Response = response.dyn_into().unwrap();
                                    if resp.status() == 200 {
                                        let text_promise = resp.text().unwrap();
                                        let text_future = wasm_bindgen_futures::JsFuture::from(text_promise);
                                        if let Ok(text_value) = text_future.await {
                                            if let Some(text) = text_value.as_string() {
                                                if let Ok(api_response) = serde_json::from_str::<ApiResponse<Vec<AutomationRule>>>(&text) {
                                                    if let Some(data) = api_response.data {
                                                        rules.set(data);
                                                    } else if let Some(err) = api_response.error {
                                                        error.set(Some(err));
                                                    }
                                                } else {
                                                    error.set(Some("Failed to parse response".to_string()));
                                                }
                                            } else {
                                                error.set(Some("Response is not a string".to_string()));
                                            }
                                        } else {
                                            error.set(Some("Failed to read response text".to_string()));
                                        }
                                    } else {
                                        error.set(Some(format!("Failed to fetch rules: {}", resp.status())));
                                    }
                                } else {
                                    error.set(Some("Network request failed".to_string()));
                                }
                            } else {
                                error.set(Some("No API key found".to_string()));
                            }
                        } else {
                            error.set(Some("No API key found".to_string()));
                        }
                    }
                }
                loading.set(false);
            });
        }
    };

    let fetch_rule_limit = move || {
        #[cfg(feature = "hydrate")]
        {
            spawn_local(async move {
                if let Some(window) = web_sys::window() {
                    if let Ok(Some(storage)) = window.local_storage() {
                        if let Ok(Some(api_key)) = storage.get_item("api_key") {
                            if !api_key.is_empty() {
                                let url = "/api/automation/rules/limit";
                                let headers = web_sys::Headers::new().unwrap();
                                headers.set("Authorization", &format!("Bearer {}", api_key)).unwrap();
                                
                                let init = {
                                    let mut i = web_sys::RequestInit::new();
                                    i.set_method("GET");
                                    i.set_headers(&headers);
                                    i
                                };
                                
                                let promise = window.fetch_with_str_and_init(url, &init);
                                let future = wasm_bindgen_futures::JsFuture::from(promise);
                                
                                if let Ok(response) = future.await {
                                    let resp: web_sys::Response = response.dyn_into().unwrap();
                                    if resp.status() == 200 {
                                        let text_promise = resp.text().unwrap();
                                        let text_future = wasm_bindgen_futures::JsFuture::from(text_promise);
                                        if let Ok(text_value) = text_future.await {
                                            if let Some(text) = text_value.as_string() {
                                                if let Ok(api_response) = serde_json::from_str::<ApiResponse<RuleLimitInfo>>(&text) {
                                                    if let Some(data) = api_response.data {
                                                        rule_limit.set(Some(data));
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            });
        }
    };

    fetch_rules();
    fetch_rule_limit();

    Effect::new(move |_| {
        let rules_clone = rules.get();
        for rule in rules_clone {
            if let Some(id) = rule.id {
                #[cfg(feature = "hydrate")]
                {
                    let rule_logs_clone = rule_logs.clone();
                    spawn_local(async move {
                        if let Some(window) = web_sys::window() {
                            if let Ok(Some(storage)) = window.local_storage() {
                                if let Ok(Some(api_key)) = storage.get_item("api_key") {
                                    if !api_key.is_empty() {
                                        let url = format!("/api/automation/rules/{}/logs?limit=10", id);
                                        let headers = web_sys::Headers::new().unwrap();
                                        headers.set("Authorization", &format!("Bearer {}", api_key)).unwrap();
                                        
                                        let init = {
                                            let mut i = web_sys::RequestInit::new();
                                            i.set_method("GET");
                                            i.set_headers(&headers);
                                            i
                                        };
                                        
                                        let promise = window.fetch_with_str_and_init(&url, &init);
                                        let future = wasm_bindgen_futures::JsFuture::from(promise);
                                        
                                        if let Ok(response) = future.await {
                                            let resp: web_sys::Response = response.dyn_into().unwrap();
                                            if resp.status() == 200 {
                                                let text_promise = resp.text().unwrap();
                                                let text_future = wasm_bindgen_futures::JsFuture::from(text_promise);
                                                if let Ok(text_value) = text_future.await {
                                                    if let Some(text) = text_value.as_string() {
                                                        if let Ok(api_response) = serde_json::from_str::<ApiResponse<Vec<ExecutionLog>>>(&text) {
                                                            if let Some(logs) = api_response.data {
                                                                rule_logs_clone.update(|map| {
                                                                    map.insert(id, logs);
                                                                });
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    });
                }
            }
        }
    });

    let toggle_rule = move |rule_id: i64, current_enabled: bool, rule_clone: AutomationRule| {
        #[cfg(feature = "hydrate")]
        {
            spawn_local(async move {
                if let Some(window) = web_sys::window() {
                    if let Ok(Some(storage)) = window.local_storage() {
                        if let Ok(Some(api_key)) = storage.get_item("api_key") {
                            if !api_key.is_empty() {
                                let url = format!("/api/automation/rules/{}", rule_id);
                                let headers = web_sys::Headers::new().unwrap();
                                headers.set("Authorization", &format!("Bearer {}", api_key)).unwrap();
                                headers.set("Content-Type", "application/json").unwrap();

                                let request_body = serde_json::json!({
                                    "name": rule_clone.name,
                                    "enabled": !current_enabled,
                                    "trigger_config": rule_clone.trigger_config,
                                    "conditions": rule_clone.conditions,
                                    "action_config": rule_clone.action_config
                                });
                                let body_str = serde_json::to_string(&request_body).unwrap();
                                let body_js = wasm_bindgen::JsValue::from_str(&body_str);

                                let init = {
                                    let mut i = web_sys::RequestInit::new();
                                    i.set_method("PUT");
                                    i.set_headers(&headers);
                                    i.set_body(&body_js);
                                    i
                                };

                                let promise = window.fetch_with_str_and_init(&url, &init);
                                let future = wasm_bindgen_futures::JsFuture::from(promise);

                                if let Ok(response) = future.await {
                                    let resp: web_sys::Response = response.dyn_into().unwrap();
                                if resp.status() == 200 {
                                    fetch_rules();
                                    fetch_rule_limit();
                                } else {
                                        let text_promise = resp.text().unwrap();
                                        let text_future = wasm_bindgen_futures::JsFuture::from(text_promise);
                                        if let Ok(text_value) = text_future.await {
                                            if let Some(text) = text_value.as_string() {
                                                error.set(Some(format!("Failed to toggle rule: {}", text)));
                                            } else {
                                                error.set(Some(format!("Failed to toggle rule: {}", resp.status())));
                                            }
                                        } else {
                                            error.set(Some(format!("Failed to toggle rule: {}", resp.status())));
                                        }
                                    }
                                } else {
                                    error.set(Some("Network request failed".to_string()));
                                }
                            }
                        }
                    }
                }
            });
        }
    };

    let delete_rule = move |rule_id: i64| {
        #[cfg(feature = "hydrate")]
        {
            spawn_local(async move {
                if let Some(window) = web_sys::window() {
                    if let Ok(Some(storage)) = window.local_storage() {
                        if let Ok(Some(api_key)) = storage.get_item("api_key") {
                            if !api_key.is_empty() {
                                let url = format!("/api/automation/rules/{}", rule_id);
                                let headers = web_sys::Headers::new().unwrap();
                                headers.set("Authorization", &format!("Bearer {}", api_key)).unwrap();
                                
                                let init = {
                                    let mut i = web_sys::RequestInit::new();
                                    i.set_method("DELETE");
                                    i.set_headers(&headers);
                                    i
                                };
                                
                                let promise = window.fetch_with_str_and_init(&url, &init);
                                let future = wasm_bindgen_futures::JsFuture::from(promise);
                                
                                if let Ok(response) = future.await {
                                    let resp: web_sys::Response = response.dyn_into().unwrap();
                                    if resp.status() == 200 || resp.status() == 204 {
                                        selected_rules.update(|set| {
                                            set.remove(&rule_id);
                                        });
                                        fetch_rules();
                                        fetch_rule_limit();
                                    } else {
                                        error.set(Some(format!("Failed to delete rule: {}", resp.status())));
                                    }
                                }
                            }
                        }
                    }
                }
            });
        }
    };

    let create_preset_rule = move |preset_name: String, trigger_minutes: u32, condition_type_val: String, condition_op: String, condition_val: f64, action_type_val: String| {
        #[cfg(feature = "hydrate")]
        {
            loading.set(true);
            error.set(None);
            spawn_local(async move {
                if let Some(window) = web_sys::window() {
                    if let Ok(Some(storage)) = window.local_storage() {
                        if let Ok(Some(api_key)) = storage.get_item("api_key") {
                            if !api_key.is_empty() {
                                let trigger_config = serde_json::json!({
                                    "Interval": {
                                        "minutes": trigger_minutes
                                    }
                                });

                                let conditions = vec![serde_json::json!({
                                    "type": condition_type_val,
                                    "operator": condition_op,
                                    "value": condition_val
                                })];

                                let action_config = serde_json::json!({
                                    "action_type": action_type_val,
                                    "params": serde_json::Value::Null
                                });

                                let request = CreateRuleRequest {
                                    name: preset_name.clone(),
                                    enabled: Some(true),
                                    trigger_config,
                                    conditions,
                                    action_config,
                                };

                                let url = "/api/automation/rules";
                                let headers = web_sys::Headers::new().unwrap();
                                headers.set("Authorization", &format!("Bearer {}", api_key)).unwrap();
                                headers.set("Content-Type", "application/json").unwrap();

                                let body = serde_json::to_string(&request).unwrap();
                                let body_js = wasm_bindgen::JsValue::from_str(&body);

                                let init = {
                                    let mut i = web_sys::RequestInit::new();
                                    i.set_method("POST");
                                    i.set_headers(&headers);
                                    i.set_body(&body_js);
                                    i
                                };

                                let promise = window.fetch_with_str_and_init(&url, &init);
                                let future = wasm_bindgen_futures::JsFuture::from(promise);

                                if let Ok(response) = future.await {
                                    let resp: web_sys::Response = response.dyn_into().unwrap();
                                    if resp.status() == 200 || resp.status() == 201 {
                                        fetch_rules();
                                        fetch_rule_limit();
                                    } else {
                                        let text_promise = resp.text().unwrap();
                                        let text_future = wasm_bindgen_futures::JsFuture::from(text_promise);
                                        if let Ok(text_value) = text_future.await {
                                            if let Some(text) = text_value.as_string() {
                                                if let Ok(api_response) = serde_json::from_str::<ApiResponse<serde_json::Value>>(&text) {
                                                    if let Some(err_msg) = api_response.error {
                                                        error.set(Some(err_msg));
                                                    } else {
                                                        error.set(Some(format!("Failed to create rule: {}", text)));
                                                    }
                                                } else {
                                                    error.set(Some(format!("Failed to create rule: {}", text)));
                                                }
                                            } else {
                                                error.set(Some(format!("Failed to create rule: {}", resp.status())));
                                            }
                                        } else {
                                            error.set(Some(format!("Failed to create rule: {}", resp.status())));
                                        }
                                        fetch_rule_limit();
                                    }
                                } else {
                                    error.set(Some("Network request failed".to_string()));
                                }
                            }
                        }
                    }
                }
                loading.set(false);
            });
        }
    };

    view! {
            <div class="flex flex-col w-full mt-10 sm:mt-12">
            <div class="mb-6">
                <div class="flex flex-col sm:flex-row items-start sm:items-center gap-4 mb-6">
                    <div class="flex flex-col sm:flex-row items-start sm:items-center gap-3 sm:ml-auto">
                        <Show when=move || rule_limit.get().is_some()>
                            <div class="text-sm" style="color: var(--text-secondary);">
                                {move || {
                                    if let Some(limit_info) = rule_limit.get() {
                                        format!("Rules: {}/{}", limit_info.current_count, limit_info.max_rules)
                                    } else {
                                        "".to_string()
                                    }
                                }}
                            </div>
                        </Show>
                        <button
                            class="px-4 py-2 rounded-lg font-medium transition-colors whitespace-nowrap shrink-0 self-start sm:self-center disabled:opacity-50 disabled:cursor-not-allowed"
                            style:background-color=move || {
                                if let Some(limit_info) = rule_limit.get() {
                                    if limit_info.current_count >= limit_info.max_rules {
                                        "var(--bg-disabled, #6b7280)"
                                    } else {
                                        "var(--accent-primary)"
                                    }
                                } else {
                                    "var(--accent-primary)"
                                }
                            }
                            style:color="var(--text-on-accent)"
                            style:width="fit-content"
                            style:max-width="none"
                            style:flex="0 0 auto"
                            disabled=move || {
                                if let Some(limit_info) = rule_limit.get() {
                                    limit_info.current_count >= limit_info.max_rules
                                } else {
                                    false
                                }
                            }
                            on:click=move |_| {
                                if let Some(limit_info) = rule_limit.get() {
                                    if limit_info.current_count < limit_info.max_rules {
                                        editing_rule_id.set(None);
                                        show_create_modal.set(true);
                                    }
                                } else {
                                    editing_rule_id.set(None);
                                    show_create_modal.set(true);
                                }
                            }
                        >
                            "+ Create Rule"
                        </button>
                    </div>
                </div>

                <Show when=move || !loading.get()>
                    <div class="mb-6 p-6 rounded-lg border" style="background-color: var(--bg-card); border-color: var(--border-secondary);">
                        <h3 class="text-lg font-semibold mb-3" style="color: var(--text-primary); line-height: 1.5;">
                            "Quick Start Presets"
                        </h3>
                        <p class="text-sm mb-4" style="color: var(--text-secondary); line-height: 1.5;">
                            "Click a preset to quickly create a common automation rule:"
                        </p>
                        <div class="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 gap-3">
                            <button
                                class="p-4 rounded-lg border text-left transition-colors hover:opacity-90 w-full"
                                style="background-color: var(--bg-secondary); border-color: var(--border-secondary); color: var(--text-primary); display: flex; flex-direction: column; align-items: flex-start;"
                                on:click=move |_| create_preset_rule("Delete Inactive Torrents".to_string(), 30, "Inactive".to_string(), "GreaterThan".to_string(), 0.0, "Delete".to_string())
                            >
                                <div class="font-semibold mb-1.5" style="line-height: 1.5;">"Delete Inactive"</div>
                                <div class="text-xs" style="color: var(--text-secondary); line-height: 1.5;">
                                    "Removes inactive torrents every 30 min"
                                </div>
                            </button>
                            <button
                                class="p-4 rounded-lg border text-left transition-colors hover:opacity-90 w-full"
                                style="background-color: var(--bg-secondary); border-color: var(--border-secondary); color: var(--text-primary); display: flex; flex-direction: column; align-items: flex-start;"
                                on:click=move |_| create_preset_rule("Delete Stalled Torrents".to_string(), 30, "StalledTime".to_string(), "GreaterThan".to_string(), 1.0, "Delete".to_string())
                            >
                                <div class="font-semibold mb-1.5" style="line-height: 1.5;">"Delete Stalled"</div>
                                <div class="text-xs" style="color: var(--text-secondary); line-height: 1.5;">
                                    "Removes stalled torrents (>1h) every 30 min"
                                </div>
                            </button>
                            <button
                                class="p-4 rounded-lg border text-left transition-colors hover:opacity-90 w-full"
                                style="background-color: var(--bg-secondary); border-color: var(--border-secondary); color: var(--text-primary); display: flex; flex-direction: column; align-items: flex-start;"
                                on:click=move |_| create_preset_rule("Stop Seeding Low Ratio".to_string(), 30, "SeedingRatio".to_string(), "GreaterThan".to_string(), 1.0, "StopSeeding".to_string())
                            >
                                <div class="font-semibold mb-1.5" style="line-height: 1.5;">"Stop Low Ratio"</div>
                                <div class="text-xs" style="color: var(--text-secondary); line-height: 1.5;">
                                    "Stops seeding when ratio > 1.0"
                                </div>
                            </button>
                            <button
                                class="p-4 rounded-lg border text-left transition-colors hover:opacity-90 w-full"
                                style="background-color: var(--bg-secondary); border-color: var(--border-secondary); color: var(--text-primary); display: flex; flex-direction: column; align-items: flex-start;"
                                on:click=move |_| create_preset_rule("Stop After Seeding Time".to_string(), 60, "SeedingTime".to_string(), "GreaterThan".to_string(), 48.0, "StopSeeding".to_string())
                            >
                                <div class="font-semibold mb-1.5" style="line-height: 1.5;">"Stop After 48h"</div>
                                <div class="text-xs" style="color: var(--text-secondary); line-height: 1.5;">
                                    "Stops seeding after 48 hours"
                                </div>
                            </button>
                            <button
                                class="p-4 rounded-lg border text-left transition-colors hover:opacity-90 w-full"
                                style="background-color: var(--bg-secondary); border-color: var(--border-secondary); color: var(--text-primary); display: flex; flex-direction: column; align-items: flex-start;"
                                on:click=move |_| create_preset_rule("Delete Old Torrents".to_string(), 60, "Age".to_string(), "GreaterThan".to_string(), 720.0, "Delete".to_string())
                            >
                                <div class="font-semibold mb-1.5" style="line-height: 1.5;">"Delete Old (30 days)"</div>
                                <div class="text-xs" style="color: var(--text-secondary); line-height: 1.5;">
                                    "Deletes torrents older than 30 days"
                                </div>
                            </button>
                            <button
                                class="p-4 rounded-lg border text-left transition-colors hover:opacity-90 w-full"
                                style="background-color: var(--bg-secondary); border-color: var(--border-secondary); color: var(--text-primary); display: flex; flex-direction: column; align-items: flex-start;"
                                on:click=move |_| create_preset_rule("Delete Completed".to_string(), 60, "DownloadFinished".to_string(), "Equal".to_string(), 1.0, "Delete".to_string())
                            >
                                <div class="font-semibold mb-1.5" style="line-height: 1.5;">"Delete Completed"</div>
                                <div class="text-xs" style="color: var(--text-secondary); line-height: 1.5;">
                                    "Deletes finished downloads"
                                </div>
                            </button>
                        </div>
                    </div>
                </Show>
                
                <Show when=move || error.get().is_some()>
                    <div class="mb-4 p-4 rounded-lg" style="background-color: var(--bg-error); color: var(--text-error); line-height: 1.5;">
                        {move || error.get().unwrap_or_default()}
                    </div>
                </Show>

                <Show when=move || loading.get()>
                    <div class="text-center py-8" style="color: var(--text-secondary); line-height: 1.5;">
                        "Loading automations..."
                    </div>
                </Show>


                <Show when=move || !loading.get() && !rules.get().is_empty()>
                    <div class="space-y-4">
                        <div class="flex items-center justify-between p-4 rounded-lg border mb-4" style="background-color: var(--bg-card); border-color: var(--border-secondary);">
                                <div class="flex items-center gap-4">
                                    <div class="flex items-center gap-3">
                                        <input
                                            type="checkbox"
                                            class="w-5 h-5 rounded cursor-pointer"
                                            style="accent-color: var(--accent-primary);"
                                            checked=move || {
                                                let rules_list = rules.get();
                                                !rules_list.is_empty() && rules_list.iter().all(|rule| {
                                                    rule.id.map(|id| selected_rules.get().contains(&id)).unwrap_or(false)
                                                })
                                            }
                                            on:change=move |ev| {
                                                let checked = event_target_checked(&ev);
                                                let rules_list = rules.get();
                                                selected_rules.update(|set| {
                                                    if checked {
                                                        for rule in &rules_list {
                                                            if let Some(id) = rule.id {
                                                                set.insert(id);
                                                            }
                                                        }
                                                    } else {
                                                        for rule in &rules_list {
                                                            if let Some(id) = rule.id {
                                                                set.remove(&id);
                                                            }
                                                        }
                                                    }
                                                });
                                            }
                                        />
                                        <span class="text-sm font-medium" style="color: var(--text-primary);">
                                            "Select All"
                                        </span>
                                    </div>
                                    <Show when=move || !selected_rules.get().is_empty()>
                                        <div class="text-sm" style="color: var(--text-secondary);">
                                            {move || format!("{} rule(s) selected", selected_rules.get().len())}
                                        </div>
                                    </Show>
                                </div>
                                <Show when=move || !selected_rules.get().is_empty()>
                                    <button
                                    class="px-4 py-2 text-sm font-medium rounded-lg transition-all border"
                                    style="background-color: var(--bg-error, #ef4444); color: var(--text-on-accent); border-color: var(--bg-error, #ef4444); line-height: 1.5;"
                                    on:click=move |_| {
                                        let selected_ids: Vec<i64> = selected_rules.get().into_iter().collect();
                                        if !selected_ids.is_empty() {
                                            let selected_ids_clone = selected_ids.clone();
                                            show_confirmation(
                                                confirmation_state.clone(),
                                                "Delete Selected Rules".to_string(),
                                                format!("Are you sure you want to delete {} rule(s)? This action cannot be undone.", selected_ids_clone.len()),
                                                move || {
                                                    let ids_to_delete = selected_ids_clone.clone();
                                                    #[cfg(feature = "hydrate")]
                                                    {
                                                        spawn_local(async move {
                                                            if let Some(window) = web_sys::window() {
                                                                if let Ok(Some(storage)) = window.local_storage() {
                                                                    if let Ok(Some(api_key)) = storage.get_item("api_key") {
                                                                        if !api_key.is_empty() {
                                                                            let headers = web_sys::Headers::new().unwrap();
                                                                            headers.set("Authorization", &format!("Bearer {}", api_key)).unwrap();
                                                                            headers.set("Content-Type", "application/json").unwrap();

                                                                            let body = serde_json::json!({ "rule_ids": ids_to_delete });
                                                                            let body_str = serde_json::to_string(&body).unwrap();
                                                                            let body_js = wasm_bindgen::JsValue::from_str(&body_str);

                                                                            let init = {
                                                                                let mut i = web_sys::RequestInit::new();
                                                                                i.set_method("POST");
                                                                                i.set_headers(&headers);
                                                                                i.set_body(&body_js);
                                                                                i
                                                                            };

                                                                            let promise = window.fetch_with_str_and_init("/api/automation/rules/bulk-delete", &init);
                                                                            let future = wasm_bindgen_futures::JsFuture::from(promise);

                                                                            if let Ok(response) = future.await {
                                                                                let resp: web_sys::Response = response.dyn_into().unwrap();
                                                                                if resp.status() == 200 {
                                                                                    let text_promise = resp.text().unwrap();
                                                                                    let text_future = wasm_bindgen_futures::JsFuture::from(text_promise);
                                                                                    if let Ok(text_value) = text_future.await {
                                                                                        if let Some(text) = text_value.as_string() {
                                                                                            if let Ok(api_response) = serde_json::from_str::<ApiResponse<serde_json::Value>>(&text) {
                                                                                                if api_response.success {
                                                                                                    selected_rules.set(std::collections::HashSet::new());
                                                                                                    fetch_rules();
                                                                                                    fetch_rule_limit();
                                                                                                } else {
                                                                                                    error.set(Some(api_response.error.unwrap_or_else(|| "Failed to delete rules".to_string())));
                                                                                                }
                                                                                            } else {
                                                                                                error.set(Some("Failed to parse response".to_string()));
                                                                                            }
                                                                                        } else {
                                                                                            error.set(Some("Invalid response format".to_string()));
                                                                                        }
                                                                                    } else {
                                                                                        error.set(Some("Failed to read response".to_string()));
                                                                                    }
                                                                                } else {
                                                                                    error.set(Some(format!("Failed to delete rules: {}", resp.status())));
                                                                                }
                                                                            } else {
                                                                                error.set(Some("Network request failed".to_string()));
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        });
                                                    }
                                                },
                                                ConfirmationVariant::Danger,
                                                Some("Delete".to_string()),
                                                None
                                            );
                                        }
                                    }
                                >
                                        "Delete Selected"
                                    </button>
                                </Show>
                        </div>
                        {move || {
                            rules.get().into_iter().map(|rule| {
                                let rule_id = rule.id;
                                let rule_name_for_delete = rule.name.clone();
                                let logs_for_rule = move || {
                                    if let Some(id) = rule_id {
                                        rule_logs.get().get(&id).cloned().unwrap_or_default()
                                    } else {
                                        Vec::new()
                                    }
                                };
                                let format_timestamp = move |timestamp_str: String| -> String {
                                    #[cfg(feature = "hydrate")]
                                    {
                                        if let Ok(parsed) = chrono::DateTime::parse_from_rfc3339(&timestamp_str) {
                                            let utc_timestamp = parsed.timestamp_millis();
                                            if let Some(_window) = web_sys::window() {
                                                let js_date = js_sys::Date::new(&wasm_bindgen::JsValue::from_f64(utc_timestamp as f64));
                                                let month = js_date.get_month() as u32;
                                                let day = js_date.get_date() as u32;
                                                let year = js_date.get_full_year() as u32;
                                                let hours = js_date.get_hours() as u32;
                                                let minutes = js_date.get_minutes() as u32;
                                                let am_pm = if hours < 12 { "AM" } else { "PM" };
                                                let display_hours = if hours == 0 { 12 } else if hours > 12 { hours - 12 } else { hours };
                                                let month_names = ["January", "February", "March", "April", "May", "June", "July", "August", "September", "October", "November", "December"];
                                                format!("{} {}, {} at {}:{:02} {}", month_names[month as usize], day, year, display_hours, minutes, am_pm)
                                            } else {
                                                timestamp_str
                                            }
                                        } else {
                                            timestamp_str
                                        }
                                    }
                                    #[cfg(not(feature = "hydrate"))]
                                    {
                                        timestamp_str
                                    }
                                };
                                let last_run = move || {
                                    logs_for_rule().first().and_then(|log| log.executed_at.clone())
                                };
                                let run_count = move || logs_for_rule().len();
                                let last_result = move || {
                                    logs_for_rule().first().map(|log| {
                                        let total = log.total_items.unwrap_or(log.items_processed);
                                        (log.success, log.items_processed, total, log.error_message.clone(), log.processed_items.clone(), log.partial.unwrap_or(false))
                                    })
                                };
                                let expanded_items_clone = expanded_items.clone();
                                let is_expanded = move || {
                                    if let Some(id) = rule_id {
                                        expanded_items_clone.get().contains(&id)
                                    } else {
                                        false
                                    }
                                };
                                let expanded_items_toggle = expanded_items.clone();
                                let toggle_expanded = move |_| {
                                    if let Some(id) = rule_id {
                                        expanded_items_toggle.update(|set| {
                                            if set.contains(&id) {
                                                set.remove(&id);
                                            } else {
                                                set.insert(id);
                                            }
                                        });
                                    }
                                };
                                let next_run = move || {
                                    if let Some(id) = rule_id {
                                        next_run_times.get().get(&id).cloned().flatten()
                                    } else {
                                        None
                                    }
                                };
                                let rule_id_for_checkbox = rule_id;
                                let is_selected = move || {
                                    if let Some(id) = rule_id_for_checkbox {
                                        selected_rules.get().contains(&id)
                                    } else {
                                        false
                                    }
                                };
                                view! {
                                    <div 
                                        class="p-4 rounded-lg border transition-colors"
                                        style="background-color: var(--bg-card); border-color: var(--border-secondary);"
                                    >
                                        <div class="flex items-center justify-between">
                                            <div class="flex-1">
                                                <div class="flex items-center space-x-3 mb-2">
                                                    <input
                                                        type="checkbox"
                                                        class="w-5 h-5 rounded cursor-pointer"
                                                        style="accent-color: var(--accent-primary);"
                                                        checked=move || is_selected()
                                                        on:change=move |ev| {
                                                            if let Some(id) = rule_id_for_checkbox {
                                                                let checked = event_target_checked(&ev);
                                                                selected_rules.update(|set| {
                                                                    if checked {
                                                                        set.insert(id);
                                                                    } else {
                                                                        set.remove(&id);
                                                                    }
                                                                });
                                                            }
                                                        }
                                                    />
                                                    <h3 class="text-lg font-semibold" style="color: var(--text-primary); line-height: 1.5;">
                                                        {rule.name.clone()}
                                                    </h3>
                                                    <span 
                                                        class="px-2 py-1 text-xs rounded"
                                                        style={move || {
                                                            if rule.enabled {
                                                                "background-color: var(--bg-success); color: var(--text-success);"
                                                            } else {
                                                                "background-color: var(--bg-secondary); color: var(--text-secondary);"
                                                            }
                                                        }}
                                                    >
                                                        {if rule.enabled { "Enabled" } else { "Disabled" }}
                                                    </span>
                                                </div>
                                                <div class="space-y-1 mt-2">
                                                    <p class="text-xs" style="color: var(--text-secondary); line-height: 1.5;">
                                                        {move || {
                                                            let count = run_count();
                                                            if count > 0 {
                                                                format!("Ran {} time{}", count, if count == 1 { "" } else { "s" })
                                                            } else {
                                                                "Never run".to_string()
                                                            }
                                                        }}
                                                    </p>
                                                    <Show when=move || last_run().is_some()>
                                                        <p class="text-xs" style="color: var(--text-secondary); line-height: 1.5;">
                                                            {move || {
                                                                if let Some(time) = last_run() {
                                                                    format!("Last run: {}", format_timestamp(time))
                                                                } else {
                                                                    String::new()
                                                                }
                                                            }}
                                                        </p>
                                                    </Show>
                                                    <Show when=move || rule.enabled && next_run().is_some()>
                                                        <p class="text-xs" style="color: var(--text-primary); line-height: 1.5;">
                                                            {move || {
                                                                if let Some(time) = next_run() {
                                                                    format!("Next run: {}", format_timestamp(time))
                                                                } else {
                                                                    String::new()
                                                                }
                                                            }}
                                                        </p>
                                                    </Show>
                                                    <Show when=move || last_result().is_some()>
                                                        <div class="space-y-2">
                                                            <p class="text-sm font-medium" style={move || {
                                                                if let Some((success, _, _, _, _, partial)) = last_result() {
                                                                    if success && !partial {
                                                                        "color: var(--text-success); line-height: 1.5;"
                                                                    } else if partial {
                                                                        "color: var(--text-warning); line-height: 1.5;"
                                                                    } else {
                                                                        "color: var(--text-error); line-height: 1.5;"
                                                                    }
                                                                } else {
                                                                    "color: var(--text-secondary); line-height: 1.5;"
                                                                }
                                                            }}>
                                                                {move || {
                                                                    if let Some((success, items_processed, total_items, error, processed_items, partial)) = last_result() {
                                                                        if partial {
                                                                            format!("{}/{} items processed (partial)", items_processed, total_items)
                                                                        } else if success {
                                                                            if total_items > 0 {
                                                                                format!("{}/{} items processed", items_processed, total_items)
                                                                            } else {
                                                                                "No items matched conditions".to_string()
                                                                            }
                                                                        } else {
                                                                            format!("Failed: {}/{} processed - {}", items_processed, total_items, error.unwrap_or_else(|| "Unknown error".to_string()))
                                                                        }
                                                                    } else {
                                                                        String::new()
                                                                    }
                                                                }}
                                                            </p>
                                                            <Show when=move || {
                                                                if let Some((_, items_processed, _, _, processed_items, _)) = last_result() {
                                                                    items_processed > 0 && processed_items.is_some() && !processed_items.as_ref().unwrap().is_empty()
                                                                } else {
                                                                    false
                                                                }
                                                            }>
                                                                <div class="mt-1">
                                                                    <button
                                                                        class="text-xs cursor-pointer hover:opacity-80 transition-opacity font-medium flex items-center gap-1"
                                                                        style="color: var(--text-secondary); background: none; border: none; padding: 0;"
                                                                        on:click=toggle_expanded
                                                                    >
                                                                        <span>{move || {
                                                                            if is_expanded() {
                                                                                "▼ Hide Items"
                                                                            } else {
                                                                                "▶ Show Items"
                                                                            }
                                                                        }}</span>
                                                                        {move || {
                                                                            if let Some((_, items_processed, total_items, _, _, _)) = last_result() {
                                                                                format!("({})", items_processed)
                                                                            } else {
                                                                                String::new()
                                                                            }
                                                                        }}
                                                                    </button>
                                                                    <Show when=move || is_expanded()>
                                                                        <div class="mt-2 p-2 rounded border" style="background-color: var(--bg-tertiary); border-color: var(--border-secondary);">
                                                                            <div class="space-y-1 max-h-64 overflow-y-auto">
                                                                                <For
                                                                                    each=move || {
                                                                                        if let Some((_, _, _, _, Some(items), _)) = last_result() {
                                                                                            items.iter().cloned().collect::<Vec<_>>()
                                                                                        } else {
                                                                                            Vec::new()
                                                                                        }
                                                                                    }
                                                                                    key=|item| item.id
                                                                                    children=move |item: ProcessedItem| {
                                                                                        let item_name = item.name.clone();
                                                                                        let item_name_title = item.name.clone();
                                                                                        let item_action = item.action.clone();
                                                                                        let item_success = item.success;
                                                                                        let item_error = item.error.clone().unwrap_or_default();
                                                                                        view! {
                                                                                            <div class="flex items-center justify-between text-xs" style="color: var(--text-secondary);">
                                                                                                <span class="truncate flex-1 mr-2" title={item_name_title}>
                                                                                                    {item_name}
                                                                                                </span>
                                                                                                <div class="flex items-center gap-2 shrink-0">
                                                                                                    <span class="px-1.5 py-0.5 rounded text-xs" style={
                                                                                                        if item_success {
                                                                                                            "background-color: var(--bg-success); color: var(--text-success);"
                                                                                                        } else {
                                                                                                            "background-color: var(--bg-error); color: var(--text-error);"
                                                                                                        }
                                                                                                    }>
                                                                                                        {item_action}
                                                                                                    </span>
                                                                                                    <Show when=move || !item_success>
                                                                                                        <span class="text-xs" style="color: var(--text-error);" title={item_error.clone()}>
                                                                                                            "⚠"
                                                                                                        </span>
                                                                                                    </Show>
                                                                                                </div>
                                                                                            </div>
                                                                                        }
                                                                                    }
                                                                                />
                                                                            </div>
                                                                        </div>
                                                                    </Show>
                                                                </div>
                                                            </Show>
                                                        </div>
                                                    </Show>
                                                </div>
                                            </div>
                                            <div class="flex items-center gap-3 flex-wrap">
                                                <button
                                                    class="px-4 py-2 text-sm font-medium rounded-lg transition-all shrink-0 border"
                                                    style={move || {
                                                        if rule.enabled {
                                                            "background-color: var(--bg-secondary); color: var(--text-secondary); border-color: var(--border-secondary); line-height: 1.5;"
                                                        } else {
                                                            "background-color: var(--bg-success); color: var(--text-success); border-color: var(--border-success); line-height: 1.5;"
                                                        }
                                                    }}
                                                    on:click=move |_| {
                                                        if let Some(id) = rule_id {
                                                            toggle_rule(id, rule.enabled, rule.clone());
                                                        }
                                                    }
                                                >
                                                    {if rule.enabled { "Disable" } else { "Enable" }}
                                                </button>
                                                <button
                                                    class="px-4 py-2 text-sm font-medium rounded-lg transition-all shrink-0 border hover:opacity-90"
                                                    style="background-color: var(--bg-secondary); color: var(--text-primary); border-color: var(--border-secondary); line-height: 1.5;"
                                                    on:click=move |_| {
                                                        if let Some(id) = rule_id {
                                                            editing_rule_id.set(Some(id));
                                                            show_create_modal.set(true);
                                                        }
                                                    }
                                                >
                                                    "Edit"
                                                </button>
                                                <button
                                                    class="px-4 py-2 text-sm font-medium rounded-lg transition-all shrink-0 border hover:opacity-90 flex items-center gap-2"
                                                    style="background-color: var(--accent-primary); color: var(--text-on-accent); border-color: var(--accent-primary); line-height: 1.5;"
                                                    disabled=move || {
                                                        if let Some(id) = rule_id {
                                                            running_rules.get().contains(&id)
                                                        } else {
                                                            false
                                                        }
                                                    }
                                                    on:click=move |_| {
                                                        if let Some(id) = rule_id {
                                                            let rule_id_clone = id;
                                                            let running_rules_clone = running_rules.clone();
                                                            let error_clone = error.clone();
                                                            let rules_clone = rules.clone();
                                                            let loading_clone = loading.clone();
                                                            let rule_limit_clone = rule_limit.clone();
                                                            #[cfg(feature = "hydrate")]
                                                            {
                                                                running_rules_clone.update(|set| {
                                                                    set.insert(rule_id_clone);
                                                                });
                                                                spawn_local(async move {
                                                                    if let Some(window) = web_sys::window() {
                                                                        if let Ok(Some(storage)) = window.local_storage() {
                                                                            if let Ok(Some(api_key)) = storage.get_item("api_key") {
                                                                                if !api_key.is_empty() {
                                                                                    let url = format!("/api/automation/rules/{}/run", rule_id_clone);
                                                                                    let headers = web_sys::Headers::new().unwrap();
                                                                                    headers.set("Authorization", &format!("Bearer {}", api_key)).unwrap();
                                                                                    headers.set("Content-Type", "application/json").unwrap();

                                                                                    let init = {
                                                                                        let mut i = web_sys::RequestInit::new();
                                                                                        i.set_method("POST");
                                                                                        i.set_headers(&headers);
                                                                                        i
                                                                                    };

                                                                                    let promise = window.fetch_with_str_and_init(&url, &init);
                                                                                    let future = wasm_bindgen_futures::JsFuture::from(promise);

                                                                                    if let Ok(response) = future.await {
                                                                                        let resp: web_sys::Response = response.dyn_into().unwrap();
                                                                                        let text_promise = resp.text().unwrap();
                                                                                        let text_future = wasm_bindgen_futures::JsFuture::from(text_promise);
                                                                                        if let Ok(text_value) = text_future.await {
                                                                                            if let Some(text) = text_value.as_string() {
                                                                                                if resp.status() == 200 {
                                                                                                    if let Ok(result) = serde_json::from_str::<serde_json::Value>(&text) {
                                                                                                        if let Some(data) = result.get("data") {
                                                                                                            if let Some(items_processed) = data.get("items_processed") {
                                                                                                                let items_count = items_processed.as_i64().unwrap_or(0);
                                                                                                                let total_items = data.get("total_items").and_then(|v| v.as_i64()).unwrap_or(items_count);
                                                                                                                let partial = data.get("partial").and_then(|v| v.as_bool()).unwrap_or(false);
                                                                                                                if let Some(success) = data.get("success") {
                                                                                                                    if success.as_bool().unwrap_or(false) && !partial {
                                                                                                                        if total_items > 0 {
                                                                                                                            error_clone.set(Some(format!("Rule executed successfully: {}/{} items processed", items_count, total_items)));
                                                                                                                        } else {
                                                                                                                            error_clone.set(Some("Rule executed: No items matched conditions".to_string()));
                                                                                                                        }
                                                                                                                    } else if partial {
                                                                                                                        error_clone.set(Some(format!("Rule partially completed: {}/{} items processed", items_count, total_items)));
                                                                                                                    } else {
                                                                                                                        if let Some(err_msg) = data.get("error_message") {
                                                                                                                            error_clone.set(Some(format!("Rule execution had errors: {}", err_msg.as_str().unwrap_or("Unknown error"))));
                                                                                                                        } else {
                                                                                                                            error_clone.set(Some("Rule execution failed".to_string()));
                                                                                                                        }
                                                                                                                    }
                                                                                                                }
                                                                                                            }
                                                                                                        }
                                                                                                    }
                                                                                                    loading_clone.set(true);
                                                                                                    error_clone.set(None);
                                                                                                    let fetch_url = "/api/automation/rules";
                                                                                                    let fetch_headers = web_sys::Headers::new().unwrap();
                                                                                                    fetch_headers.set("Authorization", &format!("Bearer {}", api_key)).unwrap();
                                                                                                    let fetch_init = {
                                                                                                        let mut i = web_sys::RequestInit::new();
                                                                                                        i.set_method("GET");
                                                                                                        i.set_headers(&fetch_headers);
                                                                                                        i
                                                                                                    };
                                                                                                    let fetch_promise = window.fetch_with_str_and_init(fetch_url, &fetch_init);
                                                                                                    let fetch_future = wasm_bindgen_futures::JsFuture::from(fetch_promise);
                                                                                                    if let Ok(fetch_response) = fetch_future.await {
                                                                                                        let fetch_resp: web_sys::Response = fetch_response.dyn_into().unwrap();
                                                                                                        if fetch_resp.status() == 200 {
                                                                                                            let fetch_text_promise = fetch_resp.text().unwrap();
                                                                                                            let fetch_text_future = wasm_bindgen_futures::JsFuture::from(fetch_text_promise);
                                                                                                            if let Ok(fetch_text_value) = fetch_text_future.await {
                                                                                                                if let Some(fetch_text) = fetch_text_value.as_string() {
                                                                                                                    if let Ok(fetch_api_response) = serde_json::from_str::<ApiResponse<Vec<AutomationRule>>>(&fetch_text) {
                                                                                                                        if let Some(fetch_data) = fetch_api_response.data {
                                                                                                                            rules_clone.set(fetch_data);
                                                                                                                        }
                                                                                                                    }
                                                                                                                }
                                                                                                            }
                                                                                                        }
                                                                                                    }
                                                                                                    let limit_url = "/api/automation/rules/limit";
                                                                                                    let limit_headers = web_sys::Headers::new().unwrap();
                                                                                                    limit_headers.set("Authorization", &format!("Bearer {}", api_key)).unwrap();
                                                                                                    let limit_init = {
                                                                                                        let mut i = web_sys::RequestInit::new();
                                                                                                        i.set_method("GET");
                                                                                                        i.set_headers(&limit_headers);
                                                                                                        i
                                                                                                    };
                                                                                                    let limit_promise = window.fetch_with_str_and_init(limit_url, &limit_init);
                                                                                                    let limit_future = wasm_bindgen_futures::JsFuture::from(limit_promise);
                                                                                                    if let Ok(limit_response) = limit_future.await {
                                                                                                        let limit_resp: web_sys::Response = limit_response.dyn_into().unwrap();
                                                                                                        if limit_resp.status() == 200 {
                                                                                                            let limit_text_promise = limit_resp.text().unwrap();
                                                                                                            let limit_text_future = wasm_bindgen_futures::JsFuture::from(limit_text_promise);
                                                                                                            if let Ok(limit_text_value) = limit_text_future.await {
                                                                                                                if let Some(limit_text) = limit_text_value.as_string() {
                                                                                                                    if let Ok(limit_api_response) = serde_json::from_str::<ApiResponse<RuleLimitInfo>>(&limit_text) {
                                                                                                                        if let Some(limit_data) = limit_api_response.data {
                                                                                                                            rule_limit_clone.set(Some(limit_data));
                                                                                                                        }
                                                                                                                    }
                                                                                                                }
                                                                                                            }
                                                                                                        }
                                                                                                    }
                                                                                                    loading_clone.set(false);
                                                                                                } else {
                                                                                                    error_clone.set(Some(format!("Failed to run rule: {}", resp.status())));
                                                                                                }
                                                                                            }
                                                                                        }
                                                                                    } else {
                                                                                        error_clone.set(Some("Network request failed".to_string()));
                                                                                    }
                                                                                } else {
                                                                                    error_clone.set(Some("No API key found".to_string()));
                                                                                }
                                                                            } else {
                                                                                error_clone.set(Some("No API key found".to_string()));
                                                                            }
                                                                        } else {
                                                                            error_clone.set(Some("Storage not available".to_string()));
                                                                        }
                                                                    } else {
                                                                        error_clone.set(Some("Window not available".to_string()));
                                                                    }
                                                                    running_rules_clone.update(|set| {
                                                                        set.remove(&rule_id_clone);
                                                                    });
                                                                });
                                                            }
                                                        }
                                                    }
                                                >
                                                    <Show when=move || {
                                                        if let Some(id) = rule_id {
                                                            running_rules.get().contains(&id)
                                                        } else {
                                                            false
                                                        }
                                                    }>
                                                        <LoadingSpinner size=SpinnerSize::Small variant=SpinnerVariant::Default/>
                                                    </Show>
                                                    {move || {
                                                        if let Some(id) = rule_id {
                                                            if running_rules.get().contains(&id) {
                                                                "Running..."
                                                            } else {
                                                                "Run Now"
                                                            }
                                                        } else {
                                                            "Run Now"
                                                        }
                                                    }}
                                                </button>
                                                <button
                                                    class="px-4 py-2 text-sm font-medium rounded-lg transition-all shrink-0 border hover:opacity-90"
                                                    style="background-color: var(--bg-error); color: var(--text-error); border-color: var(--border-error); line-height: 1.5;"
                                                    on:click={
                                                        let rule_name = rule_name_for_delete.clone();
                                                        let confirmation_state_clone = confirmation_state.clone();
                                                        let delete_rule_clone = delete_rule.clone();
                                                        move |_| {
                                                            if let Some(id) = rule_id {
                                                                show_confirmation(
                                                                    confirmation_state_clone.clone(),
                                                                    "Delete Automation Rule".to_string(),
                                                                    format!("Are you sure you want to delete the rule \"{}\"? This action cannot be undone.", rule_name),
                                                                    move || {
                                                                        delete_rule_clone(id);
                                                                    },
                                                                    ConfirmationVariant::Danger,
                                                                    Some("Delete".to_string()),
                                                                    Some("Cancel".to_string()),
                                                                );
                                                            }
                                                        }
                                                    }
                                                >
                                                    "Delete"
                                                </button>
                                            </div>
                                        </div>
                                    </div>
                                }
                            }).collect::<Vec<_>>()
                        }}
                    </div>
                </Show>

                <RuleModal 
                    show=show_create_modal
                    editing_rule_id=editing_rule_id
                    rules=rules
                    preset_data=preset_data
                    on_save=move || {
                        show_create_modal.set(false);
                        editing_rule_id.set(None);
                        preset_data.set(None);
                        fetch_rules();
                        fetch_rule_limit();
                    }
                />
            </div>
        </div>
    }
}

#[component]
fn RuleModal(
    show: RwSignal<bool>,
    editing_rule_id: RwSignal<Option<i64>>,
    rules: RwSignal<Vec<AutomationRule>>,
    preset_data: RwSignal<Option<(String, u32, String, String, f64, String)>>,
    on_save: impl Fn() + 'static + Send + Sync,
) -> impl IntoView {
    let rule_name = RwSignal::new(String::new());
    let rule_enabled = RwSignal::new(true);
    let trigger_type = RwSignal::new("interval".to_string());
    let cron_expression = RwSignal::new("0 * * * *".to_string());
    let interval_minutes = RwSignal::new(60u32);
    let condition_type = RwSignal::new("SeedingTime".to_string());
    let condition_operator = RwSignal::new("GreaterThan".to_string());
    let condition_value = RwSignal::new(24.0f64);
    let action_type = RwSignal::new("StopSeeding".to_string());
    let saving = RwSignal::new(false);
    let save_error = RwSignal::new(None::<String>);

    Effect::new(move |_| {
        if show.get() {
            if let Some(rule_id) = editing_rule_id.get() {
                if let Some(rule) = rules.get().iter().find(|r| r.id == Some(rule_id)) {
                    rule_name.set(rule.name.clone());
                    rule_enabled.set(rule.enabled);
                    
                    if let Ok(trigger) = serde_json::from_value::<serde_json::Map<String, serde_json::Value>>(rule.trigger_config.clone()) {
                        if trigger.contains_key("Cron") {
                            trigger_type.set("cron".to_string());
                            if let Some(expr) = trigger.get("Cron").and_then(|v| v.get("expression")).and_then(|v| v.as_str()) {
                                cron_expression.set(expr.to_string());
                            }
                        } else if trigger.contains_key("Interval") {
                            trigger_type.set("interval".to_string());
                            if let Some(mins) = trigger.get("Interval").and_then(|v| v.get("minutes")).and_then(|v| v.as_u64()) {
                                interval_minutes.set(mins as u32);
                            }
                        }
                    }
                    
                    if let Some(condition) = rule.conditions.first().and_then(|c| serde_json::from_value::<serde_json::Map<String, serde_json::Value>>(c.clone()).ok()) {
                        if let Some(ct) = condition.get("type").and_then(|v| v.as_str()) {
                            condition_type.set(ct.to_string());
                        }
                        if let Some(op) = condition.get("operator").and_then(|v| v.as_str()) {
                            condition_operator.set(op.to_string());
                        }
                        if let Some(val) = condition.get("value").and_then(|v| v.as_f64()) {
                            condition_value.set(val);
                        }
                    }
                    
                    if let Ok(action) = serde_json::from_value::<serde_json::Map<String, serde_json::Value>>(rule.action_config.clone()) {
                        if let Some(at) = action.get("action_type").and_then(|v| v.as_str()) {
                            action_type.set(at.to_string());
                        }
                    }
                }
            } else if let Some((name, minutes, cond_type, cond_op, cond_val, act_type)) = preset_data.get() {
                // Apply preset values
                rule_name.set(name);
                rule_enabled.set(true);
                trigger_type.set("interval".to_string());
                interval_minutes.set(minutes);
                condition_type.set(cond_type);
                condition_operator.set(cond_op);
                condition_value.set(cond_val);
                action_type.set(act_type);
                preset_data.set(None); // Clear preset after applying
            } else {
                rule_name.set(String::new());
                rule_enabled.set(true);
                trigger_type.set("interval".to_string());
                cron_expression.set("0 * * * *".to_string());
                interval_minutes.set(60);
                condition_type.set("SeedingTime".to_string());
                condition_operator.set("GreaterThan".to_string());
                condition_value.set(24.0);
                action_type.set("StopSeeding".to_string());
            }
            save_error.set(None);
        }
    });

    let on_save_arc = Arc::new(on_save);
    let on_save_arc_clone = on_save_arc.clone();
    let rule_name_save = rule_name.clone();
    let rule_enabled_save = rule_enabled.clone();
    let trigger_type_save = trigger_type.clone();
    let cron_expression_save = cron_expression.clone();
    let interval_minutes_save = interval_minutes.clone();
    let condition_type_save = condition_type.clone();
    let condition_operator_save = condition_operator.clone();
    let condition_value_save = condition_value.clone();
    let action_type_save = action_type.clone();
    let editing_rule_id_save = editing_rule_id.clone();
    let saving_save = saving.clone();
    let save_error_save = save_error.clone();

    view! {
        <Show
            when=move || show.get()
        >
            <div 
                style="position: fixed !important; top: 0 !important; left: 0 !important; right: 0 !important; bottom: 0 !important; width: 100vw !important; height: 100vh !important; background-color: rgba(0, 0, 0, 0.75) !important; backdrop-filter: blur(4px) !important; z-index: 2147483647 !important; display: flex !important; align-items: center !important; justify-content: center !important; padding: 0.5rem !important; overflow-y: auto !important; box-sizing: border-box !important;"
                on:click=move |_| {
                    if !saving.get() {
                        show.set(false);
                    }
                }
            >
                <div 
                    class="rounded-xl border shadow-2xl modal-content"
                    style="background-color: var(--bg-card) !important; border-color: var(--border-secondary) !important; z-index: 2147483647 !important; position: relative !important; width: calc(100% - 1rem) !important; max-width: 48rem !important; max-height: calc(100vh - 1rem) !important; margin: auto !important; overflow: hidden !important; display: flex !important; flex-direction: column !important;"
                    on:click=|ev| ev.stop_propagation()
                >
                    <div class="flex-shrink-0 px-4 sm:px-6 pt-4 sm:pt-6 pb-3 sm:pb-4 border-b" style="border-color: var(--border-secondary);">
                        <div class="flex items-center justify-between">
                            <div>
                                <h3 class="text-xl sm:text-2xl font-bold mb-1" style="color: var(--text-primary); line-height: 1.3;">
                                    {move || if editing_rule_id.get().is_some() { "Edit Automation Rule" } else { "Create Automation Rule" }}
                                </h3>
                                <p class="text-xs sm:text-sm" style="color: var(--text-secondary); line-height: 1.5;">
                                    {move || if editing_rule_id.get().is_some() { "Modify your automation rule settings" } else { "Set up a new automation to manage your torrents" }}
                                </p>
                            </div>
                            <button
                                class="flex items-center justify-center w-7 h-7 sm:w-8 sm:h-8 rounded-lg transition-colors shrink-0 hover:bg-opacity-10"
                                style="color: var(--text-secondary); background-color: var(--bg-secondary);"
                                on:click=move |_| {
                                    if !saving.get() {
                                        show.set(false);
                                    }
                                }
                                title="Close"
                            >
                                <span class="text-xl leading-none">"×"</span>
                            </button>
                        </div>
                    </div>

                    <div class="flex-1 overflow-y-auto px-4 sm:px-6 md:px-8 py-4 sm:py-6 md:py-8" style="min-height: 0 !important;">
                        <Show when=move || save_error.get().is_some()>
                            <div class="mb-6 p-4 rounded-lg border" style="background-color: var(--bg-error); border-color: var(--border-error, #ef4444); color: var(--text-error); line-height: 1.5;">
                                <div class="flex items-start">
                                    <span class="text-lg mr-2">"⚠"</span>
                                    <div class="flex-1">
                                        <p class="font-medium mb-1">"Error"</p>
                                        <p class="text-sm">{move || save_error.get().unwrap_or_default()}</p>
                                    </div>
                                </div>
                            </div>
                        </Show>

                        <div class="space-y-8">
                            <div class="p-6">
                                <div class="mb-5">
                                    <h4 class="text-lg font-bold mb-2" style="color: var(--text-primary); line-height: 1.4;">
                                        "Basic Information"
                                    </h4>
                                    <div class="h-0.5 w-12 rounded-full" style="background-color: var(--accent-primary);"></div>
                                </div>
                                <div class="space-y-6">
                                    <div style="margin-bottom: 1.5rem;">
                                        <label class="block text-sm font-semibold mb-3" style="color: var(--text-primary); line-height: 1.5;">
                                            "Rule Name"
                                        </label>
                                        <input
                                            type="text"
                                            placeholder="e.g., Stop seeding after 48h"
                                            class="w-full px-4 py-3 rounded-lg border transition-all focus:outline-none focus:ring-2"
                                            style="background-color: var(--bg-tertiary); border-color: var(--border-secondary); color: var(--text-primary); font-size: 0.9375rem; focus:border-color: var(--accent-primary); focus:ring-color: var(--accent-primary); focus:ring-width: 2px;"
                                            value=move || rule_name.get()
                                            on:input=move |ev| rule_name.set(event_target_value(&ev))
                                            disabled=move || saving.get()
                                        />
                                        <p class="text-xs mt-2.5" style="color: var(--text-secondary); line-height: 1.6;">
                                            "Give your rule a descriptive name to easily identify it later"
                                        </p>
                                    </div>

                                    <div class="flex items-center p-4 rounded-lg border" style="background-color: var(--bg-tertiary); border-color: var(--border-secondary); margin-top: 1.5rem;">
                                        <input
                                            type="checkbox"
                                            id="rule-enabled"
                                            class="w-5 h-5 rounded cursor-pointer"
                                            style="accent-color: var(--accent-primary);"
                                            checked=move || rule_enabled.get()
                                            on:change=move |ev| rule_enabled.set(event_target_checked(&ev))
                                            disabled=move || saving.get()
                                        />
                                        <label for="rule-enabled" class="ml-3.5 text-sm font-medium cursor-pointer" style="color: var(--text-primary); line-height: 1.5;">
                                            "Enable this rule"
                                        </label>
                                    </div>
                                </div>
                            </div>

                            <div class="p-6">
                                <div class="mb-5">
                                    <h4 class="text-lg font-bold mb-2" style="color: var(--text-primary); line-height: 1.4;">
                                        "Schedule"
                                    </h4>
                                    <div class="h-0.5 w-12 rounded-full" style="background-color: var(--accent-primary);"></div>
                                    <p class="text-xs mt-3" style="color: var(--text-secondary); line-height: 1.6;">
                                        "Configure when this rule should automatically run and check your torrents"
                                    </p>
                                </div>
                                <div class="space-y-6">
                                    <div style="margin-bottom: 1.5rem;">
                                        <label class="block text-sm font-semibold mb-3" style="color: var(--text-primary); line-height: 1.5;">
                                            "Trigger Type"
                                        </label>
                                        <select
                                            class="w-full px-4 py-3 rounded-lg border transition-all"
                                            style="background-color: var(--bg-tertiary); border-color: var(--border-secondary); color: var(--text-primary); font-size: 0.9375rem;"
                                            on:change=move |ev| trigger_type.set(event_target_value(&ev))
                                            disabled=move || saving.get()
                                        >
                                            <option value="interval" selected=move || trigger_type.get() == "interval">"Interval (minutes)"</option>
                                            <option value="cron" selected=move || trigger_type.get() == "cron">"Cron Expression"</option>
                                        </select>
                                    </div>

                                    <Show when=move || trigger_type.get() == "interval">
                                        <div style="margin-top: 1.5rem;">
                                            <label class="block text-sm font-semibold mb-3" style="color: var(--text-primary); line-height: 1.5;">
                                                "Interval (minutes)"
                                            </label>
                                            <input
                                                type="number"
                                                min="30"
                                                class="w-full px-4 py-3 rounded-lg border transition-all"
                                                style="background-color: var(--bg-tertiary); border-color: var(--border-secondary); color: var(--text-primary); font-size: 0.9375rem;"
                                                value=move || interval_minutes.get().to_string()
                                                on:input=move |ev| {
                                                    if let Ok(val) = event_target_value(&ev).parse::<u32>() {
                                                        interval_minutes.set(if val < 30 { 30 } else { val });
                                                    }
                                                }
                                                disabled=move || saving.get()
                                            />
                                            <p class="text-xs mt-2.5" style="color: var(--text-secondary); line-height: 1.6;">
                                                "How often to check and run this rule (minimum 30 minutes)"
                                            </p>
                                        </div>
                                    </Show>

                                    <Show when=move || trigger_type.get() == "cron">
                                        <div style="margin-top: 1.5rem;">
                                            <label class="block text-sm font-semibold mb-3" style="color: var(--text-primary); line-height: 1.5;">
                                                "Cron Expression"
                                            </label>
                                            <input
                                                type="text"
                                                placeholder="0 * * * *"
                                                class="w-full px-4 py-3 rounded-lg border transition-all font-mono text-sm"
                                                style="background-color: var(--bg-tertiary); border-color: var(--border-secondary); color: var(--text-primary);"
                                                value=move || cron_expression.get()
                                                on:input=move |ev| cron_expression.set(event_target_value(&ev))
                                                disabled=move || saving.get()
                                            />
                                            <p class="text-xs mt-2.5" style="color: var(--text-secondary); line-height: 1.6;">
                                                "Format: sec min hour day month day-of-week (e.g., '0 0 * * *' = every hour at minute 0)"
                                            </p>
                                        </div>
                                    </Show>
                                </div>
                            </div>

                            <div class="p-6">
                                <div class="mb-5">
                                    <h4 class="text-lg font-bold mb-2" style="color: var(--text-primary); line-height: 1.4;">
                                        "Condition"
                                    </h4>
                                    <div class="h-0.5 w-12 rounded-full" style="background-color: var(--accent-primary);"></div>
                                    <p class="text-xs mt-3" style="color: var(--text-secondary); line-height: 1.6;">
                                        "Define what criteria torrents must meet for this rule to apply. Currently, only one condition is supported per rule."
                                    </p>
                                </div>
                                <div class="space-y-6">
                                    <div style="margin-bottom: 1.5rem;">
                                        <label class="block text-sm font-semibold mb-3" style="color: var(--text-primary); line-height: 1.5;">
                                            "Condition Type"
                                        </label>
                                        <select
                                            class="w-full px-4 py-3 rounded-lg border transition-all"
                                            style="background-color: var(--bg-tertiary); border-color: var(--border-secondary); color: var(--text-primary); font-size: 0.9375rem;"
                                            on:change=move |ev| condition_type.set(event_target_value(&ev))
                                            disabled=move || saving.get()
                                        >
                                        <optgroup label="Time-Based">
                                            <option value="SeedingTime" selected=move || condition_type.get() == "SeedingTime">"Seeding Time (hours)"</option>
                                            <option value="StalledTime" selected=move || condition_type.get() == "StalledTime">"Stalled Time (hours)"</option>
                                            <option value="Age" selected=move || condition_type.get() == "Age">"Age (hours)"</option>
                                            <option value="ExpiresAt" selected=move || condition_type.get() == "ExpiresAt">"Expires At (hours remaining)"</option>
                                            <option value="ETA" selected=move || condition_type.get() == "ETA">"ETA (hours)"</option>
                                        </optgroup>
                                        <optgroup label="Performance">
                                            <option value="SeedingRatio" selected=move || condition_type.get() == "SeedingRatio">"Seeding Ratio"</option>
                                            <option value="DownloadSpeed" selected=move || condition_type.get() == "DownloadSpeed">"Download Speed (bytes/sec)"</option>
                                            <option value="UploadSpeed" selected=move || condition_type.get() == "UploadSpeed">"Upload Speed (bytes/sec)"</option>
                                            <option value="Progress" selected=move || condition_type.get() == "Progress">"Progress (%)"</option>
                                            <option value="Availability" selected=move || condition_type.get() == "Availability">"Availability (0-1)"</option>
                                        </optgroup>
                                        <optgroup label="Network">
                                            <option value="Seeds" selected=move || condition_type.get() == "Seeds">"Seeds (count)"</option>
                                            <option value="Peers" selected=move || condition_type.get() == "Peers">"Peers (count)"</option>
                                        </optgroup>
                                        <optgroup label="Size & Data">
                                            <option value="FileSize" selected=move || condition_type.get() == "FileSize">"File Size (GB)"</option>
                                            <option value="TotalUploaded" selected=move || condition_type.get() == "TotalUploaded">"Total Uploaded (GB)"</option>
                                            <option value="TotalDownloaded" selected=move || condition_type.get() == "TotalDownloaded">"Total Downloaded (GB)"</option>
                                        </optgroup>
                                        <optgroup label="Status">
                                            <option value="DownloadState" selected=move || condition_type.get() == "DownloadState">"Download State"</option>
                                            <option value="Inactive" selected=move || condition_type.get() == "Inactive">"Inactive"</option>
                                            <option value="DownloadFinished" selected=move || condition_type.get() == "DownloadFinished">"Download Finished"</option>
                                            <option value="Cached" selected=move || condition_type.get() == "Cached">"Cached"</option>
                                            <option value="DownloadPresent" selected=move || condition_type.get() == "DownloadPresent">"Download Present"</option>
                                        </optgroup>
                                        <optgroup label="Properties">
                                            <option value="Private" selected=move || condition_type.get() == "Private">"Private"</option>
                                            <option value="LongTermSeeding" selected=move || condition_type.get() == "LongTermSeeding">"Long Term Seeding"</option>
                                            <option value="SeedTorrent" selected=move || condition_type.get() == "SeedTorrent">"Seed Torrent"</option>
                                            <option value="TorrentFile" selected=move || condition_type.get() == "TorrentFile">"Has Torrent File"</option>
                                            <option value="AllowZipped" selected=move || condition_type.get() == "AllowZipped">"Allow Zipped"</option>
                                            <option value="HasMagnet" selected=move || condition_type.get() == "HasMagnet">"Has Magnet Link"</option>
                                        </optgroup>
                                    </select>
                                    </div>
                                    <Show when=move || condition_type.get() == "SeedingTime">
                                        <div class="mt-3 p-3.5 rounded-lg" style="background-color: var(--bg-tertiary); border: 1px solid var(--border-secondary);">
                                            <p class="text-xs" style="color: var(--text-secondary); line-height: 1.6;">
                                                "Hours since the torrent started seeding (from cached_at timestamp)"
                                            </p>
                                        </div>
                                    </Show>
                                    <Show when=move || condition_type.get() == "SeedingRatio">
                                        <div class="mt-3 p-3.5 rounded-lg" style="background-color: var(--bg-tertiary); border: 1px solid var(--border-secondary);">
                                            <p class="text-xs" style="color: var(--text-secondary); line-height: 1.6;">
                                                "Upload/download ratio (e.g., 1.0 = uploaded as much as downloaded)"
                                            </p>
                                        </div>
                                    </Show>
                                    <Show when=move || condition_type.get() == "StalledTime">
                                        <div class="mt-3 p-3.5 rounded-lg" style="background-color: var(--bg-tertiary); border: 1px solid var(--border-secondary);">
                                            <p class="text-xs" style="color: var(--text-secondary); line-height: 1.6;">
                                                "Hours since last update (indicates stalled torrents)"
                                            </p>
                                        </div>
                                    </Show>
                                    <Show when=move || condition_type.get() == "Age">
                                        <div class="mt-3 p-3.5 rounded-lg" style="background-color: var(--bg-tertiary); border: 1px solid var(--border-secondary);">
                                            <p class="text-xs" style="color: var(--text-secondary); line-height: 1.6;">
                                                "Hours since torrent was created"
                                            </p>
                                        </div>
                                    </Show>
                                    <Show when=move || condition_type.get() == "Inactive">
                                        <div class="mt-3 p-3.5 rounded-lg" style="background-color: var(--bg-tertiary); border: 1px solid var(--border-secondary);">
                                            <p class="text-xs" style="color: var(--text-secondary); line-height: 1.6;">
                                                "Torrent is not active or in error/failed state (use value 1.0 for Equal)"
                                            </p>
                                        </div>
                                    </Show>
                                    <Show when=move || condition_type.get() == "DownloadFinished">
                                        <div class="mt-3 p-3.5 rounded-lg" style="background-color: var(--bg-tertiary); border: 1px solid var(--border-secondary);">
                                            <p class="text-xs" style="color: var(--text-secondary); line-height: 1.6;">
                                                "Download is complete (use value 1.0 for Equal)"
                                            </p>
                                        </div>
                                    </Show>
                                    <Show when=move || condition_type.get() == "Cached">
                                        <div class="mt-3 p-3.5 rounded-lg" style="background-color: var(--bg-tertiary); border: 1px solid var(--border-secondary);">
                                            <p class="text-xs" style="color: var(--text-secondary); line-height: 1.6;">
                                                "Torrent is cached on server (use value 1.0 for Equal)"
                                            </p>
                                        </div>
                                    </Show>
                                    <Show when=move || condition_type.get() == "DownloadState">
                                        <div class="mt-3 p-3.5 rounded-lg" style="background-color: var(--bg-tertiary); border: 1px solid var(--border-secondary);">
                                            <p class="text-xs" style="color: var(--text-secondary); line-height: 1.6;">
                                                "State: 0=downloading, 1=uploading, 2=stopped, 3=cached (use Equal operator)"
                                            </p>
                                        </div>
                                    </Show>
                                    <Show when=move || condition_type.get() == "Progress">
                                        <div class="mt-3 p-3.5 rounded-lg" style="background-color: var(--bg-tertiary); border: 1px solid var(--border-secondary);">
                                            <p class="text-xs" style="color: var(--text-secondary); line-height: 1.6;">
                                                "Download progress percentage (0-100)"
                                            </p>
                                        </div>
                                    </Show>
                                    <Show when=move || condition_type.get() == "FileSize">
                                        <div class="mt-3 p-3.5 rounded-lg" style="background-color: var(--bg-tertiary); border: 1px solid var(--border-secondary);">
                                            <p class="text-xs" style="color: var(--text-secondary); line-height: 1.6;">
                                                "Torrent size in gigabytes"
                                            </p>
                                        </div>
                                    </Show>
                                    <Show when=move || condition_type.get() == "ETA">
                                        <div class="mt-3 p-3.5 rounded-lg" style="background-color: var(--bg-tertiary); border: 1px solid var(--border-secondary);">
                                            <p class="text-xs" style="color: var(--text-secondary); line-height: 1.6;">
                                                "Estimated time to completion in hours"
                                            </p>
                                        </div>
                                    </Show>
                                    <Show when=move || condition_type.get() == "Availability">
                                        <div class="mt-3 p-3.5 rounded-lg" style="background-color: var(--bg-tertiary); border: 1px solid var(--border-secondary);">
                                            <p class="text-xs" style="color: var(--text-secondary); line-height: 1.6;">
                                                "Torrent availability (0.0 to 1.0, where 1.0 = fully available)"
                                            </p>
                                        </div>
                                    </Show>
                                    <Show when=move || condition_type.get() == "ExpiresAt">
                                        <div class="mt-3 p-3.5 rounded-lg" style="background-color: var(--bg-tertiary); border: 1px solid var(--border-secondary);">
                                            <p class="text-xs" style="color: var(--text-secondary); line-height: 1.6;">
                                                "Hours until torrent expires (only for torrents with expiration)"
                                            </p>
                                        </div>
                                    </Show>

                                    <div class="grid grid-cols-1 md:grid-cols-2 gap-5" style="margin-top: 1.5rem;">
                                        <div>
                                            <label class="block text-sm font-semibold mb-3" style="color: var(--text-primary); line-height: 1.5;">
                                                "Operator"
                                            </label>
                                            <select
                                                class="w-full px-4 py-3 rounded-lg border transition-all"
                                                style="background-color: var(--bg-tertiary); border-color: var(--border-secondary); color: var(--text-primary); font-size: 0.9375rem;"
                                                on:change=move |ev| condition_operator.set(event_target_value(&ev))
                                                disabled=move || saving.get()
                                            >
                                                <option value="GreaterThan" selected=move || condition_operator.get() == "GreaterThan">"Greater Than (>)"</option>
                                                <option value="LessThan" selected=move || condition_operator.get() == "LessThan">"Less Than (<)"</option>
                                                <option value="GreaterThanOrEqual" selected=move || condition_operator.get() == "GreaterThanOrEqual">"Greater Than Or Equal (≥)"</option>
                                                <option value="LessThanOrEqual" selected=move || condition_operator.get() == "LessThanOrEqual">"Less Than Or Equal (≤)"</option>
                                                <option value="Equal" selected=move || condition_operator.get() == "Equal">"Equal (=)"</option>
                                            </select>
                                        </div>

                                        <div>
                                            <label class="block text-sm font-semibold mb-3" style="color: var(--text-primary); line-height: 1.5;">
                                                "Value"
                                            </label>
                                            <input
                                                type="number"
                                                step="0.1"
                                                class="w-full px-4 py-3 rounded-lg border transition-all"
                                                style="background-color: var(--bg-tertiary); border-color: var(--border-secondary); color: var(--text-primary); font-size: 0.9375rem;"
                                                value=move || condition_value.get().to_string()
                                                on:input=move |ev| {
                                                    if let Ok(val) = event_target_value(&ev).parse::<f64>() {
                                                        condition_value.set(val);
                                                    }
                                                }
                                                disabled=move || saving.get()
                                            />
                                        </div>
                                    </div>
                                </div>
                            </div>

                            <div class="p-6">
                                <div class="mb-5">
                                    <h4 class="text-lg font-bold mb-2" style="color: var(--text-primary); line-height: 1.4;">
                                        "Action"
                                    </h4>
                                    <div class="h-0.5 w-12 rounded-full" style="background-color: var(--accent-primary);"></div>
                                    <p class="text-xs mt-3" style="color: var(--text-secondary); line-height: 1.6;">
                                        "What action should be performed on torrents that match the condition?"
                                    </p>
                                </div>
                                <div>
                                    <select
                                        class="w-full px-4 py-3 rounded-lg border transition-all"
                                        style="background-color: var(--bg-tertiary); border-color: var(--border-secondary); color: var(--text-primary); font-size: 0.9375rem;"
                                        on:change=move |ev| action_type.set(event_target_value(&ev))
                                        disabled=move || saving.get()
                                    >
                                <optgroup label="Control">
                                    <option value="StopSeeding" selected=move || action_type.get() == "StopSeeding">"Stop Seeding"</option>
                                    <option value="Stop" selected=move || action_type.get() == "Stop">"Stop"</option>
                                    <option value="Resume" selected=move || action_type.get() == "Resume">"Resume"</option>
                                    <option value="Restart" selected=move || action_type.get() == "Restart">"Restart"</option>
                                    <option value="ForceStart" selected=move || action_type.get() == "ForceStart">"Force Start"</option>
                                    <option value="Reannounce" selected=move || action_type.get() == "Reannounce">"Reannounce"</option>
                                </optgroup>
                                <optgroup label="Remove">
                                    <option value="Delete" selected=move || action_type.get() == "Delete">"Delete"</option>
                                </optgroup>
                            </select>
                                    <Show when=move || action_type.get() == "StopSeeding">
                                        <div class="mt-4 p-3.5 rounded-lg" style="background-color: var(--bg-tertiary); border: 1px solid var(--border-secondary);">
                                            <p class="text-xs" style="color: var(--text-secondary); line-height: 1.6;">
                                                "Stops seeding for completed/seeding torrents. Use for torrents that have finished downloading."
                                            </p>
                                        </div>
                                    </Show>
                                    <Show when=move || action_type.get() == "Stop">
                                        <div class="mt-4 p-3.5 rounded-lg" style="background-color: var(--bg-tertiary); border: 1px solid var(--border-secondary);">
                                            <p class="text-xs" style="color: var(--text-secondary); line-height: 1.6;">
                                                "Stops active torrents (downloads or uploads). Use for torrents currently downloading or uploading."
                                            </p>
                                        </div>
                                    </Show>
                                    <Show when=move || action_type.get() == "Resume">
                                        <div class="mt-4 p-3.5 rounded-lg" style="background-color: var(--bg-tertiary); border: 1px solid var(--border-secondary);">
                                            <p class="text-xs" style="color: var(--text-secondary); line-height: 1.6;">
                                                "Resumes a stopped torrent. Use to restart paused or stopped downloads."
                                            </p>
                                        </div>
                                    </Show>
                                    <Show when=move || action_type.get() == "Restart">
                                        <div class="mt-4 p-3.5 rounded-lg" style="background-color: var(--bg-tertiary); border: 1px solid var(--border-secondary);">
                                            <p class="text-xs" style="color: var(--text-secondary); line-height: 1.6;">
                                                "Restarts a torrent from the beginning. Use to reset and restart downloads."
                                            </p>
                                        </div>
                                    </Show>
                                    <Show when=move || action_type.get() == "ForceStart">
                                        <div class="mt-4 p-3.5 rounded-lg" style="background-color: var(--bg-tertiary); border: 1px solid var(--border-secondary);">
                                            <p class="text-xs" style="color: var(--text-secondary); line-height: 1.6;">
                                                "Forces a torrent to start immediately. Use to start queued or stopped torrents."
                                            </p>
                                        </div>
                                    </Show>
                                    <Show when=move || action_type.get() == "Reannounce">
                                        <div class="mt-4 p-3.5 rounded-lg" style="background-color: var(--bg-tertiary); border: 1px solid var(--border-secondary);">
                                            <p class="text-xs" style="color: var(--text-secondary); line-height: 1.6;">
                                                "Reannounces to tracker. Use for stalled torrents to refresh tracker connection."
                                            </p>
                                        </div>
                                    </Show>
                                    <Show when=move || action_type.get() == "Delete">
                                        <div class="mt-4 p-3.5 rounded-lg" style="background-color: var(--bg-tertiary); border: 1px solid var(--border-secondary);">
                                            <p class="text-xs" style="color: var(--text-secondary); line-height: 1.6;">
                                                "Permanently deletes the torrent and its files. This action cannot be undone."
                                            </p>
                                        </div>
                                    </Show>
                                </div>
                            </div>
                        </div>
                    </div>

                    <div class="flex-shrink-0 px-4 sm:px-6 md:px-8 py-4 sm:py-5 md:py-6 border-t flex flex-row justify-end gap-3 sm:gap-4" style="border-color: var(--border-primary); background-color: var(--bg-secondary);">
                        <button
                            class="px-6 py-3 rounded-lg transition-all font-semibold text-sm"
                            style="background-color: var(--bg-tertiary); color: var(--text-primary); border: 1px solid var(--border-secondary); line-height: 1.5; display: inline-flex !important; align-items: center; justify-content: center; width: auto !important; min-width: 100px; max-width: none !important; flex: 0 0 auto !important;"
                            on:click=move |_| {
                                if !saving.get() {
                                    show.set(false);
                                }
                            }
                            disabled=move || saving.get()
                        >
                            "Cancel"
                        </button>
                        <button
                            class="px-6 py-3 rounded-lg font-semibold transition-all text-sm"
                            style="background-color: var(--accent-primary); color: var(--text-on-accent); line-height: 1.5; display: inline-flex !important; align-items: center; justify-content: center; box-shadow: var(--shadow-md); width: auto !important; min-width: 100px; max-width: none !important; flex: 0 0 auto !important;"
                                on:click={
                                    let on_save_for_click = on_save_arc_clone.clone();
                                    move |_| {
                                        #[cfg(feature = "hydrate")]
                                        {
                                            saving_save.set(true);
                                            save_error_save.set(None);
                                            let rule_name_clone = rule_name_save.clone();
                                            let rule_enabled_clone = rule_enabled_save.clone();
                                            let trigger_type_clone = trigger_type_save.clone();
                                            let cron_expression_clone = cron_expression_save.clone();
                                            let interval_minutes_clone = interval_minutes_save.clone();
                                            let condition_type_clone = condition_type_save.clone();
                                            let condition_operator_clone = condition_operator_save.clone();
                                            let condition_value_clone = condition_value_save.clone();
                                            let action_type_clone = action_type_save.clone();
                                            let editing_rule_id_clone = editing_rule_id_save.clone();
                                            let saving_clone = saving_save.clone();
                                            let save_error_clone = save_error_save.clone();
                                            let on_save_clone = on_save_for_click.clone();
                                        
                                        spawn_local(async move {
                                            if let Some(window) = web_sys::window() {
                                                if let Ok(Some(storage)) = window.local_storage() {
                                                    if let Ok(Some(api_key)) = storage.get_item("api_key") {
                                                        if !api_key.is_empty() {
                                                            let trigger_config = if trigger_type_clone.get() == "cron" {
                                                                serde_json::json!({
                                                                    "Cron": {
                                                                        "expression": cron_expression_clone.get()
                                                                    }
                                                                })
                                                            } else {
                                                                serde_json::json!({
                                                                    "Interval": {
                                                                        "minutes": interval_minutes_clone.get()
                                                                    }
                                                                })
                                                            };

                                                            let conditions = vec![serde_json::json!({
                                                                "type": condition_type_clone.get(),
                                                                "operator": condition_operator_clone.get(),
                                                                "value": condition_value_clone.get()
                                                            })];

                                                            let action_config = serde_json::json!({
                                                                "action_type": action_type_clone.get(),
                                                                "params": serde_json::Value::Null
                                                            });

                                                            let request = CreateRuleRequest {
                                                                name: rule_name_clone.get(),
                                                                enabled: Some(rule_enabled_clone.get()),
                                                                trigger_config,
                                                                conditions,
                                                                action_config,
                                                            };

                                                            let url = if editing_rule_id_clone.get().is_some() {
                                                                format!("/api/automation/rules/{}", editing_rule_id_clone.get().unwrap())
                                                            } else {
                                                                "/api/automation/rules".to_string()
                                                            };

                                                            let method = if editing_rule_id_clone.get().is_some() { "PUT" } else { "POST" };

                                                            let headers = web_sys::Headers::new().unwrap();
                                                            headers.set("Authorization", &format!("Bearer {}", api_key)).unwrap();
                                                            headers.set("Content-Type", "application/json").unwrap();
                                                            
                                                            let body = serde_json::to_string(&request).unwrap();
                                                            let body_js = wasm_bindgen::JsValue::from_str(&body);
                                                            
                                                            let init = {
                                                                let mut i = web_sys::RequestInit::new();
                                                                i.set_method(method);
                                                                i.set_headers(&headers);
                                                                i.set_body(&body_js);
                                                                i
                                                            };
                                                            
                                                            let promise = window.fetch_with_str_and_init(&url, &init);
                                                            let future = wasm_bindgen_futures::JsFuture::from(promise);
                                                            
                                                            if let Ok(response) = future.await {
                                                                let resp: web_sys::Response = response.dyn_into().unwrap();
                                                                if resp.status() == 200 || resp.status() == 201 {
                                                                    saving_clone.set(false);
                                                                    on_save_clone();
                                                                } else {
                                                                    let text_promise = resp.text().unwrap();
                                                                    let text_future = wasm_bindgen_futures::JsFuture::from(text_promise);
                                                                    if let Ok(text_value) = text_future.await {
                                                                        if let Some(text) = text_value.as_string() {
                                                                            if let Ok(api_response) = serde_json::from_str::<ApiResponse<serde_json::Value>>(&text) {
                                                                                if let Some(err_msg) = api_response.error {
                                                                                    save_error_clone.set(Some(err_msg));
                                                                                } else {
                                                                                    save_error_clone.set(Some(format!("Failed to save: {}", text)));
                                                                                }
                                                                            } else {
                                                                                save_error_clone.set(Some(format!("Failed to save: {}", text)));
                                                                            }
                                                                        } else {
                                                                            save_error_clone.set(Some(format!("Failed to save: {}", resp.status())));
                                                                        }
                                                                    } else {
                                                                        save_error_clone.set(Some(format!("Failed to save: {}", resp.status())));
                                                                    }
                                                                    saving_clone.set(false);
                                                                }
                                                            } else {
                                                                save_error_clone.set(Some("Network request failed".to_string()));
                                                                saving_clone.set(false);
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        });
                                    }
                                }
                            }
                            disabled=move || saving_save.get()
                            >
                                {move || if saving_save.get() { "Saving..." } else { "Save" }}
                            </button>
                        </div>
                    </div>
                </div>
        </Show>
    }
}
