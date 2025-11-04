use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ThemeVariant {
    Dark,
    Light,
    Catppuccin,
    TokyoNight,
    GitHubDark,
    OneDark,
    TorBox,
    Cyberpunk,
    Crimson,
    Custom,
}

impl Default for ThemeVariant {
    fn default() -> Self {
        ThemeVariant::Dark
    }
}

impl std::fmt::Display for ThemeVariant {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ThemeVariant::Dark => write!(f, "Dark"),
            ThemeVariant::Light => write!(f, "Light"),
            ThemeVariant::Catppuccin => write!(f, "Catppuccin"),
            ThemeVariant::TokyoNight => write!(f, "Tokyo Night"),
            ThemeVariant::GitHubDark => write!(f, "GitHub Dark"),
            ThemeVariant::OneDark => write!(f, "One Dark Pro"),
            ThemeVariant::TorBox => write!(f, "TorBox"),
            ThemeVariant::Cyberpunk => write!(f, "Cyberpunk"),
            ThemeVariant::Crimson => write!(f, "Crimson"),
            ThemeVariant::Custom => write!(f, "Custom"),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ColorPalette {
    // Background colors
    pub bg_primary: String,
    pub bg_secondary: String,
    pub bg_tertiary: String,
    pub bg_card: String,
    pub bg_card_hover: String,
    
    // Text colors
    pub text_primary: String,
    pub text_secondary: String,
    pub text_muted: String,
    pub text_accent: String,
    
    // Border colors
    pub border_primary: String,
    pub border_secondary: String,
    pub border_focus: String,
    
    // Accent colors
    pub accent_primary: String,
    pub accent_hover: String,
    pub accent_secondary: String,
    pub accent_warning: String,
    pub accent_danger: String,
    
    // Status colors
    pub success: String,
    pub error: String,
    pub warning: String,
    pub info: String,
    
    // Progress bar colors
    pub progress_bg: String,
    pub progress_fill: String,
    pub progress_border: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Theme {
    pub variant: ThemeVariant,
    pub name: String,
    pub colors: ColorPalette,
    pub custom_css: Option<String>,
}

impl Theme {
    pub fn new(variant: ThemeVariant, name: String, colors: ColorPalette) -> Self {
        Self {
            variant,
            name,
            colors,
            custom_css: None,
        }
    }
    
    pub fn with_custom_css(mut self, css: String) -> Self {
        self.custom_css = Some(css);
        self
    }
    
    pub fn apply_to_document(&self) {
        #[cfg(target_arch = "wasm32")]
        {
            use leptos::web_sys;
            if let Some(window) = web_sys::window() {
                if let Some(document) = window.document() {
                    if let Some(root) = document.document_element() {
                        self.apply_css_variables(&root);
                        
                        if let Some(ref custom_css) = self.custom_css {
                            self.apply_custom_css(&document, custom_css);
                        }
                    }
                }
            }
        }
    }
    
    fn apply_css_variables(&self, root: &leptos::web_sys::Element) {
        use leptos::web_sys;
        use leptos::wasm_bindgen::JsCast;
        
        if let Some(html_element) = root.dyn_ref::<web_sys::HtmlElement>() {
            let style = html_element.style();
            
            let _ = style.set_property("--bg-primary", &self.colors.bg_primary);
            let _ = style.set_property("--bg-secondary", &self.colors.bg_secondary);
            let _ = style.set_property("--bg-tertiary", &self.colors.bg_tertiary);
            let _ = style.set_property("--bg-card", &self.colors.bg_card);
            let _ = style.set_property("--bg-card-hover", &self.colors.bg_card_hover);
            
            let _ = style.set_property("--text-primary", &self.colors.text_primary);
            let _ = style.set_property("--text-secondary", &self.colors.text_secondary);
            let _ = style.set_property("--text-muted", &self.colors.text_muted);
            let _ = style.set_property("--text-accent", &self.colors.text_accent);
            
            let _ = style.set_property("--border-primary", &self.colors.border_primary);
            let _ = style.set_property("--border-secondary", &self.colors.border_secondary);
            let _ = style.set_property("--border-focus", &self.colors.border_focus);
            
            let _ = style.set_property("--accent-primary", &self.colors.accent_primary);
            let _ = style.set_property("--accent-hover", &self.colors.accent_hover);
            let _ = style.set_property("--accent-secondary", &self.colors.accent_secondary);
            let _ = style.set_property("--accent-warning", &self.colors.accent_warning);
            let _ = style.set_property("--accent-danger", &self.colors.accent_danger);
            
            let _ = style.set_property("--success", &self.colors.success);
            let _ = style.set_property("--error", &self.colors.error);
            let _ = style.set_property("--warning", &self.colors.warning);
            let _ = style.set_property("--info", &self.colors.info);
            
            let _ = style.set_property("--progress-bg", &self.colors.progress_bg);
            let _ = style.set_property("--progress-fill", &self.colors.progress_fill);
            let _ = style.set_property("--progress-border", &self.colors.progress_border);
        }
    }
    
    fn apply_custom_css(&self, document: &leptos::web_sys::Document, css: &str) {
        use leptos::web_sys;
        if let Some(existing_style) = document.get_element_by_id("custom-theme-style") {
            let _ = existing_style.remove();
        }
        
        if let Ok(style_element) = document.create_element("style") {
            let _ = style_element.set_id("custom-theme-style");
            let _ = style_element.set_text_content(Some(css));
            
            if let Some(head) = document.head() {
                let _ = head.append_child(&style_element);
            }
        }
    }
}