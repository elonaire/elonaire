use chrono::{NaiveDate, Utc};
use yew::prelude::*;

use crate::{
    app::AppStateContext,
    components::{
        back_home::BackHome,
        page_header::{PageHeader, PageHeaderProps},
        service_card::ServiceCard,
        styled_heading::{StyledHeading, StyledHeadingProps},
        transition::Transition,
    }, data::context::{user::get_user_by_id, user_resources::get_user_resources},
};

#[function_component(About)]
pub fn about() -> Html {
    let page_header_props = PageHeaderProps {
        hint: "Who am I?".to_owned(),
        heading: "About me".to_owned(),
    };
    let from_ymd = NaiveDate::from_ymd_opt(1995, 05, 19).unwrap();
    let today = Utc::now().date_naive();
    let styled_heading_props = StyledHeadingProps {
        heading: "My Services".to_owned(),
    };

    // log::info!("today {}", today);
    let current_state = use_context::<AppStateContext>().unwrap();
    let state_clone = current_state.clone();
    let resoures_state_clone = current_state.clone();
    let services = current_state.user_resources.services.clone();

    use_effect({
        wasm_bindgen_futures::spawn_local(async move {
            let _ = get_user_by_id("pni9fr7u9gf2bzkf6dmf".to_string(), state_clone).await;
            let _user_resources = get_user_resources("pni9fr7u9gf2bzkf6dmf".to_string(), resoures_state_clone).await;
        }); // Await the async block
        || {} 
    });

    html! {
        <>
            <Transition />
            <main class="about">
                <BackHome />
                <PageHeader hint={page_header_props.hint} heading={page_header_props.heading} />

                <div class="details">
                    <div class="images-container">
                        <img class={classes!("main-img")} src="https://imagedelivery.net/fa3SWf5GIAHiTnHQyqU8IQ/c9b133e5-fe4c-4899-4aad-f3a5cefe1400/public" alt="logo" />
                        <img class={classes!("sub-img")} src="https://imagedelivery.net/fa3SWf5GIAHiTnHQyqU8IQ/83428774-45e2-4184-9577-dc8ed8b79000/public" alt="logo" />
                    </div>

                    <div class="autobio">
                        <h2>{ "Hello, I'm " } <span class="name">{ current_state.user_details.full_name.clone() }</span></h2>
                        <p class="description">{current_state.active_professional_info.description.clone()}</p>
                        <p><strong>{"Age: "}</strong>{today.years_since(from_ymd)} {" years"}</p>
                        <p><strong>{"Residence: "}</strong>{current_state.user_details.country.clone()}</p>
                        <p><strong>{"Address: "}</strong>{current_state.user_details.address.clone()}</p>
                        <p><strong>{"Email: "}</strong>{current_state.user_details.email.clone()}</p>
                        <p><strong>{"Phone: (+254)"}</strong>{current_state.user_details.phone.clone()}</p>
                    </div>
                </div>

                <div class="heading-container">
                    <StyledHeading heading={styled_heading_props.heading} />
                </div>
                <div class="service-cards">
                    {
                        match services {
                            Some(services) => {
                                services.into_iter().map(|service_card| {
                                    html! {
                                        <ServiceCard title={service_card.title.clone()} description={service_card.description.clone()} image={service_card.image.clone()} />
                                    }
                                }).collect::<Html>()
                            },
                            None => html! { "No services available" }
                        }
                    }
                </div>

            </main>
        </>
    }
}
