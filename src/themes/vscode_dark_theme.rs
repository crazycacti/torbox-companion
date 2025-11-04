use crate::themes::{Theme, ThemeVariant, ColorPalette};

pub struct VSCodeDarkTheme;

impl VSCodeDarkTheme {
    pub fn create() -> Theme {
        Theme::new(
            ThemeVariant::VSCodeDark,
            "VS Code Dark+".to_string(),
            ColorPalette {
                // Background colors - VS Code Dark+ palette (neutral gray, signature)
                bg_primary: "#1e1e1e".to_string(),      // Neutral dark gray (VS Code signature)
                bg_secondary: "#252526".to_string(),    // Neutral gray secondary
                bg_tertiary: "#2d2d30".to_string(),     // Neutral gray tertiary
                bg_card: "rgba(45, 45, 48, 0.8)".to_string(),     // Neutral gray card
                bg_card_hover: "rgba(62, 62, 66, 0.9)".to_string(), // Neutral gray hover
                
                // Text colors - VS Code (neutral grays)
                text_primary: "#d4d4d4".to_string(),     // Neutral light gray (VS Code signature)
                text_secondary: "#858585".to_string(),   // Neutral medium gray
                text_muted: "#6a9955".to_string(),      // Green comment (VS Code signature)
                text_accent: "#007acc".to_string(),      // Link color
                
                // Border colors
                border_primary: "rgba(204, 204, 204, 0.1)".to_string(),
                border_secondary: "rgba(204, 204, 204, 0.05)".to_string(),
                border_focus: "rgba(0, 122, 204, 0.5)".to_string(),
                
                // Accent colors - VS Code Dark+ signature colors (more distinct)
                accent_primary: "#007acc".to_string(),   // VS Code blue (keep signature)
                accent_hover: "#005a9e".to_string(),    // Deeper blue
                accent_secondary: "#73c991".to_string(), // Brighter green
                accent_warning: "#dcdcaa".to_string(),    // VS Code yellow (less saturated)
                accent_danger: "#f48771".to_string(),     // Red (keep)
                
                // Status colors
                success: "#73c991".to_string(),          // Brighter green
                error: "#f48771".to_string(),            // Error red
                warning: "#dcdcaa".to_string(),          // Warning yellow
                info: "#007acc".to_string(),             // Info blue
                
                // Progress bar colors
                progress_bg: "#2d2d30".to_string(),
                progress_fill: "#007acc".to_string(),
                progress_border: "rgba(204, 204, 204, 0.1)".to_string(),
            }
        )
    }
}

