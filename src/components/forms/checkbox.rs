use leptos::ev;
use leptos::html::*;
use leptos::prelude::*;
use std::collections::HashSet;

/// Represents a single checkbox option with a value and display text.
/// You can also provide custom children for complex rendering (e.g., with icons).
#[derive(Clone)]
pub struct CheckboxOption {
    pub value: String,
    pub label: String,
    pub children: Option<ViewFn>,
}

impl std::fmt::Debug for CheckboxOption {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CheckboxOption")
            .field("value", &self.value)
            .field("label", &self.label)
            .field("children", &"<ViewFn>")
            .finish()
    }
}

/// CheckboxInputField is a component that renders a checkbox input field.
/// It can be used in forms to collect user input.
/// Example usage:
/// ```
/// // Single checkbox
/// <CheckboxInputField
///     label="Remember me"
///     name="remember"
/// />
///
/// // Multiple checkboxes in a group
/// let selected_options = create_rw_signal(HashSet::new());
/// <CheckboxInputField
///     legend="Choose your interests"
///     name="interests"
///     options=vec![
///         CheckboxOption {
///             value: "sports".to_string(),
///             label: "Sports".to_string(),
///             children: None,
///         },
///         CheckboxOption {
///             value: "music".to_string(),
///             label: "Music".to_string(),
///             children: None,
///         },
///     ]
///     selected_values=selected_options
///     required=true
/// />
/// ```
#[component]
pub fn CheckboxInputField(
    #[prop(into, default = RwSignal::new("".to_string()), optional)] initial_value: RwSignal<
        String,
    >,
    /// The legend text for the fieldset (when using multiple options)
    #[prop(into, optional)]
    legend: String,
    /// Options for multiple checkboxes
    #[prop(into, optional)]
    options: Vec<CheckboxOption>,
    /// Selected values for multiple checkboxes
    #[prop(into, optional)]
    selected_values: RwSignal<HashSet<String>>,
    #[prop(into, optional)] label: String,
    #[prop(into)] name: String,
    #[prop(optional)] input_node_ref: NodeRef<Input>,
    #[prop(default = false, optional)] readonly: bool,
    #[prop(default = false, optional)] required: bool,
    #[prop(into, default = Signal::derive(move || false), optional)] checked: Signal<bool>,
    #[prop(into, optional)] placeholder: String,
    #[prop(optional, default = Callback::new(|_| {}))] oninput: Callback<ev::Event>,
    #[prop(into, optional)] id_attr: String,
    #[prop(into, optional)] ext_input_styles: String,
    #[prop(into, optional,default = "off".to_string())] autocomplete: String,
    /// Display options horizontally instead of vertically (for multiple options)
    #[prop(default = false, optional)]
    horizontal: bool,
) -> impl IntoView {
    let (display_error, _set_display_error) = signal(false);
    // Render multiple checkboxes in a fieldset
    let base_fieldset_class = "border border-gray-300 rounded-lg p-4 bg-white relative";
    let base_legend_class =
        "text-sm font-medium text-gray-700 bg-white px-1 absolute -top-2 left-3";

    let container_class = if horizontal {
        "flex flex-wrap gap-4"
    } else {
        "space-y-3"
    };

    let fieldset_combined_class = base_fieldset_class.to_string();
    let legend_combined_class = base_legend_class.to_string();
    let options_is_empty = options.is_empty();

    if !options_is_empty {
        Some(view! {
            <fieldset class=fieldset_combined_class>
                <legend class=legend_combined_class>
                    {legend}
                    {move || if required {
                        Some(view! { <span class="text-red-500">"*"</span> })
                    } else {
                        None
                    }}
                </legend>
                <div class=container_class>
                    {options
                        .into_iter()
                        .map(|option| {
                            let option_value = option.value.clone();
                            let option_value_checked = option.value.clone();

                            let is_checked = move || selected_values.get().contains(&option_value_checked);
                            let option_id = format!("{}-{}", name, option_value);

                            view! {
                                <div class="mb-2">
                                    <label
                                        class="inline-flex items-center gap-2 text-gray-700 text-sm cursor-pointer"
                                        for=option_id.clone()
                                    >
                                        <input
                                            class=format!("leading-tight rounded border-gray-300 text-blue-950 shadow-sm focus:border-blue-950 focus:ring focus:ring-offset-0 focus:ring-indigo-200 focus:ring-opacity-50 {}", ext_input_styles)
                                            type="checkbox"
                                            value=option_value.clone()
                                            name=name.clone()
                                            checked=is_checked
                                            on:input=move |ev| oninput.run(ev)
                                            readonly=readonly
                                            autocomplete=autocomplete.clone()
                                            id=option_id.clone()
                                            required=required
                                        />
                                        <div class="flex flex-col">
                                            <span>{option.label}</span>
                                            {option.children.map(|child| child.run())}
                                        </div>
                                    </label>
                                </div>
                            }
                        })
                        .collect_view()}
                </div>
                <p class="text-red-500 text-xs italic">
                    {move || {
                        if display_error.get() {
                            if selected_values.get().is_empty() {
                                "At least one option must be selected"
                            } else {
                                ""
                            }
                        } else {
                            ""
                        }
                    }}
                </p>
            </fieldset>
        })
    } else {
        None
    };

    if options_is_empty {
        Some(view! {
            <div class="mb-2">
                <label
                    class="inline-flex items-center gap-2 text-gray-700 text-sm font-bold"
                    for=id_attr.clone()
                >
                    <input
                        class=format!("leading-tight rounded border-gray-300 text-blue-950 shadow-sm focus:border-blue-950 focus:ring focus:ring-offset-0 focus:ring-indigo-200 focus:ring-opacity-50 {}", ext_input_styles)
                        type="checkbox"
                        value=initial_value
                        name=name
                        node_ref=input_node_ref
                        readonly=readonly
                        on:input=move |ev| oninput.run(ev)
                        placeholder=placeholder
                        autocomplete=autocomplete
                        id=id_attr.clone()
                        required=required
                        checked=checked
                    />
                    <span>
                        {label}
                        {move || if required {
                            Some(view! { <span class="text-red-500">"*"</span> })
                        } else {
                            None
                        }}
                    </span>
                </label>
                <p class="text-red-500 text-xs italic">
                    {move || {
                        if display_error.get() {
                            "This field is required"
                        } else {
                            ""
                        }
                    }}
                </p>
            </div>
        })
    } else {
        None
    };
}
