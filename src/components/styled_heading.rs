use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct StyledHeadingProps {
    pub heading: String,
}

#[function_component(StyledHeading)]
pub fn styled_heading(props: &StyledHeadingProps) -> Html {
    
    html! {
        <div class="styled-heading">
        <h2>{props.heading.clone()}</h2>
        </div>
    }
}