use leptos::prelude::*;
use leptos::logging::log;
use crate::notifications::confirmation_modal::{ConfirmationModalState, ConfirmationVariant};
use std::sync::Arc;

pub fn use_confirmation() -> RwSignal<ConfirmationModalState> {
    use_context::<RwSignal<ConfirmationModalState>>()
        .unwrap_or_else(|| {
            log!("ConfirmationModalState not found, using fallback");
            RwSignal::new(ConfirmationModalState::default())
        })
}

pub fn show_confirmation(
    state: RwSignal<ConfirmationModalState>,
    title: String,
    message: String,
    on_confirm: impl Fn() + Send + Sync + 'static,
    variant: ConfirmationVariant,
    confirm_text: Option<String>,
    cancel_text: Option<String>,
) {
    state.update(|s| {
        s.is_open = true;
        s.title = title;
        s.message = message;
        s.variant = variant;
        s.confirm_text = confirm_text.unwrap_or_else(|| "Confirm".to_string());
        s.cancel_text = cancel_text.unwrap_or_else(|| "Cancel".to_string());
        s.on_confirm = Some(Arc::new(on_confirm));
    });
}

