use leptos::ev;
use leptos::prelude::*;

// Define the Leptos component
#[component]
pub fn ToggleSwitch(
    #[prop(into)] active: Signal<bool>,
    #[prop(default = Callback::new(|_| {}))] on_toggle: Callback<bool>,
    #[prop(default = "On".to_string())] label_active: String,
    #[prop(default = "Off".to_string())] label_inactive: String,
) -> impl IntoView {
    // Define the onclick handler
    let onclick = move |_| {
        leptos::logging::log!("clicked toggle");
        on_toggle.run(!active.get());
    };

    view! {
        <div class="flex items-center cursor-pointer" on:click=onclick>
            <div class="relative">
                <input type="checkbox" id="toggle-switch" class="sr-only"/>
                <div
                    class=move || format!(
                        "block w-14 h-8 rounded-full {}",
                        if active.get() { "bg-blue-950" } else { "bg-gray-300" }
                    )
                ></div>
                <div
                    class=move || format!(
                        "dot absolute left-1 top-1 w-6 h-6 rounded-full transition transform {}",
                        if active.get() { "translate-x-full" } else { "" }
                    )
                ></div>
            </div>
            <div class="ml-3 text-gray-700 font-medium">
                {move || if active.get() { label_active.clone() } else { label_inactive.clone() }}
            </div>
        </div>
    }
}
