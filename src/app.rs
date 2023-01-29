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
    pub full_name: String
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
            title: "This is my first Yew app".to_owned(),
            description: "This is my description".to_owned(),
            full_name: "Elon Aseneka Idiong'o".to_owned(),
        });

    html! {
        <ContextProvider<AppState> context={(*state).clone()}>
        <BrowserRouter>
            <Switch<Route> render={switch} /> // <- must be child of <BrowserRouter>
        </BrowserRouter>
        </ContextProvider<AppState>>
    }
}
