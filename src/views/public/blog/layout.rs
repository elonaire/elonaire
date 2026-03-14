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
    // track collapsed state
    let (collapsed, set_collapsed) = signal(false);
    let (is_loading, set_is_loading) = signal(false);
    let current_path = use_location().pathname;

    let handle_menu_click =
        move || Callback::new(move |_ev: ev::MouseEvent| set_collapsed.set(true));

    let menu_items = Memo::new(move |_| {
        vec![
            MenuItem::new("Home", IconId::AiHomeOutlined, "/"),
            MenuItem::new("Blog Feed", IconId::BsRss, "/blog"),
            MenuItem::new("About", IconId::BsInfoCircle, "/blog/about"),
            // MenuItem::new("Categories", IconId::BsFilter, "/blog/categories"),
            // MenuItem::new("Pricing", IconId::BsCashCoin, "/blog/pricing"),
            // MenuItem::new("Contact", IconId::BiContactSolid, "/blog/contact"),
        ]
    });

    // Effect::new(move || {
    //     set_is_loading.set(true);
    //     spawn_local(async move {
    //         let mut headers = HashMap::new() as HashMap<String, String>;
    //         headers.insert(
    //             "Authorization".into(),
    //             format!(
    //                 "Bearer {}",
    //                 current_state.user().auth_info().token().get_untracked()
    //             ),
    //         );
    //     });
    // });

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

    view! {
        <Title text="Techie Tenka"/>
        <main>
            <ErrorHandler />
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
                            <div class="flex flex-col mx-[5%]">
                                <div class="flex items-center justify-between h-[47px] border-y border-light-gray">
                                    <p class="text-mid-gray font-medium">NAVIGATION</p>
                                    <button
                                        class="bg-transparent border-none"
                                        on:click=move |_| set_collapsed.set(false)
                                    >
                                        <Icon width="24" height="24" icon=IconId::IoClose />
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
