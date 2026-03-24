use icondata::CgClose;
use leptos::prelude::*;
use leptos_icons::Icon;
use leptos_router::components::A;

#[component]
pub fn TopNav() -> impl IntoView {
    view! {
        <>
        <div class="flex display-constraints justify-end h-[55px] md:h-[65px]">
                <A
                    attr:class="bg-transparent border-none cursor-pointer text-secondary"
                    href="/"
                >
                    <Icon width="3rem" height="3rem" icon=CgClose />
                </A>
            </div>
        </>
    }
}
