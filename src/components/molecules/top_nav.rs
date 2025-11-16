use icondata as IconId;
use leptos::ev;
use leptos::prelude::*;
use leptos_icons::Icon;

#[component]
pub fn TopNav(
    #[prop(optional, default = Callback::new(|_| {}))] onmenuclick: Callback<ev::MouseEvent>,
) -> impl IntoView {
    view! {
        <>
        <div class="flex mx-[5%] md:mx-[10%] justify-end h-[55px] md:h-[65px]">
                <button
                    class="bg-transparent border-none cursor-pointer text-secondary"
                    on:click=move |ev| onmenuclick.run(ev)
                >
                    <Icon width="3rem" height="3rem" icon=IconId::CgClose />
                </button>
            </div>
        </>
    }
}
