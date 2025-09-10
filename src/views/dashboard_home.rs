use icondata as IconData;
use leptos::prelude::*;
use leptos_meta::*;

use crate::components::{
    general::{
        breadcrumbs::Breadcrumbs,
        timeline::{Timeline, TimelineItem, TimelineStatus},
    },
    molecules::{quick_action::QuickAction, stats_card::StatsCard},
};

#[island]
pub fn DashboardHome() -> impl IntoView {
    let timeline_items = RwSignal::new(vec![TimelineItem::new(
        "New Project Created",
        Some("Techie Tenka Project was created."),
        Some(IconData::AiPlusOutlined),
        None,
        TimelineStatus::Success,
        ViewFn::from(|| view! { <p>"Project created with Leptos!"</p> }),
    )]);

    view! {
        <>
            <Title text="Dashboard Home"/>
            <div class="mx-[20px]">
                <Breadcrumbs custom_route_names=["Home", "Dashboard"] />
            </div>

            <h1 class="mx-[20px]">Dashboard Overview</h1>

            <div class="mx-[20px] flex flex-col gap-[20px]">
                <StatsCard color="primary" title="Total Projects" figures=RwSignal::new(25) icon=IconData::BiBriefcaseRegular percentage=RwSignal::new(-2) />
            </div>

            <div class="mx-[20px] flex flex-col gap-[10px]">
                <h3>Recent Activity</h3>

                <Timeline steps=timeline_items />
            </div>

            <div class="mx-[20px] flex flex-col gap-[10px]">
                <h3>Quick Actions</h3>

                <div class="flex flex-col gap-[10px]">
                    <QuickAction title="Create Project" description="Create a new portfolio project" color="primary" icon=IconData::BsPlusLg />
                </div>
            </div>
        </>
    }
}
