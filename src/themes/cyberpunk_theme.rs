use crate::themes::{Theme, ThemeVariant, ColorPalette};

pub struct CyberpunkTheme;

impl CyberpunkTheme {
    pub fn create() -> Theme {
        Theme::new(
            ThemeVariant::Cyberpunk,
            "Cyberpunk".to_string(),
            ColorPalette {
                bg_primary: "#0a0a0a".to_string(),
                bg_secondary: "#141414".to_string(),
                bg_tertiary: "#1e1e1e".to_string(),
                bg_card: "rgba(30, 30, 30, 0.9)".to_string(),
                bg_card_hover: "rgba(40, 30, 50, 0.95)".to_string(),
                
                text_primary: "#00ffff".to_string(),
                text_secondary: "#ff00ff".to_string(),
                text_muted: "#808080".to_string(),
                text_accent: "#ffff00".to_string(),
                
                border_primary: "rgba(0, 255, 255, 0.3)".to_string(),
                border_secondary: "rgba(255, 0, 255, 0.2)".to_string(),
                border_focus: "rgba(255, 255, 0, 0.6)".to_string(),
                
                accent_primary: "#ff00ff".to_string(),
                accent_hover: "#ff33ff".to_string(),
                accent_secondary: "#00ffff".to_string(),
                accent_warning: "#ffff00".to_string(),
                accent_danger: "#ff0040".to_string(),
                
                success: "#00ff00".to_string(),
                error: "#ff0040".to_string(),
                warning: "#ffff00".to_string(),
                info: "#00ffff".to_string(),
                
                progress_bg: "#1e1e1e".to_string(),
                progress_fill: "#ff00ff".to_string(),
                progress_border: "rgba(255, 0, 255, 0.3)".to_string(),
            }
        )
    }
}

