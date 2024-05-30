use yew::prelude::*;
use yew_icons::{Icon, IconId};

use crate::views::blog::BlogCategory;

#[derive(Properties, PartialEq, Clone)]
pub struct BlogPostCardProps {
    pub image_url: String,
    pub title: String,
    pub description: String,
    pub is_hot_topic: bool,
    pub pub_date: String,
    pub category: BlogCategory,
    // Add other properties like id, link, etc.
}

#[function_component(BlogPostCard)]
pub fn release_card(props: &BlogPostCardProps) -> Html {
    html! {
        <div class="blog-post-card">
            <div class="blog-post-image-container">
                <img class="blog-post-image" src={props.image_url.clone()} alt="release-image" />
                <div class="category">
                    { &props.category.to_string() }
                </div>
                <span class="pub-date">{ &props.pub_date }</span>
            </div>
            <div class="blog-post-content">
                <h3 class="blog-post-title">{ &props.title }</h3>
                <p class="blog-post-description">{ &props.description }</p>
                <div class="blog-stats">
                    <div class="blog-stat">
                        <Icon icon_id={IconId::BootstrapEye}/>
                        <span>{"0"}</span>
                    </div>
                    <div class="blog-stat">
                        <Icon icon_id={IconId::FontAwesomeRegularComments}/>
                        <span>{"0"}</span>
                    </div>
                    <div class="blog-stat">
                        <Icon icon_id={IconId::BootstrapShare}/>
                        <span>{"0"}</span>
                    </div>
                </div>
            </div>
        </div>
    }
}
