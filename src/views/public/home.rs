use icondata as IconId;
use leptos::{ev, prelude::*};
use leptos_icons::Icon;
use leptos_meta::*;
use leptos_router::components::A;

use crate::{
    components::{general::button::BasicButton, molecules::nav::Nav},
    views::dashboard::layout::MenuItem,
};

#[island]
pub fn Home() -> impl IntoView {
    // track collapsed state
    let (collapsed, set_collapsed) = signal(false);
    let handle_menu_click =
        move || Callback::new(move |_ev: ev::MouseEvent| set_collapsed.set(true));

    let menu_items = Memo::new(move |_| {
        vec![
            MenuItem::new("About", IconId::BsInfoSquare, "/about"),
            MenuItem::new("Resume", IconId::MdiCertificateOutline, "/resume"),
            MenuItem::new("Portfolio", IconId::MdiTrophyAward, "/portfolio"),
            MenuItem::new("Marketplace", IconId::MdiStore, "/marketplace"),
            MenuItem::new("Blog", IconId::RiArticleDocumentLine, "/blog"),
        ]
    });

    view! {
        <Title text="Techie Tenka"/>
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
                            <div class="overflow-hidden mx-[5%] md:mx-[10%]">
                                <div class="flex items-center justify-between h-[45px] border-y border-gray-200">
                                    <p class="text-gray-300 font-medium">NAVIGATION</p>
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
                                        <div class="block rounded-[5px] hover:bg-gray-100 h-[45px]" on:click=move |_| set_collapsed.set(false)>
                                            <A attr:class="h-full flex items-center gap-[10px]" href=child.path>
                                                <span class="text-mid-gray"><Icon width="24" height="24" icon=child.icon /></span>
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


                    <div class="flex flex-col gap-[20px] mx-[5%] md:mx-[10%]">
                        <div class="flex flex-col gap-[20px] text-center">
                            <h3>Hello, my name is</h3>
                            <h1><span class="text-primary">Elon</span>" Aseneka Idiong'o"</h1>
                            <img alt="dp" src="http://localhost:3001/view/e564672d-04ef-4be8-84b7-067f98494f1e" class="h-[435px] object-cover rounded-[5px]" />
                            <p class="text-base font-bold">Software Engineer / UI/UX Designer / IoT Engineer</p>
                            <p>9+ years of experience overall and lately focused on designing and building intuitive and high performance Software.</p>
                            <BasicButton button_text="Download my resume" icon=Some(IconId::FiDownload) icon_before=false style_ext="bg-primary text-white" />
                        </div>
                    </div>

                </div>
            </div>
        </main>
    }
}
