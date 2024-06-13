use serde::{Deserialize, Serialize};

use crate::components::select::SelectOption;

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq, Default)]
pub struct User {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "userName", skip_serializing_if = "Option::is_none")]
    pub user_name: Option<String>,
    #[serde(rename = "firstName", skip_serializing_if = "Option::is_none")]
    pub first_name: Option<String>,
    #[serde(rename = "middleName", skip_serializing_if = "Option::is_none")]
    pub middle_name: Option<String>,
    #[serde(rename = "lastName", skip_serializing_if = "Option::is_none")]
    pub last_name: Option<String>,
    #[serde(rename = "fullName", skip_serializing_if = "Option::is_none")]
    pub full_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bio: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub age: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gender: Option<Gender>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dob: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password: Option<String>,
    #[serde(rename = "createdAt", skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(rename = "updatedAt", skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
    #[serde(rename = "roles", skip_serializing_if = "Option::is_none")]
    pub roles: Option<Vec<String>>,
    #[serde(rename = "oauthClient", skip_serializing_if = "Option::is_none")]
    pub oauth_client: Option<OAuthClientName>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct GetUserResponse {
    #[serde(rename = "getUser")]
    pub get_user: User,
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

#[derive(Clone, Debug, Serialize, Deserialize, Default, PartialEq, Eq)]
pub struct Message {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subject: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "senderName")]
    pub sender_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "senderEmail")]
    pub sender_email: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "createdAt")]
    pub created_at: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize, Copy, Eq, PartialEq)]
pub enum Subject {
    JobOffer,
    Consultation,
    Feedback,
    Complaint,
    Enquiry,
    Suggestion,
}

#[derive(Clone, Debug, Serialize, Deserialize, Eq, PartialEq)]
pub struct SubjectOption {
    pub value: String,
    pub label: String,
}

impl Subject {
    pub fn fmt(&self) -> SelectOption {
        match self {
            Subject::JobOffer => SelectOption {
                value: "JobOffer".to_string(),
                label: "Job Offer".to_string(),
            },
            Subject::Consultation => SelectOption {
                value: "Consultation".to_string(),
                label: "Consultation".to_string(),
            },
            Subject::Feedback => SelectOption {
                value: "Feedback".to_string(),
                label: "Feedback".to_string(),
            },
            Subject::Complaint => SelectOption {
                value: "Complaint".to_string(),
                label: "Complaint".to_string(),
            },
            Subject::Enquiry => SelectOption {
                value: "Enquiry".to_string(),
                label: "Enquiry".to_string(),
            },
            Subject::Suggestion => SelectOption {
                value: "".to_string(),
                label: "--Select Subject".to_string(),
            }
        }
    }

    pub fn as_vec() -> Vec<Subject> {
        vec![
            Subject::JobOffer,
            Subject::Consultation,
            Subject::Feedback,
            Subject::Complaint,
            Subject::Enquiry,
            Subject::Suggestion,
        ]
    }
}
