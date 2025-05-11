use leptos::ev;
use leptos::html::*;
use leptos::prelude::*;

#[derive(Clone, PartialEq)]
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
    #[prop(default = "".to_string())] initial_value: String,
    #[prop(default = "".to_string())] label: String,
    field_type: InputFieldType,
    name: String,
    #[prop(optional)] input_node_ref: Option<NodeRef<Input>>,
    #[prop(default = false)] readonly: bool,
    #[prop(default = false)] required: bool,
    #[prop(default = "".to_string())] placeholder: String,
    #[prop(default = None)] oninput: Option<Callback<ev::Event>>,
    #[prop(default = None)] onclick: Option<Callback<ev::MouseEvent>>,
    #[prop(default = "".to_string())] ext_wrapper_styles: String,
    #[prop(default = "".to_string())] ext_label_styles: String,
    #[prop(default = "".to_string())] ext_input_styles: String,
    #[prop(default = "on".to_string())] autocomplete: String,
) -> impl IntoView {
    let (display_error, set_display_error) = signal(false);

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
                for={name.clone()}
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
                type={input_field_type_str}
                value={initial_value}
                name={name.clone()}
                node_ref={input_node_ref.unwrap_or(NodeRef::new())}
                readonly={readonly}
                on:input={move |ev| {
                    if let Some(cb) = oninput {
                        cb.run(ev);
                    }
                }}
                placeholder={placeholder}
                autocomplete={autocomplete}
                id={name.clone()}
                on:click={move |ev| {
                    if let Some(cb) = onclick {
                        cb.run(ev);
                    }
                }}
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
