use yew::prelude::*;

#[function_component(Loader)]
pub fn loader() -> Html {

    html! {
        <div class="loader">
            <div class="lds-hourglass"></div>
        </div>
    }
}