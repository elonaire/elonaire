use yew::prelude::*;

use crate::data::models::resource::UserPortfolio;

#[function_component(ProjectCard)]
pub fn project_card(props: &UserPortfolio) -> Html {

    html! {
        <div class="project">
            <img src={props.image.clone()} alt="project-image" />
            <a rel="noreferrer" target="_blank" href={props.link.clone()}>{ format!("{}: ", props.title.clone().unwrap()) } { "View Project" }</a>
        </div>
    }
}
