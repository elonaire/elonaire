use std::collections::HashMap;

use gql_client::GraphQLErrorMessage;
use leptos::prelude::*;

use crate::{
    data::{
        context::store::{AppStateContext, AppStateContextStoreFields},
        models::graphql::acl::FetchSiteOwnerResponse,
    },
    utils::graphql_client::perform_query_without_vars,
};
use reactive_stores::Store;

pub async fn fetch_site_owner_info(
    current_state: &Store<AppStateContext>,
    headers: Option<&HashMap<String, String>>,
) -> Result<(), Vec<GraphQLErrorMessage>> {
    let fetch_site_owner_query = r#"
           query FetchSiteOwnerInfo {
                fetchSiteOwnerInfo {
                    data {
                        firstName
                        middleName
                        lastName
                        gender
                        dob
                        email
                        country
                        createdAt
                        updatedAt
                        profilePicture
                        bio
                        website
                        address
                        id
                        fullName
                        age
                    }
                    metadata {
                        newAccessToken
                        requestId
                    }
                }
           }
       "#;

    let fetch_site_owner_response = perform_query_without_vars::<FetchSiteOwnerResponse>(
        headers,
        "http://localhost:8080/api/acl",
        fetch_site_owner_query,
    )
    .await;

    match fetch_site_owner_response.get_data() {
        Some(data) => {
            let owned_data = data
                .fetch_site_owner_info
                .as_ref()
                .unwrap()
                .get_data()
                .to_owned();
            current_state.site_owner_info().set(owned_data);

            Ok(())
        }
        None => Err(fetch_site_owner_response.get_error().to_vec()),
    }
}
