use leptos::ev;
use leptos::html::*;
use leptos::prelude::*;

#[derive(Clone, PartialEq)]
#[allow(dead_code)]
pub enum InputFieldType {
    Text,
    Email,
    Date,
    Number,
    Password,
    Tel,
    Url,
    Search,
    Color,
    Range,
    File,
    Hidden,
    Image,
    Month,
    Time,
    Week,
}

/// This is a component for creating input fields with various types.
/// It provides a flexible way to create input fields with different types, such as text, email, date, number, password, tel, url, search, color, range, file, hidden, image, month, time and week by using the `InputFieldType` enum.
/// The component also supports various attributes such as label, name, placeholder, and event handlers.
/// Examples:
/// ```
/// <InputField field_type=InputFieldType::Text name="name" />
/// ```

#[component]
pub fn InputField(
    #[prop(into, optional, default = Signal::derive(move || "".to_string()))] initial_value: Signal<
        String,
    >,
    #[prop(into, optional)] label: String,
    field_type: InputFieldType,
    #[prop(into, optional)] name: String,
    #[prop(optional)] input_node_ref: NodeRef<Input>,
    #[prop(default = false)] readonly: bool,
    #[prop(default = false)] required: bool,
    #[prop(into, optional)] placeholder: String,
    #[prop(optional, default = Callback::new(|_| {}))] oninput: Callback<ev::Event>,
    #[prop(optional, default = Callback::new(|_| {}))] onchange: Callback<ev::Event>,
    #[prop(optional, default = Callback::new(|_| {}))] onclick: Callback<ev::MouseEvent>,
    #[prop(into, optional)] ext_wrapper_styles: String,
    #[prop(into, optional)] ext_label_styles: String,
    #[prop(into, optional)] ext_input_styles: String,
    #[prop(into, optional, default = "off".to_string())] autocomplete: String,
    #[prop(into, optional)] id_attr: String,
) -> impl IntoView {
    let (display_error, _set_display_error) = signal(false);

    let input_field_type_str = match field_type {
        InputFieldType::Text => "text",
        InputFieldType::Email => "email",
        InputFieldType::Date => "date",
        InputFieldType::Number => "number",
        InputFieldType::Password => "password",
        InputFieldType::Tel => "tel",
        InputFieldType::Url => "url",
        InputFieldType::Search => "search",
        InputFieldType::Color => "color",
        InputFieldType::Range => "range",
        InputFieldType::File => "file",
        InputFieldType::Hidden => "hidden",
        InputFieldType::Image => "image",
        InputFieldType::Month => "month",
        InputFieldType::Time => "time",
        InputFieldType::Week => "week",
    };

    view! {
        <div class=move || format!("mb-2 {}", ext_wrapper_styles)>
            <label
                class={format!("block text-gray-700 text-sm font-bold {}", ext_label_styles)}
                for=id_attr.clone()
            >
                {label}
                {move || if required {
                    Some(view! { <span class="text-red-500">"*"</span> })
                } else {
                    None
                }}
            </label>
            <input
                class={format!(
                    "form-input ring-0 shadow appearance-none border border-slate-400 rounded w-full py-2 px-3 text-gray-700 leading-tight focus:outline-none focus:ring-2 focus:ring-blue-500 focus:border-transparent flex-grow {}",
                    ext_input_styles
                )}
                type=input_field_type_str
                value=initial_value
                name=name
                node_ref=input_node_ref
                readonly=readonly
                on:input={move |ev| oninput.run(ev)}
                placeholder=placeholder
                autocomplete=autocomplete
                id=id_attr.clone()
                on:click={move |ev| onclick.run(ev)}
                required=required
                on:change={move |ev| onchange.run(ev)}
            />
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
    }
}
