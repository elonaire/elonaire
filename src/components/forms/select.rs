use leptos::ev;
use leptos::html::Select;
use leptos::prelude::*;
use serde::{Deserialize, Serialize};

// Define the SelectOption struct
#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
pub struct SelectOption {
    pub value: String,
    pub label: String,
}

// Define the Leptos component
#[component]
pub fn SelectInput(
    #[prop(default = "".to_string(), optional)] initial_value: String,
    label: String,
    name: String,
    #[prop(optional)] input_node_ref: NodeRef<Select>,
    #[prop(default = false, optional)] readonly: bool,
    options: Vec<SelectOption>,
    #[prop(default = false, optional)] required: bool,
    #[prop(optional, default = Callback::new(|_| {}))] onchange: Callback<ev::Event>,
    #[prop(default = "".to_string(), optional)] ext_input_styles: String,
) -> impl IntoView {
    // Create reactive state for display_error
    let (display_error, _set_display_error) = signal(false);

    view! {
        <div class="mb-4">
            <label for={name.clone()} class="block text-gray-700 text-sm font-bold mb-2">
                {label}
                {move || if required {
                    Some(view! { <span class="text-red-500">"*"</span> })
                } else {
                    None
                }}
            </label>
            <select
                node_ref=input_node_ref
                name={name.clone()}
                class=move || format!(
                    "form-input ring-0 shadow appearance-none border border-slate-400 rounded w-full py-2 px-3 text-gray-700 leading-tight focus:outline-none focus:ring-2 focus:ring-blue-500 focus:border-transparent flex-grow {}",
                    ext_input_styles
                )
                // value={initial_value.clone()}
                // readonly={readonly}
                on:change=move |ev| onchange.run(ev)
                id={name.clone()}
            >
                {options.into_iter()
                    .map(|option| {
                        view! {
                            <option value={option.value.clone()}>{option.label.clone()}</option>
                        }
                    })
                    .collect::<Vec<_>>()}
            </select>
            <p class="text-red-500 text-xs italic">
                {move || if display_error.get() {
                    "This field is required"
                } else {
                    ""
                }}
            </p>
        </div>
    }
}
