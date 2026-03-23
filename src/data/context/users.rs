use std::collections::HashMap;

use gql_client::GraphQLErrorMessage;
use leptos::prelude::*;

use crate::{
    data::{
        context::store::{AppStateContext, AppStateContextStoreFields},
        models::graphql::acl::{FetchSiteOwnerResponse, SignOutResponse},
    },
    utils::graphql_client::perform_query_without_vars,
};
use reactive_stores::Store;

const ACL_SERVICE_API: Option<&str> = option_env!("ACL_SERVICE_API");

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
                        socials {
                            name
                            url
                        }
                    }
                    metadata {
                        newAccessToken
                        requestId
                    }
                }
           }
       "#;

    let Some(acl_service_api) = ACL_SERVICE_API else {
        return Err(vec![]);
    };

    let fetch_site_owner_response = perform_query_without_vars::<FetchSiteOwnerResponse>(
        headers,
        acl_service_api,
        fetch_site_owner_query,
    )
    .await;

    match fetch_site_owner_response.get_data() {
        Some(data) => {
            let owned_data = data
                .fetch_site_owner_info
                .as_ref()
                .unwrap_or(&Default::default())
                .get_data()
                .to_owned();
            current_state.site_owner_info().set(owned_data);

            Ok(())
        }
        None => Err(fetch_site_owner_response.get_error().to_vec()),
    }
}

pub async fn sign_out(
    headers: Option<&HashMap<String, String>>,
) -> Result<(), Vec<GraphQLErrorMessage>> {
    let query = r#"
        mutation SignOut {
            signOut {
                data
                metadata {
                    requestId
                    newAccessToken
                }
            }
        }
       "#;

    let Some(acl_service_api) = ACL_SERVICE_API else {
        return Err(vec![]);
    };

    let sign_out_response =
        perform_query_without_vars::<SignOutResponse>(headers, acl_service_api, query).await;

    match sign_out_response.get_data() {
        Some(_data) => Ok(()),
        None => Err(sign_out_response.get_error().to_vec()),
    }
}
