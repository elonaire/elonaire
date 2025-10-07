use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct AuthDetails {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub token: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct UserLoginsInput {
    #[serde(rename = "userName", alias = "user_name")]
    pub user_name: Option<String>,
    pub password: Option<String>,
    #[serde(rename = "oauthClient", alias = "oauth_client")]
    pub oauth_client: Option<OauthClientName>,
}

#[derive(Clone, Copy, Debug, Deserialize, Serialize)]
pub enum OauthClientName {
    Github,
    Google,
}

/* gql_client code */
#[derive(Debug, Deserialize, Serialize)]
pub struct SignInResponse {
    #[serde(rename = "signIn")]
    pub sign_in: Option<AuthDetails>, // this is the return type expected from the API on success
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SignInVars {
    #[serde(rename = "rawUserDetails")]
    pub raw_user_details: UserLoginsInput,
}
