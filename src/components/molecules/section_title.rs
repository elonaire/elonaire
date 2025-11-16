use leptos::prelude::*;

#[component]
pub fn SectionTitle(#[prop(into)] title: String) -> impl IntoView {
    view! {
        <div class="mx-[5%] md:mx-[10%] flex justify-center items-center border-2 border-light-gray rounded-[5px] h-[50px]">
            <h2 class="text-center text-light-gray">{title}</h2>
        </div>
    }
}
