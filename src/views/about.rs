use yew::prelude::*;
use chrono::{Duration, NaiveDate};

use crate::{components::{transition::Transition, back_home::BackHome, page_header::{PageHeader, PageHeaderProps}}, app::AppState};

#[function_component(About)]
pub fn about() -> Html {
    let props = PageHeaderProps {
        hint: "Who am I?".to_owned(),
        heading: "About me".to_owned()
    };
    let from_ymd = NaiveDate::from_ymd;

    let current_state = use_context::<AppState>().expect("no state found");

    html! {
        <>
        <Transition />
        <main class="about">
        <BackHome />
        <PageHeader hint={props.hint} heading={props.heading} />
        <div class="details">
        <div class="images-container">
        <img class={classes!("main-img")} src="img/1.jpg" alt="logo" />
        <img class={classes!("sub-img")} src="img/2.jpg" alt="logo" />
        </div>
        <div class="autobio">
        <h2>{ "Hello, I'm " } <span class="name">{current_state.full_name}</span></h2>
        <p class="description">{current_state.auto_bio}</p>
        <p><strong>{"Age: "}</strong>{from_ymd(1995, 05, 19).years_since(from_ymd)}</p>
        <p><strong>{"Residence: "}</strong></p>
        <p><strong>{"Address: "}</strong></p>
        <p><strong>{"Email: "}</strong></p>
        <p><strong>{"Phone: "}</strong></p>
        </div>
        </div>
        </main>
        </>
    }
}