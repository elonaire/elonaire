use icondata as IconId;
use leptos::prelude::*;
use leptos_icons::Icon;
use leptos_router::components::A;

use crate::components::molecules::blog::author_info::AuthorInfo;

#[component]
pub fn FeaturedPost(
    #[prop(into)] thumbnail: String,
    #[prop(into)] title: String,
    #[prop(into)] short_description: String,
    #[prop(into)] link: String,
    #[prop(into)] author_profile_pic: String,
    #[prop(into)] author_name: String,
) -> impl IntoView {
    view! {
        <div class="h-[310px] relative opacity-95 text-primary">
            <img src=thumbnail class="w-full h-[252px] object-cover rounded-[5px]" />
            <div class="flex items-center justify-center py-[5px] absolute top-[9px] left-[20px] w-[175px] bg-light-gray gap-[10px] rounded-[5px] font-semibold">
                <Icon icon=IconId::BiStarRegular />
                <p>Featured Post</p>
            </div>
            <div class="flex flex-col gap-[10px] absolute top-[105px] left-[20px] bg-primary rounded-[5px] px-[18px] w-[333px] h-[205px] opacity-95">
                <A href={format!("/blog/read/{}", link)}><h1 class="line-clamp-2 text-contrast-white">{title}</h1></A>
                <p class="line-clamp-2 text-contrast-white">{short_description}</p>
                <AuthorInfo profile_pic=author_profile_pic author_name=author_name />
            </div>
        </div>
    }
}
