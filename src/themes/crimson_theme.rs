use crate::themes::{Theme, ThemeVariant, ColorPalette};

pub struct CrimsonTheme;

impl CrimsonTheme {
    pub fn create() -> Theme {
        Theme::new(
            ThemeVariant::Crimson,
            "Crimson".to_string(),
            ColorPalette {
                bg_primary: "#000000".to_string(),
                bg_secondary: "#1a1a1a".to_string(),
                bg_tertiary: "#2d2d2d".to_string(),
                bg_card: "rgba(45, 45, 45, 0.9)".to_string(),
                bg_card_hover: "rgba(61, 61, 61, 0.95)".to_string(),
                
                text_primary: "#ffffff".to_string(),
                text_secondary: "#cccccc".to_string(),
                text_muted: "#999999".to_string(),
                text_accent: "#dc143c".to_string(),
                
                border_primary: "rgba(255, 255, 255, 0.1)".to_string(),
                border_secondary: "rgba(255, 255, 255, 0.05)".to_string(),
                border_focus: "rgba(220, 20, 60, 0.5)".to_string(),
                
                accent_primary: "#dc143c".to_string(),
                accent_hover: "#ff1744".to_string(),
                accent_secondary: "#b81d24".to_string(),
                accent_warning: "#ff6b00".to_string(),
                accent_danger: "#dc143c".to_string(),
                
                success: "#46d369".to_string(),
                error: "#dc143c".to_string(),
                warning: "#ff6b00".to_string(),
                info: "#0071eb".to_string(),
                
                progress_bg: "#2d2d2d".to_string(),
                progress_fill: "#dc143c".to_string(),
                progress_border: "rgba(255, 255, 255, 0.1)".to_string(),
            }
        )
    }
}

