use yew::prelude::*;
use yew_icons::{Icon, IconId};

use crate::{app::AppStateContext, components::nav::Nav};

#[function_component(Home)]
pub fn home() -> Html {
    let current_state = use_context::<AppStateContext>().unwrap();

    html! {
        <>
        <header>
            <Nav />
        </header>
        <main>
        <div class="home">
        <div class="left">
        <h1>{ "I'm " } <span class="primary-color-text">{current_state.first_name.clone() + " "}</span>{ current_state.middle_name.clone() + " " +  &current_state.last_name}</h1>
        <img class="profile-image" src="img/me.jpeg" alt="profile-image" />
        <p class="description">{ current_state.description.clone() }</p>
        <div class="button-container">
            <button class="button button-primary glow-on-hover">{"Download CV "}<Icon width={"1em".to_owned()} height={"1em".to_owned()} icon_id={IconId::BootstrapDownload}/></button>
        </div>
        </div>

        <div class="right">
        <h2 class="title">{ current_state.title.clone() }</h2>
        </div>
        </div>
        </main>
        </>
    }
}
