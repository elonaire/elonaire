use std::ops::Deref;

use yew::prelude::*;

use crate::{components::{ad::AdComponent, blog::{blog_section::BlogSection, main_banner::MainBanner}, blog_nav::BlogNav, footer::Footer, line_separator::LineSeparator}, data::{graphql::api_call::perform_query_without_vars, models::blog::{GetBlogPostsResponse, BlogCategory}}};
use crate::app::{AppStateContext, StateAction};

#[function_component(Blog)]
pub fn blog() -> Html {
    let state_ctx_reducer = use_context::<AppStateContext>().unwrap();

    use_effect({
        let endpoint = "http://localhost:3002";
        let query = r#"
            query Query {
                getBlogPosts {
                    id
                    title
                    shortDescription
                    image
                    createdAt
                    content
                    category
                    publishedDate
                    status
                    link
                }
            }
        "#;
        let state_clone = state_ctx_reducer.clone();
        wasm_bindgen_futures::spawn_local(async move {
            let posts = perform_query_without_vars::<GetBlogPostsResponse>(endpoint, query).await;

            log::info!("posts: {:?}", posts);

            state_clone.dispatch(StateAction::UpdateBlogPosts(
                // posts.get_data().unwrap().get_blog_posts.clone(),
                match posts.get_data() {
                    Some(data) => data.get_blog_posts.clone(),
                    None => vec![],
                }
            ));
        }); // Await the async block
        || {}
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
                background_url="img/bg.jpeg"
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
