use std::collections::HashMap;

use gql_client::{Client, GraphQLErrorMessage};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
pub struct LocalGraphQLErrorMessage {
    pub message: String,
    pub locations: Option<Vec<LocalGraphQLErrorLocation>>,
    pub extensions: Option<HashMap<String, String>>,
    pub path: Option<Vec<LocalGraphQLErrorPathParam>>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
pub struct LocalGraphQLErrorLocation {
    pub line: u32,
    pub column: u32,
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
#[serde(untagged)]
pub enum LocalGraphQLErrorPathParam {
    String(String),
    Number(u32),
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub enum GraphQLResponse<T> {
    Data(T),
    Error(Vec<GraphQLErrorMessage>),
}

#[allow(dead_code)]
impl<T> GraphQLResponse<T> {
    pub fn get_data(&self) -> Option<&T> {
        match self {
            GraphQLResponse::Data(data) => Some(data),
            _ => None,
        }
    }

    pub fn get_error(&self) -> &[GraphQLErrorMessage] {
        match self {
            GraphQLResponse::Error(errors) => errors,
            _ => &[],
        }
    }
}

/// ```
/// let endpoint = "http://localhost:3001";
/// ```
/// create query
///
/// ```
///     let query = r#"
///        query Query {
///            getUsers {
///                id
///                email
///                fullName
///                age
///            }
///        }
///    "#;
/// ```
pub async fn perform_query_without_vars<Res: for<'de> Deserialize<'de> + Default>(
    headers: Option<&HashMap<String, String>>,
    endpoint: &str,
    query: &str,
) -> GraphQLResponse<Res> {
    let client = match headers {
        Some(headers) => Client::new_with_headers(endpoint, headers.to_owned()),
        None => Client::new(endpoint),
    };

    let response = client.query::<Res>(query).await;

    match response {
        Ok(data) => GraphQLResponse::Data(data.unwrap_or_default()),
        Err(err) => {
            let errors = match err.json() {
                Some(errors) => errors,
                None => vec![],
            };
            GraphQLResponse::Error(errors)
        }
    }
}

/// ```
/// let endpoint = "http://localhost:3001";
/// ```
///
/// create query
///
/// ```
/// let query = r#"
///     mutation Mutation($user: UserInput!) {
///         signUp(user: $user) {
///             id
///             email
///             fullName
///             age
///         }
///     }
/// "#;
/// ```
pub async fn perform_mutation_or_query_with_vars<
    Res: for<'de> Deserialize<'de> + Serialize + Default,
    Var: for<'de> Deserialize<'de> + Serialize,
>(
    headers: Option<&HashMap<String, String>>,
    endpoint: &str,
    query: &str,
    vars: Var,
) -> GraphQLResponse<Res> {
    let client = match headers {
        Some(headers) => Client::new_with_headers(endpoint, headers.to_owned()),
        None => Client::new(endpoint),
    };

    let response = client.query_with_vars::<Res, Var>(query, vars).await;

    match response {
        Ok(data) => GraphQLResponse::Data(data.unwrap_or_default()),
        Err(err) => {
            let errors = match err.json() {
                Some(errors) => errors,
                None => vec![],
            };
            GraphQLResponse::Error(errors)
        }
    }
}
