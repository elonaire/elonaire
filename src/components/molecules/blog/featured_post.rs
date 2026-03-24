use icondata::{BiStarRegular, BsChevronRight};
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
        <div class="relative h-[310px] md:h-[499px] md:flex md:flex-row-reverse md:gap-[20px]">
            {/* Image */}
            <img
                src=thumbnail
                class="w-full h-[252px] object-cover rounded-[5px] md:h-full md:w-1/2 md:rounded-l-none"
            />

            {/* Featured Badge */}
            <div class="absolute top-[9px] left-[20px] flex items-center justify-center py-[5px] w-[175px] bg-light-gray gap-[10px] rounded-[5px] font-semibold md:top-0 md:left-0 md:right-auto z-10 text-primary">
                <Icon icon=BiStarRegular />
                <p>"Featured Post"</p>
            </div>

            {/* Content Card */}
            <div class="absolute top-[105px] left-[20px] flex flex-col gap-[10px] px-[18px] w-[333px] h-[205px] bg-primary/95 rounded-[5px] md:relative md:top-0 md:left-0 md:w-1/2 md:h-full md:bg-transparent md:px-0 md:gap-[20px]  md:justify-center">
                <A href={format!("/blog/read/{}", link)}>
                    <h1 class="line-clamp-2 md:line-clamp-3">{title}</h1>
                </A>
                <p class="line-clamp-2 md:line-clamp-3">{short_description}</p>
                <AuthorInfo profile_pic=author_profile_pic author_name=author_name />
                <A
                    href={format!("/blog/read/{}", link)}
                    attr:class="hidden md:flex font-bold py-2 px-4 cursor-pointer rounded-[5px] bg-primary text-contrast-white w-[246px] justify-center items-center gap-[10px]"
                >
                    <span>"Read Full Article"</span>
                    <Icon icon=BsChevronRight />
                </A>
            </div>
        </div>
    }
}
