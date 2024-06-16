use yew::prelude::*;

#[derive(Clone, PartialEq, Properties)]
pub struct InputFieldProps {
    pub initial_value: Option<String>,
    // pub on_cautious_change: Callback<ChangeData>,
    pub label: String,
    pub field_type: String,
    pub name: String,
    pub input_node_ref: Option<NodeRef>,
    pub readonly: Option<bool>,
    pub required: Option<bool>,
    pub placeholder: Option<String>,
    pub oninput: Option<Callback<InputEvent>>,
}

#[function_component(InputField)]
pub fn input_field(props: &InputFieldProps) -> Html {
    let InputFieldProps {
        initial_value,
        // on_cautious_change,
        label,
        field_type,
        name,
        input_node_ref,
        readonly,
        required,
        placeholder,
        oninput,
    } = props;

    let field_required = required.unwrap_or(false);
    let display_error = use_state(|| false);

    html! {
    <div class="form-group">
        <label for={name.clone()}>
                { label }{ if field_required { html!{ <span class="error">{ "*" }</span> }  } else { html!{} } }
        </label>
        <input
                    // oninput={on_cautious_change}
                    type={field_type.clone()}
                    value={initial_value.clone().unwrap_or("".to_string())}
                    name={name.clone()}
                    ref={input_node_ref.clone().unwrap_or(NodeRef::default())}
                    readonly={readonly.unwrap_or(false)}
                    oninput={oninput.clone().unwrap_or(Callback::noop())}
                    placeholder={placeholder.clone().unwrap_or("".to_string())}
                    autocomplete="on"
                    id={name.clone()}
                />
                <p class="error">{ if *display_error { "This field is required" } else { "" }  }</p>
    </div>
    }
}
