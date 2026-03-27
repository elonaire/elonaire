// privacy.rs
use leptos::{ev, prelude::*};

use crate::components::molecules::{footer::Footer, nav::Nav};

#[component]
pub fn PrivacyPolicy() -> impl IntoView {
    let (collapsed, set_collapsed) = signal(false);
    let sections = vec![
        (
            "1. Information We Collect",
            vec![
                (
                    "Account Information",
                    "When you create an account, we collect your name, email address, and password hash.",
                ),
                (
                    "Purchase Information",
                    "We collect limited billing information necessary to process your transactions, such as your name, email address, and purchase details.

                    Payments are processed securely by our third-party payment provider, Paystack. We do not collect or store your full payment card details. Your payment information is handled directly by Paystack in accordance with their privacy and security policies.",
                ),
                (
                    "Usage Data",
                    "We collect anonymized data on how you interact with our platform — pages visited, features used, and session duration - to improve our services.",
                ),
                (
                    "Communications",
                    "If you contact us, we retain the contents of your message and your contact details to respond and maintain records.",
                ),
            ],
        ),
        (
            "2. How We Use Your Information",
            vec![
                (
                    "Service Delivery",
                    "To process purchases, deliver products, and manage your account.",
                ),
                (
                    "Communications",
                    "To send transactional emails (receipts, product updates) and, with your consent, marketing communications.",
                ),
                (
                    "Improvement",
                    "To analyze usage patterns and improve the platform, fix bugs, and develop new features.",
                ),
                (
                    "Legal Compliance",
                    "To comply with applicable laws and respond to lawful requests from authorities.",
                ),
            ],
        ),
        (
            "3. Information Sharing",
            vec![
                (
                    "Third Parties",
                    "We do not sell your personal data. We share data only with trusted service providers (payment processors, email providers) under strict data processing agreements.",
                ),
                (
                    "Legal Requirements",
                    "We may disclose information if required by law or to protect the rights, property, or safety of Techie Tenka or our users.",
                ),
            ],
        ),
        (
            "4. Data Retention",
            vec![
                (
                    "Account Data",
                    "We retain your account data for as long as your account is active. You may request deletion at any time.",
                ),
                (
                    "Purchase Records",
                    "Purchase records are retained for 7 years for accounting and legal compliance purposes.",
                ),
            ],
        ),
        (
            "5. Your Rights",
            vec![
                (
                    "Access & Correction",
                    "You have the right to access and correct your personal data at any time through your account settings.",
                ),
                (
                    "Deletion",
                    "You may request deletion of your account and associated data by contacting privacy@techietenka.com.",
                ),
                (
                    "Opt-Out",
                    "You may unsubscribe from marketing communications at any time using the link in any email we send.",
                ),
            ],
        ),
        (
            "6. Security",
            vec![
                (
                    "Measures",
                    "We use industry-standard encryption (TLS) for data in transit and bcrypt hashing for passwords. We conduct regular security reviews.",
                ),
                (
                    "Breach Notification",
                    "In the event of a data breach affecting your personal data, we will notify you within 72 hours of becoming aware.",
                ),
            ],
        ),
        (
            "7. Cookies",
            vec![(
                "Usage",
                "We use essential cookies for authentication and session management. We do not use third-party tracking or advertising cookies.",
            )],
        ),
        (
            "8. Contact",
            vec![(
                "Privacy Inquiries",
                "For any privacy-related questions or requests, contact us at privacy@techietenka.com. We aim to respond within 30 days.",
            )],
        ),
    ];

    let handle_menu_click =
        move || Callback::new(move |_ev: ev::MouseEvent| set_collapsed.set(true));

    view! {
        <div class="min-h-svh flex flex-col gap-[40px]">
            <Nav onmenuclick=handle_menu_click() />
            // Header
            <div class="display-constraints blog-display-constraints border-b border-light-gray">
                <span class="inline-block text-xs tracking-[0.3em] uppercase text-primary mb-3">
                    "Legal"
                </span>
                <h1>"Privacy Policy"</h1>
                <p class="text-caption mt-2">"Last updated: March 2026"</p>
            </div>

            <p class="display-constraints blog-display-constraints text-body">
                "At Techie Tenka, your privacy matters. This policy explains what data we collect, why we collect it, and how we protect it."
            </p>

            // Sections
            <div class="display-constraints blog-display-constraints flex flex-col gap-[40px]">
                {sections.into_iter().map(|(section_title, items)| view! {
                    <div>
                        <h4 class="mb-4 border-b border-light-gray pb-2">{section_title}</h4>
                        <div class="space-y-4">
                            {items.into_iter().map(|(subtitle, content)| view! {
                                <div class="flex gap-4">
                                    <div class="w-1 bg-primary rounded-full shrink-0 mt-1"></div>
                                    <div>
                                        <p class="text-sm font-semibold mb-1">{subtitle}</p>
                                        <p class="text-body">{content}</p>
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
