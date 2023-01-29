use yew::prelude::*;
use yew_router::prelude::use_navigator;

use crate::app::Route;

#[function_component(BackHome)]
pub fn back_home() -> Html {
    let navigator = use_navigator().unwrap();

    let onclick = Callback::from(move |_| navigator.push(&Route::Home));
    
    html! {
        <div class="back-home">
        <div {onclick} class="icon-container">
        </div>
        </div>
    }
}