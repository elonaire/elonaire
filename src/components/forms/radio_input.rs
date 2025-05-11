use leptos::ev;
use leptos::html::Input;
use leptos::prelude::*;

// Define the Leptos component
#[component]
pub fn RadioInputField(
    #[prop(default = "".to_string())] initial_value: String,
    label: String,
    name: String,
    #[prop(default = None)] input_node_ref: Option<NodeRef<Input>>,
    #[prop(default = false)] readonly: bool,
    #[prop(default = false)] required: bool,
    #[prop(default = "".to_string())] placeholder: String,
    #[prop(default = None)] oninput: Option<Callback<ev::Event>>,
    id_attr: String,
    children: Children,
    #[prop(default = "".to_string())] input_style_ext: String,
) -> impl IntoView {
    // Create reactive state for display_error
    let (display_error, set_display_error) = signal(false);

    view! {
        <div class="mb-4">
            <label class="inline-flex items-center gap-2 text-gray-700 text-sm cursor-pointer" for={id_attr.clone()}>
                <input
                    class=move || format!(
                        "leading-tight rounded-full border-gray-300 text-blue-950 shadow-sm focus:border-blue-950 focus:ring focus:ring-offset-0 focus:ring-indigo-200 focus:ring-opacity-50 {}",
                        input_style_ext
                    )
                    type="radio"
                    value={initial_value.clone()}
                    name={name.clone()}
                    node_ref=input_node_ref.unwrap_or_default()
                    readonly={readonly}
                    on:input=move |ev| {
                        if let Some(oninput) = oninput {
                            oninput.run(ev);
                        }
                    }
                    placeholder={placeholder.clone()}
                    autocomplete="on"
                    id={id_attr.clone()}
                />
                <div class="flex flex-col">
                    <span>
                        {label}
                        {move || if required {
                            Some(view! { <span class="text-red-500">"*"</span> })
                        } else {
                            None
                        }}
                    </span>
                    {children()}
                </div>
            </label>

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
