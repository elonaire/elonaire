use icondata as IconId;
use leptos::ev;
use leptos::prelude::*;
use leptos_icons::Icon;

#[component]
pub fn Nav(
    #[prop(optional, default = Callback::new(|_| {}))] onmenuclick: Callback<ev::MouseEvent>,
) -> impl IntoView {
    view! {
        <>
            <div class="md:hidden flex mx-[20px] justify-between h-[47px]">
                <button
                    class="bg-transparent border-none"
                    on:click=move |ev| onmenuclick.run(ev)
                >
                    <Icon width="24" height="24" icon=IconId::IoMenu />
                </button>
                <img src="http://localhost:3001/view/54d31727-8115-445f-b9fb-5b2457d47c41" class="w-[47px] object-cover" alt="Logo" />
                <div class="flex items-center justify-end gap-[20px]">
                    <Icon width="24" height="24" icon=IconId::IoSearchOutline />
                    <img src="http://localhost:3001/view/46fdfb66-5bbb-4690-b3f3-b1b5e936ed5a" class="size-[27px] object-cover rounded-full" alt="dp" />
                </div>
            </div>
        </>
    }
}
