use icondata as IconData;
use leptos::prelude::*;
use leptos_meta::*;

use crate::components::{
    general::{
        collapse::{Collapse, PanelInfo},
        timeline::{Timeline, TimelineItem, TimelineStatus},
    },
    molecules::{headline::Headline, section_title::SectionTitle, top_nav::TopNav},
};

#[island]
pub fn Resume() -> impl IntoView {
    let timeline_items = RwSignal::new(vec![TimelineItem {
        time_info: "Sep 1 2016 - Jun 1 2024 (10+ years)".into(),
        title: "New Project Created".into(),
        more_info: Some("Techie Tenka Project was created.".into()),
        status: TimelineStatus::Success,
        content: ViewFn::from(|| view! { <p>"Project created with Leptos!"</p> }),
        ..Default::default()
    }]);

    let technical_skills = RwSignal::new(vec![
        PanelInfo {
            title: ViewFn::from(move || view! { <p>"title 1"</p> }),
            children: ViewFn::from(move || view! { <p>"Panel content"</p> }),
            ..Default::default()
        },
        PanelInfo {
            title: ViewFn::from(move || view! { <p>"title 2"</p> }),
            children: ViewFn::from(move || view! { <p>"Panel content"</p> }),
            ..Default::default()
        },
    ]);

    view! {
        <Title text="Resume"/>
        <main>
            <div class="min-h-screen bg-navy flex flex-col gap-[40px] text-light-gray">
                <div class="sticky top-0 z-10 bg-navy">
                    <TopNav />
                </div>
                <Headline title="Resume" description="I am available for work" />
                <div class="mx-[5%] md:mx-[10%] flex flex-col md:flex-row gap-[40px]">
                    <div class="w-full md:basis-1/2 flex flex-col gap-[10px]">
                        <SectionTitle title="Education" />
                        <Timeline steps=timeline_items />
                    </div>
                    <div class="w-full md:basis-1/2 flex flex-col gap-[10px]">
                        <SectionTitle title="Work Experience" />
                        <Timeline steps=timeline_items />
                    </div>
                </div>
                <div class="mx-[5%] md:mx-[10%] flex flex-col md:flex-row gap-[40px]">
                    <div class="w-full md:basis-1/2 flex flex-col gap-[10px]">
                        <SectionTitle title="Technical Skills" />
                        <Collapse is_accordion=true panel_items=technical_skills />
                    </div>
                    <div class="w-full md:basis-1/2 flex flex-col gap-[10px]">
                        <SectionTitle title="Soft Skills" />
                        <Collapse is_accordion=true panel_items=technical_skills />
                    </div>
                </div>
            </div>
        </main>
    }
}
