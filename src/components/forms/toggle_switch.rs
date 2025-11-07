use crate::components::forms::checkbox::CheckboxInputField;
use crate::utils::forms::fire_bubbled_and_cancelable_event;
use leptos::ev;
use leptos::prelude::*;

/// ToggleSwitch is a component that renders a toggle switch input field.
/// It can be used in forms to collect user input.
/// Example usage:
/// ```
/// <ToggleSwitch
///    active=RwSignal::new(true)
///    label_active="Enabled"
///    label_inactive="Disabled"
///    name="status"
/// />
/// ```
#[component]
pub fn ToggleSwitch(
    #[prop(into)] name: String,
    #[prop(into, optional, default = RwSignal::new(false))] active: RwSignal<bool>,
    #[prop(into, default = "On".into())] label_active: String,
    #[prop(into, default = "Off".into())] label_inactive: String,
    #[prop(into, optional)] id_attr: String,
    #[prop(into, optional)] label: String,
    #[prop(default = false)] required: bool,
    #[prop(optional, default = Callback::new(|_| {}))] oninput: Callback<ev::Event>,
) -> impl IntoView {
    let checkbox_ref = NodeRef::new();
    let initial_value = RwSignal::new(String::from("false"));

    let handle_toggle = move |ev: ev::MouseEvent| {
        ev.stop_propagation();
        active.set(!active.get());
    };

    Effect::new(move |_| {
        let _value_hack = initial_value.get();
        // Fire a bubbling Change event so that the form can capture changes
        if let Some(input_el) = checkbox_ref.get() {
            fire_bubbled_and_cancelable_event("input", true, true, &input_el);
        }
    });

    Effect::new(move |_| {
        initial_value.set(active.get().to_string());
    });

    view! {
        <div class="flex flex-col cursor-pointer relative">
            <CheckboxInputField input_node_ref=checkbox_ref oninput=oninput initial_value=initial_value label=label name=name id_attr=id_attr checked=active ext_wrapper_styles="absolute opacity-0" required=required />
            <div class="flex items-center">
                <div on:click=handle_toggle class="relative">
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
                <div class="flex items-center ml-3 text-gray-500 font-medium">
                    <p>{
                        move || {
                            if active.get() {
                                label_active.clone()
                            } else {
                                label_inactive.clone()
                            }
                        }
                    }</p>
                </div>
            </div>
        </div>
    }
}
