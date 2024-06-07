use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use yew::Properties;

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq, Default)]
pub struct UserResources {
    #[serde(rename = "blogPosts", skip_serializing_if = "Option::is_none")]
    pub blog_posts: Option<Vec<BlogPost>>,
    #[serde(rename = "professionalInfo", skip_serializing_if = "Option::is_none")]
    pub professional_info: Option<Vec<UserProfessionalInfo>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub portfolio: Option<Vec<UserPortfolio>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resume: Option<Vec<UserResume>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub skills: Option<Vec<UserSkill>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub services: Option<Vec<UserService>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub achievements: Option<ResumeAchievements>,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq, Default)]
pub struct BlogPost {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(rename = "shortDescription", skip_serializing_if = "Option::is_none")]
    pub short_description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category: Option<BlogCategory>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub link: Option<String>,
    #[serde(rename = "publishedDate", skip_serializing_if = "Option::is_none")]
    pub published_date: Option<String>,
    #[serde(rename = "createdAt", skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(rename = "updatedAt", skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq, Default)]
pub struct UserProfessionalInfo {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub occupation: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "startDate")]
    pub start_date: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq, Default, Properties)]
pub struct UserPortfolio {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub link: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "startDate")]
    pub start_date: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "endDate")]
    pub end_date: Option<String>,
    #[serde(rename = "createdAt", skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(rename = "updatedAt", skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq, Default, Properties)]
pub struct UserResume {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub section: Option<UserResumeSection>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "moreInfo")]
    pub more_info: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub link: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "startDate")]
    pub start_date: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "endDate")]
    pub end_date: Option<String>,
    #[serde(rename = "createdAt", skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(rename = "updatedAt", skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq, Default)]
pub struct ResumeAchievement {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq, Default, Properties)]
pub struct UserSkill {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub level: Option<UserSkillLevel>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<UserSkillType>,
    #[serde(rename = "createdAt", skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(rename = "updatedAt", skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize, Copy, Eq, PartialEq)]
pub enum UserSkillType {
    Technical,
    Soft,
}

#[derive(Clone, Debug, Serialize, Deserialize, Copy, Eq, PartialEq)]
pub enum UserSkillLevel {
    Beginner,
    Intermediate,
    Advanced,
    Expert,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq, Default, Properties)]
pub struct UserService {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image: Option<String>,
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum BlogCategory {
    WebDevelopment,
    MobileDevelopment,
    ArtificialIntelligence,
    Technology,
    Lifestyle,
}

#[derive(Clone, Debug, Serialize, Deserialize, Copy, Eq, PartialEq)]
pub enum UserResumeSection {
    Education,
    Experience,
    Achievements,
    Projects,
    Certifications,
    Volunteer,
    Publications,
    Languages,
    Interests,
    References,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct GetUserResourcesResponse {
    #[serde(rename = "getUserResources")]
    pub get_user_resources: UserResources,
}

pub type ResumeAchievements = HashMap<String, Vec<String>>;
