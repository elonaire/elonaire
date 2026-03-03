use icondata as IconId;
use leptos::ev;
use leptos::prelude::*;
use leptos_icons::Icon;
use leptos_router::hooks::use_location;

use leptos_router::components::A;

use crate::views::dashboard::layout::MenuItem;

#[component]
pub fn Nav(
    #[prop(optional, default = Callback::new(|_| {}))] onmenuclick: Callback<ev::MouseEvent>,
) -> impl IntoView {
    let location = use_location();

    let is_dashboard = Memo::new(move |_| location.pathname.get().contains("/dashboard"));
    let is_blog = Memo::new(move |_| location.pathname.get().contains("/blog"));

    let menu_items = Memo::new(move |_| {
        vec![
            MenuItem::new("About", IconId::BsInfoCircle, "/about"),
            MenuItem::new("Resume", IconId::MdiCertificateOutline, "/resume"),
            MenuItem::new("Portfolio", IconId::MdiTrophyAward, "/portfolio"),
            MenuItem::new("Marketplace", IconId::MdiStore, "/marketplace"),
            MenuItem::new("Blog", IconId::RiArticleDocumentLine, "/blog"),
        ]
    });

    let blog_menu_items = Memo::new(move |_| {
        vec![
            MenuItem::new("Home", IconId::AiHomeOutlined, "/"),
            MenuItem::new("Blog Feed", IconId::BsRss, "/blog"),
            MenuItem::new("About", IconId::BsInfoCircle, "/blog/about"),
            // MenuItem::new("Categories", IconId::BsFilter, "/blog/categories"),
            // MenuItem::new("Pricing", IconId::BsCashCoin, "/blog/pricing"),
            MenuItem::new("Contact", IconId::BiContactSolid, "/blog/contact"),
        ]
    });

    view! {
        <>
        <div class="bg-contrast-white border-b-[1px] border-light-gray">
            <div class="display-constraints flex justify-between h-[47px]">
                    <button
                        class=move || format!("bg-transparent border-none cursor-pointer {}", if !is_dashboard.get() { "md:hidden" } else { "" })
                        on:click=move |ev| onmenuclick.run(ev)
                    >
                        <Icon width="24" height="24" icon=IconId::IoMenu />
                    </button>
                    <img src="http://localhost:3001/view/114aa7a5-66a5-4e72-aa10-6c316b05a001" class="w-[47px] object-cover" alt="Logo" />
                    <div class="flex items-center justify-end gap-[20px]">
                        { move ||
                            if !is_dashboard.get() && !is_blog.get() {
                                Some(view! {
                                    <div class="hidden md:flex items-center gap-[20px] text-sm">
                                        <For
                                            each=move || menu_items.get()
                                            key=|menu_item| menu_item.path.to_owned()
                                            let(child)
                                        >
                                            <A attr:class="h-full flex items-center cursor-pointer hover:text-primary" href=child.path>{child.label}</A>
                                        </For>
                                    </div>
                                })
                            } else {
                                None
                            }
                        }
                        { move ||
                            if is_blog.get() {
                                Some(view! {
                                    <div class="hidden md:flex items-center gap-[20px] text-sm">
                                        <For
                                            each=move || blog_menu_items.get()
                                            key=|menu_item| menu_item.path.to_owned()
                                            let(child)
                                        >
                                            <A attr:class="h-full flex items-center cursor-pointer hover:text-primary" href=child.path>{child.label}</A>
                                        </For>
                                    </div>
                                })
                            } else {
                                None
                            }
                        }
                        { move ||
                            if is_dashboard.get() || is_blog.get() {
                                Some(view! {
                                    <>
                                        <span class="md:hidden"><Icon width="24" height="24" icon=IconId::IoSearchOutline /></span>
                                        <img src="http://localhost:3001/view/e564672d-04ef-4be8-84b7-067f98494f1e" class="size-[27px] object-cover rounded-full" alt="dp" />
                                    </>
                                })
                            } else {
                                None
                            }
                        }
                        { move ||
                            if !is_dashboard.get() && !is_blog.get() {
                                Some(view! {
                                    <A attr:class="py-2 px-4 cursor-pointer rounded-[5px] border-2 border-primary text-primary hover:bg-primary hover:text-contrast-white font-bold" href="/ratecard">"Request Service"</A>
                                })
                            } else {
                                None
                            }
                        }
                    </div>
                </div>
            </div>
        </>
    }
}
