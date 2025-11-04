use crate::themes::{Theme, ThemeVariant, ColorPalette};

pub struct OneDarkTheme;

impl OneDarkTheme {
    pub fn create() -> Theme {
        Theme::new(
            ThemeVariant::OneDark,
            "One Dark Pro".to_string(),
            ColorPalette {
                bg_primary: "#21252b".to_string(),
                bg_secondary: "#282c34".to_string(),
                bg_tertiary: "#2c313c".to_string(),
                bg_card: "rgba(44, 49, 60, 0.8)".to_string(),
                bg_card_hover: "rgba(62, 68, 81, 0.9)".to_string(),
                
                text_primary: "#abb2bf".to_string(),
                text_secondary: "#5c6370".to_string(),
                text_muted: "#4b5263".to_string(),
                text_accent: "#528bcc".to_string(),
                
                border_primary: "rgba(171, 178, 191, 0.12)".to_string(),
                border_secondary: "rgba(171, 178, 191, 0.06)".to_string(),
                border_focus: "rgba(82, 139, 204, 0.6)".to_string(),
                
                accent_primary: "#528bcc".to_string(),
                accent_hover: "#3d6fa8".to_string(),
                accent_secondary: "#7cb342".to_string(),
                accent_warning: "#ffb74d".to_string(),
                accent_danger: "#d32f2f".to_string(),
                
                success: "#7cb342".to_string(),
                error: "#d32f2f".to_string(),
                warning: "#ffb74d".to_string(),
                info: "#528bcc".to_string(),
                
                progress_bg: "#3e4451".to_string(),
                progress_fill: "#528bcc".to_string(),
                progress_border: "rgba(171, 178, 191, 0.1)".to_string(),
            }
        )
    }
}
