use icondata as IconData;
use icondata::Icon as IconId;
use leptos::{ev, prelude::*};
use leptos_icons::Icon;
use leptos_router::{
    components::{A, Outlet},
    hooks::use_location,
};

use crate::components::{
    general::hocs::permission_guard::{PermissionGuard, PermissionMatch},
    molecules::nav::Nav,
};

#[derive(Debug, Clone, PartialEq)]
pub struct MenuItem<'a> {
    pub label: &'a str,
    pub icon: IconId,
    pub path: &'a str,
    pub permissions: Vec<String>,
}

impl<'a> MenuItem<'a> {
    pub fn new(label: &'a str, icon: IconId, path: &'a str, permissions: Vec<String>) -> Self {
        Self {
            label,
            icon,
            path,
            permissions,
        }
    }
}

#[component]
pub fn DashboardLayout() -> impl IntoView {
    // track collapsed state
    let (collapsed, set_collapsed) = signal(false);
    let current_path = use_location().pathname;

    let handle_menu_click =
        move || Callback::new(move |_ev: ev::MouseEvent| set_collapsed.set(true));

    let menu_items = Memo::new(move |_| {
        vec![
            MenuItem::new(
                "Dashboard",
                IconData::MdiMonitorDashboard,
                "/dashboard",
                vec!["read:stats".into(), "write:portfolio".into()],
            ),
            MenuItem::new(
                "Professional Details",
                IconData::MdiBadgeAccountHorizontalOutline,
                "/dashboard/professional-details",
                vec!["write:professional_details".into()],
            ),
            MenuItem::new(
                "Portfolio",
                IconData::MdiTrophyAward,
                "/dashboard/portfolio",
                vec!["write:portfolio".into()],
            ),
            MenuItem::new(
                "Services",
                IconData::BsWrenchAdjustable,
                "/dashboard/services",
                vec!["write:service".into()],
            ),
            MenuItem::new(
                "Service Rates",
                IconData::FaMoneyBillTransferSolid,
                "/dashboard/service-rates",
                vec!["write:service_rate".into()],
            ),
            MenuItem::new(
                "Service Requests",
                IconData::FaFileContractSolid,
                "/dashboard/service-requests",
                vec!["read:service_request".into()],
            ),
            MenuItem::new(
                "Ratecards",
                IconData::FaMoneyCheckDollarSolid,
                "/dashboard/ratecards",
                vec!["write:ratecard".into()],
            ),
            MenuItem::new(
                "Resume",
                IconData::MdiCertificateOutline,
                "/dashboard/resume",
                vec!["write:resume".into()],
            ),
            MenuItem::new(
                "Skills",
                IconData::BiCertificationRegular,
                "/dashboard/skills",
                vec!["write:skill".into()],
            ),
            MenuItem::new(
                "Users",
                IconData::FaUserGroupSolid,
                "/dashboard/users",
                vec!["write:user".into()],
            ),
            MenuItem::new(
                "Roles",
                IconData::FaUserLockSolid,
                "/dashboard/roles",
                vec!["write:role".into()],
            ),
            MenuItem::new(
                "Permissions",
                IconData::FaUserShieldSolid,
                "/dashboard/permissions",
                vec!["write:permission".into()],
            ),
            MenuItem::new(
                "Resources",
                IconData::BsTools,
                "/dashboard/resources",
                vec!["write:resource".into()],
            ),
            MenuItem::new(
                "Organizations",
                IconData::BsBuildings,
                "/dashboard/organizations",
                vec!["write:organization".into()],
            ),
            MenuItem::new(
                "Departments",
                IconData::MdiFolderAccountOutline,
                "/dashboard/departments",
                vec!["write:department".into()],
            ),
            MenuItem::new(
                "Blog",
                IconData::RiArticleDocumentLine,
                "/dashboard/blog",
                vec!["write:blog_post".into()],
            ),
            MenuItem::new(
                "Media",
                IconData::AiFileImageOutlined,
                "/dashboard/media",
                vec!["write:media".into()],
            ),
        ]
    });

    view! {
        // <Title text="Dashboard"/>
        <main>
            <div class="relative min-h-svh">
                {/* Sidebar overlay */}
                <div
                    class=move || format!(
                        "fixed top-0 left-0 h-full transition-all duration-300 bg-contrast-white shadow-md z-40 {}",
                        if collapsed.get() { "w-64" } else { "w-0" }
                    )
                >
                    {/* Only render content if expanded */}
                    {move || if collapsed.get() {
                        Some(view! {
                            <div class="flex flex-col mx-[5%]">
                                <div class="flex items-center justify-between h-[47px] border-y border-light-gray">
                                    <p class="text-mid-gray font-medium">NAVIGATION</p>
                                    <button
                                        class="bg-transparent border-none"
                                        on:click=move |_| set_collapsed.set(false)
                                    >
                                        <Icon width="24" height="24" icon=IconData::IoClose />
                                    </button>
                                </div>
                                <nav class="flex flex-col">
                                    <For
                                        each=move || menu_items.get()
                                        key=|menu_item| menu_item.path.to_owned()
                                        let(child)
                                    >
                                    <PermissionGuard match_mode=PermissionMatch::Any permissions=child.permissions>
                                        { move || {
                                            let is_active = current_path.get() == child.path;
                                            view! {
                                                <div class=format!("flex rounded-[5px] hover:bg-light-gray h-[45px] {}", if is_active { "bg-primary text-contrast-white" } else { "" }) on:click=move |_| set_collapsed.set(false)>
                                                    <A attr:class="flex-1 h-full flex items-center gap-[10px]" href=child.path>
                                                        <span class=format!("{}", if is_active { "text-contrast-white" } else { "text-mid-gray" })><Icon width="24" height="24" icon=child.icon /></span>
                                                        <span class="flex-1">{child.label}</span>
                                                    </A>
                                                </div>
                                            }
                                        }
                                        }
                                    </PermissionGuard>
                                    </For>
                                </nav>
                            </div>
                        })
                    } else {
                        None
                    }}
                </div>

                {/* Dark backdrop when sidebar is open */}
                {move || if collapsed.get() {
                    Some(view! {
                        <div
                            class="fixed inset-0 bg-light-gray opacity-50 z-30"
                            on:click=move |_| set_collapsed.set(false)
                        />
                    })
                } else {
                    None
                }}

                {/* Main content */}
                <div class="flex flex-col gap-[40px]">
                    {/* Toggle button (opens sidebar) */}
                    <Nav onmenuclick=handle_menu_click() />


                    <Outlet />

                </div>
            </div>
        </main>
    }
}
