use std::collections::HashMap;

use icondata as IconId;
use leptos::{ev, prelude::*, task::spawn_local};
use leptos_icons::Icon;
use leptos_meta::*;
use leptos_router::components::A;
use reactive_stores::Store;

use crate::{
    components::{general::button::BasicButton, molecules::nav::Nav},
    data::{
        context::{
            store::{AppStateContext, AppStateContextStoreFields},
            users::fetch_site_owner_info,
        },
        models::{
            general::acl::{AuthInfoStoreFields, UserInfoStoreFields},
            graphql::shared::{FetchSiteResourcesResponse, UserProfessionalInfo},
        },
    },
    utils::graphql_client::perform_query_without_vars,
    views::dashboard::layout::MenuItem,
};

#[island]
pub fn Home() -> impl IntoView {
    // track collapsed state
    let current_state = expect_context::<Store<AppStateContext>>();
    let (collapsed, set_collapsed) = signal(false);
    let (is_loading, set_is_loading) = signal(false);
    let (professions, set_professions) = signal(Vec::new() as Vec<UserProfessionalInfo>);
    // State: index of the currently selected profession (default to first)
    let (selected_profession, set_selected_profession) = signal(String::new());

    let site_owner_info = move || current_state.site_owner_info(); // Should return ReadSignal<UserInfo>
    // Derived signal for the current description
    let current_description = Memo::new(move |_| {
        professions
            .get()
            .iter()
            .find(|r| r.id.clone().unwrap_or_default() == selected_profession.get())
            .map(|r| r.description.clone())
            .unwrap_or_default()
    });

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

    Effect::new(move || {
        set_is_loading.set(true);
        spawn_local(async move {
            let fetch_professions_query = r#"
                   query FetchSiteResources {
                        fetchSiteResources {
                            data {
                                professionalInfo {
                                    description
                                    active
                                    occupation
                                    startDate
                                    id
                                    yearsOfExperience
                                }
                            }
                            metadata {
                                newAccessToken
                                requestId
                            }
                        }
                   }
               "#;

            let mut headers = HashMap::new() as HashMap<String, String>;
            headers.insert(
                "Authorization".into(),
                format!(
                    "Bearer {}",
                    current_state.user().auth_info().token().get_untracked()
                ),
            );

            let fetch_professions_response =
                perform_query_without_vars::<FetchSiteResourcesResponse>(
                    None,
                    "http://localhost:8080/api/shared",
                    fetch_professions_query,
                )
                .await;

            let _site_owner_info = fetch_site_owner_info(&current_state, None).await;

            match fetch_professions_response.get_data() {
                Some(data) => {
                    let professions = data
                        .fetch_site_resources
                        .as_ref()
                        .unwrap()
                        .get_data()
                        .professional_info
                        .as_ref()
                        .unwrap()
                        .to_vec();

                    if let Some(first) = &professions.first() {
                        if let Some(id) = &first.id {
                            set_selected_profession.set(id.clone());
                        }
                    }

                    set_professions.set(professions);

                    set_is_loading.set(false);
                }
                None => {
                    set_is_loading.set(false);
                }
            };
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
                                        <div class="block rounded-[5px] hover:bg-light-gray h-[45px]" on:click=move |_| set_collapsed.set(false)>
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


                    <div class="flex flex-col gap-[20px] mx-[5%] md:mx-[10%]">
                        <div class="flex flex-col gap-[20px] text-center">
                            <h3>Hello, my name is</h3>
                            <h1><span class="text-primary">{move || site_owner_info().get().first_name}</span>{move || format!(" {} {}", site_owner_info().get().middle_name.unwrap_or_default(), site_owner_info().get().last_name.unwrap_or_default())}</h1>
                            <img alt="dp" src={{move || site_owner_info().get().profile_picture}} class="h-[435px] object-cover rounded-[5px]" />
                            <p class="text-base font-bold">
                                {move || professions.get().into_iter().enumerate().map(|(idx, profession)| {
                                    let occupation = profession.occupation.unwrap_or_default();
                                    let profession_id = profession.id.unwrap_or_default();
                                    // Clone only when needed: inside the closures
                                    let is_selected = {
                                        let profession_id = profession_id.clone();
                                        move || selected_profession.get() == profession_id
                                    };
                                    let profession_id_for_click = profession_id.clone();
                                    let on_click = move |_| {
                                        set_selected_profession.set(profession_id_for_click.clone())
                                    };

                                    let is_last = move || idx == professions.get().len() - 1;

                                    view! {
                                        <span
                                            class=move || format!("font-bold {}",if is_selected() {
                                                "text-primary underline cursor-default"
                                            } else {
                                                "cursor-pointer hover:text-primary"
                                            })
                                            on:click=on_click
                                        >
                                            {occupation}
                                        </span>
                                        // Add separator except after the last one
                                        {move || if is_last() { "" } else { " / " }}
                                    }
                                }).collect::<Vec<_>>()}
                            </p>

                            // The description paragraph that updates on click
                            <p class="min-h-[90px]">
                                {current_description}
                            </p>
                            <BasicButton button_text="Download my resume" icon=Some(IconId::FiDownload) icon_before=false style_ext="bg-primary text-contrast-white" />
                        </div>
                    </div>

                </div>
            </div>
        </main>
    }
}
