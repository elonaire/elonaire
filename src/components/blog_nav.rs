use yew::prelude::*;
use yew_icons::{Icon, IconId};
use yew_router::prelude::Link;

use crate::{app::Route, data::models::blog::BlogCategory};

#[function_component(BlogNav)]
pub fn blog_nav() -> Html {
    let categories = vec![
        BlogCategory::All,
        BlogCategory::WebDevelopment,
        BlogCategory::MobileDevelopment,
        BlogCategory::ArtificialIntelligence,
        BlogCategory::Technology,
        BlogCategory::Lifestyle,
        BlogCategory::Travel,
    ];

    html! {
        <>
        <nav class="blog-nav">
            <Link<Route> classes={classes!("logo")} to={Route::Home}>
                <img class={classes!("logo-img")} src="https://imagedelivery.net/fa3SWf5GIAHiTnHQyqU8IQ/3b98be8e-df1c-41c8-a03b-0c645e98fa00/public" alt="logo" />
            </Link<Route>>

            <div class="search-bar">
                <button disabled={true} class="button-transparent"><Icon icon_id={IconId::BootstrapSearch}/></button>
            </div>
        </nav>
        // <LineSeparator />
        <ul class="blog-categories">
                {
                    for categories.iter().map(|category| {
                        let display_text = match category {
                            BlogCategory::WebDevelopment => "Web Development".to_owned(),
                            BlogCategory::MobileDevelopment => "Mobile Development".to_owned(),
                            BlogCategory::ArtificialIntelligence => "AI".to_owned(),
                            _ => format!("{:?}", category),
                        };

                        html! { <li><button class="button button-outlined">{ display_text }</button></li> }
                    })
                }
            </ul>
        </>
    }
}
