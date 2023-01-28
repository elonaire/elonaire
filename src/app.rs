use yew::prelude::*;

use crate::views::home::Home;

#[derive(Clone, Debug, PartialEq)]
pub struct AppState {
    pub title: String,
    pub description: String,
    pub full_name: String
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
        <Home />
        </ContextProvider<AppState>>
    }
}
