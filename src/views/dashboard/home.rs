use icondata::{
    AiFileAddOutlined, BiBriefcaseRegular, BsPlusLg, BsVectorPen, FaUserGroupSolid,
    RiArticleDocumentLine, RiCoinsFinanceLine,
};
use leptos::prelude::*;
use leptos_meta::*;
use leptos_router::hooks::use_navigate;

use detaxine_ui::components::{
    data_display::timeline::{Timeline, TimelineItem, TimelineStatus},
    navigation::breadcrumbs::Breadcrumbs,
};

use crate::{
    components::{
        hocs::permission_guard::PermissionMatch,
        molecules::{quick_action::QuickAction, stats_card::StatsCard},
    },
    utils::hooks::use_permissions::use_permission,
};

#[component]
pub fn DashboardHome() -> impl IntoView {
    let timeline_items = RwSignal::new(vec![TimelineItem {
        time_info: "3 mins ago".into(),
        title: "New Project Created".into(),
        more_info: Some("Techie Tenka Project was created.".into()),
        status: TimelineStatus::Info,
        content: ViewFn::from(|| view! { <p>"Project created with Leptos!"</p> }),
        ..Default::default()
    }]);
    let navigate = use_navigate();

    let can_view = use_permission(
        &vec!["read:stats".to_string(), "write:portfolio".to_string()],
        PermissionMatch::Any,
    );

    Effect::new(move |_| {
        if !can_view.get() {
            navigate("/dashboard/user/profile", Default::default());
        }
    });

    view! {
        <>
            <Title text="Dashboard Home"/>
            <div class="display-constraints">
                <Breadcrumbs custom_route_names=["Home", "Dashboard"] />
            </div>

            <h1 class="display-constraints">Dashboard Overview</h1>

            <div class="display-constraints grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-4 gap-[20px]">
                <StatsCard color="info" title="Total Projects" figures=RwSignal::new(25) icon=BiBriefcaseRegular percentage=RwSignal::new(-2) />
                <StatsCard color="success" title="Published Posts" figures=RwSignal::new(12) icon=RiArticleDocumentLine percentage=RwSignal::new(3) />
                <StatsCard color="warning" title="Monthly Visitors" figures=RwSignal::new(5) icon=FaUserGroupSolid percentage=RwSignal::new(-2) />
                <StatsCard color="primary" title="Weekly Revenue" figures=RwSignal::new(2500) icon=RiCoinsFinanceLine percentage=RwSignal::new(-2) />
            </div>
            <div class="display-constraints  flex flex-col lg:flex-row">
                <div class="flex flex-col gap-[10px] md:basis-1/2">
                    <h3>Recent Activity</h3>

                    <Timeline steps=timeline_items />
                </div>

                <div class="flex flex-col gap-[10px] md:basis-1/2">
                    <h3>Quick Actions</h3>

                    <div class="flex flex-col gap-[10px]">
                        <QuickAction title="Create Project" description="Create a new portfolio project" color="primary" icon=BsPlusLg />
                        <QuickAction title="New Blog Post" description="Create a new blog post" color="info" icon=BsVectorPen />
                        <QuickAction title="Upload Media" description="Upload images and files" color="warning" icon=AiFileAddOutlined />
                    </div>
                </div>
            </div>
        </>
    }.into_any()
}
