// faq.rs
use leptos::{ev, prelude::*};

use crate::components::{
    general::collapse::{Collapse, PanelInfo},
    molecules::nav::Nav,
};

#[component]
pub fn Faqs() -> impl IntoView {
    let (collapsed, set_collapsed) = signal(false);

    let raw_faqs = vec![
        (
            "General",
            vec![
                (
                    "What is Techie Tenka?",
                    "Techie Tenka is a blog and upcoming marketplace focused on IoT, software engineering, Rust, Leptos, and the intersection of technology and real-world problem solving.",
                ),
                (
                    "Who is this for?",
                    "Anyone who builds things — embedded developers, Rust enthusiasts, Leptos developers, IoT hobbyists, and curious minds who want to ship real products.",
                ),
                (
                    "Is the blog free to read?",
                    "Yes. All blog articles are completely free. The marketplace will have both free and paid products.",
                ),
            ],
        ),
        (
            "Marketplace",
            vec![
                (
                    "What will be available in the marketplace?",
                    "IoT product templates, firmware blueprints, Leptos UI components, full-stack starter kits, end-to-end project guides, and Ebooks on embedded systems, Rust, and systems design.",
                ),
                (
                    "When does the marketplace launch?",
                    "We are targeting Q3 2025. Join the waitlist to be notified the moment we go live.",
                ),
                (
                    "What license do purchased products come with?",
                    "Each product specifies its license at the point of purchase — typically personal use or commercial use. Redistribution or resale is not permitted unless explicitly stated.",
                ),
                (
                    "Can I sell my own templates or products?",
                    "Vendor accounts are planned for a future phase. Join the waitlist and indicate your interest — we'll reach out when vendor onboarding opens.",
                ),
                (
                    "What payment methods will be supported?",
                    "We plan to support major credit/debit cards and mobile money options including M-Pesa at launch.",
                ),
            ],
        ),
        (
            "Technical",
            vec![
                (
                    "What stack is this platform built on?",
                    "The platform is built with Rust, Leptos for the frontend, and Axum for the backend — fully type-safe from database to browser.",
                ),
                (
                    "Do the IoT templates support specific hardware?",
                    "Templates will specify supported hardware. At launch expect coverage for ESP32, Raspberry Pi, and Arduino-compatible boards.",
                ),
                (
                    "Are the Leptos templates compatible with the latest version?",
                    "Yes. All Leptos templates are maintained against the latest stable release.",
                ),
            ],
        ),
        (
            "Support",
            vec![
                (
                    "How do I report an issue with a product?",
                    "Use the contact form or email support@techietenka.com. We aim to respond within 48 hours.",
                ),
                (
                    "What is your refund policy?",
                    "Refund requests must be submitted within 7 days of purchase if the product materially differs from its description. See our Terms of Service for full details.",
                ),
            ],
        ),
    ];

    // Flatten all sections into PanelInfo, with a section heading as a non-interactive divider
    // between groups by using separate Collapse per section
    let sections: Vec<(String, RwSignal<Vec<PanelInfo>>)> =
        raw_faqs
            .into_iter()
            .map(|(category, items)| {
                let panel_items =
                    items
                        .into_iter()
                        .map(|(question, answer)| {
                            let q = question.to_string();
                            let a = answer.to_string();
                            PanelInfo::builder(
                        ViewFn::from(move || view! {
                            <span class="text-sm font-medium text-gray">{q.clone()}</span>
                        }),
                        ViewFn::from(move || view! {
                            <p class="text-body text-mid-gray">{a.clone()}</p>
                        }),
                    )
                    .build()
                        })
                        .collect::<Vec<_>>();
                (category.to_string(), RwSignal::new(panel_items))
            })
            .collect();

    let handle_menu_click =
        move || Callback::new(move |_ev: ev::MouseEvent| set_collapsed.set(true));

    view! {
        <div class="min-h-svh bg-contrast-white flex flex-col gap-[40px]">
            <Nav onmenuclick=handle_menu_click() />

            // Header
            <div class="display-constraints blog-display-constraints border-b border-light-gray pb-8">
                <span class="inline-block text-xs tracking-[0.3em] uppercase text-primary mb-3">
                    "Help"
                </span>
                <h1>"Frequently Asked Questions"</h1>
                <p class="text-body text-mid-gray mt-2">
                    "Can't find an answer? Email us at "
                    <a href="mailto:info@techietenka.com" class="text-primary hover:underline">
                        "info@techietenka.com"
                    </a>
                </p>
            </div>

            // FAQ sections
            <div class="display-constraints blog-display-constraints pb-20">
                {sections.into_iter().map(|(category, panel_items)| view! {
                    <div class="mb-8">
                        <h6 class="text-primary mb-4 uppercase tracking-widest">{category}</h6>
                        <Collapse is_accordion=true panel_items=panel_items />
                    </div>
                }).collect::<Vec<_>>()}
            </div>
        </div>
    }
}
