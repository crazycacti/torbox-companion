use crate::themes::{Theme, ThemeVariant, ColorPalette};

pub struct LightTheme;

impl LightTheme {
    pub fn create() -> Theme {
        Theme::new(
            ThemeVariant::Light,
            "Light".to_string(),
            ColorPalette {
                bg_primary: "#ffffff".to_string(),
                bg_secondary: "#f8fafc".to_string(),
                bg_tertiary: "#f1f5f9".to_string(),
                bg_card: "rgba(248, 250, 252, 0.8)".to_string(),
                bg_card_hover: "rgba(241, 245, 249, 0.9)".to_string(),
                
                text_primary: "#0f172a".to_string(),
                text_secondary: "#475569".to_string(),
                text_muted: "#64748b".to_string(),
                text_accent: "#3b82f6".to_string(),
                
                border_primary: "rgba(15, 23, 42, 0.1)".to_string(),
                border_secondary: "rgba(15, 23, 42, 0.05)".to_string(),
                border_focus: "rgba(59, 130, 246, 0.5)".to_string(),
                
                accent_primary: "#3b82f6".to_string(),
                accent_hover: "#2563eb".to_string(),
                accent_secondary: "#10b981".to_string(),
                accent_warning: "#f59e0b".to_string(),
                accent_danger: "#ef4444".to_string(),
                
                success: "#10b981".to_string(),
                error: "#ef4444".to_string(),
                warning: "#f59e0b".to_string(),
                info: "#3b82f6".to_string(),
                
                progress_bg: "#f1f5f9".to_string(),
                progress_fill: "#3b82f6".to_string(),
                progress_border: "rgba(15, 23, 42, 0.1)".to_string(),
            }
        )
    }
}
