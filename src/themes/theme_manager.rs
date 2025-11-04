use leptos::prelude::*;
use crate::themes::{Theme, ThemeVariant, ColorPalette};
use crate::themes::dark_theme::DarkTheme;
use crate::themes::light_theme::LightTheme;
use crate::themes::catppuccin_theme::CatppuccinTheme;
use crate::themes::custom_theme::CustomTheme;
use crate::themes::tokyo_night_theme::TokyoNightTheme;
use crate::themes::github_dark_theme::GitHubDarkTheme;
use crate::themes::one_dark_theme::OneDarkTheme;
use crate::themes::torbox_theme::TorBoxTheme;
use crate::themes::cyberpunk_theme::CyberpunkTheme;
use crate::themes::crimson_theme::CrimsonTheme;

#[derive(Clone)]
pub struct ThemeManager {
    pub current_theme: RwSignal<Theme>,
    pub available_themes: RwSignal<Vec<Theme>>,
}

impl ThemeManager {
    pub fn new() -> Self {
        let mut manager = Self {
            current_theme: RwSignal::new(DarkTheme::create()),
            available_themes: RwSignal::new(vec![]),
        };
        
        manager.initialize_themes();
        manager.load_saved_theme();
        
        manager
    }
    
    fn initialize_themes(&mut self) {
        let themes = vec![
            DarkTheme::create(),
            LightTheme::create(),
            CatppuccinTheme::create(),
            TokyoNightTheme::create(),
            GitHubDarkTheme::create(),
            OneDarkTheme::create(),
            TorBoxTheme::create(),
            CyberpunkTheme::create(),
            CrimsonTheme::create(),
        ];
        
        self.available_themes.set(themes);
    }
    
    pub fn set_theme(&self, theme: Theme) {
        self.current_theme.set(theme.clone());
        #[cfg(target_arch = "wasm32")]
        {
            use leptos::task::spawn_local;
            let theme_clone = theme.clone();
            spawn_local(async move {
                let _ = web_sys::window()
                    .and_then(|w| w.document())
                    .and_then(|_| {
                        theme_clone.apply_to_document();
                        Some(())
                    });
            });
        }
        #[cfg(not(target_arch = "wasm32"))]
        {
            theme.apply_to_document();
        }
        self.save_theme(&theme);
    }
    
    pub fn set_theme_by_variant(&self, variant: ThemeVariant) {
        let theme = match variant {
            ThemeVariant::Dark => DarkTheme::create(),
            ThemeVariant::Light => LightTheme::create(),
            ThemeVariant::Catppuccin => CatppuccinTheme::create(),
            ThemeVariant::TokyoNight => TokyoNightTheme::create(),
            ThemeVariant::GitHubDark => GitHubDarkTheme::create(),
            ThemeVariant::OneDark => OneDarkTheme::create(),
            ThemeVariant::TorBox => TorBoxTheme::create(),
            ThemeVariant::Cyberpunk => CyberpunkTheme::create(),
            ThemeVariant::Crimson => CrimsonTheme::create(),
            ThemeVariant::Custom => {
                self.load_custom_theme().unwrap_or_else(|| CustomTheme::create())
            }
        };
        
        self.set_theme(theme);
    }
    
    pub fn get_current_theme(&self) -> Theme {
        self.current_theme.get()
    }
    
    pub fn get_available_themes(&self) -> Vec<Theme> {
        self.available_themes.get()
    }
    
    pub fn add_custom_theme(&self, theme: Theme) {
        let mut themes = self.available_themes.get();
        themes.push(theme);
        self.available_themes.set(themes);
    }
    
    pub fn remove_custom_theme(&self, theme_name: &str) {
        let mut themes = self.available_themes.get();
        themes.retain(|theme| theme.name != theme_name);
        self.available_themes.set(themes);
    }
    
    fn save_theme(&self, _theme: &Theme) {
        #[cfg(target_arch = "wasm32")]
        {
            if let Some(window) = web_sys::window() {
                if let Ok(Some(storage)) = window.local_storage() {
                    if let Ok(theme_json) = serde_json::to_string(_theme) {
                        let _ = storage.set_item("selected_theme", &theme_json);
                    }
                }
            }
        }
    }
    
    fn load_saved_theme(&self) {
        #[cfg(target_arch = "wasm32")]
        {
            if let Some(window) = web_sys::window() {
                if let Ok(Some(storage)) = window.local_storage() {
                    if let Ok(Some(theme_json)) = storage.get_item("selected_theme") {
                        if let Ok(theme) = serde_json::from_str::<Theme>(&theme_json) {
                            self.set_theme(theme);
                            return;
                        }
                    }
                }
            }
        }
        
        self.set_theme(DarkTheme::create());
    }
    
    fn load_custom_theme(&self) -> Option<Theme> {
        #[cfg(target_arch = "wasm32")]
        {
            if let Some(window) = web_sys::window() {
                if let Ok(Some(storage)) = window.local_storage() {
                    if let Ok(Some(theme_json)) = storage.get_item("custom_theme") {
                        if let Ok(theme) = serde_json::from_str::<Theme>(&theme_json) {
                            return Some(theme);
                        }
                    }
                }
            }
        }
        None
    }
    
    pub fn save_custom_theme(&self, _theme: Theme) {
        #[cfg(target_arch = "wasm32")]
        {
            if let Some(window) = web_sys::window() {
                if let Ok(Some(storage)) = window.local_storage() {
                    if let Ok(theme_json) = serde_json::to_string(&_theme) {
                        let _ = storage.set_item("custom_theme", &theme_json);
                    }
                }
            }
        }
    }
    
    pub fn reset_to_default(&self) {
        self.set_theme(DarkTheme::create());
    }
}
