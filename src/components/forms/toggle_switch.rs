use crate::components::forms::checkbox::CheckboxInputField;
use leptos::ev;
use leptos::prelude::*;

// Define the Leptos component
#[component]
pub fn ToggleSwitch(
    name: String,
    #[prop(into)] active: RwSignal<bool>,
    #[prop(into, default = "On".into())] label_active: String,
    #[prop(into, default = "Off".into())] label_inactive: String,
    #[prop(into, optional)] id_attr: String,
    #[prop(into, optional)] label: String,
    #[prop(default = false)] required: bool,
    #[prop(optional, default = Callback::new(|_| {}))] onchange: Callback<ev::Event>,
    #[prop(optional, default = Callback::new(|_| {}))] oninput: Callback<ev::Event>,
) -> impl IntoView {
    let handle_toggle = move |ev: ev::MouseEvent| {
        ev.stop_propagation();
        active.set(!active.get());
    };

    let current_value = Memo::new(move |_| {
        if active.get() {
            label_active.clone()
        } else {
            label_inactive.clone()
        }
    });

    view! {
        <div class="flex items-center cursor-pointer">
            <div on:click=handle_toggle class="relative">
                // <input type="checkbox" on:input=move |ev| oninput.run(ev) on:change=move |ev| onchange.run(ev) required=required name=name value={move || active.get()} checked={move || active.get()} id=id_attr class="sr-only"/>
                <CheckboxInputField oninput=oninput initial_value=current_value label=label name=name id_attr=id_attr checked=active ext_input_styles="sr-only" required=required />
                <div
                    class=move || format!(
                        "block w-14 h-8 rounded-full {}",
                        if active.get() { "bg-blue-950" } else { "bg-gray-300" }
                    )
                ></div>
                <div
                    class=move || format!(
                        "dot absolute left-1 bottom-1 w-6 h-6 rounded-full transition transform {}",
                        if active.get() { "translate-x-full" } else { "" }
                    )
                ></div>
            </div>
            <div class="ml-3 text-gray-700 font-medium">
                {current_value}
            </div>
        </div>
    }
}
