use leptos::prelude::*;
#[cfg(target_arch = "wasm32")]
use web_sys;
use crate::dashboard::DashboardContext;

#[component]
pub fn UsersTab() -> impl IntoView {
    // Get the dashboard context
    let context = use_context::<DashboardContext>()
        .expect("DashboardContext should be provided");
    
    let user_data = context.user_data;
    let user_loading = context.user_loading;
    
    // Helper function to format dates
    let format_date = move |date_str: String| -> String {
        if date_str.is_empty() {
            return "N/A".to_string();
        }
        
        // Try to parse ISO 8601 date and format it nicely
        if let Ok(parsed) = chrono::DateTime::parse_from_rfc3339(&date_str) {
            parsed.format("%B %d, %Y at %I:%M %p").to_string()
        } else {
            date_str 
        }
    };


    view! {
        <div class="flex flex-col items-center w-full mt-10 sm:mt-12">
            <Show when=move || user_loading.get()>
                <div class="flex items-center justify-center py-12">
                    <div class="flex items-center space-x-2" style="color: var(--text-secondary);">
                        <div class="w-4 h-4 border-2 border-t-transparent rounded-full animate-spin" style="border-color: var(--text-secondary);"></div>
                        <span>"Loading user data..."</span>
                    </div>
                </div>
            </Show>
            
            <Show when=move || !user_loading.get() && user_data.get().is_some()>
                <div class="w-full mx-auto">
                    <div class="grid grid-cols-1 md:grid-cols-2 gap-4 sm:gap-6">
                // Basic Information
                <div class="rounded-xl p-4 sm:p-6 border" style="background-color: var(--bg-card); border-color: var(--border-secondary);">
                    <h3 class="text-lg sm:text-xl font-semibold mb-3 sm:mb-4" style="color: var(--text-primary);">"Basic Information"</h3>
                    <div class="space-y-2 sm:space-y-2">
                        <div class="flex flex-col sm:flex-row sm:justify-between gap-1 sm:gap-0">
                            <span class="text-sm sm:text-base" style="color: var(--text-secondary);">"User ID:"</span>
                            <span class="text-sm sm:text-base break-all sm:break-normal" style="color: var(--text-primary);">{move || user_data.get().map(|u| u.id.to_string()).unwrap_or_default()}</span>
                        </div>
                        <div class="flex flex-col sm:flex-row sm:justify-between gap-1 sm:gap-0">
                            <span class="text-sm sm:text-base" style="color: var(--text-secondary);">"Email:"</span>
                            <span class="text-sm sm:text-base break-all sm:break-normal" style="color: var(--text-primary);">{move || user_data.get().map(|u| u.email.clone()).unwrap_or_default()}</span>
                        </div>
                        <div class="flex flex-col sm:flex-row sm:justify-between gap-1 sm:gap-0">
                            <span class="text-sm sm:text-base" style="color: var(--text-secondary);">"Plan:"</span>
                            <span class="text-sm sm:text-base" style="color: var(--text-primary);">{move || {
                                user_data.get().map(|u| {
                                    match u.plan {
                                        0 => "Free".to_string(),
                                        1 => "Essential".to_string(),
                                        2 => "Pro".to_string(),
                                        3 => "Standard".to_string(),
                                        _ => format!("Plan {}", u.plan),
                                    }
                                }).unwrap_or_default()
                            }}</span>
                        </div>
                        <div class="flex flex-col sm:flex-row sm:justify-between gap-2 sm:gap-0 sm:items-center">
                            <span class="text-sm sm:text-base" style="color: var(--text-secondary);">"Referral Code:"</span>
                            <div class="flex items-center space-x-2">
                                <span class="text-sm sm:text-base break-all sm:break-normal" style="color: var(--text-primary);">{move || user_data.get().map(|u| u.user_referral.clone()).unwrap_or_default()}</span>
                                <button
                                    class="transition-colors p-1.5 sm:p-1 rounded-lg hover:bg-opacity-10 hover:bg-white"
                                    style="color: var(--text-secondary);"
                                    on:click=move |_| {
                                        #[cfg(target_arch = "wasm32")]
                                        {
                                            if let Some(window) = web_sys::window() {
                                                if let Some(referral_code) = user_data.get().map(|u| u.user_referral.clone()) {
                                                    let clipboard = window.navigator().clipboard();
                                                    let _ = clipboard.write_text(&referral_code);
                                                }
                                            }
                                        }
                                    }
                                    title="Copy referral code"
                                >
                                    <svg class="w-4 h-4 sm:w-4 sm:h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M8 16H6a2 2 0 01-2-2V6a2 2 0 012-2h8a2 2 0 012 2v2m-6 12h8a2 2 0 002-2v-8a2 2 0 00-2-2h-8a2 2 0 00-2 2v8a2 2 0 002 2z"></path>
                                    </svg>
                                </button>
                            </div>
                        </div>
                    </div>
                </div>
                
                // Account Statistics
                <div class="rounded-xl p-4 sm:p-6 border" style="background-color: var(--bg-card); border-color: var(--border-secondary);">
                    <h3 class="text-lg sm:text-xl font-semibold mb-3 sm:mb-4" style="color: var(--text-primary);">"Account Statistics"</h3>
                    <div class="space-y-2 sm:space-y-2">
                        <div class="flex flex-col sm:flex-row sm:justify-between gap-1 sm:gap-0">
                            <span class="text-sm sm:text-base" style="color: var(--text-secondary);">"Total Downloads:"</span>
                            <span class="text-sm sm:text-base" style="color: var(--text-primary);">{move || user_data.get().map(|u| u.total_downloaded.to_string()).unwrap_or_default()}</span>
                        </div>
                        <div class="flex flex-col sm:flex-row sm:justify-between gap-1 sm:gap-0">
                            <span class="text-sm sm:text-base" style="color: var(--text-secondary);">"Torrents Downloaded:"</span>
                            <span class="text-sm sm:text-base" style="color: var(--text-primary);">{move || user_data.get().map(|u| u.torrents_downloaded.to_string()).unwrap_or_default()}</span>
                        </div>
                        <div class="flex flex-col sm:flex-row sm:justify-between gap-1 sm:gap-0">
                            <span class="text-sm sm:text-base" style="color: var(--text-secondary);">"Web Downloads:"</span>
                            <span class="text-sm sm:text-base" style="color: var(--text-primary);">{move || user_data.get().map(|u| u.web_downloads_downloaded.to_string()).unwrap_or_default()}</span>
                        </div>
                        <div class="flex flex-col sm:flex-row sm:justify-between gap-1 sm:gap-0">
                            <span class="text-sm sm:text-base" style="color: var(--text-secondary);">"Usenet Downloads:"</span>
                            <span class="text-sm sm:text-base" style="color: var(--text-primary);">{move || user_data.get().map(|u| u.usenet_downloads_downloaded.to_string()).unwrap_or_default()}</span>
                        </div>
                        <div class="flex flex-col sm:flex-row sm:justify-between gap-1 sm:gap-0">
                            <span class="text-sm sm:text-base" style="color: var(--text-secondary);">"Referrals:"</span>
                            <span class="text-sm sm:text-base" style="color: var(--text-primary);">{move || user_data.get().map(|u| u.purchases_referred.to_string()).unwrap_or_default()}</span>
                        </div>
                    </div>
                </div>
                
                // Data Usage
                <div class="rounded-xl p-4 sm:p-6 border" style="background-color: var(--bg-card); border-color: var(--border-secondary);">
                    <h3 class="text-lg sm:text-xl font-semibold mb-3 sm:mb-4" style="color: var(--text-primary);">"Data Usage"</h3>
                    <div class="space-y-2 sm:space-y-2">
                        <div class="flex flex-col sm:flex-row sm:justify-between gap-1 sm:gap-0">
                            <span class="text-sm sm:text-base" style="color: var(--text-secondary);">"Downloaded:"</span>
                            <span class="text-sm sm:text-base break-all sm:break-normal" style="color: var(--text-primary);">{move || {
                                user_data.get().map(|u| {
                                    let bytes = u.total_bytes_downloaded;
                                    if bytes >= 1_099_511_627_776 { // 1 TB
                                        format!("{:.2} TB", bytes as f64 / 1_099_511_627_776.0)
                                    } else if bytes >= 1_073_741_824 { // 1 GB
                                        format!("{:.2} GB", bytes as f64 / 1_073_741_824.0)
                                    } else if bytes >= 1_048_576 { // 1 MB
                                        format!("{:.2} MB", bytes as f64 / 1_048_576.0)
                                    } else if bytes >= 1024 { // 1 KB
                                        format!("{:.2} KB", bytes as f64 / 1024.0)
                                    } else {
                                        format!("{} bytes", bytes)
                                    }
                                }).unwrap_or_default()
                            }}</span>
                        </div>
                        <div class="flex flex-col sm:flex-row sm:justify-between gap-1 sm:gap-0">
                            <span class="text-sm sm:text-base" style="color: var(--text-secondary);">"Uploaded:"</span>
                            <span class="text-sm sm:text-base break-all sm:break-normal" style="color: var(--text-primary);">{move || {
                                user_data.get().map(|u| {
                                    let bytes = u.total_bytes_uploaded;
                                    if bytes >= 1_099_511_627_776 { // 1 TB
                                        format!("{:.2} TB", bytes as f64 / 1_099_511_627_776.0)
                                    } else if bytes >= 1_073_741_824 { // 1 GB
                                        format!("{:.2} GB", bytes as f64 / 1_073_741_824.0)
                                    } else if bytes >= 1_048_576 { // 1 MB
                                        format!("{:.2} MB", bytes as f64 / 1_048_576.0)
                                    } else if bytes >= 1024 { // 1 KB
                                        format!("{:.2} KB", bytes as f64 / 1024.0)
                                    } else {
                                        format!("{} bytes", bytes)
                                    }
                                }).unwrap_or_default()
                            }}</span>
                        </div>
                        <div class="flex flex-col sm:flex-row sm:justify-between gap-1 sm:gap-0">
                            <span class="text-sm sm:text-base" style="color: var(--text-secondary);">"Upload/Download Ratio:"</span>
                            <span class="text-sm sm:text-base" style="color: var(--text-primary);">{move || {
                                user_data.get().map(|u| {
                                    let downloaded = u.total_bytes_downloaded as f64;
                                    let uploaded = u.total_bytes_uploaded as f64;
                                    
                                    if downloaded == 0.0 {
                                        "N/A".to_string()
                                    } else {
                                        let ratio = uploaded / downloaded;
                                        format!("{:.3}", ratio)
                                    }
                                }).unwrap_or_default()
                            }}</span>
                        </div>
                    </div>
                </div>
                
                // Account Details
                <div class="rounded-xl p-4 sm:p-6 border" style="background-color: var(--bg-card); border-color: var(--border-secondary);">
                    <h3 class="text-lg sm:text-xl font-semibold mb-3 sm:mb-4" style="color: var(--text-primary);">"Account Details"</h3>
                    <div class="space-y-2 sm:space-y-2">
                        <div class="flex flex-col sm:flex-row sm:justify-between gap-1 sm:gap-0">
                            <span class="text-sm sm:text-base" style="color: var(--text-secondary);">"Created:"</span>
                            <span class="text-sm sm:text-base break-all sm:break-normal" style="color: var(--text-primary);">{move || user_data.get().map(|u| format_date(u.created_at.clone())).unwrap_or_default()}</span>
                        </div>
                        <div class="flex flex-col sm:flex-row sm:justify-between gap-1 sm:gap-0">
                            <span class="text-sm sm:text-base" style="color: var(--text-secondary);">"Premium Expires:"</span>
                            <span class="text-sm sm:text-base break-all sm:break-normal" style="color: var(--text-primary);">{move || user_data.get().map(|u| format_date(u.premium_expires_at.clone())).unwrap_or_default()}</span>
                        </div>
                        <div class="flex flex-col sm:flex-row sm:justify-between gap-1 sm:gap-0">
                            <span class="text-sm sm:text-base" style="color: var(--text-secondary);">"Vendor:"</span>
                            <span class="text-sm sm:text-base" style="color: var(--text-primary);">{move || user_data.get().map(|u| if u.is_vendor { "Yes" } else { "No" }).unwrap_or_default()}</span>
                        </div>
                    </div>
                </div>
                    </div>
                </div>
            </Show>
            
            <Show when=move || !user_loading.get() && user_data.get().is_none()>
                <div class="text-center py-12">
                    <div style="color: var(--text-secondary);">"Failed to load user data. Please check your API connection."</div>
                </div>
            </Show>

            <div class="w-full mx-auto mt-8 sm:mt-10">
                <div class="grid grid-cols-1 md:grid-cols-2 gap-4 sm:gap-6">
                    // Developer Section
                    <div class="rounded-xl p-4 sm:p-6 border" style="background-color: var(--bg-card); border-color: var(--border-secondary);">
                        <h3 class="text-lg sm:text-xl font-semibold mb-3 sm:mb-4" style="color: var(--text-primary);">"Developer"</h3>
                        <div class="space-y-3 sm:space-y-3">
                            <a
                                href="https://github.com/crazycacti/torbox-companion"
                                target="_blank"
                                rel="noopener noreferrer"
                                class="flex items-center space-x-2 text-sm sm:text-base transition-colors hover:opacity-80"
                                style="color: var(--text-primary);"
                            >
                                <svg class="w-5 h-5" fill="currentColor" viewBox="0 0 24 24">
                                    <path fill-rule="evenodd" d="M12 2C6.477 2 2 6.484 2 12.017c0 4.425 2.865 8.18 6.839 9.504.5.092.682-.217.682-.483 0-.237-.008-.868-.013-1.703-2.782.605-3.369-1.343-3.369-1.343-.454-1.158-1.11-1.466-1.11-1.466-.908-.62.069-.608.069-.608 1.003.07 1.531 1.032 1.531 1.032.892 1.53 2.341 1.088 2.91.832.092-.647.35-1.088.636-1.338-2.22-.253-4.555-1.113-4.555-4.951 0-1.093.39-1.988 1.029-2.688-.103-.253-.446-1.272.098-2.65 0 0 .84-.27 2.75 1.026A9.564 9.564 0 0112 6.844c.85.004 1.705.115 2.504.337 1.909-1.296 2.747-1.027 2.747-1.027.546 1.379.202 2.398.1 2.651.64.7 1.028 1.595 1.028 2.688 0 3.848-2.339 4.695-4.566 4.943.359.309.678.92.678 1.855 0 1.338-.012 2.419-.012 2.747 0 .268.18.58.688.482A10.019 10.019 0 0022 12.017C22 6.484 17.522 2 12 2z" clip-rule="evenodd"/>
                                </svg>
                                <span>"GitHub"</span>
                                <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M10 6H6a2 2 0 00-2 2v10a2 2 0 002 2h10a2 2 0 002-2v-4M14 4h6m0 0v6m0-6L10 14"></path>
                                </svg>
                            </a>
                            <a
                                href="https://buymeacoffee.com/crazy1"
                                target="_blank"
                                rel="noopener noreferrer"
                                class="flex items-center space-x-2 text-sm sm:text-base transition-colors hover:opacity-80"
                                style="color: var(--text-primary);"
                            >
                                <svg class="w-5 h-5" fill="currentColor" viewBox="0 0 24 24">
                                    <path d="M12 21.35l-1.45-1.32C5.4 15.36 2 12.28 2 8.5 2 5.42 4.42 3 7.5 3c1.74 0 3.41.81 4.5 2.09C13.09 3.81 14.76 3 16.5 3 19.58 3 22 5.42 22 8.5c0 3.78-3.4 6.86-8.55 11.54L12 21.35z"/>
                                </svg>
                                <span>"Support"</span>
                                <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M10 6H6a2 2 0 00-2 2v10a2 2 0 002 2h10a2 2 0 002-2v-4M14 4h6m0 0v6m0-6L10 14"></path>
                                </svg>
                            </a>
                            <div class="flex flex-col space-y-2 pt-2 border-t" style="border-color: var(--border-secondary);">
                                <div class="flex flex-col sm:flex-row sm:justify-between gap-1 sm:gap-0 sm:items-center">
                                    <span class="text-sm sm:text-base" style="color: var(--text-secondary);">"Referral Link:"</span>
                                    <div class="flex items-center space-x-2">
                                        <a
                                            href="https://torbox.app/subscription?referral=09c3f0f3-4e61-4634-a6dc-40af39f8165c"
                                            target="_blank"
                                            rel="noopener noreferrer"
                                            class="text-sm sm:text-base break-all sm:break-normal transition-colors hover:opacity-80"
                                            style="color: var(--accent-primary);"
                                        >
                                            "View Link"
                                        </a>
                                        <button
                                            class="transition-colors p-1.5 sm:p-1 rounded-lg hover:bg-opacity-10 hover:bg-white"
                                            style="color: var(--text-secondary);"
                                            on:click=move |_| {
                                                #[cfg(target_arch = "wasm32")]
                                                {
                                                    if let Some(window) = web_sys::window() {
                                                        let clipboard = window.navigator().clipboard();
                                                        let _ = clipboard.write_text("https://torbox.app/subscription?referral=09c3f0f3-4e61-4634-a6dc-40af39f8165c");
                                                    }
                                                }
                                            }
                                            title="Copy referral link"
                                        >
                                            <svg class="w-4 h-4 sm:w-4 sm:h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M8 16H6a2 2 0 01-2-2V6a2 2 0 012-2h8a2 2 0 012 2v2m-6 12h8a2 2 0 002-2v-8a2 2 0 00-2-2h-8a2 2 0 00-2 2v8a2 2 0 002 2z"></path>
                                            </svg>
                                        </button>
                                    </div>
                                </div>
                                <div class="flex flex-col sm:flex-row sm:justify-between gap-1 sm:gap-0 sm:items-center">
                                    <span class="text-sm sm:text-base" style="color: var(--text-secondary);">"Referral Code:"</span>
                                    <div class="flex items-center space-x-2">
                                        <span class="text-sm sm:text-base break-all sm:break-normal" style="color: var(--text-primary);">"09c3f0f3-4e61-4634-a6dc-40af39f8165c"</span>
                                        <button
                                            class="transition-colors p-1.5 sm:p-1 rounded-lg hover:bg-opacity-10 hover:bg-white"
                                            style="color: var(--text-secondary);"
                                            on:click=move |_| {
                                                #[cfg(target_arch = "wasm32")]
                                                {
                                                    if let Some(window) = web_sys::window() {
                                                        let clipboard = window.navigator().clipboard();
                                                        let _ = clipboard.write_text("09c3f0f3-4e61-4634-a6dc-40af39f8165c");
                                                    }
                                                }
                                            }
                                            title="Copy referral code"
                                        >
                                            <svg class="w-4 h-4 sm:w-4 sm:h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M8 16H6a2 2 0 01-2-2V6a2 2 0 012-2h8a2 2 0 012 2v2m-6 12h8a2 2 0 002-2v-8a2 2 0 00-2-2h-8a2 2 0 00-2 2v8a2 2 0 002 2z"></path>
                                            </svg>
                                        </button>
                                    </div>
                                </div>
                            </div>
                        </div>
                    </div>

                    // Docs & Links Section
                    <div class="rounded-xl p-4 sm:p-6 border" style="background-color: var(--bg-card); border-color: var(--border-secondary);">
                        <h3 class="text-lg sm:text-xl font-semibold mb-3 sm:mb-4" style="color: var(--text-primary);">"Documentation & Links"</h3>
                        <div class="space-y-3 sm:space-y-3">
                            <a
                                href="https://www.postman.com/torbox/torbox/overview"
                                target="_blank"
                                rel="noopener noreferrer"
                                class="flex items-center space-x-2 text-sm sm:text-base transition-colors hover:opacity-80"
                                style="color: var(--text-primary);"
                            >
                                <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 6.253v13m0-13C10.832 5.477 9.246 5 7.5 5S4.168 5.477 3 6.253v13C4.168 18.477 5.754 18 7.5 18s3.332.477 4.5 1.253m0-13C13.168 5.477 14.754 5 16.5 5c1.747 0 3.332.477 4.5 1.253v13C19.832 18.477 18.247 18 16.5 18c-1.746 0-3.332.477-4.5 1.253"></path>
                                </svg>
                                <span>"API Documentation"</span>
                                <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M10 6H6a2 2 0 00-2 2v10a2 2 0 002 2h10a2 2 0 002-2v-4M14 4h6m0 0v6m0-6L10 14"></path>
                                </svg>
                            </a>
                            <a
                                href="https://torbox.app/settings"
                                target="_blank"
                                rel="noopener noreferrer"
                                class="flex items-center space-x-2 text-sm sm:text-base transition-colors hover:opacity-80"
                                style="color: var(--text-primary);"
                            >
                                <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M10.325 4.317c.426-1.756 2.924-1.756 3.35 0a1.724 1.724 0 002.573 1.066c1.543-.94 3.31.826 2.37 2.37a1.724 1.724 0 001.065 2.572c1.756.426 1.756 2.924 0 3.35a1.724 1.724 0 00-1.066 2.573c.94 1.543-.826 3.31-2.37 2.37a1.724 1.724 0 00-2.572 1.065c-.426 1.756-2.924 1.756-3.35 0a1.724 1.724 0 00-2.573-1.066c-1.543.94-3.31-.826-2.37-2.37a1.724 1.724 0 00-1.065-2.572c-1.756-.426-1.756-2.924 0-3.35a1.724 1.724 0 001.066-2.573c-.94-1.543.826-3.31 2.37-2.37.996.608 2.296.07 2.572-1.065z"></path>
                                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15 12a3 3 0 11-6 0 3 3 0 016 0z"></path>
                                </svg>
                                <span>"TorBox Settings"</span>
                                <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M10 6H6a2 2 0 00-2 2v10a2 2 0 002 2h10a2 2 0 002-2v-4M14 4h6m0 0v6m0-6L10 14"></path>
                                </svg>
                            </a>
                        </div>
                    </div>
                </div>
            </div>
        </div>
    }
}
