use leptos::prelude::*;
use leptos_router::components::A;

use crate::{
    components::molecules::blog::blog_post_metadata::BlogPostMetadata,
    data::models::graphql::shared::BlogCategory,
};

#[component]
pub fn BlogPostPreview(
    #[prop(into)] thumbnail: String,
    #[prop(into)] title: String,
    #[prop(into)] short_description: String,
    #[prop(into)] category: BlogCategory,
    #[prop(into)] read_time: u32,
    #[prop(into)] link: String,
) -> impl IntoView {
    view! {
        <div class="flex h-[136px]  h-[111px] gap-[5px] border-b-[1px] border-light-gray">
            <img src=thumbnail class="w-[106px] object-cover" alt="Blog Post Image" />
            <div class="flex-1 flex flex-col justify-between">
                <A href={format!("/blog/read/{}", link)}><h5 class="line-clamp-2">{title}</h5></A>
                <p class="text-sm line-clamp-2">{short_description}</p>
                <BlogPostMetadata category=category read_time=read_time />
            </div>
        </div>
    }
}
