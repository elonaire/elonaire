use yew::prelude::*;

#[derive(Clone, PartialEq, Properties)]
pub struct ProjectProps {
    pub title: String,
    pub description: String,
    pub image: String,
    pub link: String,
}

#[function_component(ProjectCard)]
pub fn project_card(props: &ProjectProps) -> Html {

    html! {
        <div class="project">
            <img src={props.image.clone()} alt="project-image" />
            <a rel="noreferrer" target="_blank" href={props.link.clone()}>{ format!("{}: ", props.title.clone()) } { "View Project" }</a>
        </div>
    }
}
