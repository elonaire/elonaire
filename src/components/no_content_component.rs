use yew::prelude::*;

#[function_component(NoContent)]
pub fn no_content() -> Html {
    
    html! {
        <h4 class="no-content">{"Oops! No content."}</h4>
    }
}
