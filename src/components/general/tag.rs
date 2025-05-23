use leptos::html::*;
use leptos::prelude::*;

use crate::components::schemas::props::ColorTemperature;

#[component]
pub fn LabelTag(
    label: String,
    #[prop(default = ColorTemperature::Primary)] color: ColorTemperature,
) -> impl IntoView {
    // Function to return the corresponding tailwind classes
    let color_classes = match color {
        ColorTemperature::Success => {
            "text-success text-white border-2 border-success bg-success-light"
        }
        ColorTemperature::Warning => {
            "text-warning text-white border-2 border-warning bg-warning-light"
        }
        ColorTemperature::Info => "text-info text-white border-2 border-info bg-info-light",
        ColorTemperature::Danger => "text-danger text-white border-2 border-danger bg-danger-light",
        _ => "text-primary border-2 border-primary primary-light",
    };

    view! {
        <div class=format!("inline-block px-3 text-center rounded {}", color_classes)>
            {label}
        </div>
    }
}
