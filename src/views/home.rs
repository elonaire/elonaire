use yew::prelude::*;

use crate::{app::{AppStateContext}, components::nav::Nav};

#[function_component(Home)]
pub fn home() -> Html {
    let current_state = use_context::<AppStateContext>().unwrap();

    html! {
        <main>
        <Nav />
        <div class="home">
        <img class="profile-image" src="img/me.png" alt="profile-image" />
        <div class="left">
        <h1>{ "I'm " } <span class="primary-color-text">{current_state.first_name.clone() + " "}</span>{ current_state.middle_name.clone() + " " +  &current_state.last_name}</h1>
        <p class="description">{ current_state.description.clone() }</p>
        <button class="button button-primary glow-on-hover">{"Download CV"}</button>
        </div>

        <div class="right">
        <h2 class="title">{ current_state.title.clone() }</h2>
        </div>
        </div>
        </main>
    }
}