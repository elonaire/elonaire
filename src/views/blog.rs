use std::ops::Deref;

use yew::prelude::*;

use crate::{components::{ad::AdComponent, blog::{blog_section::BlogSection, main_banner::MainBanner}, blog_nav::BlogNav, footer::Footer, line_separator::LineSeparator}, data::{context::blog::get_blog_posts, models::blog::BlogCategory}};
use crate::app::AppStateContext;

#[function_component(Blog)]
pub fn blog() -> Html {
    let state_ctx_reducer = use_context::<AppStateContext>().unwrap();
    let state_ctx_reducer_clone = state_ctx_reducer.clone();

    use_effect(move || {
        wasm_bindgen_futures::spawn_local(async move {
            if state_ctx_reducer_clone.blog_posts.is_empty() {
                let _ = get_blog_posts(state_ctx_reducer_clone).await;
            }
        }); // Await the async block
        || ()
    });

    html! {
        <>
        <header>
            <BlogNav />
        </header>
        <LineSeparator />
        <main class="blog">
            <MainBanner
                title="\"In the world of code, the best debugging tool is a fresh perspective."
                subtitle="~Chat GPT"
                background_url="https://imagedelivery.net/fa3SWf5GIAHiTnHQyqU8IQ/e4079939-5afe-46da-364a-a7524d266100/public"
            />
            <BlogSection category={BlogCategory::LatestRelease} posts={state_ctx_reducer.deref().blog_posts.to_vec()} />
            <AdComponent />
            // <BlogSection category={BlogCategory::WebDevelopment} posts={posts.clone()} />
            // <AdComponent />
            // <BlogSection category={BlogCategory::MobileDevelopment} posts={posts.clone()} />
            // <AdComponent />
            // <BlogSection category={BlogCategory::ArtificialIntelligence} posts={posts.clone()} />
            // <AdComponent />
            // <BlogSection category={BlogCategory::Technology} posts={posts.clone()} />
            // <AdComponent />
            // <BlogSection category={BlogCategory::Lifestyle} posts={posts.clone()} />
            // <AdComponent />
            // <BlogSection category={BlogCategory::Travel} posts={posts} />
            <Footer />
        </main>
        </>
    }
}
