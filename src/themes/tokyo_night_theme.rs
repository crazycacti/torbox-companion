use crate::themes::{Theme, ThemeVariant, ColorPalette};

pub struct TokyoNightTheme;

impl TokyoNightTheme {
    pub fn create() -> Theme {
        Theme::new(
            ThemeVariant::TokyoNight,
            "Tokyo Night".to_string(),
            ColorPalette {
                bg_primary: "#16161e".to_string(),
                bg_secondary: "#1f2335".to_string(),
                bg_tertiary: "#292e42".to_string(),
                bg_card: "rgba(41, 46, 66, 0.8)".to_string(),
                bg_card_hover: "rgba(52, 58, 81, 0.9)".to_string(),
                
                text_primary: "#c9cbff".to_string(),
                text_secondary: "#a9b1d6".to_string(),
                text_muted: "#565f89".to_string(),
                text_accent: "#4dabf7".to_string(),
                
                border_primary: "rgba(201, 203, 255, 0.15)".to_string(),
                border_secondary: "rgba(201, 203, 255, 0.08)".to_string(),
                border_focus: "rgba(77, 171, 247, 0.6)".to_string(),
                
                accent_primary: "#4dabf7".to_string(),
                accent_hover: "#339af0".to_string(),
                accent_secondary: "#51cf66".to_string(),
                accent_warning: "#ffd43b".to_string(),
                accent_danger: "#ff6b6b".to_string(),
                
                success: "#51cf66".to_string(),
                error: "#ff6b6b".to_string(),
                warning: "#ffd43b".to_string(),
                info: "#4dabf7".to_string(),
                
                progress_bg: "#2f3549".to_string(),
                progress_fill: "#4dabf7".to_string(),
                progress_border: "rgba(192, 202, 245, 0.1)".to_string(),
            }
        )
    }
}
