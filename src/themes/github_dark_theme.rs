use crate::themes::{Theme, ThemeVariant, ColorPalette};

pub struct GitHubDarkTheme;

impl GitHubDarkTheme {
    pub fn create() -> Theme {
        Theme::new(
            ThemeVariant::GitHubDark,
            "GitHub Dark".to_string(),
            ColorPalette {
                // Background colors - GitHub Dark palette (cool blue-gray, GitHub signature)
                bg_primary: "#0d1117".to_string(),      // Cool dark blue-gray (GitHub signature)
                bg_secondary: "#161b22".to_string(),    // Cool blue-gray secondary
                bg_tertiary: "#21262d".to_string(),     // Cool blue-gray tertiary
                bg_card: "rgba(33, 38, 45, 0.8)".to_string(),     // Cool blue-gray card
                bg_card_hover: "rgba(48, 54, 61, 0.9)".to_string(), // Cool blue-gray hover
                
                // Text colors - GitHub Dark (cool blue-tinted grays)
                text_primary: "#c9d1d9".to_string(),     // Cool blue-gray text (GitHub signature)
                text_secondary: "#b1bac4".to_string(),   // Cool blue-gray secondary
                text_muted: "#8b949e".to_string(),      // Cool blue-gray muted
                text_accent: "#1f6feb".to_string(),      // Brighter blue
                
                // Border colors
                border_primary: "rgba(240, 246, 252, 0.1)".to_string(),
                border_secondary: "rgba(240, 246, 252, 0.05)".to_string(),
                border_focus: "rgba(31, 111, 235, 0.5)".to_string(),
                
                // Accent colors - GitHub Dark signature colors (more saturated)
                accent_primary: "#1f6feb".to_string(),   // GitHub's brighter blue
                accent_hover: "#1158c7".to_string(),    // Deeper blue
                accent_secondary: "#238636".to_string(), // GitHub's brighter green
                accent_warning: "#d29922".to_string(),    // Golden yellow (keep)
                accent_danger: "#da3633".to_string(),     // GitHub's darker red
                
                // Status colors
                success: "#238636".to_string(),          // Brighter green
                error: "#da3633".to_string(),            // Darker red
                warning: "#d29922".to_string(),          // Golden yellow
                info: "#1f6feb".to_string(),             // Brighter blue
                
                // Progress bar colors
                progress_bg: "#21262d".to_string(),
                progress_fill: "#1f6feb".to_string(),
                progress_border: "rgba(240, 246, 252, 0.1)".to_string(),
            }
        )
    }
}

