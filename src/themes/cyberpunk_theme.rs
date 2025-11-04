use crate::themes::{Theme, ThemeVariant, ColorPalette};

pub struct CyberpunkTheme;

impl CyberpunkTheme {
    pub fn create() -> Theme {
        Theme::new(
            ThemeVariant::Cyberpunk,
            "Cyberpunk".to_string(),
            ColorPalette {
                // Background colors - Cyberpunk dark theme
                bg_primary: "#0a0a0a".to_string(),      // Very dark
                bg_secondary: "#141414".to_string(),    // Dark gray
                bg_tertiary: "#1e1e1e".to_string(),     // Medium dark
                bg_card: "rgba(30, 30, 30, 0.9)".to_string(),     // Card with neon glow effect potential
                bg_card_hover: "rgba(40, 30, 50, 0.95)".to_string(), // Purple-tinted hover
                
                // Text colors
                text_primary: "#00ffff".to_string(),     // Cyan
                text_secondary: "#ff00ff".to_string(),   // Magenta
                text_muted: "#808080".to_string(),      // Gray
                text_accent: "#ffff00".to_string(),      // Yellow
                
                // Border colors - Neon borders
                border_primary: "rgba(0, 255, 255, 0.3)".to_string(),
                border_secondary: "rgba(255, 0, 255, 0.2)".to_string(),
                border_focus: "rgba(255, 255, 0, 0.6)".to_string(),
                
                // Accent colors - Cyberpunk neon colors
                accent_primary: "#ff00ff".to_string(),   // Magenta
                accent_hover: "#ff33ff".to_string(),    // Bright magenta
                accent_secondary: "#00ffff".to_string(), // Cyan
                accent_warning: "#ffff00".to_string(),    // Yellow
                accent_danger: "#ff0040".to_string(),     // Hot pink-red
                
                // Status colors
                success: "#00ff00".to_string(),          // Neon green
                error: "#ff0040".to_string(),            // Hot pink-red
                warning: "#ffff00".to_string(),          // Yellow
                info: "#00ffff".to_string(),             // Cyan
                
                // Progress bar colors
                progress_bg: "#1e1e1e".to_string(),
                progress_fill: "#ff00ff".to_string(),
                progress_border: "rgba(255, 0, 255, 0.3)".to_string(),
            }
        )
    }
}

