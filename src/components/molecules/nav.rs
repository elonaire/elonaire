use std::collections::HashMap;

use icondata::{
    BsInfoCircle, BsMoon, BsPower, BsRss, BsSun, IoMenu, IoSearchOutline,
    MdiCardAccountDetailsOutline, MdiCertificateOutline, MdiStore, MdiTabletDashboard,
    MdiTrophyAward, RiArticleDocumentLine,
};
use leptos::ev;
use leptos::prelude::*;
use leptos_icons::Icon;
use leptos_router::NavigateOptions;
use leptos_router::hooks::use_location;
use leptos_router::hooks::use_navigate;
use reactive_stores::Store;

use leptos_router::components::A;
use wasm_bindgen_futures::spawn_local;

use crate::components::forms::toggle_switch::ToggleSwitch;
use crate::components::general::button::BasicButton;
use crate::components::general::hocs::permission_guard::PermissionGuard;
use crate::components::general::hocs::permission_guard::PermissionMatch;
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
            MenuItem::new("About", BsInfoCircle, "/about", vec![]),
            MenuItem::new("Resume", MdiCertificateOutline, "/resume", vec![]),
            MenuItem::new("Portfolio", MdiTrophyAward, "/portfolio", vec![]),
            MenuItem::new("Marketplace", MdiStore, "/marketplace", vec![]),
            MenuItem::new("Blog", RiArticleDocumentLine, "/blog", vec![]),
        ]
    });

    let blog_menu_items = Memo::new(move |_| {
        vec![
            // MenuItem::new("Home", AiHomeOutlined, ""),
            MenuItem::new("Blog Feed", BsRss, "/blog", vec![]),
            MenuItem::new("About", BsInfoCircle, "/blog/about", vec![]),
            // MenuItem::new("Categories", BsFilter, "/blog/categories"),
            // MenuItem::new("Pricing", BsCashCoin, "/blog/pricing"),
            // MenuItem::new("Contact", BiContactSolid, "/blog/contact"),
        ]
    });

    let current_state = expect_context::<Store<AppStateContext>>();
    let user_profile = current_state.user().user_profile();
    let dark_mode_is_active = current_state.dark_mode_is_active();
    let dark_mode_signal = Signal::derive(move || dark_mode_is_active.get());
    let navigate = use_navigate();
    let is_authenticated = Memo::new(move |_| !user_profile.get().profile_picture.is_none());

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
                navigate("/sign-in", Default::default());
            };
        });
    });

    Effect::new(move |_| {
        if let Some(doc) = document().document_element() {
            let class_list = doc.class_list();

            if !dark_mode_is_active.get() && class_list.contains("dark") {
                let _ = class_list.remove_1("dark");
            } else if !dark_mode_is_active.get() && !class_list.contains("dark") {
            } else {
                let _ = class_list.add_1("dark");
            }
        }
    });

    view! {
        <>
        <div class="border-b-[1px] border-light-gray dark:border-mid-gray">
            <div class="display-constraints flex items-center justify-between h-[47px]">

                // Left — hamburger (mobile only on non-dashboard, always on dashboard)
                <button
                    class=move || format!("bg-transparent border-none cursor-pointer shrink-0 {}", if !is_dashboard.get() { "md:hidden" } else { "" })
                    on:click=move |ev| onmenuclick.run(ev)
                >
                    <Icon width="24" height="24" icon=IoMenu />
                </button>

                // Center/Left — logo
                <A href="/" attr:class="flex items-center h-full shrink-0">
                    <img src="https://api.techietenka.com/files/view/47a6c9dd-6d87-42ff-a041-9d2a7896c47f" class="h-full w-auto object-cover" alt="Logo" />
                </A>

                // Right — all right-side items
                <div class="flex items-center gap-[20px]">

                    // Desktop menu items — public
                    {move || (!is_dashboard.get() && !is_blog.get()).then(|| view! {
                        <div class="hidden md:flex items-center gap-[20px] text-sm">
                            <For
                                each=move || menu_items.get()
                                key=|item| item.path.to_owned()
                                let(child)
                            >
                                <A attr:class="flex items-center cursor-pointer hover:text-primary" href=child.path>{child.label}</A>
                            </For>
                        </div>
                    })}

                    // Desktop menu items — blog
                    {move || is_blog.get().then(|| view! {
                        <div class="hidden md:flex items-center gap-[20px] text-sm">
                            <For
                                each=move || blog_menu_items.get()
                                key=|item| item.path.to_owned()
                                let(child)
                            >
                                <A attr:class="flex items-center cursor-pointer hover:text-primary" href=child.path>{child.label}</A>
                            </For>
                        </div>
                    })}

                    // Dark mode toggle — desktop only
                    <div class="hidden md:flex items-center gap-[5px]">
                        <ToggleSwitch
                            active=dark_mode_signal
                            label_active=""
                            label_inactive=""
                            on:change=move |_| dark_mode_is_active.set(!dark_mode_is_active.get())
                            id_attr="dark_mode_toggle"
                        />
                        {move || {
                            let icon = if dark_mode_is_active.get() { BsMoon } else { BsSun };
                            view! { <Icon icon=icon /> }
                        }}
                    </div>

                    // CTA — desktop, public only
                    {move || (!is_dashboard.get() && !is_blog.get()).then(|| view! {
                        <A
                            attr:class="hidden md:flex py-2 px-4 cursor-pointer rounded-[5px] border-2 border-primary text-primary hover:bg-primary hover:text-contrast-white font-bold"
                            href="/ratecard"
                        >
                            "Request Service"
                        </A>
                    })}

                    // Search icon — mobile, blog home only
                    {move || is_blog_home.get().then(|| view! {
                        <span
                            class="flex md:hidden items-center cursor-pointer"
                            on:click=move |_| current_state.show_mobile_search().set(true)
                        >
                            <Icon width="24" height="24" icon=IoSearchOutline />
                        </span>
                    })}

                    // Sign in — dashboard or blog, no profile pic
                    {move || {

                        ((is_dashboard.get() || is_blog.get()) && !is_authenticated.get()).then(|| view! {
                            <A
                                attr:class="hidden md:flex py-2 px-4 cursor-pointer rounded-[5px] border-2 border-primary text-primary hover:bg-primary hover:text-contrast-white font-bold text-sm"
                                href="/sign-in"
                            >
                                "Sign In"
                            </A>
                        })
                    }}

                    // Profile pic + popover — dashboard or blog, has profile pic
                    {move || {
                        let profile_pic = user_profile.get().profile_picture;
                        ((is_dashboard.get() || is_blog.get())).then(|| {
                            profile_pic.map(|pic| view! {
                                <Popover
                                    showing=showing_user_popover
                                    display_item={
                                        let pic = pic.clone();
                                        move || view! {
                                            <img
                                                src=format!("{}?width=300", pic)
                                                class="size-[35px] object-cover rounded-full cursor-pointer"
                                                alt="dp"
                                            />
                                        }
                                    }
                                >
                                    <div class="flex flex-col gap-2 p-1">
                                        <A
                                            attr:class="py-2 px-4 hover:bg-primary hover:text-contrast-white cursor-pointer rounded-[5px] font-bold"
                                            href="/dashboard/user/profile"
                                        >
                                            <span class="flex items-center justify-between gap-4">
                                                <span>"Profile"</span>
                                                <Icon icon=MdiCardAccountDetailsOutline />
                                            </span>
                                        </A>
                                        <PermissionGuard
                                            match_mode=PermissionMatch::Any
                                            permissions=vec![
                                                "read:stats".into(),
                                                "write:portfolio".into(),
                                                "write:blog_post".into(),
                                                "write:media".into(),
                                            ]
                                        >
                                            <A
                                                attr:class="py-2 px-4 hover:bg-primary hover:text-contrast-white cursor-pointer rounded-[5px] font-bold"
                                                href="/dashboard"
                                            >
                                                <span class="flex items-center justify-between gap-4">
                                                    <span>"Dashboard"</span>
                                                    <Icon icon=MdiTabletDashboard />
                                                </span>
                                            </A>
                                        </PermissionGuard>
                                        <BasicButton
                                            style_ext="text-danger px-4 hover:bg-danger hover:text-contrast-white"
                                            onclick=handle_sign_out
                                        >
                                            <span class="flex items-center justify-between gap-4">
                                                <span>"Logout"</span>
                                                <Icon icon=BsPower />
                                            </span>
                                        </BasicButton>
                                    </div>
                                </Popover>
                            })
                        })
                    }}
                </div>
            </div>
        </div>
        </>
    }
}
