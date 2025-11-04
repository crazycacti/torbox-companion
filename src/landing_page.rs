use leptos::prelude::*;
use leptos::task::spawn_local;
#[cfg(target_arch = "wasm32")]
use web_sys;
use crate::api::{TorboxClient, ApiError};

#[component]
pub fn LandingPage() -> impl IntoView {
    let api_key = RwSignal::new(String::new());
    let show_key = RwSignal::new(false);
    let is_connecting = RwSignal::new(false);
    let error_message = RwSignal::new(String::new());
    
    let stored_api_key = RwSignal::new(String::new());
    #[cfg(target_arch = "wasm32")]
    {
        spawn_local(async move {
            if let Some(window) = web_sys::window() {
                if let Ok(Some(storage)) = window.local_storage() {
                        if let Ok(Some(key)) = storage.get_item("api_key") {
                            if !key.is_empty() {
                                stored_api_key.set(key);
                                let _ = window.location().set_href("/dashboard");
                            }
                        }
                }
            }
        });
    }

    let toggle_visibility = move |_| {
        show_key.update(|show| *show = !*show);
    };

    let connect_to_api = move |_| {
        let key = api_key.get();
        if key.is_empty() {
            error_message.set("Please enter your API key".to_string());
            return;
        }

        is_connecting.set(true);
        error_message.set(String::new());
        
        spawn_local(async move {
            #[cfg(target_arch = "wasm32")]
            {
                if let Some(window) = web_sys::window() {
                    if let Ok(Some(storage)) = window.local_storage() {
                        let _ = storage.set_item("api_key", &key);
                    }
                    let _ = window.location().set_href("/dashboard");
                }
            }
            
            #[cfg(not(target_arch = "wasm32"))]
            {
                error_message.set("Processing...".to_string());
            }
        });
    };

    view! {
        <div class="min-h-screen flex items-center justify-center p-4 sm:p-6 animate-fade-in">
            <div class="w-full max-w-xl">
                <div class="text-center mb-6 sm:mb-8 animate-slide-up">
                    <div class="flex items-center justify-center space-x-2 sm:space-x-3 mb-2">
                        <img src="/tclogo.png" alt="Torbox Companion Logo" class="w-6 h-6 sm:w-8 sm:h-8 object-contain"/>
                        <h1 class="text-2xl sm:text-3xl md:text-4xl font-bold text-white">"Torbox Companion"</h1>
                    </div>
                    <p class="text-slate-300 text-base sm:text-lg">"The power user's alternative for TorBox management"</p>
                    <p class="text-slate-400 text-xs sm:text-sm mt-2">"Built with Rust because why not?"</p>
                </div>

                <div class="bg-slate-800/50 border rounded-xl p-4 sm:p-6 md:p-8 shadow-2xl animate-slide-up">
                    <h2 class="text-xl sm:text-2xl font-semibold text-white mb-3 sm:mb-4">"Get Started"</h2>
                    <p class="text-slate-300 text-sm sm:text-base mb-4 sm:mb-6">"Enter your API key to begin managing your connections"</p>
                    
                    <Show when=move || !error_message.get().is_empty()>
                        <div class="mb-4 p-3 bg-red-900/20 border border-red-500/30 rounded-lg text-red-300 text-sm">
                            {move || error_message.get()}
                        </div>
                    </Show>
                    
                    <div class="mb-6">
                        <label class="block text-sm font-medium text-slate-300 mb-2">
                            "API Key"
                        </label>
                        <div class="password-container">
                            <input
                                type=move || if show_key.get() { "text" } else { "password" }
                                placeholder="Enter your API key"
                                class="password-input"
                                prop:value=move || api_key.get()
                                on:input=move |ev| api_key.set(event_target_value(&ev))
                                autocomplete="off"
                                spellcheck="false"
                            />
                            <button
                                type="button"
                                class="toggle-button"
                                on:click=toggle_visibility
                                aria-label=move || if show_key.get() { "Hide API key" } else { "Show API key" }
                            >
                                {move || if show_key.get() { 
                                    view! {
                                        <svg class="eye-icon" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                                            <path d="M1 12s4-8 11-8 11 8 11 8-4 8-11 8-11-8-11-8z"/>
                                            <circle cx="12" cy="12" r="3"/>
                                        </svg>
                                    }
                                } else { 
                                    view! {
                                        <svg class="eye-icon" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                                            <path d="M17.94 17.94A10.07 10.07 0 0 1 12 20c-7 0-11-8-11-8a18.45 18.45 0 0 1 5.06-5.94M9.9 4.24A9.12 9.12 0 0 1 12 4c7 0 11 8 11 8a18.5 18.5 0 0 1-2.16 3.19m-6.72-1.07a3 3 0 1 1-4.24-4.24"/>
                                            <line x1="1" y1="1" x2="23" y2="23"/>
                                        </svg>
                                    }
                                }}
                            </button>
                        </div>
                    </div>

                    <button
                        class=move || {
                            let base = "w-full font-medium py-2.5 sm:py-3 px-4 rounded-lg mb-3 sm:mb-4 text-sm sm:text-base";
                            if is_connecting.get() || api_key.get().is_empty() {
                                format!("{} bg-gray-500 text-gray-300 cursor-not-allowed", base)
                            } else {
                                format!("{} bg-blue-600 text-white hover:bg-blue-700 cursor-pointer", base)
                            }
                        }
                        disabled=move || is_connecting.get() || api_key.get().is_empty()
                        on:click=connect_to_api
                    >
                        {move || if is_connecting.get() { "Connecting..." } else { "Connect to API" }}
                    </button>

                    <div class="text-center">
                        <p class="text-slate-400 text-sm mb-3">
                            "Your API key is stored locally and never shared"
                        </p>
                        <p class="text-slate-400 text-sm">
                            "Get your API key from "
                            <a href="https://torbox.app/settings" class="text-blue-400 hover:text-blue-300 transition-colors" target="_blank" rel="noopener noreferrer">"Torbox Settings"</a>
                        </p>
                    </div>
                </div>

                <div class="mt-20 text-center text-slate-400 text-sm">
                    <div class="border-t border-slate-700/30 pt-8">
                        <p class="mb-4">
                            "Need a TorBox account? "
                            <a 
                                href="https://torbox.app/subscription?referral=09c3f0f3-4e61-4634-a6dc-40af39f8165c" 
                                class="text-blue-400 hover:text-blue-300 transition-colors" 
                                target="_blank" 
                                rel="noopener noreferrer"
                            >
                                "Sign up here"
                            </a>
                            " to get started!"
                        </p>
                        
                        <p class="mb-4">
                            "We don't store any of your information. This app is fully open source. "
                            <a href="https://github.com/crazycacti/torbox-companion" class="text-blue-400 hover:text-blue-300 transition-colors" target="_blank" rel="noopener noreferrer">"View on GitHub"</a>
                        </p>
                        
                        <p>
                            "This is a complete rewrite of "
                            <a href="https://github.com/jittarao/torbox-app" class="text-blue-400 hover:text-blue-300 transition-colors" target="_blank" rel="noopener noreferrer">"Torbox Manager"</a>
                            " "
                        </p>
                    </div>
                </div>
            </div>
        </div>
    }
}

async fn validate_api_key(api_key: &str) -> Result<bool, ApiError> {
    let client = TorboxClient::new(api_key.to_string());
    match client.get_user(None).await {
        Ok(_) => Ok(true),
        Err(ApiError::AuthenticationError) => Ok(false),
        Err(e) => Err(e),
    }
}
