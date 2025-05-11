use leptos::ev;
use leptos::html::*;
use leptos::prelude::*;

#[component]
pub fn CheckboxInputField(
    #[prop(default = "".to_string())] initial_value: String,
    #[prop(default = "".to_string())] label: String,
    name: String,
    #[prop(optional)] input_node_ref: Option<NodeRef<Input>>,
    #[prop(default = false)] readonly: bool,
    #[prop(default = false)] required: bool,
    #[prop(default = "".to_string())] placeholder: String,
    #[prop(default = None)] oninput: Option<Callback<ev::Event>>,
    id_attr: String,
) -> impl IntoView {
    let (display_error, set_display_error) = signal(false);

    view! {
        <div class="mb-4">
            <label
                class="inline-flex items-center gap-2 text-gray-700 text-sm font-bold"
                for={id_attr.clone()}
            >
                <input
                    class="leading-tight rounded border-gray-300 text-blue-950 shadow-sm focus:border-blue-950 focus:ring focus:ring-offset-0 focus:ring-indigo-200 focus:ring-opacity-50"
                    type="checkbox"
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
                    autocomplete="on"
                    id={id_attr.clone()}
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
    }
}
