use leptos::html::*;
use leptos::prelude::*;

use crate::components::schemas::props::ColorTemperature;

/// This is a component for displaying a label tag with a color temperature.
/// Example usage:
/// ```
/// <LabelTag label="Success" color=ColorTemperature::Success />
/// ```
#[component]
pub fn LabelTag(
    #[prop(into, optional)] label: String,
    #[prop(default = ColorTemperature::Primary)] color: ColorTemperature,
) -> impl IntoView {
    // Function to return the corresponding tailwind classes
    let color_classes = match color {
        ColorTemperature::Success => "text-success border-2 border-success bg-success-light",
        ColorTemperature::Warning => "text-warning border-2 border-warning bg-warning-light",
        ColorTemperature::Info => "text-info border-2 border-info bg-info-light",
        ColorTemperature::Danger => "text-danger border-2 border-danger bg-danger-light",
        _ => "text-primary border-2 border-primary primary-light",
    };

    view! {
        <div class=format!("inline-block px-3 text-center rounded text-sm {}", color_classes)>
            {label}
        </div>
    }
}
