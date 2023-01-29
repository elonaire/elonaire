use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct PageHeaderProps {
    pub hint: String,
    pub heading: String,
}

#[function_component(PageHeader)]
pub fn page_header(props: &PageHeaderProps) -> Html {

    html! {
        <div class="page-header">
        <p class="who">{ props.hint.clone() }</p>
        <h2 class="about-head">{ props.heading.clone() }</h2>
        <div class="underline-container">
        <div class="left"></div>
        <div class="underline"></div>
        <div class="right"></div>
        </div>
        </div>
    }
}