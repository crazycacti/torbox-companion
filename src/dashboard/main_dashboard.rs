use leptos::prelude::*;
use crate::dashboard::{DashboardHeader, DashboardContext};
use crate::dashboard::tabs::{OverviewTab, UsersTab, ThemesTab, AutomationsTab};
use crate::themes::ThemeManager;

#[component]
pub fn MainDashboard() -> impl IntoView {
    let context = DashboardContext::new();
    provide_context(context.clone());
    
    let theme_manager = ThemeManager::new();
    provide_context(theme_manager.clone());
    
    let context = use_context::<DashboardContext>()
        .expect("DashboardContext should be provided");
    
    let active_tab = context.active_tab;

    view! {
        <div class="min-h-screen" style="background-color: var(--bg-primary);">
            <DashboardHeader/>
            
            <main class="p-4 sm:p-5 md:p-6">
                <Show when=move || active_tab.get() == "overview">
                    <OverviewTab/>
                </Show>
                
                <Show when=move || active_tab.get() == "user">
                    <UsersTab/>
                </Show>
                
                <Show when=move || active_tab.get() == "themes">
                    <ThemesTab/>
                </Show>
                
                <Show when=move || active_tab.get() == "automations">
                    <AutomationsTab/>
                </Show>
            </main>
        </div>
    }
}