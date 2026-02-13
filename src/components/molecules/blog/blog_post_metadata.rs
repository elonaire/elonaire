use icondata as IconId;
use leptos::prelude::*;
use leptos_icons::Icon;

use crate::{
    components::molecules::blog::author_info::AuthorInfo,
    data::models::graphql::shared::BlogCategory,
    utils::time::convert_date_to_human_readable_format,
};

/// This is a component that displays metadata for a blog post.
#[component]
pub fn BlogPostMetadata(
    #[prop(into)] category: BlogCategory,
    #[prop(into)] read_time: u32,
) -> impl IntoView {
    view! {
        <div class="flex items-center gap-[16px] text-mid-gray">
            <p class="text-xs">{category.to_string()}</p><div class="size-[5px] rounded-full bg-mid-gray" /><p class="text-xs">{format!("{} min read", read_time)}</p>
        </div>
    }
}

/// This is a component that displays metadata for a blog post detail.
#[component]
pub fn BlogDetailMetadata(
    #[prop(into)] date_of_creation: String,
    #[prop(into)] read_time: u32,
    #[prop(into)] author_profile_pic: String,
    #[prop(into)] author_name: String,
) -> impl IntoView {
    let formated_creation_date = convert_date_to_human_readable_format(&date_of_creation);

    view! {
        <div class="flex items-center justify-between w-full md:gap-[20px] md:w-auto">
            <AuthorInfo author_name=author_name profile_pic=author_profile_pic />
            <div class="size-[5px] rounded-full bg-mid-gray" />
            <p class="text-xs flex items-center gap-[10px]"><span><Icon icon=IconId::BsCalendar2Date /></span><span>{formated_creation_date}</span></p>
            <div class="size-[5px] rounded-full bg-mid-gray" />
            <p class="text-xs">{format!("{} min read", read_time)}</p>
        </div>
    }
}
