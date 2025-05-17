use leptos::ev;
use leptos::html::*;
use leptos::prelude::*;

// Define the Leptos component
#[component]
pub fn Textarea(
    #[prop(default = "".to_string(), optional)] initial_value: String,
    label: String,
    name: String,
    #[prop(optional)] input_node_ref: NodeRef<Textarea>,
    #[prop(default = false, optional)] readonly: bool,
    #[prop(default = false, optional)] required: bool,
    #[prop(default = "".to_string(), optional)] placeholder: String,
    #[prop(optional, default = Callback::new(|_| {}))] oninput: Callback<ev::Event>,
    #[prop(default = "".to_string(), optional)] ext_input_styles: String,
) -> impl IntoView {
    // Create reactive state for display_error
    let (display_error, set_display_error) = signal(false);

    view! {
        <div class="mb-4">
            <label class="block text-gray-700 text-sm font-bold mb-2" for={name.clone()}>
                {label}
                {move || if required {
                    Some(view! { <span class="text-red-500">"*"</span> })
                } else {
                    None
                }}
            </label>
            <textarea
                class=move || format!(
                    "form-input ring-0 shadow appearance-none border border-slate-400 rounded w-full py-2 px-3 text-gray-700 leading-tight focus:outline-none focus:ring-2 focus:ring-blue-500 focus:border-transparent flex-grow {}",
                    ext_input_styles
                )
                // value={initial_value.clone()}
                name={name.clone()}
                node_ref=input_node_ref
                readonly={readonly}
                on:input=move |ev| oninput.run(ev)
                placeholder={placeholder.clone()}
                id={name.clone()}
            />
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
