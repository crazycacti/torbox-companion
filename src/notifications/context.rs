use leptos::prelude::*;
use leptos::logging::log;
use std::collections::HashMap;
use crate::notifications::notification::Notification;

#[derive(Clone)]
pub struct NotificationContext {
    notifications: RwSignal<HashMap<usize, Notification>>,
    next_id: RwSignal<usize>,
}

impl NotificationContext {
    pub fn new() -> Self {
        Self {
            notifications: RwSignal::new(HashMap::new()),
            next_id: RwSignal::new(0),
        }
    }

    pub fn add(&self, notification: Notification) {
        let mut notif = notification;
        let id = self.next_id.get_untracked();
        notif.id = id;
        
        self.notifications.update(|notifications| {
            notifications.insert(id, notif.clone());
        });
        
        self.next_id.set(id + 1);

        if let Some(duration) = notif.duration {
            let notifications_clone = self.notifications.clone();
            let id_clone = id;
            
            #[cfg(target_arch = "wasm32")]
            {
                use wasm_bindgen::prelude::*;
                let closure = Closure::wrap(Box::new(move || {
                    notifications_clone.update(|notifications| {
                        notifications.remove(&id_clone);
                    });
                }) as Box<dyn FnMut()>);
                
                if let Some(window) = web_sys::window() {
                    let _ = window.set_timeout_with_callback_and_timeout_and_arguments_0(
                        closure.as_ref().unchecked_ref(),
                        duration as i32,
                    );
                }
                
                closure.forget();
            }
        }
    }

    pub fn success(&self, message: String) {
        self.add(Notification::success(message));
    }

    pub fn error(&self, message: String) {
        self.add(Notification::error(message));
    }

    pub fn warning(&self, message: String) {
        self.add(Notification::warning(message));
    }

    pub fn info(&self, message: String) {
        self.add(Notification::info(message));
    }

    pub fn remove(&self, id: usize) {
        self.notifications.update(|notifications| {
            notifications.remove(&id);
        });
    }

    pub fn clear(&self) {
        self.notifications.update(|notifications| {
            notifications.clear();
        });
    }

    pub fn get_notifications(&self) -> ReadSignal<HashMap<usize, Notification>> {
        self.notifications.read_only()
    }
}

pub fn use_notification() -> NotificationContext {
    use_context::<NotificationContext>()
        .unwrap_or_else(|| {
            log!("NotificationContext not found, using fallback");
            NotificationContext::new()
        })
}

