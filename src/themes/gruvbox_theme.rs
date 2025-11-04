use crate::themes::{Theme, ThemeVariant, ColorPalette};

pub struct GruvboxTheme;

impl GruvboxTheme {
    pub fn create() -> Theme {
        Theme::new(
            ThemeVariant::Gruvbox,
            "Gruvbox Dark".to_string(),
            ColorPalette {
                // Background colors - Gruvbox Dark palette
                bg_primary: "#282828".to_string(),      // bg0
                bg_secondary: "#1d2021".to_string(),    // bg0_h
                bg_tertiary: "#3c3836".to_string(),     // bg1
                bg_card: "rgba(60, 56, 54, 0.8)".to_string(),     // bg1 with opacity
                bg_card_hover: "rgba(80, 73, 69, 0.9)".to_string(), // bg2 with opacity
                
                // Text colors
                text_primary: "#ebdbb2".to_string(),     // fg0
                text_secondary: "#d5c4a1".to_string(),   // fg1
                text_muted: "#a89984".to_string(),      // fg3
                text_accent: "#83a598".to_string(),      // Blue
                
                // Border colors
                border_primary: "rgba(235, 219, 178, 0.1)".to_string(),
                border_secondary: "rgba(235, 219, 178, 0.05)".to_string(),
                border_focus: "rgba(131, 165, 152, 0.5)".to_string(),
                
                // Accent colors - Gruvbox signature colors
                accent_primary: "#83a598".to_string(),   // Blue
                accent_hover: "#689d6a".to_string(),     // Green
                accent_secondary: "#98971a".to_string(), // Yellow (lime)
                accent_warning: "#d79921".to_string(),    // Orange
                accent_danger: "#cc241d".to_string(),     // Red
                
                // Status colors
                success: "#98971a".to_string(),          // Yellow (lime) - Gruvbox uses yellow for success
                error: "#cc241d".to_string(),            // Red
                warning: "#d79921".to_string(),          // Orange
                info: "#83a598".to_string(),             // Blue
                
                // Progress bar colors
                progress_bg: "#3c3836".to_string(),
                progress_fill: "#83a598".to_string(),
                progress_border: "rgba(235, 219, 178, 0.1)".to_string(),
            }
        )
    }
}
