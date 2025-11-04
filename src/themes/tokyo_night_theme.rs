use crate::themes::{Theme, ThemeVariant, ColorPalette};

pub struct TokyoNightTheme;

impl TokyoNightTheme {
    pub fn create() -> Theme {
        Theme::new(
            ThemeVariant::TokyoNight,
            "Tokyo Night".to_string(),
            ColorPalette {
                // Background colors - Tokyo Night palette (more blue-purple tinted)
                bg_primary: "#16161e".to_string(),      // Deep blue-purple background
                bg_secondary: "#1f2335".to_string(),    // Blue-purple secondary
                bg_tertiary: "#292e42".to_string(),     // Blue-purple tertiary
                bg_card: "rgba(41, 46, 66, 0.8)".to_string(),     // Blue-purple card
                bg_card_hover: "rgba(52, 58, 81, 0.9)".to_string(), // Blue-purple hover
                
                // Text colors - Tokyo Night (warmer, more purple-tinted)
                text_primary: "#c9cbff".to_string(),     // Warmer purple-tinted text
                text_secondary: "#a9b1d6".to_string(),   // Purple-gray
                text_muted: "#565f89".to_string(),      // Muted purple
                text_accent: "#4dabf7".to_string(),      // Bright saturated blue
                
                // Border colors - Tokyo Night (purple-tinted)
                border_primary: "rgba(201, 203, 255, 0.15)".to_string(),
                border_secondary: "rgba(201, 203, 255, 0.08)".to_string(),
                border_focus: "rgba(77, 171, 247, 0.6)".to_string(),
                
                // Accent colors - Tokyo Night signature colors (more saturated, deeper)
                accent_primary: "#4dabf7".to_string(),   // Bright saturated blue
                accent_hover: "#339af0".to_string(),    // Deeper blue
                accent_secondary: "#51cf66".to_string(),  // Vibrant green
                accent_warning: "#ffd43b".to_string(),   // Bright yellow
                accent_danger: "#ff6b6b".to_string(),    // Vibrant red
                
                // Status colors
                success: "#51cf66".to_string(),          // Vibrant green
                error: "#ff6b6b".to_string(),            // Vibrant red
                warning: "#ffd43b".to_string(),          // Bright yellow
                info: "#4dabf7".to_string(),             // Bright saturated blue
                
                // Progress bar colors
                progress_bg: "#2f3549".to_string(),
                progress_fill: "#4dabf7".to_string(),
                progress_border: "rgba(192, 202, 245, 0.1)".to_string(),
            }
        )
    }
}
