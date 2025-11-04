use crate::themes::{Theme, ThemeVariant, ColorPalette};

pub struct GitHubDarkTheme;

impl GitHubDarkTheme {
    pub fn create() -> Theme {
        Theme::new(
            ThemeVariant::GitHubDark,
            "GitHub Dark".to_string(),
            ColorPalette {
                bg_primary: "#0d1117".to_string(),
                bg_secondary: "#161b22".to_string(),
                bg_tertiary: "#21262d".to_string(),
                bg_card: "rgba(33, 38, 45, 0.8)".to_string(),
                bg_card_hover: "rgba(48, 54, 61, 0.9)".to_string(),
                
                text_primary: "#c9d1d9".to_string(),
                text_secondary: "#b1bac4".to_string(),
                text_muted: "#8b949e".to_string(),
                text_accent: "#1f6feb".to_string(),
                
                border_primary: "rgba(240, 246, 252, 0.1)".to_string(),
                border_secondary: "rgba(240, 246, 252, 0.05)".to_string(),
                border_focus: "rgba(31, 111, 235, 0.5)".to_string(),
                
                accent_primary: "#1f6feb".to_string(),
                accent_hover: "#1158c7".to_string(),
                accent_secondary: "#238636".to_string(),
                accent_warning: "#d29922".to_string(),
                accent_danger: "#da3633".to_string(),
                
                success: "#238636".to_string(),
                error: "#da3633".to_string(),
                warning: "#d29922".to_string(),
                info: "#1f6feb".to_string(),
                
                progress_bg: "#21262d".to_string(),
                progress_fill: "#1f6feb".to_string(),
                progress_border: "rgba(240, 246, 252, 0.1)".to_string(),
            }
        )
    }
}

