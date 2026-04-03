// terms.rs
use leptos::prelude::*;

#[component]
pub fn TermsOfService() -> impl IntoView {
    let sections = vec![
        (
            "1. Acceptance of Terms",
            "By accessing or using Techie Tenka's marketplace, blog, or any associated services, you agree to be bound by these Terms of Service. If you do not agree, please do not use our services.",
        ),
        (
            "2. Use of Services",
            "You agree to use our services only for lawful purposes. You may not use our platform to distribute malicious code, infringe on intellectual property rights, harass other users, or engage in any activity that disrupts or damages our services.",
        ),
        (
            "3. Intellectual Property",
            "All content on this platform - including IoT products, Leptos templates, Ebooks, articles, and source code - is protected by copyright. Purchased products are licensed for personal or commercial use as specified at the point of purchase. Redistribution or resale of any product without explicit permission is prohibited.",
        ),
        (
            "4. User Accounts",
            "You are responsible for maintaining the confidentiality of your account credentials. You agree to notify us immediately of any unauthorized use of your account. We reserve the right to terminate accounts that violate these terms.",
        ),
        (
            "5. Purchases & Refunds",
            "All purchases are final unless a product is found to be materially different from its description. Refund requests must be submitted within 7 days of purchase. We reserve the right to deny refund requests that do not meet our refund criteria.",
        ),
        (
            "6. Limitation of Liability",
            "Techie Tenka is not liable for any indirect, incidental, or consequential damages arising from the use of our services or products. Our total liability shall not exceed the amount paid for the specific product or service in question.",
        ),
        (
            "7. Privacy",
            "Your use of our services is also governed by our Privacy Policy, which is incorporated into these terms by reference. Please review it carefully.",
        ),
        (
            "8. Changes to Terms",
            "We reserve the right to modify these terms at any time. We will notify users of significant changes via email or a prominent notice on our platform. Continued use of our services after changes constitutes acceptance of the new terms.",
        ),
        (
            "9. Governing Law",
            "These terms are governed by the laws of Kenya. Any disputes shall be resolved in the courts of Nairobi, Kenya.",
        ),
        (
            "10. Contact",
            "For questions about these terms, contact us at legal@techietenka.com.",
        ),
    ];

    view! {
        <div class="min-h-svh flex flex-col gap-[40px]">
            // Header
            <div class="display-constraints blog-display-constraints border-b border-light-gray">
                <span class="inline-block text-xs tracking-[0.3em] uppercase text-primary mb-3">
                    "Legal"
                </span>
                <h1>"Terms of Service"</h1>
                <p class="text-caption mt-2">"Last updated: March 2026"</p>
            </div>

            // Intro
            <p class="text-body display-constraints blog-display-constraints">
                "These Terms of Service govern your access to and use of Techie Tenka's platform, marketplace, and services. Please read them carefully before proceeding."
            </p>

            // Sections
            <div class="display-constraints blog-display-constraints flex flex-col gap-[40px]">
                {sections.into_iter().map(|(title, content)| view! {
                    <div class="border-l-2 border-primary pl-6">
                        <h4 class="mb-2">{title}</h4>
                        <p class="text-body">{content}</p>
                    </div>
                }).collect::<Vec<_>>()}
            </div>
        </div>
    }
}
