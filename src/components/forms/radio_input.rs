use leptos::ev;
use leptos::prelude::*;

/// Represents a single radio option with a value and display text.
/// You can also provide custom children for complex rendering (e.g., with icons).
#[derive(Clone)]
pub struct RadioOption {
    pub value: String,
    pub label: String,
    pub children: Option<ViewFn>,
}

impl std::fmt::Debug for RadioOption {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("RadioOption")
            .field("value", &self.value)
            .field("label", &self.label)
            .field("children", &"<ViewFn>")
            .finish()
    }
}

impl RadioOption {
    #[allow(dead_code)]
    pub fn new(value: &str, label: &str, children: Option<ViewFn>) -> Self {
        Self {
            value: value.to_string(),
            label: label.to_string(),
            children,
        }
    }
}

/// This component represents a radio input field.
/// Example usage:
/// ```
/// <RadioInputField label="Male" id_attr="male_lone" />
/// ```
/// It also allows the use of children properties to customize the appearance and behavior of the radio input field. You may use the children property to add custom content to the radio input field. e.g images
/// Example usage:
/// ```
/// <RadioInputField id_attr="male">
///     <div class="flex items-center gap-2">
///         <span class="text-gray-700 text-sm">Male</span>
///     </div>
/// </RadioInputField>
/// ```
#[component]
pub fn RadioInputField(
    #[prop(into, optional, default = RwSignal::new("".to_string()))] initial_value: RwSignal<
        String,
    >,
    /// The legend text for the fieldset
    #[prop(into, optional)]
    legend: String,
    #[prop(into, optional)] options: Vec<RadioOption>,
    #[prop(into, optional)] name: String,
    #[prop(default = false, optional)] required: bool,
    #[prop(optional, default = Callback::new(|_| {}))] oninput: Callback<ev::Event>,
    #[prop(default = false, optional)] horizontal: bool,
    /// Additional CSS classes for the fieldset
    #[prop(into, optional, default = "".to_string())]
    fieldset_class: String,
    /// Additional CSS classes for the legend
    #[prop(into, optional, default = "".to_string())]
    legend_class: String,
) -> impl IntoView {
    // Create reactive state for display_error
    let base_fieldset_class = "border border-gray-300 rounded p-4";
    let base_legend_class = "text-mid-gray text-sm font-bold px-2";

    let container_class = if horizontal {
        "flex flex-wrap gap-4"
    } else {
        "space-y-3"
    };

    let fieldset_combined_class = format!("{} {}", base_fieldset_class, fieldset_class);
    let legend_combined_class = format!("{} {}", base_legend_class, legend_class);

    view! {
        <fieldset class=fieldset_combined_class>
                    <legend class=legend_combined_class>
                        {legend}
                        {if required {
                            Some(view! { <span class="text-red-500 ml-1">*</span> })
                        } else {
                            None
                        }}
                    </legend>
                    <div class=container_class>
                        {options
                            .into_iter()
                            .map(|option| {
                                let option_value_selected = option.value.clone();
                                let option_value = option.value.clone();

                                let is_selected = move || initial_value.get() == option_value_selected;

                                view! {
                                    <label class="inline-flex items-center gap-2 text-mid-gray text-sm cursor-pointer px-2 py-1 rounded">
                                        <input
                                            class="leading-tight rounded-full border-gray-300 text-blue-600 shadow-sm focus:border-blue-600 focus:ring focus:ring-offset-0 focus:ring-indigo-200 focus:ring-opacity-50"
                                            type="radio"
                                            name=name.clone()
                                            value=option_value
                                            checked=is_selected
                                            required=required
                                            on:input=move |ev| {
                                                oninput.run(ev);
                                            }
                                        />
                                        <div class="flex flex-col">
                                            <span>{option.label}</span>
                                            {option.children.map(|children| children.run())}
                                        </div>
                                    </label>
                                }
                            })
                            .collect_view()}
                    </div>
                </fieldset>
    }
}
