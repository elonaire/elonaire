use std::collections::HashMap;

use icondata::{AiHomeOutlined, BsInfoCircle, BsMoon, BsRss, BsSun, IoClose};
use leptos::{ev, prelude::*, task::spawn_local};
use leptos_icons::Icon;
use leptos_meta::*;
use leptos_router::{
    components::{A, Outlet},
    hooks::use_location,
};
use reactive_stores::Store;

use crate::{
    components::{forms::toggle_switch::ToggleSwitch, molecules::nav::Nav},
    data::{
        context::{
            shared::{check_auth, fetch_single_user},
            store::{AppStateContext, AppStateContextStoreFields},
        },
        models::{
            general::acl::{AuthInfo, AuthInfoStoreFields, UserInfoStoreFields},
            graphql::acl::FetchSingleUserVars,
        },
    },
    views::{dashboard::layout::MenuItem, public::error_handler::ErrorHandler},
};

#[component]
pub fn BlogLayout() -> impl IntoView {
    let current_state = expect_context::<Store<AppStateContext>>();
    let user_auth = current_state.user().auth_info();
    let dark_mode_is_active = current_state.dark_mode_is_active();
    let dark_mode_signal = Signal::derive(move || dark_mode_is_active.get());
    // track collapsed state
    let (collapsed, set_collapsed) = signal(false);
    let (is_loading, set_is_loading) = signal(false);
    let current_path = use_location().pathname;

    let handle_menu_click =
        move || Callback::new(move |_ev: ev::MouseEvent| set_collapsed.set(true));

    let menu_items = Memo::new(move |_| {
        vec![
            MenuItem::new("Home", AiHomeOutlined, "/", vec![]),
            MenuItem::new("Blog Feed", BsRss, "/blog", vec![]),
            MenuItem::new("About", BsInfoCircle, "/blog/about", vec![]),
            // MenuItem::new("Categories", BsFilter, "/blog/categories"),
            // MenuItem::new("Pricing", BsCashCoin, "/blog/pricing"),
            // MenuItem::new("Contact", BiContactSolid, "/blog/contact"),
        ]
    });

    // Effect to refresh user auth status
    Effect::new(move |_| {
        let current_state = current_state.clone();
        spawn_local(async move {
            let mut headers = HashMap::new() as HashMap<String, String>;
            headers.insert(
                "Authorization".into(),
                format!(
                    "Bearer {}",
                    current_state.user().auth_info().token().get_untracked()
                ),
            );

            let check_auth = check_auth(Some(&headers)).await;

            match check_auth {
                Ok(auth) => {
                    current_state.user().auth_info().set(AuthInfo {
                        token: auth
                            .new_access_token
                            .as_ref()
                            .unwrap_or(&String::new())
                            .to_owned(),
                        current_role: auth.current_role.clone(),
                        current_role_permissions: auth.current_role_permissions,
                    });
                    let user_id_vars = FetchSingleUserVars {
                        user_id: auth.sub.clone(),
                    };

                    let fetch_user_info_query = r#"
                        query FetchSingleUser($userId: String!) {
                            fetchSingleUser(userId: $userId) {
                                data {
                                    firstName
                                    middleName
                                    lastName
                                    gender
                                    dob
                                    email
                                    country
                                    phone
                                    createdAt
                                    updatedAt
                                    oauthClient
                                    oauthUserId
                                    profilePicture
                                    bio
                                    website
                                    address
                                    id
                                    fullName
                                    age
                                }
                                metadata {
                                    requestId
                                    newAccessToken
                                }
                            }
                        }
                       "#;

                    if let Ok(user_profile) =
                        fetch_single_user(&user_id_vars, None, fetch_user_info_query).await
                    {
                        current_state.user().user_profile().set(user_profile);
                    };
                }
                Err(_) => {}
            }
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
        <Title text="Techie Tenka"/>
        <main>
            <ErrorHandler />
            <div class="relative min-h-svh">
                {/* Sidebar overlay */}
                <div
                    class=move || format!(
                        "fixed top-0 left-0 h-full transition-all duration-300 bg-contrast-white dark:bg-navy shadow-md z-40 {}",
                        if collapsed.get() { "w-64" } else { "w-0" }
                    )
                >
                    {/* Only render content if expanded */}
                    {move || if collapsed.get() {
                        Some(view! {
                            <div class="flex flex-col mx-[5%] min-h-svh">
                                <div class="flex items-center justify-between h-[47px] border-b border-light-gray">
                                    <p class="font-medium">NAVIGATION</p>
                                    <button
                                        class="bg-transparent border-none"
                                        on:click=move |_| set_collapsed.set(false)
                                    >
                                        <Icon width="24" height="24" icon=IoClose />
                                    </button>
                                </div>
                                <nav class="flex flex-col">
                                    <For
                                        each=move || menu_items.get()
                                        key=|menu_item| menu_item.path.to_owned()
                                        let(child)
                                    >
                                        { move || {
                                            let is_active = current_path.get() == child.path;
                                            view! {
                                                <div class=format!("flex rounded-[5px] hover:bg-light-gray h-[40px] my-[5px] {}", if is_active { "bg-primary text-contrast-white" } else { "" }) on:click=move |_| set_collapsed.set(false)>
                                                    <A attr:class="flex-1 h-full flex items-center gap-[10px]" href=child.path>
                                                        <span class=format!("{}", if is_active { "text-contrast-white" } else { "" })><Icon width="24" height="24" icon=child.icon /></span>
                                                        <span class="flex-1">{child.label}</span>
                                                    </A>
                                                </div>
                                            }
                                        }

                                        }
                                    </For>
                                </nav>
                                {
                                    move || {
                                        let is_authenticated = !user_auth.get().token.is_empty();

                                        if !is_authenticated {
                                            Some(
                                                view! {
                                                    <div class="md:hidden flex flex-col gap-[10px]">
                                                        <div class="flex items-center h-[40px] border-b border-light-gray">
                                                            <p class="font-medium text-xs">ACTIONS</p>
                                                        </div>
                                                        <A attr:class="block md:hidden py-2 px-4 cursor-pointer rounded-[5px] border-2 border-primary text-primary hover:bg-primary hover:text-contrast-white font-bold text-center" href="/sign-in">"Sign In"</A>
                                                    </div>
                                                }
                                            )
                                        } else {
                                            None
                                        }
                                    }
                                }
                                <div class="md:hidden mt-auto flex flex-col gap-[10px]">
                                    <div class="flex items-center h-[40px] border-b border-light-gray">
                                        <p class="font-medium text-xs">PREFERENCES</p>
                                    </div>
                                    <div class="flex md:hidden items-center gap-[5px]">
                                        <ToggleSwitch
                                        active=dark_mode_signal
                                        label_active=""
                                        label_inactive=""
                                        on:change=move |_| dark_mode_is_active.set(!dark_mode_is_active.get())
                                        />
                                        {
                                            move || {
                                                let icon = if !dark_mode_is_active.get() { BsSun } else { BsMoon };

                                                view! {
                                                    <Icon icon=icon />
                                                }
                                            }
                                        }

                                    </div>
                                </div>
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
