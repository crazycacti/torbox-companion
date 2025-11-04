use crate::themes::{Theme, ThemeVariant, ColorPalette};

pub struct TorBoxTheme;

impl TorBoxTheme {
    pub fn create() -> Theme {
        Theme::new(
            ThemeVariant::TorBox,
            "TorBox".to_string(),
            ColorPalette {
                bg_primary: "#0f1419".to_string(),
                bg_secondary: "#1a1f2e".to_string(),
                bg_tertiary: "#252b38".to_string(),
                bg_card: "rgba(37, 43, 56, 0.9)".to_string(),
                bg_card_hover: "rgba(45, 52, 66, 0.95)".to_string(),
                
                text_primary: "#ffffff".to_string(),
                text_secondary: "#b3b8c4".to_string(),
                text_muted: "#8a8f9a".to_string(),
                text_accent: "#059669".to_string(),
                
                border_primary: "rgba(255, 255, 255, 0.1)".to_string(),
                border_secondary: "rgba(255, 255, 255, 0.05)".to_string(),
                border_focus: "rgba(5, 150, 105, 0.5)".to_string(),
                
                accent_primary: "#059669".to_string(),
                accent_hover: "#047857".to_string(),
                accent_secondary: "#065f46".to_string(),
                accent_warning: "#ffaa00".to_string(),
                accent_danger: "#ff3333".to_string(),
                
                success: "#059669".to_string(),
                error: "#ff3333".to_string(),
                warning: "#ffaa00".to_string(),
                info: "#60a5fa".to_string(),
                
                progress_bg: "#252b38".to_string(),
                progress_fill: "#059669".to_string(),
                progress_border: "rgba(255, 255, 255, 0.1)".to_string(),
            }
        )
    }
}

