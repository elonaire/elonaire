use yew::prelude::*;

use crate::{
    app::AppStateContext,
    components::{
        back_home::BackHome,
        page_header::{PageHeader, PageHeaderProps},
        project_card::ProjectCard,
        tabs::Tabs,
        transition::Transition,
        no_content_component::NoContent,
    },
    data::context::user_resources::get_user_resources,
};

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
            if current_state.user_resources.portfolio.is_none() {
                let user_id = option_env!("MAIN_USER_ID").expect("MAIN_USER_ID env var not set");

                let _user_resources =
                    get_user_resources(user_id.to_string(), resoures_state_clone)
                        .await;
            }
        }); // Await the async block
        || ()
    });

    let state_clone_for_filters = state_clone.clone();
    let projects_local_state_clone = projects.clone();
    use_effect_with_deps(
        move |_| {
            // filter active portfolio tab
            let active_tab = state_clone_for_filters
                .portfolio_tabs
                .iter()
                .find(|tab| tab.active);
            match active_tab {
                Some(tab) => {
                    let projects = match state_clone_for_filters.user_resources.portfolio.clone() {
                        Some(projects) => projects,
                        None => vec![],
                    };
                    let filtered_projects = projects
                        .iter()
                        .filter(|project| project.category.unwrap() == tab.category)
                        .cloned()
                        .collect::<Vec<_>>();
                    projects_local_state_clone.set(filtered_projects);
                }
                None => {
                    projects_local_state_clone.set(
                        state_clone_for_filters
                            .user_resources
                            .portfolio
                            .clone()
                            .unwrap(),
                    );
                }
            }
        },
        (
            state_clone_for_deps.portfolio_tabs.clone(),
            state_clone_for_deps.user_resources.portfolio.clone(),
        ),
    );

    html! {
        <>
        <Transition />
        <main class="portfolio-wrapper">
            <div class="portfolio">
                <BackHome />
            <PageHeader hint={page_header_props.hint} heading={page_header_props.heading} />
            <div class="tabs-container">
                <Tabs tabs={state_clone.portfolio_tabs.clone().to_vec()} />
            </div>
            <div class="projects">
                {
                    match projects.len() {
                        0 => html! { <NoContent /> },
                        _ => projects.iter().map(|project| {
                            html! {
                                <ProjectCard id={project.id.clone()} title={project.title.clone().unwrap()} description={project.description.clone().unwrap()} image={project.image.clone().unwrap()} link={project.link.clone().unwrap()} />
                            }
                        }).collect::<Html>()
                    }    
                }
            </div>
            </div>
        </main>
        </>
    }
}
