use std::collections::HashMap;

use icondata::{
    BsChevronDoubleDown, BsGithub, BsInfoCircle, BsMoon, BsSun, IoClose, MdiCertificateOutline,
    MdiStore, MdiTrophyAward, RiArticleDocumentLine,
};
use leptos::{ev, prelude::*, task::spawn_local};
use leptos_icons::Icon;
use leptos_meta::*;
use leptos_router::components::A;
use reactive_stores::Store;

use crate::{
    components::{
        forms::toggle_switch::ToggleSwitch,
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

const SHARED_SERVICE_API: Option<&str> = option_env!("SHARED_SERVICE_API");

#[component]
pub fn Home() -> impl IntoView {
    // track collapsed state
    let current_state = expect_context::<Store<AppStateContext>>();
    let (collapsed, set_collapsed) = signal(false);
    let (is_loading, set_is_loading) = signal(false);
    let (professions, set_professions) = signal(Vec::new() as Vec<UserProfessionalInfo>);
    // State: index of the currently selected profession (default to first)
    let (selected_profession, set_selected_profession) = signal(String::new());

    let site_owner_info = move || current_state.site_owner_info(); // Should return ReadSignal<UserInfo>
    let dark_mode_is_active = current_state.dark_mode_is_active();
    let dark_mode_signal = Signal::derive(move || dark_mode_is_active.get());
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
            MenuItem::new("About", BsInfoCircle, "/about", vec![]),
            MenuItem::new("Resume", MdiCertificateOutline, "/resume", vec![]),
            MenuItem::new("Portfolio", MdiTrophyAward, "/portfolio", vec![]),
            MenuItem::new("Marketplace", MdiStore, "/marketplace", vec![]),
            MenuItem::new("Blog", RiArticleDocumentLine, "/blog", vec![]),
        ]
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

    let ethics = vec![
        (
            "bg-[url('https://api.techietenka.com/files/view/485ce03d-de84-499e-9696-b6e605c02eec?width=800')]",
            "Commitment to Security",
            "From physical to application - I ensure security at every layer.",
            "md:row-span-1 md:col-span-1", // ethic1: spans 1 cols, 1 row
        ),
        (
            "bg-[url('https://api.techietenka.com/files/view/a7641914-73a2-4747-9728-8fd74177259c?width=800')]",
            "Commitment to Quality",
            "I never compromise on delivering exceptional results",
            "md:row-span-1 md:col-span-1", // ethic1: spans 1 cols, 1 row
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

            let Some(shared_service_api) = SHARED_SERVICE_API else {
                return;
            };

            let fetch_professions_response =
                perform_query_without_vars::<FetchSiteResourcesResponse>(
                    None,
                    shared_service_api,
                    fetch_professions_query,
                )
                .await;

            let _site_owner_info = fetch_site_owner_info(&current_state, None).await;

            match fetch_professions_response.get_data() {
                Some(data) => {
                    let professions = data
                        .fetch_site_resources
                        .as_ref()
                        .unwrap_or(&Default::default())
                        .get_data()
                        .professional_info
                        .as_ref()
                        .unwrap_or(&Default::default())
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
                                        <div class="flex rounded-[5px] hover:bg-light-gray h-[45px]" on:click=move |_| set_collapsed.set(false)>
                                            <A attr:class="flex-1 h-full flex items-center gap-[10px]" href=child.path>
                                                <span><Icon width="24" height="24" icon=child.icon /></span>
                                                <span class="flex-1">{child.label}</span>
                                            </A>
                                        </div>
                                    </For>
                                </nav>
                                <div class="md:hidden flex flex-col gap-[10px]">
                                    <div class="flex items-center h-[40px] border-b border-light-gray">
                                        <p class="font-medium text-xs">ACTIONS</p>
                                    </div>
                                    <A attr:class="block md:hidden py-2 px-4 cursor-pointer rounded-[5px] border-2 border-primary text-primary hover:bg-primary hover:text-contrast-white font-bold text-center" href="/ratecard">"Request Service"</A>
                                </div>
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
                <div class="h-svh overflow-y-scroll snap-y snap-mandatory">
                    <div class="snap-start min-h-svh flex flex-col gap-[40px] pb-[40px] relative">
                        {/* Toggle button (opens sidebar) */}
                        <Nav onmenuclick=handle_menu_click() />


                        // My User Info
                        <div class="flex flex-col md:flex-row gap-[20px] md:justify-between md:items-center display-constraints">
                            <div class="flex flex-col gap-[20px] md:gap-[40px] text-center md:text-left">
                                <p class="text-salutation">Hello, I am</p>
                                <p class="text-owner-name dark:text-light-gray"><span class="text-primary">{move || site_owner_info().get().first_name}</span>{move || format!(" {} {}", site_owner_info().get().middle_name.unwrap_or_default(), site_owner_info().get().last_name.unwrap_or_default())}</p>
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
                                // <BasicButton button_text="Download my resume" icon=Some(FiDownload) icon_before=false style_ext="bg-primary text-contrast-white md:w-[292px]" />
                                <A attr:class="font-bold py-2 px-4 cursor-pointer rounded-[5px] bg-primary text-contrast-white md:w-[292px] flex items-center justify-center gap-2" href={move || site_owner_info().get()
                                        .socials
                                        .as_ref()
                                        .and_then(|socials| socials.iter().find(|s| s.name.to_lowercase() == "github"))
                                        .and_then(|social| Some(social.url.clone())).unwrap_or_default()} target="_blank">
                                    <span>"Checkout my GitHub"</span>
                                    <span><Icon width="24" height="24" icon=BsGithub /></span>
                                </A>
                            </div>
                            <div class="hidden md:block w-[50%]">
                                // Define the SVG clip path
                                <svg width="0" height="0" class="absolute">
                                    <defs>
                                        <clipPath id="blob-clip" clipPathUnits="objectBoundingBox">
                                            <path d="
                                                M 0.15,0.05
                                                C 0.3,-0.08 0.55,-0.02 0.7,0.04
                                                C 0.85,0.1 1.02,0.18 0.98,0.35
                                                C 0.94,0.5 1.06,0.62 0.97,0.75
                                                C 0.88,0.88 0.72,1.04 0.55,0.99
                                                C 0.38,0.94 0.25,1.06 0.12,0.97
                                                C -0.02,0.88 -0.06,0.72 0.03,0.58
                                                C 0.1,0.45 -0.04,0.32 0.04,0.2
                                                C 0.08,0.12 0.05,0.14 0.15,0.05
                                                Z
                                            " />
                                        </clipPath>
                                    </defs>
                                </svg>
                                <div class="bg-contrast-white dark:bg-transparent rounded-[5px]">
                                    <img
                                        alt="dp"
                                        src={move || site_owner_info().get().profile_picture}
                                        class="object-cover w-full h-auto mix-blend-multiply dark:mix-blend-normal"
                                        style="clip-path: url(#blob-clip);"
                                    />
                                </div>
                            </div>
                        </div>

                        {/* Scroll button */}
                        <BasicButton style_ext="hidden absolute bottom-8 left-1/2 -translate-x-1/2 md:flex flex-col items-center gap-2 text-sm opacity-60 hover:opacity-100 transition-opacity duration-300 cursor-pointer animate-bounce text-secondary">
                            <span class="text-xs tracking-widest uppercase">"Scroll"</span>
                            <Icon icon=BsChevronDoubleDown width="2rem" height="2rem" />
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
                                            <div class=format!("row-span-1 {} relative rounded-[5px] overflow-hidden group cursor-pointer min-h-[200px]", span_class)>
                                                // Background image — scales on hover independently
                                                <div
                                                    class=format!("absolute inset-0 bg-cover bg-center transition-transform duration-500 group-hover:scale-110 {}", bg_image)
                                                ></div>
                                                // Overlay
                                                <div class="absolute inset-0 bg-black/40 group-hover:bg-black/60 transition-colors duration-300"></div>
                                                // Content
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
                                class="absolute inset-0 bg-cover bg-center rounded-[5px]"
                                style="background-image: url('https://api.techietenka.com/files/view/be829ce6-1c68-4c88-8491-9e0df8817967?width=1500');"
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
