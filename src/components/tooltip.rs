use yew::prelude::*;

#[derive(Clone, Properties, PartialEq)]
pub struct TooltipProps {
    pub children: Children,
    pub text: String,
}

#[function_component(Tooltip)]
pub fn tool_tip(props: &TooltipProps) -> Html {
    
    html! {
        <div class="tooltip">
            {props.children.clone()}
            <span class="tooltiptext">{ props.text.clone() }</span>
        </div>
    }
}