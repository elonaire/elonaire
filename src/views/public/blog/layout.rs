use std::collections::HashMap;

use icondata as IconId;
use leptos::{ev, prelude::*, task::spawn_local};
use leptos_icons::Icon;
use leptos_meta::*;
use leptos_router::{
    components::{A, Outlet},
    hooks::use_location,
};
use reactive_stores::Store;

use crate::{
    components::molecules::nav::Nav,
    data::{
        context::store::{AppStateContext, AppStateContextStoreFields},
        models::general::acl::{AuthInfoStoreFields, UserInfoStoreFields},
    },
    views::dashboard::layout::MenuItem,
};

#[island]
pub fn BlogLayout() -> impl IntoView {
    let current_state = expect_context::<Store<AppStateContext>>();
    // track collapsed state
    let (collapsed, set_collapsed) = signal(false);
    let (is_loading, set_is_loading) = signal(false);
    let current_path = use_location().pathname;

    let handle_menu_click =
        move || Callback::new(move |_ev: ev::MouseEvent| set_collapsed.set(true));

    let menu_items = Memo::new(move |_| {
        vec![
            MenuItem::new("Blog Home", IconId::MdiFolderHomeOutline, "/blog"),
            MenuItem::new("About", IconId::BsInfo, "/blog/about"),
            MenuItem::new("Categories", IconId::BiFilterAltRegular, "/blog/categories"),
            MenuItem::new("Pricing", IconId::BsCashCoin, "/blog/pricing"),
            MenuItem::new("Contact", IconId::BiMessageAddRegular, "/blog/contact"),
        ]
    });

    Effect::new(move || {
        set_is_loading.set(true);
        spawn_local(async move {
            let mut headers = HashMap::new() as HashMap<String, String>;
            headers.insert(
                "Authorization".into(),
                format!(
                    "Bearer {}",
                    current_state.user().auth_info().token().get_untracked()
                ),
            );
        });
    });

    view! {
        <Title text="Techie Tenka"/>
        <main>
            <div class="relative min-h-svh bg-contrast-white">
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
                            <div class="overflow-hidden mx-[5%] md:mx-[10%]">
                                <div class="flex items-center justify-between h-[45px] border-y border-light-gray">
                                    <p class="text-mid-gray font-medium">NAVIGATION</p>
                                    <button
                                        class="bg-transparent border-none"
                                        on:click=move |_| set_collapsed.set(false)
                                    >
                                        <Icon width="24" height="24" icon=IconId::IoClose />
                                    </button>
                                </div>
                                <nav>
                                    <For
                                        each=move || menu_items.get()
                                        key=|menu_item| menu_item.path.to_owned()
                                        let(child)
                                    >
                                        { move || {
                                            let is_active = current_path.get() == child.path;
                                            view! {
                                                <div class=format!("block rounded-[5px] hover:bg-light-gray h-[40px] my-[5px] {}", if is_active { "bg-primary text-contrast-white" } else { "" }) on:click=move |_| set_collapsed.set(false)>
                                                    <A attr:class="h-full flex items-center gap-[10px]" href=child.path>
                                                        <span class=format!("{}", if is_active { "text-contrast-white" } else { "text-mid-gray" })><Icon width="24" height="24" icon=child.icon /></span>
                                                        <span class="flex-1">{child.label}</span>
                                                    </A>
                                                </div>
                                            }
                                        }

                                        }
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
                            class="fixed inset-0 bg-contrast-white opacity-50 z-30"
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
