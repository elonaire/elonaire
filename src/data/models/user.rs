use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq, Default)]
pub struct User {
    pub id: String,
    pub user_name: String,
    pub first_name: String,
    pub middle_name: Option<String>,
    pub last_name: String,
    pub full_name: String,
    pub age: u32,
    pub gender: Gender,
    pub dob: String,
    pub email: String,
    pub country: String,
    pub phone: String,
    pub password: String,
    pub created_at: String,
    pub updated_at: String,
    pub roles: Vec<String>,
    pub oauth_client: Option<OAuthClientName>,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum OAuthClientName {
    Google,
    Github,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum Gender {
    Male,
    Female,
}

impl Default for Gender {
    fn default() -> Self {
        Gender::Male
    }
}

impl OAuthClientName {
    fn fmt(&self) -> String {
        match self {
            OAuthClientName::Google => format!("Google"),
            OAuthClientName::Github => format!("Github"),
        }
    }

    pub fn from_str(s: &str) -> OAuthClientName {
        match s {
            "Google" => OAuthClientName::Google,
            "Github" => OAuthClientName::Github,
            _ => panic!("Invalid OAuthClientName"),
        }
    }
}
