use chrono::NaiveDate;
use yew::prelude::*;

#[derive(Properties, PartialEq, Clone)]
pub struct TimelineItemProps {
    pub start_date: String,
    pub end_date: Option<String>,
    pub title: String,
    pub more_info: Option<String>,
    pub description: String,
}

#[derive(Properties, PartialEq)]
pub struct TimelineProps {
    pub items: Vec<TimelineItemProps>,
}

#[function_component(TimelineItem)]
pub fn timeline_item(props: &TimelineItemProps) -> Html {
    let start_date = NaiveDate::parse_from_str(&props.start_date, "%Y-%m-%d").unwrap();
    // let end_date = NaiveDate::parse_from_str(&props.end_date.unwrap(), "%Y-%m-%d").unwrap();
    let end_date = match &props.end_date {
        None => format!(" - Present"),
        Some(d) => NaiveDate::parse_from_str(d, "%Y-%m-%d").unwrap().format(" - %b %e %Y").to_string(),
    };
    let mut split_description = props.description.split(".").collect::<Vec<&str>>();
    let _ = &split_description.pop(); // remove last empty string

    html! {
        <li class="timeline-item">
            <p class="timeline-icon"></p><p class="timeline-date">{start_date.format("%b %e %Y")}{end_date}</p>
            <div class="timeline-content">
                <h4 class="heading">{&props.title.clone()}{ match &props.more_info {
                    None => html! {},
                    _ => html! { <span class="more-info">{format!(" - {}", &props.more_info.clone().unwrap())}</span> }
                } }</h4>
                <div class="description">
                    <ol>
                        {split_description.iter().map(|s| html! { <li>{format!("{}.", s)}</li> }).collect::<Html>()}
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
            { props.items.iter().map(|item| html! { <TimelineItem more_info={item.more_info.clone()} start_date={item.start_date.clone()} end_date={item.end_date.clone()} title={item.title.clone()} description={item.description.clone()} /> }).collect::<Html>()
            }
        </ul>
    }
}
