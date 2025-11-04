pub mod theme_manager;
pub mod theme_types;
pub mod dark_theme;
pub mod light_theme;
pub mod catppuccin_theme;
pub mod custom_theme;
pub mod tokyo_night_theme;
pub mod github_dark_theme;
pub mod one_dark_theme;
pub mod torbox_theme;
pub mod cyberpunk_theme;
pub mod crimson_theme;

pub use theme_manager::ThemeManager;
pub use theme_types::{Theme, ThemeVariant, ColorPalette};
