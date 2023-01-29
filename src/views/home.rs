use yew::prelude::*;

use crate::{app::AppState, components::nav::Nav};

#[function_component(Home)]
pub fn home() -> Html {
    let current_state = use_context::<AppState>().expect("no state found");

    html! {
        <main>
        <Nav />
            <h1>{ current_state.title }</h1>
        </main>
    }
}