use crate::{
    components::{blog_nav::BlogNav, footer::Footer, line_separator::LineSeparator},
    data::{
        graphql::api_call::perform_mutation_or_query_with_vars,
        models::blog::GetSingleBlogPostResponse,
    },
};
use serde::{Deserialize, Serialize};
use yew::prelude::*;

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

    use_effect({
        let endpoint = "http://localhost:3002";
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
                    author
                }
            }
        "#;
        let blog_post_clone = blog_post.clone();
        let props = props.clone();
        wasm_bindgen_futures::spawn_local(async move {
            let var = GetSingleBlogVars {
                link: props.id.clone(),
            };
            let post = perform_mutation_or_query_with_vars::<
                GetSingleBlogPostResponse,
                GetSingleBlogVars,
            >(endpoint, query, var)
            .await;

            // log::info!("posts: {:?}", post);

            blog_post_clone.set(Some(post.get_data().unwrap().get_single_blog_post.clone()));
        }); // Await the async block
        || {}
    });
    let inner = match blog_post.as_ref() {
        Some(post) => Html::from_html_unchecked(post.content.clone().into()),
        None => {
            html! {
                <div>{"Loading..."}</div>
            }
        }
    };
    html! {
        <>
            <header>
                <BlogNav />
            </header>
            <LineSeparator />
            <main class="blog-post">
                // render blog post here using markdown
                <div class="content-wrapper">
                    { inner }
                </div>
                <Footer />
            </main>
        </>
    }
}
