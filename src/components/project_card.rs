use yew::prelude::*;
use yew_icons::{Icon, IconId};

use crate::data::models::resource::UserPortfolio;

#[function_component(ProjectCard)]
pub fn project_card(props: &UserPortfolio) -> Html {

    html! {
        <div class="project">
            <img src={props.image.clone().unwrap()} alt="project-image" />
            <a rel="noreferrer" target="_blank" href={props.link.clone().unwrap()}>{ format!("{} ", props.title.clone().unwrap()) } <Icon width={"1em".to_owned()} height={"1em".to_owned()} icon_id={IconId::OcticonsLinkExternal16}/></a>
        </div>
    }
}
