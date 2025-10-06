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

impl SelectOption {
    #[allow(dead_code)]
    pub fn new(value: &str, label: &str) -> Self {
        Self {
            value: value.into(),
            label: label.into(),
        }
    }
}

/// This is a custom select input component that allows you to create a dropdown menu with custom options.
/// Example usage:
/// ```
/// <SelectInput
///    initial_value="est"
///    label="Time Zone"
///    name="timezone"
///    required=true
///    options=vec![
///       SelectOption::new("", "--Select Timezone"),
///       SelectOption::new("utc", "UTC"),
///       SelectOption::new("est", "EST"),
///    ]
/// />
/// ```
/// You may use the SelectOption struct to create custom options for the SelectInput component.
#[component]
pub fn SelectInput(
    #[prop(into, default = Signal::derive(move || "".to_string()), optional)] initial_value: Signal<
        String,
    >,
    #[prop(into)] label: String,
    #[prop(into)] name: String,
    #[prop(optional)] input_node_ref: NodeRef<Select>,
    #[prop(default = false, optional)] readonly: bool,
    options: Vec<SelectOption>,
    #[prop(default = false, optional)] required: bool,
    #[prop(optional, default = Callback::new(|_| {}))] onchange: Callback<ev::Event>,
    #[prop(into, optional)] ext_input_styles: String,
    #[prop(into, optional)] id_attr: String,
) -> impl IntoView {
    // Create reactive state for display_error
    let (display_error, _set_display_error) = signal(false);

    view! {
        <div>
            <label for=id_attr.clone() class="block text-gray-700 text-sm font-bold">
                {label}
                {move || if required {
                    Some(view! { <span class="text-red-500">"*"</span> })
                } else {
                    None
                }}
            </label>
            <select
                node_ref=input_node_ref
                name=name
                class=move || format!(
                    "form-input ring-0 shadow appearance-none border border-slate-400 rounded w-full py-2 px-3 text-gray-700 leading-tight focus:outline-none focus:ring-2 focus:ring-blue-500 focus:border-transparent flex-grow {}",
                    ext_input_styles
                )
                // value={initial_value.clone()}
                // readonly={readonly}
                on:change=move |ev| onchange.run(ev)
                id=id_attr.clone()
                required=required
            >
                {options.into_iter()
                    .map(|option| {
                        view! {
                            <option
                                value={option.value.clone()}
                                selected={ move ||
                                    !initial_value.get().is_empty() && initial_value.get() == option.value.clone()
                                }
                            >
                                {option.label.clone()}
                            </option>
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
