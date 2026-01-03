use leptos::{prelude::*, task::spawn_local};
use leptos_meta::*;

use crate::{
    components::molecules::{headline::Headline, ratecard::RatecardComponent, top_nav::TopNav},
    data::{context::shared::fetch_ratecards, models::graphql::shared::Ratecard},
};

#[island]
pub fn Ratecard() -> impl IntoView {
    let (ratecards, set_ratecards) = signal(vec![] as Vec<Ratecard>);

    Effect::new(move || {
        spawn_local(async move {
            let ratecards_res = fetch_ratecards(None).await;

            if let Ok(ratecards) = ratecards_res {
                // Process ratecards data here
                set_ratecards.set(ratecards);
            }
        });
    });

    view! {
        <Title text="My Ratecard"/>
        <main>
            <div class="min-h-screen bg-navy flex flex-col gap-[40px]">
                <div class="sticky top-0 z-10 bg-navy">
                    <TopNav />
                </div>
                <Headline title="My Ratecard" description="How much do I charge?" />
                <div class="mx-[5%] md:mx-[10%] flex flex-col md:flex-row gap-[10px]">
                    <For
                        each=move || ratecards.get()
                        key=|ratecard| ratecard.id.clone()
                        children=move |ratecard| {
                            view! {
                                <RatecardComponent
                                    name=RwSignal::new(ratecard.name.clone())
                                    services=RwSignal::new(ratecard.services.to_vec())
                                />
                            }
                        }
                    />
                </div>
            </div>
        </main>
    }
}
