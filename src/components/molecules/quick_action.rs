use icondata as IconData;
use icondata::Icon as IconId;
use leptos::prelude::*;
use leptos_icons::Icon;

#[component]
pub fn QuickAction(
    #[prop(into)] title: String,
    #[prop(into)] description: String,
    #[prop(into)] color: String,
    icon: IconId,
) -> impl IntoView {
    view! {
        <div class="flex flex-row items-center justify-between h-[45px]">
            <div class="flex items-center gap-[10px]">
                <div class=format!("w-[45px] h-[45px] flex items-center justify-center rounded-[5px] text-{color} bg-{color}/20")>
                    <Icon width="24" height="24" icon=icon />
                </div>
                <div class="flex flex-col">
                    <p>{title}</p>
                    <p class="text-small">{description}</p>
                </div>
            </div>

            <div class="text-gray">
                <Icon width="24" height="24" icon=IconData::BsChevronRight />
            </div>
        </div>
    }
}
