use crate::themes::{Theme, ThemeVariant, ColorPalette};

pub struct CatppuccinTheme;

impl CatppuccinTheme {
    pub fn create() -> Theme {
        Theme::new(
            ThemeVariant::Catppuccin,
            "Catppuccin".to_string(),
            ColorPalette {
                // Background colors - Catppuccin Mocha palette
                bg_primary: "#1e1e2e".to_string(),      // Base
                bg_secondary: "#181825".to_string(),    // Mantle
                bg_tertiary: "#313244".to_string(),     // Surface0
                bg_card: "rgba(49, 50, 68, 0.8)".to_string(),     // Surface0 with opacity
                bg_card_hover: "rgba(69, 71, 90, 0.9)".to_string(), // Surface1 with opacity
                
                // Text colors
                text_primary: "#cdd6f4".to_string(),     // Text
                text_secondary: "#bac2de".to_string(), // Subtext1
                text_muted: "#a6adc8".to_string(),      // Subtext0
                text_accent: "#89b4fa".to_string(),      // Blue
                
                // Border colors
                border_primary: "rgba(205, 214, 244, 0.1)".to_string(),
                border_secondary: "rgba(205, 214, 244, 0.05)".to_string(),
                border_focus: "rgba(137, 180, 250, 0.5)".to_string(),
                
                // Accent colors - Catppuccin colors
                accent_primary: "#89b4fa".to_string(),   // Blue
                accent_hover: "#74c7ec".to_string(),      // Sapphire
                accent_secondary: "#a6e3a1".to_string(), // Green
                accent_warning: "#f9e2af".to_string(),   // Yellow
                accent_danger: "#f38ba8".to_string(),    // Red
                
                // Status colors
                success: "#a6e3a1".to_string(),          // Green
                error: "#f38ba8".to_string(),            // Red
                warning: "#f9e2af".to_string(),          // Yellow
                info: "#89b4fa".to_string(),             // Blue
                
                // Progress bar colors
                progress_bg: "#313244".to_string(),
                progress_fill: "#89b4fa".to_string(),
                progress_border: "rgba(205, 214, 244, 0.1)".to_string(),
            }
        )
    }
}
