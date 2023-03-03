use chrono::{NaiveDate, Utc};
use yew::prelude::*;

use crate::{
    app::AppStateContext,
    components::{
        back_home::BackHome,
        page_header::{PageHeader, PageHeaderProps},
        service_card::{ServiceCard, ServiceCardProps},
        styled_heading::{StyledHeading, StyledHeadingProps},
        transition::Transition,
    },
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

    let service_cards = use_state(|| vec![
        ServiceCardProps {
            title: "Web Development".to_owned(),
            description: "I can develop robust, scalable and responsive Web Applications using technologies such as JavaScript, TypeScript, Node.js, NestJS, React, Angular, Yew(Rust) and Rocket(Rust). I am also an expert in configuring Nginx Web server.".to_owned(),
            cover_image: "https://thumbs.dreamstime.com/b/web-development-coding-programming-internet-technology-business-concept-web-development-coding-programming-internet-technology-121903546.jpg".to_owned(),
        },
        ServiceCardProps {
            title: "Data Visualization".to_owned(),
            description: "With the use of visualization tools such as D3.js and Plotters(Rust), I can help you gain insights into your data including metrics and logs.".to_owned(),
            cover_image: "https://www.datameer.com/wp-content/uploads/2019/12/Data-Vizualisation-924x512.png".to_owned(),
        },
        ServiceCardProps {
            title: "Embedded Systems & IoT".to_owned(),
            description: "I can program embedded systems using Rust as the main language. I am also an IoT enthusiast.".to_owned(),
            cover_image: "https://www.ul.com/sites/g/files/qbfpbp251/files/styles/hero_boxed_width/public/2019-11/CT_EPT_FunctionalSafety-Webinar_740x530.jpg?itok=yFA2BSuN".to_owned(),
        },
        ServiceCardProps {
            title: "DevOps".to_owned(),
            description: "I can help you setup your CI/CD pipeline using Github Actions, Gitlab CI/CD, Docker, Kubernetes and Terraform.".to_owned(),
            cover_image: "https://imageio.forbes.com/specials-images/imageserve/60f1e792c7e89f933811814c/0x0.jpg?format=jpg&width=1200".to_owned(),
        },
        ServiceCardProps {
            title: "Cloud Computing".to_owned(),
            description: "I can help you setup your cloud infrastructure using AWS and GCP.".to_owned(),
            cover_image: "https://imageio.forbes.com/specials-images/imageserve/5f9fa9e815da35da1356a28b/The-5-Biggest-Cloud-Computing-Trends-In-2021/960x0.jpg?format=jpg&width=960".to_owned(),
        },
        ServiceCardProps {
            title: "Software Architecture".to_owned(),
            description: "I can help you design your software architecture using the best practices. Whether you need to cut down on costs, improve fault tolerance, improve turnaround time, improve security or improve scalability. Most of the time, all these depend on the appropriate architecture.".to_owned(),
            cover_image: "https://t3.ftcdn.net/jpg/01/09/47/78/360_F_109477885_MOzjguXVI1Q5exvurSlJjog9l0NZUPFh.jpg".to_owned(),
        },
    ]);

    html! {
        <>
            <Transition />
            <main class="about">
                <BackHome />
                <PageHeader hint={page_header_props.hint} heading={page_header_props.heading} />

                <div class="details">
                    <div class="images-container">
                        <img class={classes!("main-img")} src="img/1.jpg" alt="logo" />
                        <img class={classes!("sub-img")} src="img/2.jpg" alt="logo" />
                    </div>

                    <div class="autobio">
                        <h2>{ "Hello, I'm " } <span class="name">{format!("{} {} {}", current_state.first_name.clone(), current_state.middle_name.clone(), current_state.last_name.clone())}</span></h2>
                        <p class="description">{&current_state.auto_bio}</p>
                        <p><strong>{"Age: "}</strong>{today.years_since(from_ymd)} {" years"}</p>
                        <p><strong>{"Residence: "}</strong>{current_state.residence.clone()}</p>
                        <p><strong>{"Address: "}</strong>{current_state.address.clone()}</p>
                        <p><strong>{"Email: "}</strong>{current_state.email.clone()}</p>
                        <p><strong>{"Phone: "}</strong>{current_state.phone.clone()}</p>
                    </div>
                </div>

                <div class="heading-container">
                    <StyledHeading heading={styled_heading_props.heading} />
                </div>
                <div class="service-cards">
                    {
                        service_cards.iter().map(|service_card| {
                            html! {
                                <ServiceCard title={service_card.title.clone()} description={service_card.description.clone()} cover_image={service_card.cover_image.clone()} />
                            }
                        }).collect::<Html>()
                    }
                </div>

            </main>
        </>
    }
}
