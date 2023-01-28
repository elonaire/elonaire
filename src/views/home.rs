use yew::prelude::*;

use crate::app::AppState;

#[function_component(Home)]
pub fn home() -> Html {
    let current_state = use_context::<AppState>().expect("no ctx found");
    println!("current_state {:?}", current_state);

    html! {
        <main>
            <h1>{ current_state.title }</h1>
        </main>
    }
}