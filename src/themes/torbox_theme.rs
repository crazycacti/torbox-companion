use crate::themes::{Theme, ThemeVariant, ColorPalette};

pub struct TorBoxTheme;

impl TorBoxTheme {
    pub fn create() -> Theme {
        Theme::new(
            ThemeVariant::TorBox,
            "TorBox".to_string(),
            ColorPalette {
                // Background colors - TorBox dark theme (dark blue/black with grey tones)
                bg_primary: "#0f1419".to_string(),      // Dark blue-black
                bg_secondary: "#1a1f2e".to_string(),    // Dark blue-gray secondary
                bg_tertiary: "#252b38".to_string(),     // Medium blue-gray
                bg_card: "rgba(37, 43, 56, 0.9)".to_string(),     // Card background (blue-gray)
                bg_card_hover: "rgba(45, 52, 66, 0.95)".to_string(), // Card hover (lighter blue-gray)
                
                // Text colors
                text_primary: "#ffffff".to_string(),     // White
                text_secondary: "#b3b8c4".to_string(),   // Light gray-blue
                text_muted: "#8a8f9a".to_string(),      // Medium gray
                text_accent: "#059669".to_string(),      // Dark muted green (not neon)
                
                // Border colors
                border_primary: "rgba(255, 255, 255, 0.1)".to_string(),
                border_secondary: "rgba(255, 255, 255, 0.05)".to_string(),
                border_focus: "rgba(5, 150, 105, 0.5)".to_string(),
                
                // Accent colors - TorBox signature colors (dark muted green, not bright/neon)
                accent_primary: "#059669".to_string(),   // Dark muted green
                accent_hover: "#047857".to_string(),    // Darker green for hover
                accent_secondary: "#065f46".to_string(), // Even darker green
                accent_warning: "#ffaa00".to_string(),    // Orange
                accent_danger: "#ff3333".to_string(),     // Red
                
                // Status colors
                success: "#059669".to_string(),          // Dark muted green
                error: "#ff3333".to_string(),            // Red
                warning: "#ffaa00".to_string(),          // Orange
                info: "#60a5fa".to_string(),             // Blue
                
                // Progress bar colors
                progress_bg: "#252b38".to_string(),
                progress_fill: "#059669".to_string(),
                progress_border: "rgba(255, 255, 255, 0.1)".to_string(),
            }
        )
    }
}

