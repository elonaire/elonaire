use leptos::prelude::*;

// Define the Leptos component
#[component]
pub fn ToggleSwitch(
    name: String,
    #[prop(into)] active: Signal<bool>,
    #[prop(default = Callback::new(|_| {}), optional)] on_toggle: Callback<bool>,
    #[prop(default = "On".to_string())] label_active: String,
    #[prop(default = "Off".to_string())] label_inactive: String,
    #[prop(optional)] id_attr: String,
    #[prop(optional)] label: String,
    #[prop(default = false)] required: bool,
) -> impl IntoView {
    // Define the onclick handler
    let onclick = move |_| {
        on_toggle.run(!active.get());
    };

    view! {
        <div class="flex items-center cursor-pointer" on:click=onclick>
            <div class="relative">
                <label for=id_attr.clone() class="block text-gray-700 text-sm font-bold mb-2">
                    {label}
                    {move || if required {
                        Some(view! { <span class="text-red-500">"*"</span> })
                    } else {
                        None
                    }}
                </label>
                <input type="checkbox" required=required name=name value=active id=id_attr class="sr-only"/>
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
