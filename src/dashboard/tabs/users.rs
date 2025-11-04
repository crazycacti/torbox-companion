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
            date_str // Return original if parsing fails
        }
    };

    // User data is already loaded in DashboardContext by the header
    // No need to fetch again - just use the shared context

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
        </div>
    }
}
