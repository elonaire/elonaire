use serde::{Deserialize, Serialize};

use crate::data::models::general::shared::ApiResponse;

#[derive(Debug, Copy, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum SubscriberStatus {
    Active,
    Unsubscribed,
    Bounced,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Subscriber {
    pub id: Option<String>,
    pub email: Option<String>,
    #[serde(rename = "firstName", alias = "first_name")]
    pub first_name: Option<String>,
    #[serde(rename = "lastName", alias = "last_name")]
    pub last_name: Option<String>,
    pub status: Option<SubscriberStatus>,
    #[serde(rename = "createdAt", alias = "created_at")]
    pub created_at: Option<String>,
    #[serde(rename = "updatedAt", alias = "updated_at")]
    pub updated_at: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubscriberInput {
    pub email: String,
    #[serde(rename = "firstName", alias = "first_name")]
    pub first_name: Option<String>,
    #[serde(rename = "lastName", alias = "last_name")]
    pub last_name: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MailingList {
    pub id: Option<String>,
    pub name: Option<String>,
    pub description: Option<String>,
    #[serde(rename = "createdAt", alias = "created_at")]
    pub created_at: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MailingListInput {
    pub name: String,
    pub description: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Subscription {
    pub id: Option<String>,
    pub subscriber: Option<Subscriber>,
    #[serde(rename = "mailingList", alias = "mailing_list")]
    pub mailing_list: Option<MailingList>,
    #[serde(rename = "createdAt", alias = "created_at")]
    pub created_at: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubscriptionInput {
    pub subscriber: SubscriberInput,
    #[serde(
        rename = "subscriptionInputMetadata",
        alias = "subscription_input_metadata"
    )]
    pub subscription_input_metadata: SubscriptionInputMetadata,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubscriptionInputMetadata {
    #[serde(rename = "mailingListId", alias = "mailing_list_id")]
    pub mailing_list_id: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateSubscriptionVars {
    #[serde(rename = "subscriptionInput")]
    pub subscription_input: SubscriptionInput,
}

#[derive(Debug, Deserialize, Serialize, Clone, Default)]
pub struct CreateSubscriptionResponse {
    #[serde(rename = "subscribeToMailingList")]
    pub subscribe_to_mailing_list: Option<ApiResponse<Subscription>>, // this is the return type expected from the API on success
}
