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

#[island]
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
            .filter(|category| !category.is_empty())
            .map(|category| {
                let owned_category = category.to_owned(); // must implement Clone

                TabLabel::new(ViewFn::from(move || {
                    let owned_category = owned_category.clone();
                    view! { <p>{owned_category}</p> }
                }))
            })
            .collect::<Vec<TabLabel>>(),
    );

    Effect::new(move || {
        portfolio().get().iter().for_each(|portfolio| {
            match portfolio.category.as_ref().unwrap().to_owned() {
                UserPortfolioCategory::JavaScript => {
                    javascript_projects.write().push(portfolio.clone());
                }
                UserPortfolioCategory::Rust => {
                    rust_projects.write().push(portfolio.clone());
                }
                UserPortfolioCategory::Database => {
                    database_projects.write().push(portfolio.clone());
                }
                UserPortfolioCategory::DevOps => {
                    devops_projects.write().push(portfolio.clone());
                }
                UserPortfolioCategory::Cloud => {
                    cloud_projects.write().push(portfolio.clone());
                }
                UserPortfolioCategory::Mobile => {
                    mobile_projects.write().push(portfolio.clone());
                }
                _ => {}
            };
        });
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
            <div class="min-h-screen bg-navy flex flex-col gap-[40px] text-light-gray">
                <div class="sticky top-0 z-10 bg-navy">
                    <TopNav />
                </div>
                <Headline title="Portfolio" description="Showcase of my best work" />
                <div class="mx-[5%] md:mx-[10%] flex flex-col md:flex-row gap-[40px]">
                    <Tabs tab_labels=portfolio_tabs>
                        <Tab slot>
                            <div class="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-4 gap-[20px]">
                                {
                                    move || javascript_projects.get().iter().map(|project| {
                                        view!{
                                            <FlipCard title={project.title.as_ref().unwrap().clone()} image_url={project.thumbnail.as_ref().unwrap().clone()} description={project.description.as_ref().unwrap().clone()} />
                                        }
                                    }).collect::<Vec<_>>()
                                }
                            </div>
                        </Tab>
                        <Tab slot>
                            <div class="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-4 gap-[20px]">
                                {
                                    move || rust_projects.get().iter().map(|project| {
                                        view!{
                                            <FlipCard title={project.title.as_ref().unwrap().clone()} image_url={project.thumbnail.as_ref().unwrap().clone()} description={project.description.as_ref().unwrap().clone()} />
                                        }
                                    }).collect::<Vec<_>>()
                                }
                            </div>
                        </Tab>
                        <Tab slot>
                            <div class="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-4 gap-[20px]">
                                {
                                    move || database_projects.get().iter().map(|project| {
                                        view!{
                                            <FlipCard title={project.title.as_ref().unwrap().clone()} image_url={project.thumbnail.as_ref().unwrap().clone()} description={project.description.as_ref().unwrap().clone()} />
                                        }
                                    }).collect::<Vec<_>>()
                                }
                            </div>
                        </Tab>
                        <Tab slot>
                            <div class="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-4 gap-[20px]">
                                {
                                    move || devops_projects.get().iter().map(|project| {
                                        view!{
                                            <FlipCard title={project.title.as_ref().unwrap().clone()} image_url={project.thumbnail.as_ref().unwrap().clone()} description={project.description.as_ref().unwrap().clone()} />
                                        }
                                    }).collect::<Vec<_>>()
                                }
                            </div>
                        </Tab>
                        <Tab slot>
                            <div class="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-4 gap-[20px]">
                                {
                                    move || cloud_projects.get().iter().map(|project| {
                                        view!{
                                            <FlipCard title={project.title.as_ref().unwrap().clone()} image_url={project.thumbnail.as_ref().unwrap().clone()} description={project.description.as_ref().unwrap().clone()} />
                                        }
                                    }).collect::<Vec<_>>()
                                }
                            </div>
                        </Tab>
                        <Tab slot>
                            <div class="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-4 gap-[20px]">
                                {
                                    move || mobile_projects.get().iter().map(|project| {
                                        view!{
                                            <FlipCard title={project.title.as_ref().unwrap().clone()} image_url={project.thumbnail.as_ref().unwrap().clone()} description={project.description.as_ref().unwrap().clone()} />
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
