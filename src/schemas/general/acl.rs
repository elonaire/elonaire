use leptos::Params;
use leptos_router::params::Params;
use reactive_stores::Store;
use serde::{Deserialize, Serialize};

#[derive(Params, PartialEq, Serialize)]
pub struct AuthCode {
    pub auth_code: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AuthStatus {
    #[serde(rename = "isAuth")]
    pub is_auth: bool,
    pub sub: String,
    pub current_role: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AuthDetailsRest {
    pub url: Option<String>,
    pub token: Option<String>,
}

#[derive(Clone, Debug, Default, Store)]
pub struct AppStateContext {
    user: UserInfo,
}

#[derive(Clone, Debug, Default, Store)]
pub struct UserInfo {
    pub user_profile: UserProfile,
    pub auth_info: AuthInfo,
}

#[derive(Clone, Debug, Default, Store)]
pub struct UserProfile {
    pub id: Option<String>,
    pub user_name: Option<String>,
    pub first_name: Option<String>,
    pub middle_name: Option<String>,
    pub last_name: Option<String>,
    pub gender: Option<Gender>,
    pub dob: Option<String>,
    pub email: String,
    pub country: Option<String>,
    pub phone: Option<String>,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
    pub status: Option<AccountStatus>,
    pub oauth_client: Option<OAuthClientName>,
    pub oauth_user_id: Option<String>,
    pub profile_picture: Option<String>,
    pub bio: Option<String>,
    pub website: Option<String>,
    pub address: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize, Copy, Eq, PartialEq)]
pub enum OAuthClientName {
    Google,
    Github,
}

#[derive(Clone, Debug, Serialize, Deserialize, Copy, Eq, PartialEq)]
pub enum Gender {
    Male,
    Female,
    Other,
}

#[derive(Clone, Debug, Serialize, Deserialize, Copy, Eq, PartialEq, Default)]
pub enum AccountStatus {
    Active,
    #[default]
    Inactive,
    Suspended,
    Deleted,
}

#[derive(Clone, Debug, Default, Store)]
pub struct AuthInfo {
    pub token: String,
}
