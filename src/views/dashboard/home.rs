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
    let timeline_items = RwSignal::new(vec![TimelineItem {
        time_info: "3 mins ago".into(),
        title: "New Project Created".into(),
        more_info: Some("Techie Tenka Project was created.".into()),
        status: TimelineStatus::Info,
        content: ViewFn::from(|| view! { <p>"Project created with Leptos!"</p> }),
        ..Default::default()
    }]);

    view! {
        <>
            <Title text="Dashboard Home"/>
            <div class="mx-[5%] md:mx-[10%]">
                <Breadcrumbs custom_route_names=["Home", "Dashboard"] />
            </div>

            <h1 class="mx-[5%] md:mx-[10%]">Dashboard Overview</h1>

            <div class="mx-[5%] md:mx-[10%] grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-4 gap-[20px]">
                <StatsCard color="info" title="Total Projects" figures=RwSignal::new(25) icon=IconData::BiBriefcaseRegular percentage=RwSignal::new(-2) />
                <StatsCard color="success" title="Published Posts" figures=RwSignal::new(12) icon=IconData::RiArticleDocumentLine percentage=RwSignal::new(3) />
                <StatsCard color="warning" title="Monthly Visitors" figures=RwSignal::new(5) icon=IconData::FaUserGroupSolid percentage=RwSignal::new(-2) />
                <StatsCard color="primary" title="Weekly Revenue" figures=RwSignal::new(2500) icon=IconData::RiCoinsFinanceLine percentage=RwSignal::new(-2) />
            </div>
            <div class="flex flex-col lg:flex-row">
                <div class="mx-[5%] md:mx-[10%] flex flex-col gap-[10px] md:basis-1/2">
                    <h3>Recent Activity</h3>

                    <Timeline steps=timeline_items />
                </div>

                <div class="mx-[5%] md:mx-[10%] flex flex-col gap-[10px] md:basis-1/2">
                    <h3>Quick Actions</h3>

                    <div class="flex flex-col gap-[10px]">
                        <QuickAction title="Create Project" description="Create a new portfolio project" color="primary" icon=IconData::BsPlusLg />
                        <QuickAction title="New Blog Post" description="Create a new blog post" color="info" icon=IconData::BsVectorPen />
                        <QuickAction title="Upload Media" description="Upload images and files" color="warning" icon=IconData::AiFileAddOutlined />
                    </div>
                </div>
            </div>
        </>
    }
}
