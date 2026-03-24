use leptos::prelude::*;

#[component]
pub fn BlogSection(#[prop(into)] title: String) -> impl IntoView {
    view! {
        <div class="flex flex-col gap-[5px] w-full">
            <p>{title.to_uppercase()}</p>
            <div class="flex items-center w-full">
                <div class="basis-1/3 h-[6px] bg-primary" />
                <div class="basis-2/3 h-[6px] border-b border-light-gray dark:border-mid-gray" />
            </div>
        </div>
    }
}
