use icondata as IconData;
use icondata::Icon as IconId;
use leptos::{ev, prelude::*};
use leptos_icons::Icon;
use leptos_meta::*;
use leptos_router::components::{A, Outlet};

use crate::components::molecules::nav::Nav;

#[derive(Debug, Clone, Copy, PartialEq)]
struct MenuItem<'a> {
    label: &'a str,
    icon: IconId,
    path: &'a str,
}

impl<'a> MenuItem<'a> {
    fn new(label: &'a str, icon: IconId, path: &'a str) -> Self {
        Self { label, icon, path }
    }
}

#[island]
pub fn DashboardLayout() -> impl IntoView {
    // track collapsed state
    let (collapsed, set_collapsed) = signal(false);

    let handle_menu_click =
        move || Callback::new(move |_ev: ev::MouseEvent| set_collapsed.set(true));

    let menu_items = Memo::new(move |_| {
        vec![
            MenuItem::new("Dashboard", IconData::MdiMonitorDashboard, "/dashboard"),
            MenuItem::new(
                "Professional Details",
                IconData::MdiBadgeAccountHorizontalOutline,
                "/dashboard/professional-details",
            ),
            MenuItem::new(
                "Portfolio",
                IconData::MdiTrophyAward,
                "/dashboard/portfolio",
            ),
            MenuItem::new("Services", IconData::BiWrenchRegular, "/dashboard/services"),
            MenuItem::new(
                "Resume",
                IconData::MdiCertificateOutline,
                "/dashboard/resume",
            ),
            MenuItem::new(
                "Skills",
                IconData::BiCertificationRegular,
                "/dashboard/skills",
            ),
            MenuItem::new("Blog", IconData::RiArticleDocumentLine, "/dashboard/blog"),
            MenuItem::new("Media", IconData::AiFileImageOutlined, "/dashboard/media"),
        ]
    });

    view! {
        <Title text="Dashboard"/>
        <main>
            <div class="relative min-h-screen bg-gray-100">
                {/* Sidebar overlay */}
                <div
                    class=move || format!(
                        "fixed top-0 left-0 h-full transition-all duration-300 bg-white shadow-md z-40 {}",
                        if collapsed.get() { "w-64" } else { "w-0" }
                    )
                >
                    {/* Only render content if expanded */}
                    {move || if collapsed.get() {
                        Some(view! {
                            <div class="overflow-hidden mx-[20px]">
                                <div class="flex items-center justify-between h-[45px] border-y border-gray-200">
                                    <p class="text-gray-300 font-medium">NAVIGATION</p>
                                    <button
                                        class="bg-transparent border-none"
                                        on:click=move |_| set_collapsed.set(false)
                                    >
                                        <Icon width="24" height="24" icon=IconData::IoClose />
                                    </button>
                                </div>
                                <nav>
                                    <For
                                        each=move || menu_items.get()
                                        key=|menu_item| menu_item.path.to_owned()
                                        let(child)
                                    >
                                        <div class="block rounded-md hover:bg-gray-100 h-[45px]" on:click=move |_| set_collapsed.set(false)>
                                            <A attr:class="h-full flex items-center gap-[10px]" href=child.path>
                                                <Icon width="24" height="24" icon=child.icon />
                                                <span class="flex-1">{child.label}</span>
                                            </A>
                                        </div>
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
                            class="fixed inset-0 bg-gray-100 opacity-50 z-30"
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
