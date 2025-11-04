use crate::themes::{Theme, ThemeVariant, ColorPalette};

pub struct CatppuccinTheme;

impl CatppuccinTheme {
    pub fn create() -> Theme {
        Theme::new(
            ThemeVariant::Catppuccin,
            "Catppuccin".to_string(),
            ColorPalette {
                bg_primary: "#1e1e2e".to_string(),
                bg_secondary: "#181825".to_string(),
                bg_tertiary: "#313244".to_string(),
                bg_card: "rgba(49, 50, 68, 0.8)".to_string(),
                bg_card_hover: "rgba(69, 71, 90, 0.9)".to_string(),
                
                text_primary: "#cdd6f4".to_string(),
                text_secondary: "#bac2de".to_string(),
                text_muted: "#a6adc8".to_string(),
                text_accent: "#89b4fa".to_string(),
                
                border_primary: "rgba(205, 214, 244, 0.1)".to_string(),
                border_secondary: "rgba(205, 214, 244, 0.05)".to_string(),
                border_focus: "rgba(137, 180, 250, 0.5)".to_string(),
                
                accent_primary: "#89b4fa".to_string(),
                accent_hover: "#74c7ec".to_string(),
                accent_secondary: "#a6e3a1".to_string(),
                accent_warning: "#f9e2af".to_string(),
                accent_danger: "#f38ba8".to_string(),
                
                success: "#a6e3a1".to_string(),
                error: "#f38ba8".to_string(),
                warning: "#f9e2af".to_string(),
                info: "#89b4fa".to_string(),
                
                progress_bg: "#313244".to_string(),
                progress_fill: "#89b4fa".to_string(),
                progress_border: "rgba(205, 214, 244, 0.1)".to_string(),
            }
        )
    }
}
