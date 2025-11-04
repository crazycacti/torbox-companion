use crate::themes::{Theme, ThemeVariant, ColorPalette};

pub struct DarkTheme;

impl DarkTheme {
    pub fn create() -> Theme {
        Theme::new(
            ThemeVariant::Dark,
            "Dark".to_string(),
            ColorPalette {
                // Background colors - Deep dark theme
                bg_primary: "#0a0a0a".to_string(),
                bg_secondary: "#111111".to_string(),
                bg_tertiary: "#1a1a1a".to_string(),
                bg_card: "rgba(17, 17, 17, 0.8)".to_string(),
                bg_card_hover: "rgba(26, 26, 26, 0.9)".to_string(),
                
                // Text colors
                text_primary: "#ffffff".to_string(),
                text_secondary: "#a3a3a3".to_string(),
                text_muted: "#737373".to_string(),
                text_accent: "#60a5fa".to_string(),
                
                // Border colors
                border_primary: "rgba(255, 255, 255, 0.1)".to_string(),
                border_secondary: "rgba(255, 255, 255, 0.05)".to_string(),
                border_focus: "rgba(96, 165, 250, 0.5)".to_string(),
                
                // Accent colors
                accent_primary: "#3b82f6".to_string(),
                accent_hover: "#2563eb".to_string(),
                accent_secondary: "#10b981".to_string(),
                accent_warning: "#f59e0b".to_string(),
                accent_danger: "#ef4444".to_string(),
                
                // Status colors
                success: "#10b981".to_string(),
                error: "#ef4444".to_string(),
                warning: "#f59e0b".to_string(),
                info: "#3b82f6".to_string(),
                
                // Progress bar colors
                progress_bg: "#1a1a1a".to_string(),
                progress_fill: "#3b82f6".to_string(),
                progress_border: "rgba(255, 255, 255, 0.1)".to_string(),
            }
        )
    }
}
