use std::fmt::Error;

use yew::UseReducerHandle;

use crate::{
    app::{AppState, StateAction},
    data::{graphql::api_call::perform_query_without_vars, models::blog::GetBlogPostsResponse},
};

pub async fn get_blog_posts(state_clone: UseReducerHandle<AppState>) -> Result<(), Error> {
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

    let posts = perform_query_without_vars::<GetBlogPostsResponse>(endpoint, query).await;

    // log::info!("posts: {:?}", posts);

    state_clone.dispatch(StateAction::UpdateBlogPosts(
        // posts.get_data().unwrap().get_blog_posts.clone(),
        match posts.get_data() {
            Some(data) => data.get_blog_posts.clone(),
            None => vec![],
        },
    ));
    Ok(())
}
