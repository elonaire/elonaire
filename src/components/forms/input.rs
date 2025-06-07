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

#[component]
pub fn InputField(
    #[prop(into, optional, default = Signal::derive(move || "".to_string()))] initial_value: Signal<
        String,
    >,
    #[prop(optional)] label: String,
    field_type: InputFieldType,
    #[prop(optional)] name: String,
    #[prop(optional)] input_node_ref: NodeRef<Input>,
    #[prop(default = false)] readonly: bool,
    #[prop(default = false)] required: bool,
    #[prop(optional)] placeholder: String,
    #[prop(optional, default = Callback::new(|_| {}))] oninput: Callback<ev::Event>,
    #[prop(optional, default = Callback::new(|_| {}))] onclick: Callback<ev::MouseEvent>,
    #[prop(optional)] ext_wrapper_styles: String,
    #[prop(optional)] ext_label_styles: String,
    #[prop(optional)] ext_input_styles: String,
    #[prop(optional,default = "off".to_string())] autocomplete: String,
    #[prop(optional)] id_attr: String,
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
        <div class={ext_wrapper_styles}>
            <label
                class={format!("block text-gray-700 text-sm font-bold {}", ext_label_styles)}
                for={
                    if id_attr.is_empty() {
                        name.clone()
                    } else {
                        id_attr.clone()
                    }
                }
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
                name=name.clone()
                node_ref=input_node_ref
                readonly=readonly
                on:input={move |ev| oninput.run(ev)}
                placeholder=placeholder
                autocomplete=autocomplete
                id={
                    if id_attr.is_empty() {
                        name.clone()
                    } else {
                        id_attr.clone()
                    }
                }
                on:click={move |ev| onclick.run(ev)}
                required=required
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
