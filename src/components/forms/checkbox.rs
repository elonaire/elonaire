use leptos::ev;
use leptos::html::*;
use leptos::prelude::*;

#[component]
pub fn CheckboxInputField(
    #[prop(into, default = Signal::derive(move || "".to_string()), optional)] initial_value: Signal<
        String,
    >,
    #[prop(optional)] label: String,
    name: String,
    #[prop(optional)] input_node_ref: NodeRef<Input>,
    #[prop(default = false, optional)] readonly: bool,
    #[prop(default = false, optional)] required: bool,
    #[prop(optional)] placeholder: String,
    #[prop(optional, default = Callback::new(|_| {}))] oninput: Callback<ev::Event>,
    #[prop(optional)] id_attr: String,
    #[prop(optional)] ext_input_styles: String,
    #[prop(optional,default = "off".to_string())] autocomplete: String,
) -> impl IntoView {
    let (display_error, _set_display_error) = signal(false);

    view! {
        <div class="mb-4">
            <label
                class="inline-flex items-center gap-2 text-gray-700 text-sm font-bold"
                for={
                    if id_attr.is_empty() {
                        name.clone()
                    } else {
                        id_attr.clone()
                    }
                }
            >
                <input
                    class=format!("leading-tight rounded border-gray-300 text-blue-950 shadow-sm focus:border-blue-950 focus:ring focus:ring-offset-0 focus:ring-indigo-200 focus:ring-opacity-50 {}", ext_input_styles)
                    type="checkbox"
                    value=initial_value
                    name=name.clone()
                    node_ref=input_node_ref
                    readonly=readonly
                    on:input=move |ev| oninput.run(ev)
                    placeholder=placeholder
                    autocomplete=autocomplete
                    id=id_attr.clone()
                    required=required
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
