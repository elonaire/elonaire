use icondata as IconData;
use icondata::Icon as IconId;
use leptos::ev;
use leptos::html::*;
use leptos::prelude::*;
use leptos_icons::Icon;

use crate::components::general::button::BasicButton;

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
    #[prop(into, optional)] ext_wrapper_styles: String,
    #[prop(into, optional)] ext_label_styles: String,
    #[prop(into, optional)] ext_input_styles: String,
    #[prop(into, optional, default = "off".to_string())] autocomplete: String,
    #[prop(into, optional)] id_attr: String,
    #[prop(into, optional)] accept: String,
    #[prop(into, optional)] multiple: bool,
    #[prop(optional, default = None)] icon: Option<IconId>,
    #[prop(optional, default = true)] icon_is_leading: bool,
    #[prop(into, optional)] min: String,
    #[prop(into, optional)] max: String,
    #[prop(optional, default = Callback::new(|_| {}))] onfocus: Callback<ev::FocusEvent>,
    #[prop(optional, default = Callback::new(|_| {}))] onblur: Callback<ev::FocusEvent>,
) -> impl IntoView {
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
    let (show_password, set_show_password) = signal(false);

    let step = match field_type {
        InputFieldType::Number => Some("any"),
        _ => None,
    };

    view! {
        <div class=move || format!("{}", ext_wrapper_styles)>
            {
                if label.is_empty() {
                    None
                } else {
                    Some(
                        view! {
                            <label
                                class={format!("block text-mid-gray text-sm font-bold {}", ext_label_styles)}
                                for=id_attr.clone()
                            >
                                {label}
                                {move || required.then_some(view! {
                                    <span class="text-danger ml-1">*</span>
                                })}
                            </label>
                        }
                    )
                }
            }
            <div
            class=move || format!(
                    "h-[45px] flex items-center border border-mid-gray rounded-[5px]
                     shadow appearance-none
                     focus-within:ring-2 focus-within:ring-secondary
                     focus-within:border-transparent
                     {} {}",
                    if icon_is_leading { "" } else { "flex-row-reverse" },
                    ext_input_styles
                )
                >
                {
                    icon.map(|icon_id| view!{
                        <div class=format!("h-full flex items-center px-3 justify-center")>
                            <Icon icon=icon_id width="1rem" height="1rem" />
                        </div>
                    })
                }
                <input
                    class=format!(
                        "w-full h-full py-2 px-3 text-gray leading-tight flex-grow focus:outline-none"
                    )
                    type=move || if show_password.get() { "text" } else { input_field_type_str }
                    prop:value=initial_value
                    name=name
                    node_ref=input_node_ref
                    readonly=readonly
                    placeholder=placeholder
                    autocomplete=autocomplete
                    id=id_attr.clone()
                    required=required
                    accept=accept
                    multiple=multiple
                    step=step
                    on:focus=move |e| onfocus.run(e)
                    on:blur=move |e| onblur.run(e)
                    min=min
                    max=max
                />
                {move ||
                    {
                        let show_password_val = show_password.get();

                        if field_type == InputFieldType::Password {
                            Some(
                                view!{
                                    <div on:click=move |_e| set_show_password.set(!show_password.get()) class=format!("h-full flex items-center px-3 justify-center cursor-pointer")>
                                        <Icon icon={if show_password_val { IconData::BsEyeSlash } else { IconData::BsEye }} width="1rem" height="1rem" />
                                    </div>
                                }
                            )
                        } else {
                            None
                        }
                    }
                }
            </div>
        </div>
    }
}

#[component]
pub fn CustomFileInput(
    #[prop(into, optional)] label: String,
    #[prop(into, optional)] name: String,
    #[prop(default = false, optional)] required: bool,
    #[prop(into, optional)] id_attr: String,
    #[prop(into, optional)] ext_label_styles: String,
    #[prop(optional)] input_node_ref: NodeRef<Input>,
    #[prop(into, optional)] multiple: bool,
    #[prop(into, optional)] accept: String,
) -> impl IntoView {
    view! {
        <div class="relative">
            <InputField
                name=name
                label=label
                required=required
                field_type=InputFieldType::File
                ext_input_styles="absolute inset-y-0 left-0 w-full opacity-0"
                id_attr=id_attr.clone()
                input_node_ref=input_node_ref
                multiple=multiple
                accept=accept
            />
               <BasicButton
                   button_text="Choose File"
                   icon=Some(IconData::FiUpload)
                   icon_before=true
                   style_ext="w-full bg-primary text-contrast-white"
                />
        </div>
    }
}
