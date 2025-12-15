use leptos::prelude::*;

#[component]
pub fn Headline(#[prop(into)] title: String, #[prop(into)] description: String) -> impl IntoView {
    view! {
        <div class="flex flex-col items-center justify-center gap-[20px]">
            <h3 class="text-primary">{description}</h3>
            <p class="text-4xl md:text-5xl text-light-gray">{title}</p>
            <div class="flex flex-row">
                <div class="w-[100px] h-[5px] border-t-2 border-primary" />
            </div>
        </div>
    }
}
