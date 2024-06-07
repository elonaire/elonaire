use yew::prelude::*;

use crate::{components::{blog::blog_post_card::BlogPostCard, line_separator::LineSeparator}, data::models::blog::{BlogPost, BlogCategory}};

#[derive(Properties, PartialEq)]
pub struct BlogSectionProps {
    pub category: BlogCategory,
    pub posts: Vec<BlogPost>, // This will be a list of release properties
}

#[function_component(BlogSection)]
pub fn blog_section(props: &BlogSectionProps) -> Html {
    // log::info!("posts: {:?}", &props.posts);
    // Map over the remaining releases to create their HTML representations
    let release_cards_html = &props.posts.iter().map(|release| {
        html! { <BlogPostCard category={release.category.clone()} published_date={release.published_date.clone()} image={release.image.clone()} title={release.title.clone()} short_description={release.short_description.clone()} created_at={release.created_at.clone()} id={release.id.clone()} link={release.link.clone()} content={release.content.clone()} /> }
    }).collect::<Html>();

    html! {
        <section class="blog-section">
            <div class="section-title">
                <span>{ &props.category.to_string() }</span>
                <button class="button button-outlined-primary">{"View All"}</button>
            </div>
            <LineSeparator />
            <div class="blog-post-cards-container">
                { release_cards_html.to_owned() }
            </div>
        </section>
    }
}
