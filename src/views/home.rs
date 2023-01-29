use yew::prelude::*;

use crate::{app::AppState, components::nav::Nav};

#[function_component(Home)]
pub fn home() -> Html {
    let current_state = use_context::<AppState>().expect("no state found");

    html! {
        <main>
        <Nav />
        <div class="home">
        <div class="left">
        <h1>{ "I'm " } <span class="primary-color-text"></span>{ current_state.full_name }</h1>
        <p class="description">{ current_state.description }</p>
        <button class="button button-primary">{"Download CV"}</button>
        </div>

        <div class="right">
        <h2 class="title">{ current_state.title }</h2>
        </div>
        </div>
        </main>
    }
}