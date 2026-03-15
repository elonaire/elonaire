use crate::{
    data::context::store::{AppStateContext, AppStateContextStoreFields},
    utils::graphql_client::{GraphQLResponse, LocalGraphQLErrorMessage},
};
use leptos::prelude::*;
use reactive_stores::Store;

pub fn handle_graphql_errors<T>(
    response: &GraphQLResponse<T>,
    current_state: &Store<AppStateContext>,
    redirect_to: &str,
) -> () {
    let errors = response.get_error();
    errors.iter().for_each(|e| {
        // The problem is e: &GraphQLErrorMessage is from an external crate(gql_client), all fields are private and implements only Deserialize trait
        if let Ok(value) = serde_json::to_value(e) {
            if let Ok(err) = serde_json::from_value(value) as Result<LocalGraphQLErrorMessage, _> {
                current_state.redirect_to().set(Some(redirect_to.into()));
                current_state.error().set(Some(err));
            } else {
                // leptos::logging::log!("Failed to parse error: {:?}", e);
            };
        } else {
            // leptos::logging::log!("Failed to serialize error: {:?}", e);
        };
    });
}
