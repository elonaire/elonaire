use serde::{Deserialize, Serialize};

use crate::utils::custom_traits::EnumerableEnum;

/* This is the beginning of UserPortfolio GraphQL schema */
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[allow(dead_code)]
pub struct CreatePortfolioItemResponse {
    #[serde(rename = "createPortfolioItem")]
    pub create_portfolio_item: Option<UserPortfolio>, // this is the return type expected from the API on success, the key is the resolver name
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[allow(dead_code)]
pub struct UserPortfolio {
    pub id: Option<String>,
    pub title: Option<String>,
    pub description: Option<String>,
    #[serde(rename = "startDate", alias = "start_date")]
    pub start_date: Option<String>,
    #[serde(rename = "endDate", alias = "end_date")]
    pub end_date: Option<String>,
    #[serde(rename = "yearsOfExperience", alias = "years_of_experience")]
    pub years_of_experience: Option<usize>,
    pub link: Option<String>,
    pub category: Option<UserPortfolioCategory>,
    pub thumbnail: Option<String>,
    pub skills: Option<Vec<UserSkill>>,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct UserPortfolioInputVars {
    #[serde(rename = "portfolioItem")]
    pub portfolio_item: UserPortfolioInput,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct UserPortfolioInput {
    pub title: String,
    pub description: String,
    #[serde(rename = "startDate", alias = "start_date")]
    pub start_date: String,
    #[serde(rename = "endDate", alias = "end_date")]
    pub end_date: Option<String>,
    pub link: String,
    pub category: UserPortfolioCategory,
    pub thumbnail: String,
}

#[derive(Clone, Debug, Copy, Eq, PartialEq, Serialize, Deserialize)]
pub enum UserPortfolioCategory {
    JavaScript,
    Rust,
    Database,
    DevOps,
    Cloud,
    Mobile,
}

impl EnumerableEnum for UserPortfolioCategory {
    fn variants_slice() -> Vec<String> {
        vec![
            String::new(),
            format!("{:?}", Self::JavaScript),
            format!("{:?}", Self::Rust),
            format!("{:?}", Self::Database),
            format!("{:?}", Self::DevOps),
            format!("{:?}", Self::Cloud),
            format!("{:?}", Self::Mobile),
        ]
    }
}

/* This is the beginning of UserProfessionalInfo GraphQL schema */
#[derive(Debug, Serialize, Deserialize)]
#[allow(dead_code)]
pub struct CreateProfessionalDetailsResponse {
    #[serde(rename = "createProfessionalDetails")]
    pub create_professional_details: Option<UserProfessionalInfo>, // this is the return type expected from the API on success, the key is the resolver name
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProfessionalDetailsInputVars {
    #[serde(rename = "professionalDetails")]
    pub professional_details: UserProfessionalInfoInput,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct UserProfessionalInfoInput {
    pub occupation: String,
    pub description: String,
    pub active: bool,
    #[serde(rename = "startDate", alias = "start_date")]
    pub start_date: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[allow(dead_code)]
pub struct UserProfessionalInfo {
    pub id: Option<String>,
    pub occupation: Option<String>,
    pub description: Option<String>,
    #[serde(rename = "startDate", alias = "start_date")]
    pub start_date: Option<String>,
    #[serde(rename = "yearsOfExperience", alias = "years_of_experience")]
    pub years_of_experience: Option<usize>,
    pub active: Option<bool>,
}

/* This is the beginning of UserService GraphQL schema */
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[allow(dead_code)]
pub struct CreateUserServiceResponse {
    #[serde(rename = "createUserService")]
    pub create_user_service: Option<UserService>, // this is the return type expected from the API on success, the key is the resolver name
}

// This struct name should match the variables arg in the cynic macro of the corresponding query fragment
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct UserServiceInputVars {
    #[serde(rename = "userService")]
    pub user_service: UserServiceInput, // The key should match the value provided in the corresponding query fragment
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct UserServiceInput {
    pub title: String,
    pub description: String,
    pub thumbnail: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[allow(dead_code)]
pub struct UserService {
    pub id: Option<String>,
    pub title: Option<String>,
    pub description: Option<String>,
    pub thumbnail: Option<String>,
}

/* This is the beginning of Resume GraphQL schema */
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[allow(dead_code)]
pub struct CreateResumeItemResponse {
    #[serde(rename = "createResumeItem")]
    pub create_resume_item: Option<UserResume>, // this is the return type expected from the API on success, the key is the resolver name
}

// This struct name should match the variables arg in the cynic macro of the corresponding query fragment
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct ResumeItemInputVars {
    #[serde(rename = "resumeItem")]
    pub resume_item: UserResumeInput, // The key should match the value provided in the corresponding query fragment
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct UserResumeInput {
    pub title: String,
    #[serde(rename = "moreInfo", alias = "more_info")]
    pub more_info: String,
    #[serde(rename = "startDate", alias = "start_date")]
    pub start_date: String,
    #[serde(rename = "endDate", alias = "end_date")]
    pub end_date: String,
    pub link: String,
    pub section: UserResumeSection,
}

#[derive(Clone, Debug, Copy, Eq, PartialEq, Deserialize, Serialize)]
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

impl EnumerableEnum for UserResumeSection {
    fn variants_slice() -> Vec<String> {
        vec![
            String::new(),
            format!("{:?}", Self::Education),
            format!("{:?}", Self::Experience),
            format!("{:?}", Self::Achievements),
            format!("{:?}", Self::Projects),
            format!("{:?}", Self::Certifications),
            format!("{:?}", Self::Volunteer),
            format!("{:?}", Self::Publications),
            format!("{:?}", Self::Languages),
            format!("{:?}", Self::Interests),
            format!("{:?}", Self::References),
        ]
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[allow(dead_code)]
pub struct UserResume {
    pub id: Option<String>,
    pub title: Option<String>,
    #[serde(rename = "moreinfo", alias = "more_info")]
    pub more_info: Option<String>,
    #[serde(rename = "startDate", alias = "start_date")]
    pub start_date: Option<String>,
    #[serde(rename = "endDate", alias = "end_date")]
    pub end_date: Option<String>,
    #[serde(rename = "yearsOfExperience", alias = "years_of_experience")]
    pub years_of_experience: Option<usize>,
    pub link: Option<String>,
    pub section: Option<UserResumeSection>,
    pub achievements: Option<Vec<ResumeAchievement>>,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[allow(dead_code)]
pub struct ResumeAchievement {
    pub id: Option<String>,
    pub description: Option<String>,
}

/* This is the beginning of UserSkill GraphQL schema */
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[allow(dead_code)]
pub struct CreateUserSkillResponse {
    #[serde(rename = "createSkill")]
    pub create_skill: Option<UserSkill>, // this is the return type expected from the API on success, the key is the resolver name
}

// This struct name should match the variables arg in the cynic macro of the corresponding query fragment
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct CreateUserSkillVars {
    pub skill: UserSkillInput, // The key should match the value provided in the corresponding query fragment
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct UserSkillInput {
    pub name: String,
    #[serde(rename = "type")]
    pub r#type: UserSkillType,
    pub level: UserSkillLevel,
    #[serde(rename = "startDate", alias = "start_date")]
    pub start_date: String,
    pub thumbnail: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[allow(dead_code)]
pub struct UserSkill {
    pub id: Option<String>,
    pub thumbnail: Option<String>,
    pub name: Option<String>,
    pub level: Option<UserSkillLevel>,
    #[serde(rename = "type")]
    pub r#type: Option<UserSkillType>,
    #[serde(rename = "startDate")]
    pub start_date: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub enum UserSkillLevel {
    Beginner,
    Intermediate,
    Advanced,
    Expert,
}

impl EnumerableEnum for UserSkillLevel {
    fn variants_slice() -> Vec<String> {
        vec![
            String::new(),
            format!("{:?}", Self::Beginner),
            format!("{:?}", Self::Intermediate),
            format!("{:?}", Self::Advanced),
            format!("{:?}", Self::Expert),
        ]
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub enum UserSkillType {
    Technical,
    Soft,
}

impl EnumerableEnum for UserSkillType {
    fn variants_slice() -> Vec<String> {
        vec![
            String::new(),
            format!("{:?}", Self::Technical),
            format!("{:?}", Self::Soft),
        ]
    }
}

/* This is the beginning of Blog GraphQL schema */
#[derive(Debug, Deserialize, Serialize)]
pub struct CreateBlogPostResponse {
    #[serde(rename = "createBlogPost")]
    pub create_blog_post: Option<BlogPost>, // this is the return type expected from the API on success
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateBlogPostVars {
    #[serde(rename = "blogPost")]
    pub blog_post: BlogPostInput,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct BlogPostInput {
    pub title: String,
    #[serde(rename = "shortDescription", alias = "short_description")]
    pub short_description: String,
    pub status: BlogStatus,
    pub thumbnail: String,
    #[serde(rename = "contentFile", alias = "content_file")]
    pub content_file: String,
    pub category: BlogCategory,
    #[serde(rename = "isFeatured", alias = "is_featured")]
    pub is_featured: Option<bool>,
    #[serde(rename = "isPremium", alias = "is_premium")]
    pub is_premium: Option<bool>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub enum BlogStatus {
    Draft,
    Published,
    Archived,
}

impl EnumerableEnum for BlogStatus {
    fn variants_slice() -> Vec<String> {
        vec![
            String::new(),
            format!("{:?}", Self::Draft),
            format!("{:?}", Self::Published),
            format!("{:?}", Self::Archived),
        ]
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub enum BlogCategory {
    WebDevelopment,
    MobileDevelopment,
    ArtificialIntelligence,
    Technology,
    Lifestyle,
}

impl EnumerableEnum for BlogCategory {
    fn variants_slice() -> Vec<String> {
        vec![
            String::new(),
            format!("{:?}", Self::WebDevelopment),
            format!("{:?}", Self::MobileDevelopment),
            format!("{:?}", Self::ArtificialIntelligence),
            format!("{:?}", Self::Technology),
            format!("{:?}", Self::Lifestyle),
        ]
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[allow(dead_code)]
pub struct BlogPost {
    pub id: Option<String>,
    pub title: Option<String>,
    #[serde(rename = "shortDescription")]
    pub short_description: Option<String>,
    pub status: Option<BlogStatus>,
    pub thumbnail: Option<String>,
    pub content: Option<String>,
    pub category: Option<BlogCategory>,
    pub link: Option<String>,
    #[serde(rename = "publishedDate")]
    pub published_date: Option<String>,
    #[serde(rename = "isFeatured")]
    pub is_featured: Option<bool>,
    #[serde(rename = "isPremium")]
    pub is_premium: Option<bool>,
    pub comments: Option<Vec<BlogComment>>,
    #[serde(rename = "createdAt")]
    pub created_at: Option<String>,
    #[serde(rename = "updatedAt")]
    pub updated_at: Option<String>,
    #[serde(rename = "contentFile")]
    pub content_file: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[allow(dead_code)]
pub struct BlogComment {
    pub content: Option<String>,
    pub id: Option<String>,
}

/* This is a Query for UserResources */
#[derive(Debug, Serialize, Deserialize)]
pub struct FetchSiteResourcesResponse {
    #[serde(rename = "fetchSiteResources")]
    pub fetch_site_resources: Option<PublicSiteResources>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PublicSiteResources {
    #[serde(rename = "blogPosts")]
    pub blog_posts: Option<Vec<BlogPost>>,
    #[serde(rename = "professionalInfo")]
    pub professional_info: Option<Vec<UserProfessionalInfo>>,
    pub portfolio: Option<Vec<UserPortfolio>>,
    pub resume: Option<Vec<UserResume>>,
    pub skills: Option<Vec<UserSkill>>,
    pub services: Option<Vec<UserService>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserResourcesVars {
    #[serde(rename = "userId")]
    user_id: String,
}
