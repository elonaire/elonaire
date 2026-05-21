use leptos::prelude::*;
use std::collections::HashMap;

use reactive_stores::Store;

use crate::data::{
    context::{
        shared::{check_auth, fetch_single_user},
        store::{AppStateContext, AppStateContextStoreFields},
    },
    models::{
        general::acl::{AuthInfo, AuthInfoStoreFields, UserInfoStoreFields},
        graphql::acl::FetchSingleUserVars,
    },
};

pub async fn check_auth_and_update_store_auth_info(token: &str, store: &Store<AppStateContext>) {
    let token = token.to_string();
    let token_ref = &token;
    let mut headers = HashMap::new() as HashMap<String, String>;
    headers.insert("Authorization".into(), format!("Bearer {}", token_ref));

    let check_auth = check_auth(Some(&headers)).await;

    match check_auth {
        Ok(auth) => {
            store.user().auth_info().set(AuthInfo {
                token: auth
                    .new_access_token
                    .as_ref()
                    .unwrap_or(token_ref)
                    .to_owned(),
                current_role: auth.current_role.clone(),
                current_role_permissions: auth.current_role_permissions,
            });
            let user_id_vars = FetchSingleUserVars {
                user_id: auth.sub.clone(),
            };

            let fetch_user_info_query = r#"
                query FetchSingleUser($userId: String!) {
                    fetchSingleUser(userId: $userId) {
                        data {
                            firstName
                            middleName
                            lastName
                            gender
                            dob
                            email
                            country
                            phone
                            createdAt
                            updatedAt
                            oauthClient
                            oauthUserId
                            profilePicture
                            bio
                            website
                            address
                            id
                            fullName
                            age
                        }
                        metadata {
                            requestId
                            newAccessToken
                        }
                    }
                }
               "#;

            if let Ok(user_profile) =
                fetch_single_user(&user_id_vars, None, fetch_user_info_query).await
            {
                store.user().user_profile().set(user_profile);
            };
        }
        Err(_) => {}
    }
}
