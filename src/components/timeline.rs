use chrono::NaiveDateTime;
use yew::prelude::*;

use crate::{app::AppStateContext, data::models::resource::UserResume};

#[derive(Properties, PartialEq)]
pub struct TimelineProps {
    pub items: Vec<UserResume>,
}

#[function_component(TimelineItem)]
pub fn timeline_item(props: &UserResume) -> Html {
    let current_state = use_context::<AppStateContext>().unwrap();
    let start_date = NaiveDateTime::parse_from_str(&props.start_date.clone().unwrap_or("".to_string()), "%Y-%m-%dT%H:%M:%S%.3fZ").unwrap().format("%b %0e %Y - ").to_string();
    
    let end_date = match &props.end_date {
        None => format!(" - Present"),
        Some(d) => NaiveDateTime::parse_from_str(d, "%Y-%m-%dT%H:%M:%S%.3fZ").unwrap().format("%b %0e %Y").to_string(),
    };

    html! {
        <li class="timeline-item">
            <p class="timeline-icon"></p><p class="timeline-date">{start_date}{end_date}</p>
            <div class="timeline-content">
                <h4 class="heading">{&props.title.clone().unwrap()}{ match &props.more_info {
                    None => html! {},
                    _ => html! { <span class="more-info">{format!(" - {}", &props.more_info.clone().unwrap())}</span> }
                } }</h4>
                <div class="description">
                    <ol>
                        // {current_state.resume_achievements.get(&props.id.clone().unwrap()).unwrap().into_iter().map(|s| html! { <li>{s.description.clone().unwrap()}</li> }).collect::<Html>()}
                        {
                            match &props.id {
                                Some(id) => {
                                    match current_state.user_resources.achievements.clone() {
                                        Some(achievements) => {
                                            let hashed_achievements = achievements.get(&id.clone());
                                            match hashed_achievements {
                                                Some(ach) => {
                                                    ach.into_iter().map(|s| html! { <li>{s.clone()}</li> }).collect::<Html>()
                                                },
                                                None => html! {}
                                            }
                                        },
                                        None => html! {}
                                    }
                                },
                                None => html! {}
                            }
                        }
                    </ol>
                </div>
            </div>
        </li>
    }
}

#[function_component(Timeline)]
pub fn timeline(props: &TimelineProps) -> Html {
    html! {
        <ul class="timeline">
            { props.items.iter().map(|item| html! { <TimelineItem more_info={item.more_info.clone()} start_date={item.start_date.clone()} end_date={item.end_date.clone()} title={item.title.clone()} id={item.id.clone()} /> }).collect::<Html>()
            }
        </ul>
    }
}
