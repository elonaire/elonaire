use yew::prelude::*;

use crate::{components::{transition::Transition, page_header::{PageHeaderProps, PageHeader}, back_home::BackHome, tabs::Tabs, project_card::{ProjectProps, ProjectCard}}, app::AppStateContext};

#[function_component(Portfolio)]
pub fn portfolio() -> Html {
    let page_header_props = PageHeaderProps {
        hint: "Showcase of my best works".to_owned(),
        heading: "My Portfolio".to_owned(),
    };

    let current_state = use_context::<AppStateContext>().unwrap();
    let projects = use_state(|| vec![
        ProjectProps {
            title: "Project 1".to_owned(),
            description: "Lorem ipsum dolor sit amet, consectetur adipiscing elit. Sed euismod, nisl vitae ultricies tincidunt, nisl nisl aliquet nisl, nec lacinia nisl nisl sit amet nisl. Sed euismod, nisl vitae ultricies tincidunt, nisl nisl aliquet nisl, nec lacinia nisl nisl sit amet nisl.".to_owned(),
            image: "https://wallpaper.dog/large/10950119.jpg".to_owned(),
            link: "https://www.google.com".to_owned(),
        },
        ProjectProps {
            title: "Project 2".to_owned(),
            description: "Lorem ipsum dolor sit amet, consectetur adipiscing elit. Sed euismod, nisl vitae ultricies tincidunt, nisl nisl aliquet nisl, nec lacinia nisl nisl sit amet nisl. Sed euismod, nisl vitae ultricies tincidunt, nisl nisl aliquet nisl, nec lacinia nisl nisl sit amet nisl.".to_owned(),
            image: "https://wallpaper.dog/large/10950119.jpg".to_owned(),
            link: "https://www.google.com".to_owned(),
        },
        ProjectProps {
            title: "Project 3".to_owned(),
            description: "Lorem ipsum dolor sit amet, consectetur adipiscing elit. Sed euismod, nisl vitae ultricies tincidunt, nisl nisl aliquet nisl, nec lacinia nisl nisl sit amet nisl. Sed euismod, nisl vitae ultricies tincidunt, nisl nisl aliquet nisl, nec lacinia nisl nisl sit amet nisl.".to_owned(),
            image: "https://wallpaper.dog/large/10950119.jpg".to_owned(),
            link: "https://www.google.com".to_owned(),
        },
        ProjectProps {
            title: "Project 4".to_owned(),
            description: "Lorem ipsum dolor sit amet, consectetur adipiscing elit. Sed euismod, nisl vitae ultricies tincidunt, nisl nisl aliquet nisl, nec lacinia nisl nisl sit amet nisl. Sed euismod, nisl vitae ultricies tincidunt, nisl nisl aliquet nisl, nec lacinia nisl nisl sit amet nisl.".to_owned(),
            image: "https://wallpaper.dog/large/10950119.jpg".to_owned(),
            link: "https://www.google.com".to_owned(),
        },
        ProjectProps {
            title: "Project 5".to_owned(),
            description: "Lorem ipsum dolor sit amet, consectetur adipiscing elit. Sed euismod, nisl vitae ultricies tincidunt, nisl nisl aliquet nisl, nec lacinia nisl nisl sit amet nisl. Sed euismod, nisl vitae ultricies tincidunt, nisl nisl aliquet nisl, nec lacinia nisl nisl sit amet nisl.".to_owned(),
            image: "https://wallpaper.dog/large/10950119.jpg".to_owned(),
            link: "https://www.google.com".to_owned(),
        },
    ]);
    
    html! {
        <>
        <Transition />
        <main class="portfolio">
            <BackHome />
            <PageHeader hint={page_header_props.hint} heading={page_header_props.heading} />
            <div class="tabs-container">
                <Tabs tabs={current_state.portfolio_tabs.clone().to_vec()} />
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