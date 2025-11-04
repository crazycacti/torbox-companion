use crate::themes::{Theme, ThemeVariant, ColorPalette};

pub struct MaterialDarkTheme;

impl MaterialDarkTheme {
    pub fn create() -> Theme {
        Theme::new(
            ThemeVariant::MaterialDark,
            "Material Dark".to_string(),
            ColorPalette {
                // Background colors - Material Dark palette (warmer brown-gray)
                bg_primary: "#1a1a1a".to_string(),      // Warm dark gray (slightly brown)
                bg_secondary: "#242424".to_string(),    // Warm gray secondary
                bg_tertiary: "#2e2e2e".to_string(),     // Warm gray tertiary
                bg_card: "rgba(46, 46, 46, 0.8)".to_string(),     // Warm gray card
                bg_card_hover: "rgba(66, 66, 66, 0.9)".to_string(), // Warm gray hover
                
                // Text colors - Material Dark (slightly warmer white)
                text_primary: "#ffffff".to_string(),     // Pure white
                text_secondary: "rgba(255, 255, 255, 0.87)".to_string(),   // High emphasis (Material)
                text_muted: "rgba(255, 255, 255, 0.6)".to_string(),      // Medium emphasis
                text_accent: "#2196f3".to_string(),      // Blue 500
                
                // Border colors
                border_primary: "rgba(255, 255, 255, 0.12)".to_string(),
                border_secondary: "rgba(255, 255, 255, 0.06)".to_string(),
                border_focus: "rgba(33, 150, 243, 0.5)".to_string(),
                
                // Accent colors - Material Design vibrant colors
                accent_primary: "#2196f3".to_string(),   // Blue 500 (more vibrant)
                accent_hover: "#1976d2".to_string(),    // Blue 700 (deeper)
                accent_secondary: "#4caf50".to_string(), // Green 500 (more vibrant)
                accent_warning: "#ff9800".to_string(),    // Orange 500 (vibrant)
                accent_danger: "#f44336".to_string(),     // Red 500 (vibrant)
                
                // Status colors
                success: "#4caf50".to_string(),          // Green 500 (vibrant)
                error: "#f44336".to_string(),            // Red 500 (vibrant)
                warning: "#ff9800".to_string(),          // Orange 500 (vibrant)
                info: "#2196f3".to_string(),             // Blue 500 (vibrant)
                
                // Progress bar colors
                progress_bg: "#2d2d2d".to_string(),
                progress_fill: "#2196f3".to_string(),
                progress_border: "rgba(255, 255, 255, 0.12)".to_string(),
            }
        )
    }
}

