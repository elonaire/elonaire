use yew::prelude::*;

use crate::{components::{blog::blog_post_card::{BlogPostCard, BlogPostCardProps}, line_separator::LineSeparator}, views::blog::BlogCategory};

#[derive(Properties, PartialEq)]
pub struct BlogSectionProps {
    pub category: BlogCategory,
    pub posts: Vec<BlogPostCardProps>, // This will be a list of release properties
}

#[function_component(BlogSection)]
pub fn latest_release(props: &BlogSectionProps) -> Html {
    // Map over the remaining releases to create their HTML representations
    let release_cards_html = &props.posts.iter().map(|release| {
        html! { <BlogPostCard category={release.category.clone()} pub_date={release.pub_date.clone()} is_hot_topic={release.is_hot_topic} image_url={release.image_url.clone()} title={release.title.clone()} description={release.description.clone()} /> }
    }).collect::<Html>();

    html! {
        <section class="blog-section">
            <div class="section-title">
                <span>{ props.category.to_string() }</span>
                <button class="button button-outlined-primary">{"View All"}</button>
            </div>
            <LineSeparator />
            <div class="blog-post-cards-container">
                { release_cards_html.to_owned() }
            </div>
        </section>
    }
}
