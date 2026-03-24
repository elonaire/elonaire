use leptos::prelude::*;
use leptos::task::spawn_local;
use leptos_meta::*;
use reactive_stores::Store;

use crate::components::molecules::flip_card::FlipCard;
use crate::data::context::shared::fetch_portfolio;
use crate::data::context::store::{AppStateContext, AppStateContextStoreFields};
use crate::data::models::graphql::shared::UserPortfolio;
use crate::utils::custom_traits::EnumerableEnum;
use crate::{
    components::{
        general::tabs::{Tab, TabLabel, Tabs},
        molecules::{headline::Headline, top_nav::TopNav},
    },
    data::models::graphql::shared::UserPortfolioCategory,
};

#[component]
pub fn Portfolio() -> impl IntoView {
    let current_state = expect_context::<Store<AppStateContext>>();
    let portfolio = move || current_state.portfolio();
    let (is_loading, set_is_loading) = signal(false);
    let javascript_projects = RwSignal::new(vec![] as Vec<UserPortfolio>);
    let rust_projects = RwSignal::new(vec![] as Vec<UserPortfolio>);
    let database_projects = RwSignal::new(vec![] as Vec<UserPortfolio>);
    let devops_projects = RwSignal::new(vec![] as Vec<UserPortfolio>);
    let cloud_projects = RwSignal::new(vec![] as Vec<UserPortfolio>);
    let mobile_projects = RwSignal::new(vec![] as Vec<UserPortfolio>);
    let portfolio_tabs = RwSignal::new(
        UserPortfolioCategory::variants_slice()
            .iter()
            .map(|category| {
                let owned_category = category.to_string(); // must implement Clone

                TabLabel::new(ViewFn::from(move || {
                    let owned_category = owned_category.clone();
                    view! { <p>{owned_category}</p> }
                }))
            })
            .collect::<Vec<TabLabel>>(),
    );

    Effect::new(move || {
        javascript_projects.set(
            portfolio()
                .get()
                .iter()
                .filter(|project| {
                    project.category.as_ref() == Some(&UserPortfolioCategory::JavaScript)
                })
                .map(|project| project.to_owned())
                .collect(),
        );

        rust_projects.set(
            portfolio()
                .get()
                .iter()
                .filter(|project| project.category.as_ref() == Some(&UserPortfolioCategory::Rust))
                .map(|project| project.to_owned())
                .collect(),
        );

        database_projects.set(
            portfolio()
                .get()
                .iter()
                .filter(|project| {
                    project.category.as_ref() == Some(&UserPortfolioCategory::Database)
                })
                .map(|project| project.to_owned())
                .collect(),
        );

        devops_projects.set(
            portfolio()
                .get()
                .iter()
                .filter(|project| project.category.as_ref() == Some(&UserPortfolioCategory::DevOps))
                .map(|project| project.to_owned())
                .collect(),
        );

        cloud_projects.set(
            portfolio()
                .get()
                .iter()
                .filter(|project| project.category.as_ref() == Some(&UserPortfolioCategory::Cloud))
                .map(|project| project.to_owned())
                .collect(),
        );

        mobile_projects.set(
            portfolio()
                .get()
                .iter()
                .filter(|project| project.category.as_ref() == Some(&UserPortfolioCategory::Mobile))
                .map(|project| project.to_owned())
                .collect(),
        );
    });

    Effect::new(move || {
        set_is_loading.set(true);
        spawn_local(async move {
            let _portfolio_res = fetch_portfolio(&current_state, None).await;

            set_is_loading.set(false);
        });
    });

    view! {
        <Title text="Portfolio"/>
        <main>
            <div class="min-h-svh flex flex-col gap-[40px]">
                <div class="sticky top-0 z-10 bg-contrast-white dark:bg-navy">
                    <TopNav />
                </div>
                <Headline title="Portfolio" description="Showcase of my best work" />
                <div class="display-constraints flex flex-col md:flex-row gap-[40px]">
                    <Tabs tab_labels=portfolio_tabs>
                        <Tab slot>
                            <div class="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 gap-[20px]">
                                {
                                    move || javascript_projects.get().iter().map(|project| {
                                        view!{
                                            <FlipCard title={project.title.as_ref().unwrap_or(&Default::default()).clone()} image_url={project.thumbnail.as_ref().unwrap_or(&Default::default()).clone()} description={project.description.as_ref().unwrap_or(&Default::default()).clone()} />
                                        }
                                    }).collect::<Vec<_>>()
                                }
                            </div>
                        </Tab>
                        <Tab slot>
                            <div class="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 gap-[20px]">
                                {
                                    move || rust_projects.get().iter().map(|project| {
                                        view!{
                                            <FlipCard title={project.title.as_ref().unwrap_or(&Default::default()).clone()} image_url={project.thumbnail.as_ref().unwrap_or(&Default::default()).clone()} description={project.description.as_ref().unwrap_or(&Default::default()).clone()} />
                                        }
                                    }).collect::<Vec<_>>()
                                }
                            </div>
                        </Tab>
                        <Tab slot>
                            <div class="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 gap-[20px]">
                                {
                                    move || database_projects.get().iter().map(|project| {
                                        view!{
                                            <FlipCard title={project.title.as_ref().unwrap_or(&Default::default()).clone()} image_url={project.thumbnail.as_ref().unwrap_or(&Default::default()).clone()} description={project.description.as_ref().unwrap_or(&Default::default()).clone()} />
                                        }
                                    }).collect::<Vec<_>>()
                                }
                            </div>
                        </Tab>
                        <Tab slot>
                            <div class="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 gap-[20px]">
                                {
                                    move || devops_projects.get().iter().map(|project| {
                                        view!{
                                            <FlipCard title={project.title.as_ref().unwrap_or(&Default::default()).clone()} image_url={project.thumbnail.as_ref().unwrap_or(&Default::default()).clone()} description={project.description.as_ref().unwrap_or(&Default::default()).clone()} />
                                        }
                                    }).collect::<Vec<_>>()
                                }
                            </div>
                        </Tab>
                        <Tab slot>
                            <div class="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 gap-[20px]">
                                {
                                    move || cloud_projects.get().iter().map(|project| {
                                        view!{
                                            <FlipCard title={project.title.as_ref().unwrap_or(&Default::default()).clone()} image_url={project.thumbnail.as_ref().unwrap_or(&Default::default()).clone()} description={project.description.as_ref().unwrap_or(&Default::default()).clone()} />
                                        }
                                    }).collect::<Vec<_>>()
                                }
                            </div>
                        </Tab>
                        <Tab slot>
                            <div class="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 gap-[20px]">
                                {
                                    move || mobile_projects.get().iter().map(|project| {
                                        view!{
                                            <FlipCard title={project.title.as_ref().unwrap_or(&Default::default()).clone()} image_url={project.thumbnail.as_ref().unwrap_or(&Default::default()).clone()} description={project.description.as_ref().unwrap_or(&Default::default()).clone()} />
                                        }
                                    }).collect::<Vec<_>>()
                                }
                            </div>
                        </Tab>
                    </Tabs>
                </div>
            </div>
        </main>
    }
}
