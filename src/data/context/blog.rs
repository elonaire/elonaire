use std::fmt::Error;

use yew::UseReducerHandle;

use crate::{
    app::{AppState, StateAction},
    data::{graphql::api_call::perform_query_without_vars, models::blog::GetBlogPostsResponse},
};


pub async fn get_blog_posts(state_clone: UseReducerHandle<AppState>) -> Result<(), Error> {
    let endpoint = option_env!("SHARED_SERVICE_URL").expect("SHARED_SERVICE_URL env var not set");
    
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
                    isFeatured
                }
            }
        "#;

    let posts = perform_query_without_vars::<GetBlogPostsResponse>(endpoint, query).await;

    state_clone.dispatch(StateAction::UpdateBlogPosts(
        match posts.get_data() {
            Some(data) => data.get_blog_posts.clone(),
            None => vec![],
        },
    ));
    Ok(())
}
