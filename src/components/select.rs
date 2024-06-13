use yew::prelude::*;


#[derive(Clone, PartialEq, Debug, Properties)]
pub struct SelectOption {
    pub value: String,
    pub label: String,
}

#[derive(Clone, PartialEq, Properties, Debug)]
pub struct SelectInputProps {
    pub initial_value: Option<String>,
    pub label: String,
    pub name: String,
    pub input_node_ref: NodeRef,
    pub readonly: Option<bool>,
    pub options: Vec<SelectOption>,
    pub required: Option<bool>,
    pub onchange: Option<Callback<Event>>,
}

#[function_component(SelectInput)]
pub fn select_input(props: &SelectInputProps) -> Html {

    let field_required = props.required.unwrap_or(false);
    let display_error = use_state(|| false);
    
    html! {
        <div class="form-group">
            <label for={props.name.clone()}>{props.label.clone()}{ if field_required { html!{ <span class="error">{ "*" }</span> }  } else { html!{} } }</label>
            <select
                ref={props.input_node_ref.clone()}
                name={props.name.clone()}
                class="form-control"
                value={props.initial_value.clone().unwrap_or("".to_string())}
                readonly={props.readonly.unwrap_or(false)}
                onchange={props.onchange.clone().unwrap_or(Callback::noop())}
            >
                { for props.options.iter().map(|option| {
                    html! {
                        <option value={option.value.clone()}>{option.clone().label.clone()}</option>
                    }
                })}
            </select>
            <p class="error">{ if *display_error { "This field is required" } else { "" }  }</p>
        </div>
    }
}