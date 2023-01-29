use yew::prelude::*;

use crate::components::{transition::Transition, back_home::BackHome};

#[function_component(About)]
pub fn about() -> Html {

    html! {
        <>
        <Transition />
        <main class="about">
        <BackHome />
        <div>
        <p class="who">{ "Who am I?" }</p>
        <h2 class="about-head">{ "About me" }</h2>
        </div>
        </main>
        </>
    }
}