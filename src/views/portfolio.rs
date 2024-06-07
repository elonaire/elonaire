use yew::prelude::*;

use crate::{app::AppStateContext, components::{back_home::BackHome, page_header::{PageHeader, PageHeaderProps}, project_card::ProjectCard, tabs::Tabs, transition::Transition}, data::context::user_resources::get_user_resources};

#[function_component(Portfolio)]
pub fn portfolio() -> Html {
    let page_header_props = PageHeaderProps {
        hint: "Showcase of my best works".to_owned(),
        heading: "My Portfolio".to_owned(),
    };

    let current_state = use_context::<AppStateContext>().unwrap();
    let resoures_state_clone = current_state.clone();
    let state_clone = current_state.clone();
    let state_clone_for_deps = current_state.clone();
    let projects = use_state_eq(|| match current_state.user_resources.portfolio.clone() {
        Some(projects) => projects,
        None => vec![],
    });

    use_effect({
        wasm_bindgen_futures::spawn_local(async move {
            log::info!("Resume component: {:?}", current_state.user_resources.resume.clone());
            let _user_resources = get_user_resources("pni9fr7u9gf2bzkf6dmf".to_string(), resoures_state_clone).await;
        }); // Await the async block
        || () 
    });

    let projects_clone = projects.clone();
    let state_clone_for_match = state_clone.clone();
    use_effect_with_deps(
        move |_| {
            match state_clone_for_match.user_resources.portfolio.clone() {
                Some(projects) => {
                    projects_clone.set(projects);
                }
                None => projects_clone.set(vec![]),
            }
        },
        state_clone_for_deps.user_resources.portfolio.clone(),
    );
    
    html! {
        <>
        <Transition />
        <main class="portfolio">
            <BackHome />
            <PageHeader hint={page_header_props.hint} heading={page_header_props.heading} />
            <div class="tabs-container">
                <Tabs tabs={state_clone.portfolio_tabs.clone().to_vec()} />
            </div>
            <div class="projects">
                {
                    projects.iter().map(|project| {
                        html! {
                            <ProjectCard title={project.title.clone()} description={project.description.clone()} image={project.image.clone()} link={project.link.clone()} />
                        }
                    }).collect::<Html>()
                }
            </div>
        </main>
        </>
    }
}