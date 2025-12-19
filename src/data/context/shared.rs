use std::collections::HashMap;

use gql_client::GraphQLErrorMessage;
use leptos::prelude::*;

use crate::{
    data::{
        context::store::{AppStateContext, AppStateContextStoreFields},
        models::graphql::shared::FetchSiteResourcesResponse,
    },
    utils::graphql_client::perform_query_without_vars,
};
use reactive_stores::Store;

pub async fn fetch_services(
    current_state: &Store<AppStateContext>,
    headers: Option<&HashMap<String, String>>,
) -> Result<(), Vec<GraphQLErrorMessage>> {
    let fetch_services_query = r#"
           query FetchSiteResources {
                fetchSiteResources {
                    services {
                        title
                        description
                        thumbnail
                        id
                    }
                }
           }
       "#;

    let fetch_services_response = perform_query_without_vars::<FetchSiteResourcesResponse>(
        headers,
        "http://localhost:8080/api/shared",
        fetch_services_query,
    )
    .await;

    match fetch_services_response.get_data() {
        Some(data) => {
            let owned_data = data
                .fetch_site_resources
                .as_ref()
                .unwrap()
                .services
                .as_ref()
                .unwrap()
                .to_vec();
            *current_state.services().write() = owned_data;

            Ok(())
        }
        None => Err(fetch_services_response.get_error().to_vec()),
    }
}
