use crate::themes::{Theme, ThemeVariant, ColorPalette};

pub struct OneDarkTheme;

impl OneDarkTheme {
    pub fn create() -> Theme {
        Theme::new(
            ThemeVariant::OneDark,
            "One Dark Pro".to_string(),
            ColorPalette {
                // Background colors - One Dark Pro palette (cooler gray-blue)
                bg_primary: "#21252b".to_string(),      // Cooler gray-blue background
                bg_secondary: "#282c34".to_string(),    // Gray-blue secondary
                bg_tertiary: "#2c313c".to_string(),     // Gray-blue tertiary
                bg_card: "rgba(44, 49, 60, 0.8)".to_string(),     // Gray-blue card
                bg_card_hover: "rgba(62, 68, 81, 0.9)".to_string(), // Gray-blue hover
                
                // Text colors - One Dark (cooler gray)
                text_primary: "#abb2bf".to_string(),     // Cool gray
                text_secondary: "#5c6370".to_string(),  // Cooler muted gray
                text_muted: "#4b5263".to_string(),      // Cooler comment
                text_accent: "#528bcc".to_string(),      // Deeper blue
                
                // Border colors - One Dark (cool gray)
                border_primary: "rgba(171, 178, 191, 0.12)".to_string(),
                border_secondary: "rgba(171, 178, 191, 0.06)".to_string(),
                border_focus: "rgba(82, 139, 204, 0.6)".to_string(),
                
                // Accent colors - One Dark signature colors (more contrast, less pastel)
                accent_primary: "#528bcc".to_string(),   // Deeper, more saturated blue
                accent_hover: "#3d6fa8".to_string(),    // Darker blue
                accent_secondary: "#7cb342".to_string(), // More vibrant green
                accent_warning: "#ffb74d".to_string(),   // Vibrant orange-yellow
                accent_danger: "#d32f2f".to_string(),     // Stronger red
                
                // Status colors
                success: "#7cb342".to_string(),          // More vibrant green
                error: "#d32f2f".to_string(),            // Stronger red
                warning: "#ffb74d".to_string(),          // Vibrant orange-yellow
                info: "#528bcc".to_string(),             // Deeper blue
                
                // Progress bar colors
                progress_bg: "#3e4451".to_string(),
                progress_fill: "#528bcc".to_string(),
                progress_border: "rgba(171, 178, 191, 0.1)".to_string(),
            }
        )
    }
}
