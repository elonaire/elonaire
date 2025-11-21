use leptos::prelude::*;
use leptos_meta::*;

use crate::components::molecules::flip_card::FlipCard;
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
                                <FlipCard title="Software Engineering" image_url="https://miro.medium.com/v2/resize:fit:1400/1*CEGmzCboef_rJ6si2eiExQ.png" description="I can design and develop your software system" />
                                // <FlipCard />
                                // <FlipCard />
                            </div>
                        </Tab>
                        <Tab slot>
                        <div class="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-4 gap-[20px]">
                            <FlipCard title="Software Engineering" image_url="https://miro.medium.com/v2/resize:fit:1400/1*CEGmzCboef_rJ6si2eiExQ.png" description="I can design and develop your software system" />
                            // <FlipCard />
                            // <FlipCard />
                        </div>
                        </Tab>
                        <Tab slot>
                            <div class="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-4 gap-[20px]">
                                <FlipCard title="Software Engineering" image_url="https://miro.medium.com/v2/resize:fit:1400/1*CEGmzCboef_rJ6si2eiExQ.png" description="I can design and develop your software system" />
                                // <FlipCard />
                                // <FlipCard />
                            </div>
                        </Tab>
                        <Tab slot>
                            <div class="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-4 gap-[20px]">
                                <FlipCard title="Software Engineering" image_url="https://miro.medium.com/v2/resize:fit:1400/1*CEGmzCboef_rJ6si2eiExQ.png" description="I can design and develop your software system" />
                                // <FlipCard />
                                // <FlipCard />
                            </div>
                        </Tab>
                        <Tab slot>
                            <div class="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-4 gap-[20px]">
                                <FlipCard title="Software Engineering" image_url="https://miro.medium.com/v2/resize:fit:1400/1*CEGmzCboef_rJ6si2eiExQ.png" description="I can design and develop your software system" />
                                // <FlipCard />
                                // <FlipCard />
                            </div>
                        </Tab>
                        <Tab slot>
                            <div class="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-4 gap-[20px]">
                                <FlipCard title="Software Engineering" image_url="https://miro.medium.com/v2/resize:fit:1400/1*CEGmzCboef_rJ6si2eiExQ.png" description="I can design and develop your software system" />
                                // <FlipCard />
                                // <FlipCard />
                            </div>
                        </Tab>
                    </Tabs>
                </div>
            </div>
        </main>
    }
}
