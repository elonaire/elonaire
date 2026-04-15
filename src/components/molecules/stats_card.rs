use icondata as IconData;
use icondata::Icon as IconId;
use leptos::prelude::*;
use leptos_icons::Icon;

#[component]
pub fn StatsCard(
    #[prop(into)] color: String,
    #[prop(into)] title: String,
    #[prop(into)] figures: RwSignal<u32>,
    icon: IconId,
    #[prop(into)] percentage: RwSignal<i32>,
) -> impl IntoView {
    let trend_color = Memo::new(move |_| {
        if percentage.get() > 0 {
            String::from("success")
        } else {
            String::from("danger")
        }
    });

    let trend_icon = Memo::new(move |_| {
        if percentage.get() > 0 {
            IconData::FiTrendingUp
        } else {
            IconData::FiTrendingDown
        }
    });

    view! {
        <div class="flex flex-col gap-[10px] p-[10px] rounded-[5px] shadow-sm bg-white dark:bg-navy-light md:grow">
            <div class="flex flex-row justify-between h-[45px]">
                <div class=format!("w-[45px] h-full flex items-center justify-center rounded-[5px] text-{color} bg-{color}/20")>
                    <Icon width="24" height="24" icon=icon />
                </div>

                <div class=move || format!("flex flex-row items-center gap-[10px] text-{}", trend_color.get())>
                    <Icon width="24" height="24" icon=trend_icon />
                    <span>{move || percentage.get()}%</span>
                </div>
            </div>
            <p class="font-extrabold text-3xl">{move || figures.get()}</p>
            <p>{title}</p>
        </div>
    }
}
