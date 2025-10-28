use leptos::Params;
use leptos_router::params::Params;
use reactive_stores::Store;
use serde::{Deserialize, Serialize};

use crate::data::models::graphql::acl::User;

#[derive(Params, PartialEq, Serialize)]
pub struct AuthCode {
    pub auth_code: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AuthDetails {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub token: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

#[derive(Clone, Debug, Default, Store)]
pub struct AppStateContext {
    user: UserInfo,
}

#[derive(Clone, Debug, Default, Store)]
#[allow(dead_code)]
pub struct UserInfo {
    pub user_profile: User,
    pub auth_info: AuthInfo,
}

#[derive(Clone, Debug, Default, Store)]
pub struct AuthInfo {
    pub token: String,
}

#[derive(Clone, Copy, Debug, Deserialize, Serialize)]
pub enum OauthClientName {
    Github,
    Google,
}
