use std::ops::Deref;

use yew::prelude::*;

use crate::app::AppStateContext;
use crate::{
    components::{
        ad::AdComponent,
        blog::{blog_section::BlogSection, main_banner::FeaturedPosts},
        blog_nav::BlogNav,
        footer::Footer,
    },
    data::{context::blog::get_blog_posts, models::blog::BlogCategory},
};

#[function_component(Blog)]
pub fn blog() -> Html {
    let state_ctx_reducer = use_context::<AppStateContext>().unwrap();
    let state_ctx_reducer_clone = state_ctx_reducer.clone();

    use_effect_with_deps(
        move |_| {
            wasm_bindgen_futures::spawn_local(async move {
                if state_ctx_reducer_clone.blog_posts.is_empty() {
                    let _ = get_blog_posts(state_ctx_reducer_clone).await;
                }
            }); // Await the async block
            || ()
        },
        (),
    );

    html! {
        <>
        <header>
            <BlogNav />
        </header>
        // <LineSeparator />
        <main class="blog-wrapper">
            <div class="blog">
            <FeaturedPosts />
            <AdComponent />
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
            </div>
            <Footer />
        </main>
        </>
    }
}
