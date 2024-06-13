use yew::prelude::*;

#[derive(Clone, PartialEq, Properties)]
pub struct TextAreaProps {
    pub initial_value: Option<String>,
    pub label: String,
    pub name: String,
    pub input_node_ref: Option<NodeRef>,
    pub readonly: Option<bool>,
    pub required: Option<bool>,
    pub placeholder: Option<String>,
    pub oninput: Option<Callback<InputEvent>>,
}

#[function_component(TextArea)]
pub fn text_area(props: &TextAreaProps) -> Html {
    let TextAreaProps {
        initial_value,
        label,
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
        <div class="form-group
            textarea
            ">
            <label for={name.clone()}>
                { label }{ if field_required { html!{ <span class="error">{ "*" }</span> }  } else { html!{} } }
            </label>
            <textarea
                value={initial_value.clone().unwrap_or("".to_string())}
                name={name.clone()}
                ref={input_node_ref.clone().unwrap_or(NodeRef::default())}
                readonly={readonly.unwrap_or(false)}
                oninput={oninput.clone().unwrap_or(Callback::noop())}
                placeholder={placeholder.clone().unwrap_or("".to_string())}
            />
            <p class="error">{ if *display_error { "This field is required" } else { "" }  }</p>
        </div>
    }
}