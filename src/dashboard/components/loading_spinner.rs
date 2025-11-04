use leptos::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SpinnerSize {
    Small,
    Medium,
    Large,
    ExtraLarge,
}

impl SpinnerSize {
    fn size_class(&self) -> &'static str {
        match self {
            SpinnerSize::Small => "w-4 h-4",
            SpinnerSize::Medium => "w-6 h-6",
            SpinnerSize::Large => "w-8 h-8",
            SpinnerSize::ExtraLarge => "w-12 h-12",
        }
    }
    
    fn border_width(&self) -> &'static str {
        match self {
            SpinnerSize::Small => "2px",
            SpinnerSize::Medium => "3px",
            SpinnerSize::Large => "4px",
            SpinnerSize::ExtraLarge => "5px",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SpinnerVariant {
    Default,
    Accent,
    Success,
    Warning,
    Danger,
}

impl SpinnerVariant {
    fn color(&self) -> &'static str {
        match self {
            SpinnerVariant::Default => "var(--text-secondary)",
            SpinnerVariant::Accent => "var(--accent-primary)",
            SpinnerVariant::Success => "var(--accent-secondary)",
            SpinnerVariant::Warning => "var(--accent-warning)",
            SpinnerVariant::Danger => "var(--accent-danger)",
        }
    }
    
    fn border_color(&self) -> &'static str {
        match self {
            SpinnerVariant::Default => "rgba(163, 163, 163, 0.2)",
            SpinnerVariant::Accent => "rgba(59, 130, 246, 0.2)",
            SpinnerVariant::Success => "rgba(16, 185, 129, 0.2)",
            SpinnerVariant::Warning => "rgba(245, 158, 11, 0.2)",
            SpinnerVariant::Danger => "rgba(239, 68, 68, 0.2)",
        }
    }
}

#[component]
pub fn LoadingSpinner(
    #[prop(optional)] size: Option<SpinnerSize>,
    #[prop(optional)] variant: Option<SpinnerVariant>,
    #[prop(optional)] text: Option<String>,
    #[prop(optional)] centered: Option<bool>,
    #[prop(optional)] class: Option<String>,
) -> impl IntoView {
    let size = size.unwrap_or(SpinnerSize::Medium);
    let variant = variant.unwrap_or(SpinnerVariant::Default);
    let centered = centered.unwrap_or(false);
    let size_class = size.size_class();
    let border_width = size.border_width();
    let color = variant.color();
    let border_color = variant.border_color();
    
    let container_class = if centered {
        "flex items-center justify-center gap-3 py-8"
    } else {
        "flex items-center gap-3"
    };
    
    view! {
        <div class=move || format!("{} {}", container_class, class.as_ref().map(|c| c.as_str()).unwrap_or(""))>
            <div
                class=move || format!("{} rounded-full animate-spin", size_class)
                style=move || format!(
                    "border: {} solid {}; border-top-color: {}; display: block; box-sizing: border-box;",
                    border_width,
                    border_color,
                    color
                )
                role="status"
                aria-label="Loading"
            >
                <span class="sr-only">"Loading..."</span>
            </div>
            {move || {
                if let Some(text_val) = &text {
                    view! {
                        <span class="text-sm font-medium" style=format!("color: {}", color)>
                            {text_val.clone()}
                        </span>
                    }.into_any()
                } else {
                    view! {}.into_any()
                }
            }}
        </div>
    }
}

