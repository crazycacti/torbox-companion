#![recursion_limit = "768"]

pub mod app;
pub mod api;
pub mod landing_page;
pub mod dashboard;
pub mod themes;
pub mod stream_page;
pub mod notifications;

#[cfg(feature = "ssr")]
pub mod logging;

#[cfg(feature = "hydrate")]
#[wasm_bindgen::prelude::wasm_bindgen]
pub fn hydrate() {
    use crate::app::*;
    console_error_panic_hook::set_once();
    leptos::mount::hydrate_body(App);
}
