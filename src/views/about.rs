use yew::prelude::*;

use crate::components::{transition::Transition, back_home::BackHome, page_header::{PageHeader, PageHeaderProps}};

#[function_component(About)]
pub fn about() -> Html {
    let props = PageHeaderProps {
        hint: "Who am I?".to_owned(),
        heading: "About me".to_owned()
    };

    html! {
        <>
        <Transition />
        <main class="about">
        <BackHome />
        <PageHeader hint={props.hint} heading={props.heading} />
        </main>
        </>
    }
}