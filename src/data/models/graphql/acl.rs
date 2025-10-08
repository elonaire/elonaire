use reactive_stores::Store;
use serde::{Deserialize, Serialize};

use crate::data::models::general::acl::{AuthDetails, OauthClientName};

#[derive(Debug, Deserialize, Serialize)]
pub struct UserLoginsInput {
    #[serde(rename = "userName", alias = "user_name")]
    pub user_name: Option<String>,
    pub password: Option<String>,
    #[serde(rename = "oauthClient", alias = "oauth_client")]
    pub oauth_client: Option<OauthClientName>,
}

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

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AuthStatus {
    #[serde(rename = "isAuth")]
    pub is_auth: bool,
    pub sub: String,
    #[serde(rename = "currentRole")]
    pub current_role: String,
}

#[derive(Clone, Debug, Default, Store)]
#[allow(dead_code)]
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
    pub oauth_client: Option<OauthClientName>,
    pub oauth_user_id: Option<String>,
    pub profile_picture: Option<String>,
    pub bio: Option<String>,
    pub website: Option<String>,
    pub address: Option<String>,
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
