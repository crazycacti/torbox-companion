use leptos::prelude::*;
use leptos::task::spawn_local;
#[cfg(target_arch = "wasm32")]
use web_sys;
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::{self, JsCast};
use crate::api::{TorboxClient, User};

#[derive(Clone)]
pub struct DashboardContext {
    pub active_tab: RwSignal<String>,
    pub user_data: RwSignal<Option<User>>,
    pub user_loading: RwSignal<bool>,
    pub api_connected: RwSignal<bool>,
    pub is_loading: RwSignal<bool>,
}

impl DashboardContext {
    pub fn new() -> Self {
        Self {
            active_tab: RwSignal::new("overview".to_string()),
            user_data: RwSignal::new(None),
            user_loading: RwSignal::new(false),
            api_connected: RwSignal::new(false),
            is_loading: RwSignal::new(true),
        }
    }
}

#[component]
pub fn DashboardHeader() -> impl IntoView {
    let context = use_context::<DashboardContext>()
        .expect("DashboardContext should be provided by MainDashboard");
    
    let active_tab = context.active_tab;
    let user_data = context.user_data;
    let user_loading = context.user_loading;
    let api_connected = context.api_connected;
    let is_loading = context.is_loading;
    
    let format_date = move |date_str: String| -> String {
        if date_str.is_empty() {
            return "N/A".to_string();
        }
        
        if let Ok(parsed) = chrono::DateTime::parse_from_rfc3339(&date_str) {
            parsed.format("%B %d, %Y at %I:%M %p").to_string()
        } else {
            date_str
        }
    };
    
    let fetch_user_data = move || {
        #[cfg(target_arch = "wasm32")]
        {
            user_loading.set(true);
            spawn_local(async move {
                if let Some(window) = web_sys::window() {
                    if let Ok(Some(storage)) = window.local_storage() {
                        if let Ok(Some(api_key)) = storage.get_item("api_key") {
                            if !api_key.is_empty() {
                                let client = TorboxClient::new(api_key);
                                match client.get_user(Some(false)).await {
                                    Ok(response) => {
                                        if let Some(user) = response.data {
                                            user_data.set(Some(user));
                                        }
                                    }
                                    Err(_) => {}
                                }
                            }
                        }
                    }
                }
                user_loading.set(false);
            });
        }
    };

    let fetch_user_data_initial = move || {
        #[cfg(target_arch = "wasm32")]
        {
            if user_data.get_untracked().is_none() && !user_loading.get_untracked() {
                user_loading.set(true);
                is_loading.set(true);
                spawn_local(async move {
                    if let Some(window) = web_sys::window() {
                        if let Ok(Some(storage)) = window.local_storage() {
                            if let Ok(Some(api_key)) = storage.get_item("api_key") {
                                if !api_key.is_empty() {
                                    let client = TorboxClient::new(api_key);
                                    match client.get_user(Some(false)).await {
                                        Ok(response) => {
                                            api_connected.set(true);
                                            if let Some(user) = response.data {
                                                user_data.set(Some(user));
                                            }
                                        }
                                        Err(_) => {
                                            api_connected.set(false);
                                        }
                                    }
                                } else {
                                    api_connected.set(false);
                                }
                            } else {
                                api_connected.set(false);
                            }
                        } else {
                            api_connected.set(false);
                        }
                    } else {
                        api_connected.set(false);
                    }
                    user_loading.set(false);
                    is_loading.set(false);
                });
            }
        }
    };
    
    fetch_user_data_initial();
    
    #[cfg(target_arch = "wasm32")]
    {
        let user_data_poll = user_data.clone();
        let api_connected_poll = api_connected.clone();
        let interval_created = RwSignal::new(false);
        
        Effect::new(move |_| {
            if interval_created.get() {
                return;
            }
            
            interval_created.set(true);
            let user_data_effect = user_data_poll.clone();
            let api_connected_effect = api_connected_poll.clone();
            
            spawn_local(async move {
                if let Some(window) = web_sys::window() {
                    let closure = wasm_bindgen::closure::Closure::wrap(Box::new(move || {
                        if let Some(window) = web_sys::window() {
                            if let Ok(Some(storage)) = window.local_storage() {
                                if let Ok(Some(api_key)) = storage.get_item("api_key") {
                                    if !api_key.is_empty() {
                                        let user_data = user_data_effect.clone();
                                        let api_connected = api_connected_effect.clone();
                                        spawn_local(async move {
                                            let client = TorboxClient::new(api_key);
                                            match client.get_user(Some(false)).await {
                                                Ok(response) => {
                                                    api_connected.set(true);
                                                    if let Some(user) = response.data {
                                                        user_data.set(Some(user));
                                                    }
                                                }
                                                Err(_) => {
                                                    api_connected.set(false);
                                                }
                                            }
                                        });
                                    }
                                }
                            }
                        }
                    }) as Box<dyn FnMut()>);
                    
                    let _ = window.set_interval_with_callback_and_timeout_and_arguments_0(
                        closure.as_ref().unchecked_ref(),
                        60_000,
                    );
                    
                    closure.forget();
                }
            });
        });
    }
    
    let logout = move |_| {
        #[cfg(target_arch = "wasm32")]
        {
            if let Some(window) = web_sys::window() {
                if let Ok(Some(storage)) = window.local_storage() {
                    let _ = storage.remove_item("api_key");
                }
                let _ = window.location().set_href("/");
            }
        }
    };

    let mobile_menu_open = RwSignal::new(false);
    
    let toggle_mobile_menu = move |_| {
        mobile_menu_open.update(|open| *open = !*open);
    };
    
    let close_mobile_menu = move |_| {
        mobile_menu_open.set(false);
    };
    
    {
        let mobile_menu_open_clone = mobile_menu_open.clone();
        Effect::new(move |_| {
            let _ = active_tab.get();
            mobile_menu_open_clone.set(false);
        });
    }

    view! {
        <header class="border-b px-4 md:px-6 py-3 md:py-4 sticky top-0 z-50" style="background-color: var(--bg-card); border-color: var(--border-secondary);">
            <div class="flex items-center justify-between">
                <div class="flex items-center space-x-4">
                    <div class="flex items-center space-x-3">
                        <div class="w-10 h-10 rounded-lg flex items-center justify-center">
                            <img src="/tclogo.png" alt="Torbox Companion Logo" class="w-8 h-8 object-contain"/>
                        </div>
                        <div>
                            <h1 class="text-xl font-bold" style="color: var(--text-primary);">"Torbox Companion"</h1>
                            <span class="text-sm hidden sm:inline" style="color: var(--text-secondary);">"Dashboard"</span>
                        </div>
                    </div>
                </div>

                <nav class="hidden md:flex items-center space-x-1">
                    <button 
                        class={move || {
                            let base = "px-4 py-2 text-sm font-medium rounded-lg transition-colors";
                            base.to_string()
                        }}
                        style={move || {
                            if active_tab.get() == "overview" {
                                "color: var(--text-primary); background-color: var(--bg-tertiary);"
                            } else {
                                "color: var(--text-secondary);"
                            }
                        }}
                        on:click=move |_| active_tab.set("overview".to_string())
                    >
                        "Overview"
                    </button>
                    <button 
                        class={move || {
                            let base = "px-4 py-2 text-sm font-medium rounded-lg transition-colors";
                            base.to_string()
                        }}
                        style={move || {
                            if active_tab.get() == "themes" {
                                "color: var(--text-primary); background-color: var(--bg-tertiary);"
                            } else {
                                "color: var(--text-secondary);"
                            }
                        }}
                        on:click=move |_| active_tab.set("themes".to_string())
                    >
                        "Themes"
                    </button>
                    <button 
                        class={move || {
                            let base = "px-4 py-2 text-sm font-medium rounded-lg transition-colors";
                            base.to_string()
                        }}
                        style={move || {
                            if active_tab.get() == "user" {
                                "color: var(--text-primary); background-color: var(--bg-tertiary);"
                            } else {
                                "color: var(--text-secondary);"
                            }
                        }}
                        on:click=move |_| active_tab.set("user".to_string())
                    >
                        "Info"
                    </button>
                </nav>

                <div class="flex items-center space-x-2 md:space-x-4">
                    <div class="hidden sm:flex items-center space-x-2">
                        <div class="flex items-center space-x-1 md:space-x-2">
                            <div class="w-2 h-2 rounded-full" class:bg-green-500={move || api_connected.get()} class:bg-red-500={move || !api_connected.get() && !is_loading.get()} class:bg-yellow-500={move || is_loading.get()}></div>
                            <span class="text-xs md:text-sm hidden sm:inline" style="color: var(--text-secondary);">
                                {move || {
                                    if is_loading.get() {
                                        "Checking..."
                                    } else if api_connected.get() {
                                        "API Connected"
                                    } else {
                                        "API Disconnected"
                                    }
                                }}
                            </span>
                        </div>
                    </div>

                    <button
                        class="hidden md:flex px-4 py-2 rounded-lg transition-colors items-center space-x-2"
                        style="color: var(--text-secondary);"
                        on:click=logout
                    >
                        <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M17 16l4-4m0 0l-4-4m4 4H7m6 4v1a3 3 0 01-3 3H6a3 3 0 01-3-3V7a3 3 0 013-3h4a3 3 0 013 3v1"></path>
                        </svg>
                        <span>"Logout"</span>
                    </button>

                    <button
                        class="md:hidden p-2 rounded-lg transition-colors"
                        style="color: var(--text-secondary);"
                        on:click=toggle_mobile_menu
                        aria-label="Toggle menu"
                    >
                        <Show when=move || !mobile_menu_open.get()>
                            <svg class="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 6h16M4 12h16M4 18h16"></path>
                            </svg>
                        </Show>
                        <Show when=move || mobile_menu_open.get()>
                            <svg class="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12"></path>
                            </svg>
                        </Show>
                    </button>
                </div>
            </div>

            <Show when=move || mobile_menu_open.get()>
                <div 
                    class="md:hidden absolute top-full left-0 right-0 rounded-b-xl overflow-hidden shadow-2xl z-40"
                    style="background-color: var(--bg-card); border: 1px solid var(--border-secondary); border-top: none; margin-left: 0.5rem; margin-right: 0.5rem; width: calc(100% - 1rem);"
                    on:click=|ev| ev.stop_propagation()
                >
                    <nav class="p-4 space-y-1">
                        <button
                            class="block w-full text-left px-4 py-3 text-base font-medium"
                            style={move || {
                                if active_tab.get() == "overview" {
                                    "color: var(--text-primary); background-color: var(--bg-tertiary);"
                                } else {
                                    "color: var(--text-secondary);"
                                }
                            }}
                            on:click=move |_| {
                                active_tab.set("overview".to_string());
                                mobile_menu_open.set(false);
                            }
                        >
                            "Overview"
                        </button>
                        <button
                            class="block w-full text-left px-4 py-3 text-base font-medium"
                            style={move || {
                                if active_tab.get() == "themes" {
                                    "color: var(--text-primary); background-color: var(--bg-tertiary);"
                                } else {
                                    "color: var(--text-secondary);"
                                }
                            }}
                            on:click=move |_| {
                                active_tab.set("themes".to_string());
                                mobile_menu_open.set(false);
                            }
                        >
                            "Themes"
                        </button>
                        <button
                            class="block w-full text-left px-4 py-3 text-base font-medium"
                            style={move || {
                                if active_tab.get() == "user" {
                                    "color: var(--text-primary); background-color: var(--bg-tertiary);"
                                } else {
                                    "color: var(--text-secondary);"
                                }
                            }}
                            on:click=move |_| {
                                active_tab.set("user".to_string());
                                mobile_menu_open.set(false);
                            }
                        >
                            "Info"
                        </button>
                        <div class="border-t my-3" style="border-color: var(--border-secondary);"></div>
                        <div class="px-4 py-2">
                            <div class="flex items-center space-x-2 mb-2">
                                <div class="w-2 h-2 rounded-full" class:bg-green-500={move || api_connected.get()} class:bg-red-500={move || !api_connected.get() && !is_loading.get()} class:bg-yellow-500={move || is_loading.get()}></div>
                                <span class="text-sm" style="color: var(--text-secondary);">
                                    {move || {
                                        if is_loading.get() {
                                            "Checking..."
                                        } else if api_connected.get() {
                                            "API Connected"
                                        } else {
                                            "API Disconnected"
                                        }
                                    }}
                                </span>
                            </div>
                        </div>
                        <button
                            class="block w-full text-left px-4 py-3 text-base font-medium flex items-center space-x-2"
                            style="color: var(--accent-danger);"
                            on:click=move |ev| {
                                mobile_menu_open.set(false);
                                logout(ev);
                            }
                        >
                            <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M17 16l4-4m0 0l-4-4m4 4H7m6 4v1a3 3 0 01-3 3H6a3 3 0 01-3-3V7a3 3 0 013-3h4a3 3 0 013 3v1"></path>
                            </svg>
                            <span>"Logout"</span>
                        </button>
                    </nav>
                </div>
            </Show>
            
            <Show when=move || mobile_menu_open.get()>
                <div 
                    class="md:hidden fixed inset-0 z-30"
                    style="background-color: rgba(0, 0, 0, 0.5);"
                    on:click=close_mobile_menu
                ></div>
            </Show>
        </header>
    }
}
