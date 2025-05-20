use leptos::html::*;
use leptos::prelude::*;

#[derive(Clone, PartialEq)]
#[allow(dead_code)]
pub enum BadgeTemperature {
    Danger,
    Success,
    Warning,
    Info,
    Primary,
}

#[component]
pub fn Badge(
    text: String,
    #[prop(default = BadgeTemperature::Primary)] color: BadgeTemperature,
    #[prop(default = "".to_string())] parent_class: String,
    children: Children,
    #[prop(default = "".to_string())] badge_position: String,
) -> impl IntoView {
    let color_classes = move || match color {
        BadgeTemperature::Danger => "bg-danger",
        BadgeTemperature::Success => "bg-success",
        BadgeTemperature::Warning => "bg-warning",
        BadgeTemperature::Info => "bg-blue-100",
        _ => "bg-primary", // default color
    };

    let text_clone = text.clone();
    let width_classes = move || {
        if text_clone.is_empty() {
            "w-2 h-2"
        } else {
            "min-w-4 h-4 p-1"
        }
    };

    view! {
        <div class=format!("relative {}", parent_class)>
            {children()}
            <span class=format!(
                "inline-flex items-center justify-center rounded-full text-xs font-medium text-white absolute top-0 right-0 transform translate-x-1/2 -translate-y-1/2 {} {} {}",
                color_classes(),
                width_classes(),
                badge_position
            )>
                {text}
            </span>
        </div>
    }
}
