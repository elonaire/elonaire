use icondata as IconId;
use leptos::html::*;
use leptos::prelude::*;
use leptos_icons::Icon;

#[component]
pub fn Accordion(
    title: String,
    #[prop(optional)] children: Option<Children>,
    #[prop(into)] icon: ViewFn,
) -> impl IntoView {
    let (is_open, set_is_open) = signal(false);

    let toggle_content = move |_| set_is_open.update(|value| *value = !*value);

    view! {
        <div>
            <span
                on:click=toggle_content
                class="flex flex-row items-center justify-between gap-2 mb-2 p-2 rounded cursor-pointer hover:bg-primary hover:text-white"
            >
                <span class="flex flex-row items-center gap-2">
                    { icon.run() }
                    <span>{title}</span>
                </span>
                {
                    move || {
                        if is_open.get() {
                            view!{ <Icon icon=IconId::BsDashLg /> }
                        } else {
                            view!{ <Icon icon=IconId::BsPlusLg /> }
                        }
                    }
                }
            </span>
            <div
                class=move || {
                    if is_open.get() {
                        "transition-max-height duration-700 ease-in-out overflow-hidden max-h-svh p-2 ml-2"
                    } else {
                        "overflow-hidden h-0 transition-max-height duration-700 ease-in-out"
                    }
                }
            >
                {children.map(|c| c())}
            </div>
        </div>
    }
}
