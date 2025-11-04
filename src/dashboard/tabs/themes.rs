use leptos::prelude::*;
use crate::themes::{ThemeManager, ThemeVariant, ColorPalette};

#[component]
pub fn ThemesTab() -> impl IntoView {
    let theme_manager = use_context::<ThemeManager>()
        .expect("ThemeManager should be provided");
    
    let current_theme = theme_manager.current_theme;
    
    // Custom theme editor state
    let show_custom_editor = RwSignal::new(false);
    let custom_colors = RwSignal::new(ColorPalette {
        bg_primary: "#0a0a0a".to_string(),
        bg_secondary: "#111111".to_string(),
        bg_tertiary: "#1a1a1a".to_string(),
        bg_card: "rgba(17, 17, 17, 0.8)".to_string(),
        bg_card_hover: "rgba(26, 26, 26, 0.9)".to_string(),
        text_primary: "#ffffff".to_string(),
        text_secondary: "#a3a3a3".to_string(),
        text_muted: "#737373".to_string(),
        text_accent: "#60a5fa".to_string(),
        border_primary: "rgba(255, 255, 255, 0.1)".to_string(),
        border_secondary: "rgba(255, 255, 255, 0.05)".to_string(),
        border_focus: "rgba(96, 165, 250, 0.5)".to_string(),
        accent_primary: "#3b82f6".to_string(),
        accent_hover: "#2563eb".to_string(),
        accent_secondary: "#10b981".to_string(),
        accent_warning: "#f59e0b".to_string(),
        accent_danger: "#ef4444".to_string(),
        success: "#10b981".to_string(),
        error: "#ef4444".to_string(),
        warning: "#f59e0b".to_string(),
        info: "#3b82f6".to_string(),
        progress_bg: "#1a1a1a".to_string(),
        progress_fill: "#3b82f6".to_string(),
        progress_border: "rgba(255, 255, 255, 0.1)".to_string(),
    });
    
    let apply_theme = {
        let theme_manager = theme_manager.clone();
        move |variant: ThemeVariant| {
            theme_manager.set_theme_by_variant(variant);
        }
    };
    
    let open_custom_editor = move |_| {
        show_custom_editor.set(true);
    };
    
    let reset_to_default = {
        let theme_manager = theme_manager.clone();
        move |_| {
            theme_manager.reset_to_default();
        }
    };
    
    view! {
        <div class="flex flex-col w-full space-y-4 sm:space-y-6 mt-10 sm:mt-12">
            // Current Theme Display
            <div class="rounded-xl p-4 sm:p-6 border mb-4 sm:mb-6" style="background-color: var(--bg-card); border-color: var(--border-secondary);">
                <h3 class="text-lg sm:text-xl font-semibold mb-3 sm:mb-4" style="color: var(--text-primary);">"Current Theme"</h3>
                <div class="flex items-center space-x-4">
                    <div class="w-8 h-8 rounded-lg" style:background-color=move || current_theme.get().colors.bg_primary></div>
                    <div>
                        <div class="font-medium" style:color="var(--text-primary)">{move || current_theme.get().name}</div>
                        <div class="text-sm" style:color="var(--text-secondary)">{move || format!("{:?}", current_theme.get().variant)}</div>
                    </div>
                </div>
            </div>
            
            // Theme Selection
            <div class="rounded-xl p-4 sm:p-6 border mb-4 sm:mb-6" style="background-color: var(--bg-card); border-color: var(--border-secondary);">
                <h3 class="text-lg sm:text-xl font-semibold mb-3 sm:mb-4" style="color: var(--text-primary);">"Choose Theme"</h3>
                <div class="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 gap-3 sm:gap-4">
                    // Dark Theme
                    <button
                        class={move || {
                            let base = "p-4 rounded-lg border transition-colors text-left";
                            if current_theme.get().variant == ThemeVariant::Dark {
                                format!("{} {}", base, "background-color: var(--bg-card);")
                            } else {
                                base.to_string()
                            }
                        }}
                        style="border-color: var(--border-secondary);"
                        on:click={
                            let apply_theme = apply_theme.clone();
                            move |_| apply_theme(ThemeVariant::Dark)
                        }
                    >
                        <div class="flex items-center space-x-3">
                            <div class="w-6 h-6 rounded flex-shrink-0" style="background-color: var(--bg-primary); border: 1px solid var(--border-secondary);"></div>
                            <div>
                                <div class="font-medium" style:color="var(--text-primary)">"Dark"</div>
                                <div class="text-sm" style:color="var(--text-secondary)">"Modern dark interface"</div>
                            </div>
                        </div>
                    </button>
                    
                    // Light Theme
                    <button
                        class={move || {
                            let base = "p-4 rounded-lg border transition-colors text-left";
                            if current_theme.get().variant == ThemeVariant::Light {
                                format!("{} {}", base, "background-color: var(--bg-card);")
                            } else {
                                base.to_string()
                            }
                        }}
                        style="border-color: var(--border-secondary);"
                        on:click={
                            let apply_theme = apply_theme.clone();
                            move |_| apply_theme(ThemeVariant::Light)
                        }
                    >
                        <div class="flex items-center space-x-3">
                            <div class="w-6 h-6 rounded border" style="background: linear-gradient(135deg, #f8fafc 0%, #e2e8f0 50%, #cbd5e1 100%); border-color: var(--border-secondary);"></div>
                            <div>
                                <div class="font-medium" style:color="var(--text-primary)">"Light"</div>
                                <div class="text-sm" style:color="var(--text-secondary)">"Clean light interface"</div>
                            </div>
                        </div>
                    </button>
                    
                    // Catppuccin Theme
                    <button
                        class={move || {
                            let base = "p-4 rounded-lg border transition-colors text-left";
                            if current_theme.get().variant == ThemeVariant::Catppuccin {
                                format!("{} {}", base, "background-color: var(--bg-card);")
                            } else {
                                base.to_string()
                            }
                        }}
                        style="border-color: var(--border-secondary);"
                        on:click={
                            let apply_theme = apply_theme.clone();
                            move |_| apply_theme(ThemeVariant::Catppuccin)
                        }
                    >
                        <div class="flex items-center space-x-3">
                            <div class="w-6 h-6 rounded relative overflow-hidden" style="background: #1e1e2e;">
                                <div class="absolute inset-0" style="background: linear-gradient(45deg, #f38ba8 0%, #a6e3a1 25%, #f9e2af 50%, #89b4fa 75%, #cba6f7 100%); opacity: 0.8;"></div>
                                <div class="absolute inset-0 flex items-center justify-center">
                                    <div class="w-2 h-2 rounded-full" style="background: #cdd6f4;"></div>
                                </div>
                            </div>
                            <div>
                                <div class="font-medium" style:color="var(--text-primary)">"Catppuccin"</div>
                                <div class="text-sm" style:color="var(--text-secondary)">"Pastel color palette"</div>
                            </div>
                        </div>
                    </button>
                    
                    // Tokyo Night Theme
                    <button
                        class={move || {
                            let base = "p-4 rounded-lg border transition-colors text-left";
                            let border_color = format!("border-color: var(--border-secondary);");
                            let hover_border = "hover:border-color: var(--border-primary);";
                            if current_theme.get().variant == ThemeVariant::TokyoNight {
                                format!("{} {}", base, "background-color: var(--bg-card);")
                            } else {
                                format!("{} {}", base, border_color)
                            }
                        }}
                        style="border-color: var(--border-secondary);"
                        on:click={
                            let apply_theme = apply_theme.clone();
                            move |_| apply_theme(ThemeVariant::TokyoNight)
                        }
                    >
                        <div class="flex items-center space-x-3">
                            <div class="w-6 h-6 rounded relative overflow-hidden" style="background: #1a1b26;">
                                <div class="absolute inset-0" style="background: linear-gradient(45deg, #7aa2f7 0%, #9ece6a 25%, #e0af68 50%, #f7768e 75%, #bb9af7 100%); opacity: 0.8;"></div>
                                <div class="absolute inset-0 flex items-center justify-center">
                                    <div class="w-2 h-2 rounded-full" style="background: #c0caf5;"></div>
                                </div>
                            </div>
                            <div>
                                <div class="font-medium" style:color="var(--text-primary)">"Tokyo Night"</div>
                                <div class="text-sm" style:color="var(--text-secondary)">"Modern dark with blue accents"</div>
                            </div>
                        </div>
                    </button>
                    
                    // GitHub Dark Theme
                    <button
                        class={move || {
                            let base = "p-4 rounded-lg border transition-colors text-left";
                            if current_theme.get().variant == ThemeVariant::GitHubDark {
                                format!("{} {}", base, "background-color: var(--bg-card);")
                            } else {
                                base.to_string()
                            }
                        }}
                        style="border-color: var(--border-secondary);"
                        on:click={
                            let apply_theme = apply_theme.clone();
                            move |_| apply_theme(ThemeVariant::GitHubDark)
                        }
                    >
                        <div class="flex items-center space-x-3">
                            <div class="w-6 h-6 rounded relative overflow-hidden" style="background: #0d1117;">
                                <div class="absolute inset-0" style="background: linear-gradient(45deg, #1f6feb 0%, #238636 25%, #d29922 50%, #da3633 75%, #8b949e 100%); opacity: 0.8;"></div>
                                <div class="absolute inset-0 flex items-center justify-center">
                                    <div class="w-2 h-2 rounded-full" style="background: #c9d1d9;"></div>
                                </div>
                            </div>
                            <div>
                                <div class="font-medium" style:color="var(--text-primary)">"GitHub Dark"</div>
                                <div class="text-sm" style:color="var(--text-secondary)">"GitHub's official dark theme"</div>
                            </div>
                        </div>
                    </button>
                    
                    // One Dark Pro Theme
                    <button
                        class={move || {
                            let base = "p-4 rounded-lg border transition-colors text-left";
                            if current_theme.get().variant == ThemeVariant::OneDark {
                                format!("{} {}", base, "background-color: var(--bg-card);")
                            } else {
                                base.to_string()
                            }
                        }}
                        style="border-color: var(--border-secondary);"
                        on:click={
                            let apply_theme = apply_theme.clone();
                            move |_| apply_theme(ThemeVariant::OneDark)
                        }
                    >
                        <div class="flex items-center space-x-3">
                            <div class="w-6 h-6 rounded relative overflow-hidden" style="background: #21252b;">
                                <div class="absolute inset-0" style="background: linear-gradient(45deg, #528bcc 0%, #7cb342 25%, #ffb74d 50%, #d32f2f 75%, #98c379 100%); opacity: 0.8;"></div>
                                <div class="absolute inset-0 flex items-center justify-center">
                                    <div class="w-2 h-2 rounded-full" style="background: #abb2bf;"></div>
                                </div>
                            </div>
                            <div>
                                <div class="font-medium" style:color="var(--text-primary)">"One Dark Pro"</div>
                                <div class="text-sm" style:color="var(--text-secondary)">"Classic VS Code dark theme"</div>
                            </div>
                        </div>
                    </button>
                    
                    // TorBox Theme
                    <button
                        class={move || {
                            let base = "p-4 rounded-lg border transition-colors text-left";
                            if current_theme.get().variant == ThemeVariant::TorBox {
                                format!("{} {}", base, "background-color: var(--bg-card);")
                            } else {
                                base.to_string()
                            }
                        }}
                        style="border-color: var(--border-secondary);"
                        on:click={
                            let apply_theme = apply_theme.clone();
                            move |_| apply_theme(ThemeVariant::TorBox)
                        }
                    >
                        <div class="flex items-center space-x-3">
                            <div class="w-6 h-6 rounded relative overflow-hidden" style="background: #0a0a0a;">
                                <div class="absolute inset-0" style="background: linear-gradient(45deg, #ff4500 0%, #00ff00 25%, #ffaa00 50%, #ff1744 75%, #1e90ff 100%); opacity: 0.9;"></div>
                                <div class="absolute inset-0 flex items-center justify-center">
                                    <div class="w-2 h-2 rounded-full" style="background: #ffffff;"></div>
                                </div>
                            </div>
                            <div>
                                <div class="font-medium" style:color="var(--text-primary)">"TorBox"</div>
                                <div class="text-sm" style:color="var(--text-secondary)">"TorBox-inspired vibrant theme"</div>
                            </div>
                        </div>
                    </button>
                    
                    // Cyberpunk Theme
                    <button
                        class={move || {
                            let base = "p-4 rounded-lg border transition-colors text-left";
                            if current_theme.get().variant == ThemeVariant::Cyberpunk {
                                format!("{} {}", base, "background-color: var(--bg-card);")
                            } else {
                                base.to_string()
                            }
                        }}
                        style="border-color: var(--border-secondary);"
                        on:click={
                            let apply_theme = apply_theme.clone();
                            move |_| apply_theme(ThemeVariant::Cyberpunk)
                        }
                    >
                        <div class="flex items-center space-x-3">
                            <div class="w-6 h-6 rounded relative overflow-hidden" style="background: #0a0a0a;">
                                <div class="absolute inset-0" style="background: linear-gradient(45deg, #ff00ff 0%, #00ffff 25%, #ffff00 50%, #ff0040 75%, #00ff00 100%); opacity: 0.9;"></div>
                                <div class="absolute inset-0 flex items-center justify-center">
                                    <div class="w-2 h-2 rounded-full" style="background: #00ffff;"></div>
                                </div>
                            </div>
                            <div>
                                <div class="font-medium" style:color="var(--text-primary)">"Cyberpunk"</div>
                                <div class="text-sm" style:color="var(--text-secondary)">"Neon cyberpunk aesthetic"</div>
                            </div>
                        </div>
                    </button>
                    
                    // Crimson Theme
                    <button
                        class={move || {
                            let base = "p-4 rounded-lg border transition-colors text-left";
                            if current_theme.get().variant == ThemeVariant::Crimson {
                                format!("{} {}", base, "background-color: var(--bg-card);")
                            } else {
                                base.to_string()
                            }
                        }}
                        style="border-color: var(--border-secondary);"
                        on:click={
                            let apply_theme = apply_theme.clone();
                            move |_| apply_theme(ThemeVariant::Crimson)
                        }
                    >
                        <div class="flex items-center space-x-3">
                            <div class="w-6 h-6 rounded relative overflow-hidden" style="background: #000000;">
                                <div class="absolute inset-0" style="background: linear-gradient(45deg, #dc143c 0%, #b81d24 25%, #ff6b00 50%, #46d369 75%, #0071eb 100%); opacity: 0.8;"></div>
                                <div class="absolute inset-0 flex items-center justify-center">
                                    <div class="w-2 h-2 rounded-full" style="background: #ffffff;"></div>
                                </div>
                            </div>
                            <div>
                                <div class="font-medium" style:color="var(--text-primary)">"Crimson"</div>
                                <div class="text-sm" style:color="var(--text-secondary)">"Crimson red dark theme"</div>
                            </div>
                        </div>
                    </button>
                    
                    // Custom Theme
                    <button
                        class={move || {
                            let base = "p-4 rounded-lg border transition-colors text-left";
                            if current_theme.get().variant == ThemeVariant::Custom {
                                format!("{} {}", base, "background-color: var(--bg-card);")
                            } else {
                                base.to_string()
                            }
                        }}
                        style="border-color: var(--border-secondary);"
                        on:click=open_custom_editor
                    >
                        <div class="flex items-center space-x-3">
                            <div class="w-6 h-6 rounded" style="background: linear-gradient(45deg, var(--accent-primary), var(--accent-secondary), var(--accent-warning), var(--accent-danger));"></div>
                            <div>
                                <div class="font-medium" style:color="var(--text-primary)">"Custom"</div>
                                <div class="text-sm" style:color="var(--text-secondary)">"Create your own theme"</div>
                            </div>
                        </div>
                    </button>
                </div>
            </div>
            
            // Actions
            <div class="flex flex-row gap-3 sm:gap-4">
                <button
                    class="px-4 sm:px-6 py-2.5 sm:py-3 rounded-lg transition-colors font-medium whitespace-nowrap text-sm sm:text-base"
                    style="background-color: var(--accent-primary); color: var(--text-primary);"
                    on:click=open_custom_editor
                >
                    "Customize Theme"
                </button>
                <button
                    class="px-4 sm:px-6 py-2.5 sm:py-3 rounded-lg transition-colors font-medium whitespace-nowrap text-sm sm:text-base"
                    style="background-color: var(--bg-tertiary); color: var(--text-primary);"
                    on:click=reset_to_default
                >
                    "Reset to Default"
                </button>
            </div>
            
            // Custom Theme Editor Modal
            <Show when=move || show_custom_editor.get()>
                {
                    let theme_manager = theme_manager.clone();
                    let custom_colors = custom_colors.clone();
                    let show_custom_editor_clone = show_custom_editor.clone();
                    let show_custom_editor_clone2 = show_custom_editor.clone();
                    let show_custom_editor_clone3 = show_custom_editor.clone();
                    move || {
                        let theme_manager = theme_manager.clone();
                        let custom_colors = custom_colors.clone();
                        let show_custom_editor = show_custom_editor_clone.clone();
                        let show_custom_editor2 = show_custom_editor_clone2.clone();
                        let show_custom_editor3 = show_custom_editor_clone3.clone();
                        view! {
                            <div class="fixed inset-0 flex items-center justify-center z-50 p-4" style="background-color: rgba(0, 0, 0, 0.5);">
                                <div class="rounded-xl border max-w-4xl w-full max-h-[90vh] overflow-y-auto mx-2 sm:mx-4" style="background-color: var(--bg-secondary); border-color: var(--border-secondary);">
                                    <div class="p-4 sm:p-6">
                                        <div class="flex items-center justify-between mb-4 sm:mb-6">
                                            <h3 class="text-xl sm:text-2xl font-bold" style="color: var(--text-primary);">"Custom Theme Editor"</h3>
                                            <button
                                                class="transition-colors"
                                                style="color: var(--text-secondary);"
                                                on:click=move |_| show_custom_editor.set(false)
                                            >
                                                <svg class="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12"></path>
                                                </svg>
                                            </button>
                                        </div>
                                        
                                        <CustomThemeEditor colors=custom_colors/>
                                        
                                        <div class="flex flex-col sm:flex-row justify-end gap-3 sm:gap-4 sm:space-x-0 mt-4 sm:mt-6 pt-4 sm:pt-6 border-t" style="border-color: var(--border-secondary);">
                                            <button
                                                class="px-4 sm:px-6 py-2.5 sm:py-3 rounded-lg transition-colors font-medium text-sm sm:text-base w-full sm:w-auto"
                                                style="background-color: var(--bg-tertiary); color: var(--text-primary);"
                                                on:click=move |_| show_custom_editor2.set(false)
                                            >
                                                "Cancel"
                                            </button>
                                            <button
                                                class="px-4 sm:px-6 py-2.5 sm:py-3 rounded-lg transition-colors font-medium text-sm sm:text-base w-full sm:w-auto"
                                                style="background-color: var(--accent-primary); color: var(--text-primary);"
                                                on:click=move |_| {
                                                    let colors = custom_colors.get();
                                                    let custom_theme = crate::themes::custom_theme::CustomTheme::create_with_colors(colors);
                                                    theme_manager.set_theme(custom_theme.clone());
                                                    theme_manager.save_custom_theme(custom_theme);
                                                    show_custom_editor3.set(false);
                                                }
                                            >
                                                "Save Theme"
                                            </button>
                                        </div>
                                    </div>
                                </div>
                            </div>
                        }
                    }
                }
            </Show>
        </div>
    }
}

#[component]
fn CustomThemeEditor(colors: RwSignal<ColorPalette>) -> impl IntoView {
    let theme_manager = use_context::<ThemeManager>()
        .expect("ThemeManager should be provided");
    
    let update_color = {
        let colors = colors.clone();
        let theme_manager = theme_manager.clone();
        move |field: &str, value: String| {
            colors.update(|colors| {
                match field {
                    "bg_primary" => colors.bg_primary = value,
                    "bg_secondary" => colors.bg_secondary = value,
                    "bg_tertiary" => colors.bg_tertiary = value,
                    "text_primary" => colors.text_primary = value,
                    "text_secondary" => colors.text_secondary = value,
                    "text_muted" => colors.text_muted = value,
                    "accent_primary" => colors.accent_primary = value,
                    "accent_secondary" => colors.accent_secondary = value,
                    "accent_warning" => colors.accent_warning = value,
                    "accent_danger" => colors.accent_danger = value,
                    _ => {}
                }
            });
            
            // Apply the updated theme in real-time
            let updated_colors = colors.get();
            let custom_theme = crate::themes::custom_theme::CustomTheme::create_with_colors(updated_colors);
            theme_manager.set_theme(custom_theme);
        }
    };
    
    let update_bg_primary = {
        let update_color = update_color.clone();
        move |value| update_color("bg_primary", value)
    };
    
    let update_bg_secondary = {
        let update_color = update_color.clone();
        move |value| update_color("bg_secondary", value)
    };
    
    let update_bg_tertiary = {
        let update_color = update_color.clone();
        move |value| update_color("bg_tertiary", value)
    };
    
    let update_text_primary = {
        let update_color = update_color.clone();
        move |value| update_color("text_primary", value)
    };
    
    let update_text_secondary = {
        let update_color = update_color.clone();
        move |value| update_color("text_secondary", value)
    };
    
    let update_text_muted = {
        let update_color = update_color.clone();
        move |value| update_color("text_muted", value)
    };
    
    let update_accent_primary = {
        let update_color = update_color.clone();
        move |value| update_color("accent_primary", value)
    };
    
    let update_accent_secondary = {
        let update_color = update_color.clone();
        move |value| update_color("accent_secondary", value)
    };
    
    let update_accent_warning = {
        let update_color = update_color.clone();
        move |value| update_color("accent_warning", value)
    };
    
    let update_accent_danger = {
        let update_color = update_color.clone();
        move |value| update_color("accent_danger", value)
    };
    
    view! {
        <div class="grid grid-cols-1 md:grid-cols-2 gap-4 sm:gap-6">
            // Background Colors
            <div class="space-y-3 sm:space-y-4">
                <h4 class="text-base sm:text-lg font-semibold" style="color: var(--text-primary);">"Background Colors"</h4>
                <ColorInput
                    label="Primary Background"
                    value=move || colors.get().bg_primary
                    on_change=update_bg_primary
                />
                <ColorInput
                    label="Secondary Background"
                    value=move || colors.get().bg_secondary
                    on_change=update_bg_secondary
                />
                <ColorInput
                    label="Tertiary Background"
                    value=move || colors.get().bg_tertiary
                    on_change=update_bg_tertiary
                />
            </div>
            
            // Text Colors
            <div class="space-y-3 sm:space-y-4">
                <h4 class="text-base sm:text-lg font-semibold" style="color: var(--text-primary);">"Text Colors"</h4>
                <ColorInput
                    label="Primary Text"
                    value=move || colors.get().text_primary
                    on_change=update_text_primary
                />
                <ColorInput
                    label="Secondary Text"
                    value=move || colors.get().text_secondary
                    on_change=update_text_secondary
                />
                <ColorInput
                    label="Muted Text"
                    value=move || colors.get().text_muted
                    on_change=update_text_muted
                />
            </div>
            
            // Accent Colors
            <div class="space-y-3 sm:space-y-4">
                <h4 class="text-base sm:text-lg font-semibold" style="color: var(--text-primary);">"Accent Colors"</h4>
                <ColorInput
                    label="Primary Accent"
                    value=move || colors.get().accent_primary
                    on_change=update_accent_primary
                />
                <ColorInput
                    label="Secondary Accent"
                    value=move || colors.get().accent_secondary
                    on_change=update_accent_secondary
                />
                <ColorInput
                    label="Warning"
                    value=move || colors.get().accent_warning
                    on_change=update_accent_warning
                />
                <ColorInput
                    label="Danger"
                    value=move || colors.get().accent_danger
                    on_change=update_accent_danger
                />
            </div>
            
            // Preview
            <div class="space-y-3 sm:space-y-4">
                <h4 class="text-base sm:text-lg font-semibold" style="color: var(--text-primary);">"Preview"</h4>
                <div class="p-4 rounded-lg border" style:background-color=move || colors.get().bg_primary style:border-color=move || colors.get().border_primary>
                    <div class="text-lg font-semibold" style:color=move || colors.get().text_primary>"Sample Text"</div>
                    <div class="text-sm" style:color=move || colors.get().text_secondary>"Secondary text"</div>
                    <div class="text-xs" style:color=move || colors.get().text_muted>"Muted text"</div>
                    <div class="mt-2 flex space-x-2">
                        <div class="px-3 py-1 rounded text-sm" style:background-color=move || colors.get().accent_primary style:color=move || colors.get().text_primary>"Primary"</div>
                        <div class="px-3 py-1 rounded text-sm" style:background-color=move || colors.get().accent_secondary style:color=move || colors.get().text_primary>"Success"</div>
                    </div>
                </div>
            </div>
        </div>
    }
}

#[component]
fn ColorInput(
    label: &'static str,
    value: impl Fn() -> String + 'static + Send + Clone,
    on_change: impl Fn(String) + 'static + Clone,
) -> impl IntoView {
    let value_clone = value.clone();
    let on_change_clone = on_change.clone();
    
    view! {
        <div class="space-y-2">
            <label class="text-xs sm:text-sm font-medium" style:color="var(--text-secondary)">{label}</label>
            <div class="flex items-center space-x-2">
                <input
                    type="color"
                    class="w-7 h-7 sm:w-8 sm:h-8 rounded border cursor-pointer flex-shrink-0"
                    style:border-color="var(--border-primary)" style:background-color="var(--bg-card)"
                    prop:value=value
                    on:input=move |ev| {
                        let value = event_target_value(&ev);
                        on_change(value);
                    }
                />
                <input
                    type="text"
                    class="flex-1 px-2 sm:px-3 py-1.5 sm:py-2 rounded text-xs sm:text-sm min-w-0"
                    style:background-color="var(--bg-card)" style:border-color="var(--border-primary)" style:color="var(--text-primary)"
                    prop:value=value_clone
                    on:input=move |ev| {
                        let value = event_target_value(&ev);
                        on_change_clone(value);
                    }
                />
            </div>
        </div>
    }
}
