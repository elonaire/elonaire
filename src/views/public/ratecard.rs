use leptos::prelude::*;
use leptos_meta::*;

use crate::components::molecules::{
    headline::Headline, ratecard::RatecardComponent, top_nav::TopNav,
};

#[island]
pub fn Ratecard() -> impl IntoView {
    view! {
        <Title text="My Ratecard"/>
        <main>
            <div class="min-h-screen bg-navy flex flex-col gap-[40px]">
                <div class="sticky top-0 z-10 bg-navy">
                    <TopNav />
                </div>
                <Headline title="My Ratecard" description="How much do I charge?" />
                <div class="mx-[5%] md:mx-[10%] flex flex-col md:flex-row gap-[10px]">
                    <RatecardComponent />
                </div>
            </div>
        </main>
    }
}
