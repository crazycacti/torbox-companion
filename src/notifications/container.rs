use leptos::prelude::*;
use crate::notifications::context::NotificationContext;

#[component]
pub fn NotificationContainer() -> impl IntoView {
    let context = use_context::<NotificationContext>()
        .expect("NotificationContext must be provided");
    
    let notifications = context.get_notifications();

    view! {
        <div
            class="fixed top-4 right-4 z-[9999] flex flex-col gap-2 pointer-events-none"
            style="max-width: 400px; width: calc(100% - 2rem);"
        >
            <For
                each=move || {
                    let mut items: Vec<_> = notifications.get().values().cloned().collect();
                    items.sort_by_key(|n| n.id);
                    items
                }
                key=|notif| notif.id
                children=move |notif| {
                    let notif_type = notif.notification_type.clone();
                    let message = notif.message.clone();
                    let id = notif.id;
                    let context_clone = context.clone();
                    
                    let bg_color = notif_type.bg_color().to_string();
                    let icon_color = notif_type.icon_color().to_string();
                    let text_color = notif_type.text_color().to_string();
                    let icon = notif_type.icon().to_string();
                    
                    let icon_color_clone = icon_color.clone();
                    
                    view! {
                        <div
                            class="pointer-events-auto notification-toast animate-slide-in-right"
                            style=move || format!(
                                "background-color: {}; border-left: 4px solid {};",
                                bg_color, icon_color_clone
                            )
                        >
                            <div class="flex items-start gap-3">
                                <div
                                    class="flex-shrink-0 w-6 h-6 flex items-center justify-center rounded-full notification-icon"
                                    style=move || format!(
                                        "background-color: {};",
                                        icon_color
                                    )
                                >
                                    <span class="text-white text-xs font-semibold">{icon}</span>
                                </div>
                                <div class="flex-1 min-w-0 py-0.5">
                                    <p
                                        class="text-sm font-medium break-words notification-message"
                                        style=move || format!("color: {};", text_color)
                                    >
                                        {message}
                                    </p>
                                </div>
                                <button
                                    class="flex-shrink-0 w-5 h-5 flex items-center justify-center text-xs hover:opacity-70 transition-opacity rounded-full hover:bg-black/10 notification-close"
                                    style="color: var(--text-secondary);"
                                    on:click=move |_| context_clone.remove(id)
                                    aria-label="Close notification"
                                >
                                    <svg class="w-3.5 h-3.5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12"></path>
                                    </svg>
                                </button>
                            </div>
                        </div>
                    }
                }
            />
        </div>
    }
}

