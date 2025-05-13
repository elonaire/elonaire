use crate::{
    app::Route,
    components::{footer::Footer, loader::Loader},
    data::{
        graphql::api_call::perform_mutation_or_query_with_vars,
        models::blog::GetSingleBlogPostResponse,
    },
};
use serde::{Deserialize, Serialize};
use yew::prelude::*;
use yew_icons::{Icon, IconId};
use yew_router::hooks::use_navigator;

#[derive(Clone, PartialEq, Eq, Debug, Deserialize, Properties)]
pub struct RouteParams {
    pub id: String,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct GetSingleBlogVars {
    pub link: String,
}

#[function_component(BlogPostDetails)]
pub fn blog_post_details(props: &RouteParams) -> Html {
    let blog_post = use_state_eq(|| None);
    let navigator = use_navigator().unwrap();

    let blog_post_clone = blog_post.clone();
    let props_clone = props.clone();
    use_effect_with_deps(
        move |_| {
            let endpoint =
                option_env!("SHARED_SERVICE_URL").expect("SHARED_SERVICE_URL env var not set");

            let query = r#"
            query Query($link: String!) {
                getSingleBlogPost(link: $link) {
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
                    isFeatured
                }
            }
        "#;
            let blog_post_clone = blog_post_clone.clone();
            let props_clone = props_clone.clone();
            wasm_bindgen_futures::spawn_local(async move {
                let var = GetSingleBlogVars {
                    link: props_clone.id.clone(),
                };
                let post = perform_mutation_or_query_with_vars::<
                    GetSingleBlogPostResponse,
                    GetSingleBlogVars,
                >(endpoint, query, var)
                .await;

                blog_post_clone.set(Some(post.get_data().unwrap().get_single_blog_post.clone()));
            }); // Await the async block
            || {}
        },
        (),
    );
    let inner = match blog_post.as_ref() {
        Some(post) => Html::from_html_unchecked(post.content.clone().into()),
        None => {
            html! {
                <div>{"Loading..."}</div>
            }
        }
    };

    let navigate_to_blog_home = {
        Callback::from(move |_: MouseEvent| {
            navigator.push(&Route::BlogRoot);
        })
    };

    html! {
        <>
            <main class="blog-post">
                { if blog_post.is_none() { html!{ <Loader /> } } else { html!{ } } }
                <div class="blog-container">
                    <button onclick={navigate_to_blog_home} class="button button-primary back"><Icon icon_id={IconId::BootstrapSkipBackward}/></button>
                    <div class="left">

                    </div>
                    <div class="content-wrapper">
                        { inner }
                    </div>
                    <div class="right">
                    </div>
                </div>
                <Footer />
            </main>
        </>
    }
}
