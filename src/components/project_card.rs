use yew::prelude::*;
use yew_icons::{Icon, IconId};

use crate::{data::models::resource::UserPortfolio, app::AppStateContext, components::tooltip::Tooltip};

#[function_component(ProjectCard)]
pub fn project_card(props: &UserPortfolio) -> Html {
    let current_state = use_context::<AppStateContext>().unwrap();

    html! {
        <div class="project">
            <img src={props.image.clone().unwrap()} alt="project-image" />
            <a rel="noreferrer" target="_blank" href={props.link.clone().unwrap()}>{ format!("{} ", props.title.clone().unwrap()) } <Icon width={"1em".to_owned()} height={"1em".to_owned()} icon_id={IconId::OcticonsLinkExternal16}/></a>
            <div class="overlay">
            {
                match current_state.user_resources.portfolio_skills.as_ref() {
                    Some(skills) => {
                        let project_skills = skills.get(&props.id.clone().unwrap());
                        
                        match project_skills {
                            Some(project_skills) => {
                                html!{
                                    {
                                        project_skills.iter().map(|skill| {
                                            html!{
                                                <Tooltip text={skill.name.clone().unwrap()}><img src={skill.image.clone()} alt="skill" /></Tooltip>
                                            }
                                        }).collect::<Html>()
                                    
                                    }
                                }
                            },
                            None => html!{}
                            
                        }
                    },
                    None => html!{}
                }
            }
            </div>
        </div>
    }
}
