use icondata as IconId;
use leptos::ev;
use leptos::prelude::*;
use leptos_icons::Icon;
use leptos_router::hooks::use_location;

use leptos_router::components::A;

#[component]
pub fn Nav(
    #[prop(optional, default = Callback::new(|_| {}))] onmenuclick: Callback<ev::MouseEvent>,
) -> impl IntoView {
    let location = use_location();

    let is_dashboard = Memo::new(move |_| location.pathname.get().contains("/dashboard"));
    let is_blog = Memo::new(move |_| location.pathname.get().contains("/blog"));

    view! {
        <>
        <div class="flex mx-[5%] md:mx-[10%] justify-between h-[47px]">
                <button
                    class="bg-transparent border-none cursor-pointer"
                    on:click=move |ev| onmenuclick.run(ev)
                >
                    <Icon width="24" height="24" icon=IconId::IoMenu />
                </button>
                <img src="http://localhost:3001/view/114aa7a5-66a5-4e72-aa10-6c316b05a001" class="w-[47px] object-cover" alt="Logo" />
                <div class="flex items-center justify-end gap-[20px]">
                    { move ||
                        if is_dashboard.get() || is_blog.get() {
                            Some(view! {
                                <>
                                    <Icon width="24" height="24" icon=IconId::IoSearchOutline />
                                    <img src="http://localhost:3001/view/e564672d-04ef-4be8-84b7-067f98494f1e" class="size-[27px] object-cover rounded-full" alt="dp" />
                                </>
                            })
                        } else {
                            None
                        }
                    }
                    { move ||
                        if !is_dashboard.get() && !is_blog.get() {
                            Some(view! {
                                <A attr:class="py-2 px-4 cursor-pointer rounded-[5px] border-2 border-primary text-primary hover:bg-primary hover:text-white font-bold" href="/ratecard">"Request Service"</A>
                            })
                        } else {
                            None
                        }
                    }
                </div>
            </div>
        </>
    }
}
