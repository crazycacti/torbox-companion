use crate::themes::{Theme, ThemeVariant, ColorPalette};

pub struct CrimsonTheme;

impl CrimsonTheme {
    pub fn create() -> Theme {
        Theme::new(
            ThemeVariant::Crimson,
            "Crimson".to_string(),
            ColorPalette {
                // Background colors - Crimson dark theme
                bg_primary: "#000000".to_string(),      // Deep black
                bg_secondary: "#1a1a1a".to_string(),    // Dark gray
                bg_tertiary: "#2d2d2d".to_string(),     // Medium gray
                bg_card: "rgba(45, 45, 45, 0.9)".to_string(),     // Card background
                bg_card_hover: "rgba(61, 61, 61, 0.95)".to_string(), // Card hover
                
                // Text colors
                text_primary: "#ffffff".to_string(),     // White
                text_secondary: "#cccccc".to_string(),   // Light gray
                text_muted: "#999999".to_string(),      // Medium gray
                text_accent: "#dc143c".to_string(),      // Crimson red
                
                // Border colors
                border_primary: "rgba(255, 255, 255, 0.1)".to_string(),
                border_secondary: "rgba(255, 255, 255, 0.05)".to_string(),
                border_focus: "rgba(220, 20, 60, 0.5)".to_string(),
                
                // Accent colors - Crimson signature colors
                accent_primary: "#dc143c".to_string(),   // Crimson red
                accent_hover: "#ff1744".to_string(),    // Brighter crimson
                accent_secondary: "#b81d24".to_string(), // Dark red
                accent_warning: "#ff6b00".to_string(),    // Orange
                accent_danger: "#dc143c".to_string(),     // Crimson red
                
                // Status colors
                success: "#46d369".to_string(),          // Green
                error: "#dc143c".to_string(),            // Crimson red
                warning: "#ff6b00".to_string(),          // Orange
                info: "#0071eb".to_string(),             // Blue
                
                // Progress bar colors
                progress_bg: "#2d2d2d".to_string(),
                progress_fill: "#dc143c".to_string(),
                progress_border: "rgba(255, 255, 255, 0.1)".to_string(),
            }
        )
    }
}

