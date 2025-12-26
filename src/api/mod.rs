pub mod client;
pub mod types;
pub mod endpoints;
pub mod request_handler;
pub mod examples;
pub mod error_messages;

#[cfg(feature = "ssr")]
pub mod server_routes;

#[cfg(feature = "ssr")]
pub mod proxy;

pub use client::TorboxClient;
pub use types::*;
pub use request_handler::{RequestHandler, create_handler, create_handler_with_config, create_handler_with_user_ip, demonstrate_all_apis};
pub use error_messages::{format_api_error, get_error_summary};
