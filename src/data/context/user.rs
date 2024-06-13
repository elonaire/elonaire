use std::fmt::Error;

use serde::{Deserialize, Serialize};
use yew::UseReducerHandle;

use crate::{app::{AppState, StateAction}, data::{graphql::api_call::perform_mutation_or_query_with_vars, models::user::GetUserResponse}};

#[derive(Debug, Serialize, Deserialize)]
pub struct GetUserVar {
    id: String,
}

pub async fn get_user_by_id(id: String, state_clone: UseReducerHandle<AppState>) -> Result<(), Error> {
    let endpoint = match option_env!("TRUNK_BUILD_ACL_SERVICE_URL") {
        Some(url) => url,
        None => option_env!("TRUNK_SERVE_ACL_SERVICE_URL").unwrap(),
    
    };
    let query = r#"
            query Query($id: String!) {
                getUser(id: $id) {
                    id
                    firstName
                    middleName
                    lastName
                    fullName
                    bio
                    address
                    country
                    email
                    gender
                    dob
                    phone
                }
            }
        "#;

    let variables = GetUserVar { id };

    let user = perform_mutation_or_query_with_vars::<GetUserResponse, GetUserVar>(endpoint, query, variables).await;

    state_clone.dispatch(StateAction::UpdateUserInfo(
        match user.get_data() {
            Some(data) => data.get_user.clone(),
            None => Default::default(),
        },
    ));

    Ok(())
}
