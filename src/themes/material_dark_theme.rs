use crate::themes::{Theme, ThemeVariant, ColorPalette};

pub struct MaterialDarkTheme;

impl MaterialDarkTheme {
    pub fn create() -> Theme {
        Theme::new(
            ThemeVariant::MaterialDark,
            "Material Dark".to_string(),
            ColorPalette {
                bg_primary: "#1a1a1a".to_string(),
                bg_secondary: "#242424".to_string(),
                bg_tertiary: "#2e2e2e".to_string(),
                bg_card: "rgba(46, 46, 46, 0.8)".to_string(),
                bg_card_hover: "rgba(66, 66, 66, 0.9)".to_string(),
                
                text_primary: "#ffffff".to_string(),
                text_secondary: "rgba(255, 255, 255, 0.87)".to_string(),
                text_muted: "rgba(255, 255, 255, 0.6)".to_string(),
                text_accent: "#2196f3".to_string(),
                
                border_primary: "rgba(255, 255, 255, 0.12)".to_string(),
                border_secondary: "rgba(255, 255, 255, 0.06)".to_string(),
                border_focus: "rgba(33, 150, 243, 0.5)".to_string(),
                
                accent_primary: "#2196f3".to_string(),
                accent_hover: "#1976d2".to_string(),
                accent_secondary: "#4caf50".to_string(),
                accent_warning: "#ff9800".to_string(),
                accent_danger: "#f44336".to_string(),
                
                success: "#4caf50".to_string(),
                error: "#f44336".to_string(),
                warning: "#ff9800".to_string(),
                info: "#2196f3".to_string(),
                
                progress_bg: "#2d2d2d".to_string(),
                progress_fill: "#2196f3".to_string(),
                progress_border: "rgba(255, 255, 255, 0.12)".to_string(),
            }
        )
    }
}

