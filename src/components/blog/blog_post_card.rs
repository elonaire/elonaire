
use yew::prelude::*;
use yew_icons::{Icon, IconId};
use yew_router::hooks::use_navigator;

use crate::{app::BlogRoute, data::models::blog::BlogPost};

#[function_component(BlogPostCard)]
pub fn release_card(props: &BlogPost) -> Html {
    let navigator = use_navigator().unwrap();
    let cloned_props = props.clone();
    let view_blog = Callback::from(move |_| {
        // navigate to blog post
        navigator.push(&BlogRoute::BlogPostDetails { id: cloned_props.link.clone() });
    });
    html! {
        <div class="blog-post-card">
            <div onclick={view_blog} class="blog-post-image-container">
                <img class="blog-post-image" src={props.image.clone()} alt="release-image" />
                <div class="category">
                    { &props.category.to_string() }
                </div>
                <span class="pub-date">{ &props.published_date.clone().unwrap_or("".to_string()) }</span>
            </div>
            <div class="blog-post-content">
                <h3 class="blog-post-title">{ &props.title }</h3>
                <p class="blog-post-description">{ &props.short_description }</p>
                <div class="blog-stats">
                    <div class="blog-stat">
                        <Icon width={"1.5em".to_owned()} height={"1.5em".to_owned()} icon_id={IconId::BootstrapEye}/>
                        <span>{"0"}</span>
                    </div>
                    <div class="blog-stat">
                        <Icon width={"1.5em".to_owned()} height={"1.5em".to_owned()} icon_id={IconId::FontAwesomeRegularComments}/>
                        <span>{"0"}</span>
                    </div>
                    <div class="blog-stat">
                        <Icon width={"1.5em".to_owned()} height={"1.5em".to_owned()} icon_id={IconId::BootstrapShare}/>
                        <span>{"0"}</span>
                    </div>
                </div>
            </div>
        </div>
    }
}
