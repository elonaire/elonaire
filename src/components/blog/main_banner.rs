use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct MainBannerProps {
    pub title: String,
    pub subtitle: String,
    pub background_url: String,
}

#[function_component(MainBanner)]
pub fn main_banner(props: &MainBannerProps) -> Html {
    html! {
        <div class="main-banner" style={format!("background-image: url({});", props.background_url)}>
            <div class="content">
                <h1>{ &props.title }</h1>
                <p>{ &props.subtitle }</p>
            </div>
        </div>
    }
}
