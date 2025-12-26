use leptos::prelude::*;
use leptos::logging::log;
use std::sync::Arc;
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::JsCast;

pub type ConfirmationCallback = Arc<dyn Fn() + Send + Sync>;

#[derive(Clone)]
pub struct ConfirmationModalState {
    pub is_open: bool,
    pub title: String,
    pub message: String,
    pub confirm_text: String,
    pub cancel_text: String,
    pub variant: ConfirmationVariant,
    pub on_confirm: Option<ConfirmationCallback>,
}

impl Default for ConfirmationModalState {
    fn default() -> Self {
        Self {
            is_open: false,
            title: String::new(),
            message: String::new(),
            confirm_text: "Confirm".to_string(),
            cancel_text: "Cancel".to_string(),
            variant: ConfirmationVariant::Danger,
            on_confirm: None,
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum ConfirmationVariant {
    Danger,
    Warning,
    Info,
}

impl ConfirmationVariant {
    pub fn button_color(&self) -> &'static str {
        match self {
            ConfirmationVariant::Danger => "var(--error)",
            ConfirmationVariant::Warning => "var(--warning)",
            ConfirmationVariant::Info => "var(--info)",
        }
    }

    pub fn button_hover_color(&self) -> &'static str {
        match self {
            ConfirmationVariant::Danger => "#dc2626",
            ConfirmationVariant::Warning => "#d97706",
            ConfirmationVariant::Info => "#2563eb",
        }
    }
}

#[component]
pub fn ConfirmationModal() -> impl IntoView {
    let state = use_context::<RwSignal<ConfirmationModalState>>()
        .unwrap_or_else(|| {
            log!("ConfirmationModalState not found in ConfirmationModal, using fallback");
            RwSignal::new(ConfirmationModalState::default())
        });

    let close_modal = move |_| {
        state.update(|s| {
            s.is_open = false;
            s.on_confirm = None;
        });
    };

    let handle_confirm = move |_| {
        let current_state = state.get_untracked();
        if let Some(callback) = current_state.on_confirm {
            callback();
        }
        state.update(|s| {
            s.is_open = false;
            s.on_confirm = None;
        });
    };

    view! {
        <Show when=move || state.get().is_open>
            <div
                style="position: fixed !important; top: 0 !important; left: 0 !important; right: 0 !important; bottom: 0 !important; width: 100vw !important; height: 100vh !important; background-color: rgba(0, 0, 0, 0.75) !important; backdrop-filter: blur(4px) !important; z-index: 2147483647 !important; display: flex !important; align-items: center !important; justify-content: center !important; padding: 1rem; overflow-y: auto;"
                on:click=close_modal
            >
                <div
                    class="rounded-xl border max-w-md w-full shadow-2xl modal-content"
                    style="background-color: var(--bg-card) !important; border-color: var(--border-secondary) !important; z-index: 2147483647 !important; position: relative !important;"
                    on:click=|ev| ev.stop_propagation()
                >
                    <div class="p-6">
                        <div class="mb-4">
                            <h3
                                class="text-xl font-semibold mb-2"
                                style="color: var(--text-primary);"
                            >
                                {move || state.get().title}
                            </h3>
                            <p
                                class="text-sm"
                                style="color: var(--text-secondary); line-height: 1.6;"
                            >
                                {move || state.get().message}
                            </p>
                        </div>
                        <div class="flex gap-3 justify-end">
                            <button
                                class="px-4 py-2 rounded-lg transition-colors font-medium text-sm"
                                style="background-color: var(--bg-tertiary); color: var(--text-primary); border: 1px solid var(--border-secondary);"
                                on:click=close_modal
                            >
                                {move || state.get().cancel_text}
                            </button>
                            <button
                                class="px-4 py-2 rounded-lg transition-colors font-medium text-sm text-white"
                                style=move || format!(
                                    "background-color: {};",
                                    state.get().variant.button_color()
                                )
                                on:click=handle_confirm
                                on:mouseenter=move |ev| {
                                    #[cfg(target_arch = "wasm32")]
                                    {
                                        let variant = state.get().variant.clone();
                                        let hover_color = variant.button_hover_color();
                                        ev.target()
                                            .and_then(|t| t.dyn_into::<web_sys::HtmlElement>().ok())
                                            .map(|el| el.set_attribute("style", &format!("background-color: {};", hover_color)).ok());
                                    }
                                }
                                on:mouseleave=move |ev| {
                                    #[cfg(target_arch = "wasm32")]
                                    {
                                        let variant = state.get().variant.clone();
                                        let color = variant.button_color();
                                        ev.target()
                                            .and_then(|t| t.dyn_into::<web_sys::HtmlElement>().ok())
                                            .map(|el| el.set_attribute("style", &format!("background-color: {};", color)).ok());
                                    }
                                }
                            >
                                {move || state.get().confirm_text}
                            </button>
                        </div>
                    </div>
                </div>
            </div>
        </Show>
    }
}

