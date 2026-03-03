use std::collections::HashMap;

use icondata as IconId;
use leptos::{ev, prelude::*, task::spawn_local};
use leptos_icons::Icon;
use leptos_meta::*;
use leptos_router::components::A;
use reactive_stores::Store;

use crate::{
    components::{
        general::button::BasicButton,
        molecules::{footer::Footer, nav::Nav},
    },
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
            MenuItem::new("About", IconId::BsInfoCircle, "/about"),
            MenuItem::new("Resume", IconId::MdiCertificateOutline, "/resume"),
            MenuItem::new("Portfolio", IconId::MdiTrophyAward, "/portfolio"),
            MenuItem::new("Marketplace", IconId::MdiStore, "/marketplace"),
            MenuItem::new("Blog", IconId::RiArticleDocumentLine, "/blog"),
        ]
    });

    let ethics = vec![
        (
            "bg-[url('https://api.techietenka.com/files/view/a7641914-73a2-4747-9728-8fd74177259c?width=800')]",
            "Commitment to Quality",
            "I never compromise on delivering exceptional results",
            "md:row-span-1 md:col-span-2", // ethic1: spans 2 cols, 1 row
        ),
        (
            "bg-[url('https://api.techietenka.com/files/view/6162cb5a-62b6-43e4-9eab-1f2df0204140?width=800')]",
            "Timely Delivery",
            "Meeting deadlines is not negotiable",
            "md:row-span-1 md:col-span-2", // ethic2: spans 2 cols, 1 row
        ),
        (
            "bg-[url('https://api.techietenka.com/files/view/29eb4743-b4fe-4e74-b2a6-f74592829e85?width=800')]",
            "Clear Communication",
            "Transparency at every step of the process",
            "md:row-span-2 md:col-span-2", // ethic3: spans 2 cols, 2 rows
        ),
        (
            "bg-[url('https://api.techietenka.com/files/view/a6b4ac55-c487-4f82-bac8-fb1e4920869f?width=800')]",
            "Continuous Learning",
            "Always staying ahead with latest technologies",
            "md:row-span-2 md:col-span-2", // ethic4: spans 2 cols, 2 rows
        ),
        (
            "bg-[url('https://api.techietenka.com/files/view/8671a04d-8fbe-4b17-a75d-7e79217e3188?width=800')]",
            "Client-Focused",
            "Your success is my priority",
            "md:row-span-1 md:col-span-2", // ethic5: spans 2 cols, 1 row
        ),
    ];

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
                                        <Icon width="24" height="24" icon=IconId::IoClose />
                                    </button>
                                </div>
                                <nav class="flex flex-col">
                                    <For
                                        each=move || menu_items.get()
                                        key=|menu_item| menu_item.path.to_owned()
                                        let(child)
                                    >
                                        <div class="flex rounded-[5px] hover:bg-light-gray h-[45px]" on:click=move |_| set_collapsed.set(false)>
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
                <div class="h-svh overflow-y-scroll snap-y snap-mandatory">
                    <div class="snap-start min-h-svh flex flex-col gap-[40px] pb-[40px] relative">
                        {/* Toggle button (opens sidebar) */}
                        <Nav onmenuclick=handle_menu_click() />


                        // My User Info
                        <div class="flex flex-col md:flex-row gap-[20px] md:justify-between md:items-center display-constraints">
                            <div class="flex flex-col gap-[20px] md:gap-[40px] text-center md:text-left">
                                <p class="text-salutation">Hello, I am</p>
                                <p class="text-owner-name"><span class="text-primary">{move || site_owner_info().get().first_name}</span>{move || format!(" {} {}", site_owner_info().get().middle_name.unwrap_or_default(), site_owner_info().get().last_name.unwrap_or_default())}</p>
                                <img alt="dp" src={{move || site_owner_info().get().profile_picture}} class="h-[435px] object-cover rounded-[5px] md:hidden" />
                                <p class="text-2xl font-bold">
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
                                <p class="min-h-[90px] max-w-[600px] text-base">
                                    {current_description}
                                </p>
                                <BasicButton button_text="Download my resume" icon=Some(IconId::FiDownload) icon_before=false style_ext="bg-primary text-contrast-white md:w-[292px]" />
                            </div>
                            <img alt="dp" src={{move || site_owner_info().get().profile_picture}} class="object-cover rounded-[5px] hidden md:block w-[50%] h-auto bg-contrast-white mix-blend-multiply" />
                        </div>

                        {/* Scroll button */}
                        <BasicButton style_ext="hidden absolute bottom-8 left-1/2 -translate-x-1/2 md:flex flex-col items-center gap-2 text-sm opacity-60 hover:opacity-100 transition-opacity duration-300 cursor-pointer animate-bounce text-secondary">
                            <span class="text-xs tracking-widest uppercase">"More"</span>
                            <Icon icon=IconId::BsChevronDoubleDown width="2rem" height="2rem" />
                        </BasicButton>
                    </div>

                    // My work ethic
                    <section class="snap-start min-h-svh flex flex-col justify-center py-[40px]">
                        <div class="grid grid-cols-1 md:grid-cols-4 md:grid-rows-4 gap-[10px] md:auto-rows-[200px] display-constraints">

                                <div class="md:row-span-1 md:col-span-2 bg-primary text-contrast-white rounded-[5px] p-8 flex flex-col justify-center min-h-[200px]">
                                    <h2 class="text-3xl md:text-5xl font-bold mb-4 text-contrast-white">"My Work Ethic"</h2>
                                    <p class="text-lg opacity-90">"The principles that drive my work"</p>
                                </div>

                                <For
                                    each=move || ethics.clone()
                                    key=|(bg, title, _, _)| format!("{}{}", bg, title)
                                    children=move |(bg_image, title, description, span_class)| {
                                        view! {
                                            <div class={format!("row-span-1 {} relative rounded-[5px] overflow-hidden bg-cover bg-center group cursor-pointer min-h-[200px] {}", span_class, bg_image)}>
                                                <div class="absolute inset-0 bg-black/40 group-hover:bg-black/60 transition-colors duration-300"></div>
                                                <div class="relative h-full p-4 md:p-6 flex flex-col justify-end text-contrast-white">
                                                    <h3 class="text-lg md:text-xl font-bold mb-2 text-contrast-white">{title}</h3>
                                                    <p class="text-xs md:text-sm opacity-0 group-hover:opacity-100 transition-opacity duration-300">{description}</p>
                                                </div>
                                            </div>
                                        }
                                    }
                                />
                            </div>
                    </section>

                    <div class="snap-start min-h-svh flex flex-col bg-primary">
                        <div class="flex-1 relative overflow-hidden min-h-[400px] border-b-[1px] border-contrast-white display-constraints">
                            {/* Background image — full width */}
                            <div
                                class="absolute inset-0 bg-cover bg-center"
                                style="background-image: url('http://localhost:3001/view/e932cec5-3e8a-4b79-9f4a-0ec3af066d50?width=1500');"
                            />

                            {/* Primary color — left half with diagonal right edge */}
                            <div
                                class="absolute inset-0 bg-primary"
                                style="clip-path: polygon(0 0, 60% 0, 40% 100%, 0 100%);"
                            />

                            {/* Content — spans full width, centered over both halves */}
                            <div class="relative z-10 flex flex-col gap-[40px] items-center justify-center h-full p-[10%]">
                                <p class="text-owner-name text-contrast-white text-center">
                                    "Let's turn that technical idea you have in mind into reality. Shall we?"
                                </p>
                                <A
                                    attr:class="py-2 px-4 cursor-pointer rounded-[5px] border-2 border-contrast-white text-contrast-white hover:bg-contrast-white hover:text-primary font-bold"
                                    href="/ratecard"
                                >
                                    "Request Service"
                                </A>
                            </div>
                        </div>
                        <div class="mt-auto">
                            <Footer />
                        </div>
                    </div>
                </div>
            </div>
        </main>
    }
}
