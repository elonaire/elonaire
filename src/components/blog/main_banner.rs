use yew::prelude::*;
use crate::{app::AppStateContext, data::models::blog::BlogPost, components::blog::featured_blog_post_card::FeaturedBlogPostCard};

// #[derive(Properties, PartialEq)]
// pub struct FeaturedProps {
//     pub title: String,
//     pub subtitle: String,
//     pub background_url: String,
// }

#[function_component(FeaturedPosts)]
pub fn featured_posts() -> Html {
    let state_ctx_reducer = use_context::<AppStateContext>().unwrap();
    let state_ctx_reducer_clone = state_ctx_reducer.clone();

    let featured_posts = use_state(|| vec![] as Vec<BlogPost>);

    let state_clone = state_ctx_reducer_clone.clone();
    let featured_posts_state_clone = featured_posts.clone();
    use_effect_with_deps(move |_| {
        let updated_featured_posts: Vec<BlogPost> = state_clone.blog_posts.iter().filter(|post| post.is_featured).cloned().collect();
            featured_posts_state_clone.set(updated_featured_posts.to_vec());

        || {}
    }, state_ctx_reducer_clone.blog_posts.clone());

    html! {
        <div class="featured">
            {
                for featured_posts.iter().map(|post| {
                    html! {
                        <FeaturedBlogPostCard is_featured={post.is_featured} category={post.category.clone()} published_date={post.published_date.clone()} image={post.image.clone()} title={post.title.clone()} short_description={post.short_description.clone()} created_at={post.created_at.clone()} id={post.id.clone()} link={post.link.clone()} content={post.content.clone()} />
                    }
                })
            }
        </div>
    }
}
