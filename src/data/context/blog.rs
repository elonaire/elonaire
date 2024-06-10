use std::fmt::Error;

use serde::{Deserialize, Serialize};
use yew::UseReducerHandle;

use crate::{
    app::{AppState, StateAction},
    data::{graphql::api_call::perform_mutation_or_query_with_vars, models::blog::GetBlogPostsResponse},
};

#[derive(Debug, Serialize, Deserialize)]
pub struct GetBlogsVar {
    id: Option<String>,
}

pub async fn get_blog_posts(state_clone: UseReducerHandle<AppState>) -> Result<(), Error> {
    let endpoint = "https://techietenka.com/tt-shared-service";
    let query = r#"
            query Query($id: String) {
                getBlogPosts(id: $id) {
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

    let variables = GetBlogsVar { id: Some("all".to_string()) };

    let posts = perform_mutation_or_query_with_vars::<GetBlogPostsResponse, GetBlogsVar>(endpoint, query, variables).await;

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
