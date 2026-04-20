use crate::{
    data::{
        context::store::{AppStateContext, AppStateContextStoreFields},
        models::general::shared::RestResponse,
    },
    utils::graphql_client::{GraphQLResponse, LocalGraphQLErrorMessage},
};
use leptos::prelude::*;
use reactive_stores::Store;
use serde::{Deserialize, Serialize};

pub fn handle_graphql_errors<T>(
    response: &GraphQLResponse<T>,
    store: &Store<AppStateContext>,
    redirect_to: Option<&str>,
) -> () {
    let errors = response.get_error();
    errors.iter().for_each(|e| {
        if let Ok(value) = serde_json::to_value(e) {
            if let Ok(err) = serde_json::from_value(value) as Result<LocalGraphQLErrorMessage, _> {
                store
                    .redirect_to()
                    .set(redirect_to.map(|link| link.to_string()));
                store.error().set(Some(LocalErrorMessage::GraphQL(err)));
            }
        }
    });
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
pub struct LocalRestErrorBody {
    pub success: bool,
    pub error: LocalRestErrorMessage,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
pub struct LocalRestErrorMessage {
    pub code: String,
    pub message: String,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
pub enum LocalErrorMessage {
    GraphQL(LocalGraphQLErrorMessage),
    Rest(LocalRestErrorMessage),
}

impl LocalErrorMessage {
    pub fn message(&self) -> &str {
        match self {
            LocalErrorMessage::GraphQL(e) => &e.message,
            LocalErrorMessage::Rest(e) => &e.message,
        }
    }

    pub fn code(&self) -> Option<&str> {
        match self {
            LocalErrorMessage::GraphQL(e) => e
                .extensions
                .as_ref()
                .and_then(|ext| ext.get("code"))
                .map(String::as_str),
            LocalErrorMessage::Rest(e) => Some(&e.code),
        }
    }

    pub fn is_unauthorized(&self) -> bool {
        self.code().map(|c| c == "401").unwrap_or(false)
    }

    pub fn is_not_found(&self) -> bool {
        self.code().map(|c| c == "404").unwrap_or(false)
    }

    pub fn is_bad_request(&self) -> bool {
        self.code().map(|c| c == "400").unwrap_or(false)
    }

    pub fn is_forbidden(&self) -> bool {
        self.code().map(|c| c == "403").unwrap_or(false)
    }

    pub fn is_unprocessable(&self) -> bool {
        self.code().map(|c| c == "422").unwrap_or(false)
    }

    pub fn is_internal(&self) -> bool {
        self.code().map(|c| c == "500").unwrap_or(false)
    }

    /// Parse a REST error response from a raw JSON string
    pub fn from_rest_json(json: &str) -> Option<Self> {
        serde_json::from_str::<LocalRestErrorBody>(json)
            .ok()
            .map(|body| LocalErrorMessage::Rest(body.error))
    }
}

pub fn unwrap_rest_response<T>(
    body: RestResponse<T>,
    store: &Store<AppStateContext>,
    redirect_to: Option<&str>,
) -> Option<T> {
    if !body.success {
        let error = body.error.map(LocalErrorMessage::Rest).unwrap_or_else(|| {
            LocalErrorMessage::Rest(LocalRestErrorMessage {
                code: "UNKNOWN".into(),
                message: "An unknown error occurred".into(),
            })
        });
        store
            .redirect_to()
            .set(redirect_to.map(|link| link.to_string()));
        store.error().set(Some(error));
        return None;
    }
    body.data
}
