use leptos::prelude::*;

#[component]
pub fn AuthorInfo(
    #[prop(into)] profile_pic: String,
    #[prop(into)] author_name: String,
) -> impl IntoView {
    view! {
        <div class="flex items-center gap-[10px]">
            <img src=profile_pic alt="Author Avatar" class="rounded-full size-6" />
            <p class="text-xs">{author_name}</p>
        </div>
    }
}
