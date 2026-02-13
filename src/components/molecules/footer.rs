use chrono::Datelike;
use leptos::prelude::*;
use leptos_router::components::A;

#[component]
pub fn Footer() -> impl IntoView {
    view! {
        <footer class="bg-primary text-contrast-white">
            <div class="mx-[5%] md:mx-[10%] flex flex-col gap-[20px] justify-between py-[15px]">
                <div class="flex flex-col gap-[20px]">
                    <div class="flex flex-col gap-[20px] items-center">
                        <img src="http://localhost:3001/view/b56df055-5f8c-45c8-98ab-eab20c164bd5" class="brightness-300 w-[192px]" alt="Techie Tenka Logo" />
                        <p>"Welcome to my personal blog that majorly covers topics in Software Engineering and also a space where I can speak my mind out on anything and everything."</p>
                    </div>
                    <div class="flex flex-col gap-[10px]">
                        <h4 class="text-contrast-white">"Support"</h4>
                        <A href="/contact">"Contact"</A>
                        <A href="/faq">"FAQ"</A>
                    </div>
                    <div class="flex flex-col gap-[10px]">
                        <h4 class="text-contrast-white">"Learn More"</h4>
                        <A href="/pricing">"Pricing"</A>
                        <A href="/about">"About"</A>
                    </div>
                    <div class="flex flex-col gap-[10px]">
                        <h4 class="text-contrast-white">"Legal"</h4>
                        <A href="/privacy">"Privacy Policy"</A>
                        <A href="/terms">"Terms of Service"</A>
                    </div>
                </div>
                <div class="text-xs border-t border-contrast-white flex items-center justify-center h-[88px]">
                    <span>"©"{format!(" {} Techie Tenka. All rights reserved.", chrono::Local::now().year()).to_uppercase()}</span>
                </div>
            </div>
        </footer>
    }
}
