use chrono::Datelike;
use leptos::prelude::*;
use leptos_router::components::A;

#[component]
pub fn Footer() -> impl IntoView {
    view! {
        <footer class="bg-primary text-contrast-white">
            <div class="display-constraints flex flex-col gap-[20px] justify-between py-[15px]">
                <div class="flex flex-col gap-[20px] md:flex-row md:gap-[40px]">
                    <div class="flex flex-col gap-[20px] items-center md:items-start max-w-[319px]">
                        <img src="https://api.techietenka.com/files/view/5e8cfbe8-8380-49e1-814e-581dea3dde8b" class="brightness-300 w-[192px]" alt="Techie Tenka Logo" />
                        <p class="md:text-left">"Welcome to my personal blog that majorly covers topics in Software Engineering and also a space where I can speak my mind out on anything and everything."</p>
                    </div>
                    <div class="flex flex-col gap-[10px]">
                        <h5 class="text-contrast-white">"Support"</h5>
                        <A href="/contact" attr:class="text-sm">"Contact"</A>
                        <A href="/faq" attr:class="text-sm">"FAQ"</A>
                    </div>
                    <div class="flex flex-col gap-[10px]">
                        <h5 class="text-contrast-white">"Learn More"</h5>
                        <A href="/ratecard" attr:class="text-sm">"My Ratecard"</A>
                        <A href="/about" attr:class="text-sm">"About"</A>
                    </div>
                    <div class="flex flex-col gap-[10px]">
                        <h5 class="text-contrast-white">"Legal"</h5>
                        <A href="/privacy" attr:class="text-sm">"Privacy Policy"</A>
                        <A href="/terms" attr:class="text-sm">"Terms of Service"</A>
                        <A href="/attributions" attr:class="text-sm">"Attributions"</A>
                    </div>
                </div>
                <div class="text-xs border-t border-contrast-white flex items-center justify-center h-[88px]">
                    <span>"©"{format!(" {} Techie Tenka. All rights reserved.", chrono::Local::now().year()).to_uppercase()}</span>
                </div>
            </div>
        </footer>
    }
}
