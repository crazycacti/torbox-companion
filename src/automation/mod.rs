#[cfg(feature = "ssr")]
pub mod database;
#[cfg(feature = "ssr")]
pub mod encryption;
#[cfg(feature = "ssr")]
pub mod engine;
#[cfg(feature = "ssr")]
pub mod routes;
#[cfg(feature = "ssr")]
pub mod scheduler;
#[cfg(feature = "ssr")]
pub mod types;

#[cfg(feature = "ssr")]
pub use database::Database;
#[cfg(feature = "ssr")]
pub use engine::AutomationEngine;
#[cfg(feature = "ssr")]
pub use routes::{create_routes, AppState};
#[cfg(feature = "ssr")]
pub use scheduler::AutomationScheduler;
#[cfg(feature = "ssr")]
pub use types::*;
