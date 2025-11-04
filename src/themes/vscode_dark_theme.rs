use crate::themes::{Theme, ThemeVariant, ColorPalette};

pub struct VSCodeDarkTheme;

impl VSCodeDarkTheme {
    pub fn create() -> Theme {
        Theme::new(
            ThemeVariant::VSCodeDark,
            "VS Code Dark+".to_string(),
            ColorPalette {
                bg_primary: "#1e1e1e".to_string(),
                bg_secondary: "#252526".to_string(),
                bg_tertiary: "#2d2d30".to_string(),
                bg_card: "rgba(45, 45, 48, 0.8)".to_string(),
                bg_card_hover: "rgba(62, 62, 66, 0.9)".to_string(),
                
                text_primary: "#d4d4d4".to_string(),
                text_secondary: "#858585".to_string(),
                text_muted: "#6a9955".to_string(),
                text_accent: "#007acc".to_string(),
                
                border_primary: "rgba(204, 204, 204, 0.1)".to_string(),
                border_secondary: "rgba(204, 204, 204, 0.05)".to_string(),
                border_focus: "rgba(0, 122, 204, 0.5)".to_string(),
                
                accent_primary: "#007acc".to_string(),
                accent_hover: "#005a9e".to_string(),
                accent_secondary: "#73c991".to_string(),
                accent_warning: "#dcdcaa".to_string(),
                accent_danger: "#f48771".to_string(),
                
                success: "#73c991".to_string(),
                error: "#f48771".to_string(),
                warning: "#dcdcaa".to_string(),
                info: "#007acc".to_string(),
                
                progress_bg: "#2d2d30".to_string(),
                progress_fill: "#007acc".to_string(),
                progress_border: "rgba(204, 204, 204, 0.1)".to_string(),
            }
        )
    }
}

