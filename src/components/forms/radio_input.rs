use leptos::ev;
use leptos::html::Input;
use leptos::prelude::*;

// Define the Leptos component
#[component]
pub fn RadioInputField(
    #[prop(into, optional, default = Signal::derive(move || "".to_string()))] initial_value: Signal<
        String,
    >,
    #[prop(into, optional)] label: String,
    #[prop(into, optional)] name: String,
    #[prop(optional)] input_node_ref: NodeRef<Input>,
    #[prop(default = false, optional)] readonly: bool,
    #[prop(default = false, optional)] required: bool,
    #[prop(into, optional)] placeholder: String,
    #[prop(optional, default = Callback::new(|_| {}))] oninput: Callback<ev::Event>,
    #[prop(into, optional)] id_attr: String,
    #[prop(optional)] children: Option<Children>,
    #[prop(into, optional)] input_style_ext: String,
    #[prop(into, optional, default = "off".to_string())] autocomplete: String,
) -> impl IntoView {
    // Create reactive state for display_error
    let (display_error, _set_display_error) = signal(false);

    view! {
        <div class="mb-2">
            <label class="inline-flex items-center gap-2 text-gray-700 text-sm cursor-pointer" for=id_attr.clone()>
                <input
                    class=move || format!(
                        "leading-tight rounded-full border-gray-300 text-blue-950 shadow-sm focus:border-blue-950 focus:ring focus:ring-offset-0 focus:ring-indigo-200 focus:ring-opacity-50 {}",
                        input_style_ext
                    )
                    type="radio"
                    value=initial_value
                    name=name
                    node_ref=input_node_ref
                    readonly=readonly
                    on:input=move |ev| oninput.run(ev)
                    placeholder=placeholder
                    autocomplete=autocomplete
                    id=id_attr.clone()
                    required=required
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
                    {children.map(|child| child())}
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
