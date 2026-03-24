use leptos::{ev, prelude::*};
use leptos_router::components::A;

use crate::components::molecules::{footer::Footer, nav::Nav};

#[derive(Clone)]
pub struct Attribution {
    pub title: &'static str,
    pub author: &'static str,
    pub source: &'static str,
    pub url: &'static str,
    pub license: &'static str,
    pub license_url: &'static str,
    pub modifications: Option<&'static str>,
}

#[component]
pub fn Attributions() -> impl IntoView {
    let (collapsed, set_collapsed) = signal(false);

    let attributions: Vec<(&'static str, Vec<Attribution>)> = vec![
        (
            "Icons",
            vec![Attribution {
                title: "Icondata",
                author: "Carlos Kiki & Contributors",
                source: "icondata",
                url: "https://carloskiki.github.io/icondata/",
                license: "MIT License",
                license_url: "https://github.com/carloskiki/icondata/blob/main/LICENSE",
                modifications: None,
            }],
        ),
        (
            "Fonts",
            vec![Attribution {
                title: "Inter",
                author: "Rasmus Andersson",
                source: "Google Fonts",
                url: "https://fonts.googleapis.com/css2?family=Inter:ital,opsz,wght@0,14..32,100..900;1,14..32,100..900&display=swap",
                license: "SIL Open Font License 1.1",
                license_url: "https://scripts.sil.org/OFL",
                modifications: None,
            }],
        ),
        (
            "Images & Photography",
            vec![Attribution {
                title: "Security Stock Photos",
                author: "Various Contributors",
                source: "Vecteezy",
                url: "https://www.vecteezy.com/free-photos/security",
                license: "Vecteezy License",
                license_url: "https://www.vecteezy.com/licensing-agreement",
                modifications: None,
            }],
        ),
    ];

    let handle_menu_click =
        move || Callback::new(move |_ev: ev::MouseEvent| set_collapsed.set(true));

    view! {
        <div class="min-h-svh flex flex-col gap-[40px]">
            <Nav onmenuclick=handle_menu_click() />

            // Header
            <div class="display-constraints blog-display-constraints border-b border-light-gray pb-6">
                <span class="inline-block text-xs tracking-[0.3em] uppercase text-primary mb-3">
                    "Credits"
                </span>
                <h1>"Attributions"</h1>
                <p class="text-caption mt-2">
                    "This page credits all third-party media, icons, fonts, and other resources used on this website."
                </p>
            </div>

            <p class="display-constraints blog-display-constraints text-body">
                "Techie Tenka makes use of free and open resources from the community. "
                "We are grateful to all creators whose work makes this site possible. "
                "All resources are used in compliance with their respective licenses."
            </p>

            // Sections
            <div class="display-constraints blog-display-constraints space-y-10 pb-20">
                {attributions.into_iter().map(|(category, items)| view! {
                    <div>
                        <h4 class="mb-4 border-b border-light-gray pb-2">{category}</h4>
                        <div class="space-y-4">
                            {items.into_iter().map(|attr| view! {
                                <div class="flex gap-4">
                                    <div class="w-1 bg-primary rounded-full shrink-0 mt-1"></div>
                                    <div class="flex flex-col gap-1">
                                        <p class="text-sm font-semibold">{attr.title}</p>
                                        <p class="text-body">
                                            "By "
                                            <span class="font-medium">{attr.author}</span>
                                            " via "
                                            <A
                                                href=attr.url
                                                target="_blank"
                                                attr:class="text-primary hover:underline"
                                            >
                                                {attr.source}
                                            </A>
                                        </p>
                                        <p class="text-caption">
                                            "License: "
                                            <A
                                                href=attr.license_url
                                                target="_blank"
                                                attr:class="text-primary hover:underline"
                                            >
                                                {attr.license}
                                            </A>
                                        </p>
                                        {attr.modifications.map(|note| view! {
                                            <p class="text-caption italic">
                                                "Modifications: " {note}
                                            </p>
                                        })}
                                    </div>
                                </div>
                            }).collect::<Vec<_>>()}
                        </div>
                    </div>
                }).collect::<Vec<_>>()}
            </div>

            <div class="mt-auto">
                <Footer />
            </div>
        </div>
    }
}
