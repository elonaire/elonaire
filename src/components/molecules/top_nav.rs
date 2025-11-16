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
            <div class="flex mx-[20px] md:mx-[100px] justify-end h-[47px]">
                <button
                    class="bg-transparent border-none cursor-pointer text-light-gray"
                    on:click=move |ev| onmenuclick.run(ev)
                >
                    <Icon width="24" height="24" icon=IconId::CgClose />
                </button>
            </div>
        </>
    }
}
