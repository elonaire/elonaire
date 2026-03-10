use std::collections::HashMap;

use icondata as IconId;
use leptos::ev;
use leptos::prelude::*;
use leptos_icons::Icon;
use leptos_router::NavigateOptions;
use leptos_router::hooks::use_location;
use leptos_router::hooks::use_navigate;
use reactive_stores::Store;

use leptos_router::components::A;
use wasm_bindgen_futures::spawn_local;

use crate::components::general::button::BasicButton;
use crate::components::general::popover::Popover;
use crate::data::context::users::sign_out;
use crate::data::{
    context::store::{AppStateContext, AppStateContextStoreFields},
    models::general::acl::{AuthInfoStoreFields, UserInfoStoreFields},
};
use crate::views::dashboard::layout::MenuItem;

#[component]
pub fn Nav(
    #[prop(optional, default = Callback::new(|_| {}))] onmenuclick: Callback<ev::MouseEvent>,
) -> impl IntoView {
    let location = use_location();
    let showing_user_popover = RwSignal::new(false);

    let is_dashboard = Memo::new(move |_| location.pathname.get().contains("/dashboard"));
    let is_blog = Memo::new(move |_| location.pathname.get().contains("/blog"));
    let is_blog_home = Memo::new(move |_| location.pathname.get() == "/blog");

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
            // MenuItem::new("Contact", IconId::BiContactSolid, "/blog/contact"),
        ]
    });

    let current_state = expect_context::<Store<AppStateContext>>();
    let user_profile = current_state.user().user_profile();
    let navigate = use_navigate();

    let handle_sign_out = Callback::new(move |_| {
        let navigate = navigate.clone();
        let mut headers = HashMap::new() as HashMap<String, String>;
        headers.insert(
            "Authorization".into(),
            format!(
                "Bearer {}",
                current_state.user().auth_info().token().get_untracked()
            ),
        );

        spawn_local(async move {
            if let Ok(_) = sign_out(Some(&headers)).await {
                current_state.user().set(Default::default());
                navigate(
                    "/sign-in",
                    NavigateOptions {
                        resolve: false,
                        replace: true,
                        ..Default::default()
                    },
                );
            };
        });
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
                            if is_blog_home.get() {
                                Some(view! {
                                    <>
                                        <span class="md:hidden" on:click=move |_| {
                                            current_state.show_mobile_search().set(true);
                                        }><Icon width="24" height="24" icon=IconId::IoSearchOutline /></span>
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
                        { move ||
                            {
                                let profile_pic = user_profile.get().profile_picture.unwrap_or_default();
                                if is_dashboard.get() || is_blog.get() {
                                    Some(view! {
                                        <Popover showing=showing_user_popover display_item={
                                                        let profile_pic = profile_pic.clone(); // clone before moving into ViewFn
                                                        leptos::logging::log!("profile_pic: {profile_pic}");
                                                        move || view!{
                                                            <img src=format!("{}?width=300", profile_pic) class="size-[27px] object-cover rounded-full cursor-pointer" alt="dp" />
                                                        }
                                                    }>
                                            <div class="flex flex-col gap-2">
                                                <A attr:class="py-2 px-4 text-gray px-0 hover:bg-primary hover:text-contrast-white cursor-pointer rounded-[5px] font-bold" href="/dashboard/user/profile">
                                                    <span class="flex items-center justify-between">
                                                        <span>Profile</span>
                                                        <Icon icon=IconId::MdiCardAccountDetailsOutline />
                                                    </span>
                                                </A>
                                                <BasicButton
                                                    style_ext="text-danger px-0 hover:bg-danger hover:text-contrast-white"
                                                    onclick=handle_sign_out
                                                    >
                                                    <span class="flex items-center justify-between">
                                                        <span>Logout</span>
                                                        <Icon icon=IconId::BsPower />
                                                    </span>
                                                </BasicButton>
                                            </div>
                                        </Popover>
                                    })
                                } else {
                                    None
                                }
                            }
                        }
                    </div>
                </div>
            </div>
        </>
    }
}
