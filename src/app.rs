use yew::prelude::*;
use yew_router::prelude::*;

use crate::views::{home::Home, about::About, resume::Resume, portfolio::Portfolio};

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/about")]
    About,
    #[at("/resume")]
    Resume,
    #[at("/portfolio")]
    Portfolio,
    #[not_found]
    #[at("/404")]
    NotFound,
}

#[derive(Clone, Debug, PartialEq)]
pub struct AppState {
    pub title: String,
    pub description: String,
    pub full_name: String,
    pub auto_bio: String,
    pub first_name: String,
    pub middle_name: String,
    pub last_name: String,
    // pub date_of_birth: String,
}

pub fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => html! { <Home /> },
        Route::About => html! { <About /> },
        Route::Resume => html! { <Resume /> },
        Route::Portfolio => html! { <Portfolio /> },
        Route::NotFound => html! { <h1>{ "404" }</h1> },
    }
}

#[function_component(App)]
pub fn app() -> Html {
    let state = use_state(
        || AppState {
            title: "Software Engineer".to_owned(),
            description: "I am a talented full-stack software engineer, with 6+ years of experience in full-stack development. I have an interest in Game Development and the Internet of Things technology.".to_owned(),
            full_name: "Elon Aseneka Idiong'o".to_owned(),
            first_name: "Elon".to_owned(),
            middle_name: "Aseneka".to_owned(),
            last_name: "Idiong'o".to_owned(),
            auto_bio: "I am a talented full-stack software engineer and I have 6+ years of experience in building robust small and enterprise applications. I have built various web applications using Node.js, NestJS, Angular, and React. I have built desktop applications using ElectronJS and mobile applications using React Native. Besides that, I use Figma for application designing and prototyping. I have an interest in Game Development and the Internet of Things.".to_owned()
        });

    html! {
        <ContextProvider<AppState> context={(*state).clone()}>
        <BrowserRouter>
            <Switch<Route> render={switch} /> 
        </BrowserRouter>
        </ContextProvider<AppState>>
    }
}
