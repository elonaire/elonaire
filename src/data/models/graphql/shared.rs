use serde::{Deserialize, Serialize};

/* This is the beginning of UserPortfolio GraphQL schema */
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[allow(dead_code)]
pub struct CreatePortfolioItemResponse {
    #[serde(rename = "createPortfolioItem")]
    pub create_portfolio_item: UserPortfolio, // this is the return type expected from the API on success, the key is the resolver name
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[allow(dead_code)]
pub struct UserPortfolio {
    pub id: String,
    pub title: String,
    pub description: String,
    #[serde(rename = "startDate", alias = "start_date")]
    pub start_date: String,
    #[serde(rename = "endDate", alias = "end_date")]
    pub end_date: Option<String>,
    #[serde(rename = "yearsOfExperience", alias = "years_of_experience")]
    pub years_of_experience: Option<usize>,
    pub link: Option<String>,
    pub category: UserPortfolioCategory,
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

/* This is the beginning of UserProfessionalInfo GraphQL schema */
#[derive(Debug, Serialize, Deserialize)]
#[allow(dead_code)]
pub struct CreateProfessionalDetailsResponse {
    #[serde(rename = "createProfessionalDetails")]
    pub create_professional_details: UserProfessionalInfo, // this is the return type expected from the API on success, the key is the resolver name
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
    pub id: String,
    pub occupation: String,
    pub description: String,
    #[serde(rename = "startDate", alias = "start_date")]
    pub start_date: String,
    #[serde(rename = "yearsOfExperience", alias = "years_of_experience")]
    pub years_of_experience: Option<usize>,
    pub active: bool,
}

/* This is the beginning of UserService GraphQL schema */
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[allow(dead_code)]
pub struct CreateUserServiceResponse {
    #[serde(rename = "createUserService")]
    pub create_user_service: UserService, // this is the return type expected from the API on success, the key is the resolver name
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
    pub id: String,
    pub title: String,
    pub description: String,
    pub thumbnail: String,
}

/* This is the beginning of Resume GraphQL schema */
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[allow(dead_code)]
pub struct CreateResumeItemResponse {
    #[serde(rename = "createResumeItem")]
    pub create_resume_item: UserResume, // this is the return type expected from the API on success, the key is the resolver name
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

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[allow(dead_code)]
pub struct UserResume {
    pub id: String,
    pub title: String,
    #[serde(rename = "moreinfo", alias = "more_info")]
    pub more_info: Option<String>,
    #[serde(rename = "startDate", alias = "start_date")]
    pub start_date: String,
    #[serde(rename = "endDate", alias = "end_date")]
    pub end_date: Option<String>,
    #[serde(rename = "yearsOfExperience", alias = "years_of_experience")]
    pub years_of_experience: Option<usize>,
    pub link: Option<String>,
    pub section: UserResumeSection,
    pub achievements: Option<Vec<ResumeAchievement>>,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[allow(dead_code)]
pub struct ResumeAchievement {
    pub id: String,
    pub description: String,
}

/* This is the beginning of UserSkill GraphQL schema */
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[allow(dead_code)]
pub struct CreateUserSkillResponse {
    #[serde(rename = "createSkill")]
    pub create_skill: UserSkill, // this is the return type expected from the API on success, the key is the resolver name
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
    pub id: String,
    pub thumbnail: String,
    pub name: String,
    pub level: Option<UserSkillLevel>,
    #[serde(rename = "type")]
    pub r#type: UserSkillType,
    #[serde(rename = "startDate")]
    pub start_date: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub enum UserSkillLevel {
    Beginner,
    Intermediate,
    Advanced,
    Expert,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub enum UserSkillType {
    Technical,
    Soft,
}

/* This is the beginning of Blog GraphQL schema */
#[derive(Debug, Deserialize, Serialize)]
pub struct CreateBlogPostResponse {
    #[serde(rename = "createBlogPost")]
    pub create_blog_post: BlogPost, // this is the return type expected from the API on success
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

#[derive(Clone, Copy, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub enum BlogCategory {
    WebDevelopment,
    MobileDevelopment,
    ArtificialIntelligence,
    Technology,
    Lifestyle,
}

#[derive(Debug, Serialize, Deserialize)]
#[allow(dead_code)]
pub struct BlogPost {
    pub id: String,
    pub title: String,
    #[serde(rename = "shortDescription")]
    pub short_description: String,
    pub status: Option<BlogStatus>,
    pub thumbnail: String,
    pub content: Option<String>,
    pub category: BlogCategory,
    pub link: String,
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
    pub content: String,
    pub id: String,
}

/* This is a Query for UserResources */
// #[derive(cynic::QueryFragment, Debug)]
// #[cynic(
//     graphql_type = "Query",
//     schema = "shared",
//     variables = "UserResourcesArguments"
// )]
// #[allow(dead_code)]
// pub struct FetchUserResources {
//     #[arguments(userId: $user_id)]
//     pub fetch_user_resources: UserResources,
// }

// #[derive(cynic::QueryFragment, Debug)]
// #[cynic(schema = "shared")]
// #[allow(dead_code)]
// pub struct UserResources {
//     blog_posts: Vec<BlogPost>,
//     professional_info: Vec<UserProfessionalInfo>,
//     portfolio: Vec<UserPortfolio>,
//     resume: Vec<UserResume>,
//     skills: Vec<UserSkill>,
//     services: Vec<UserService>,
// }

// #[derive(cynic::QueryVariables)]
// pub struct UserResourcesVars {
//     should_include: bool,
// }

// // This struct name should match the variables arg in the cynic macro of the corresponding query fragment
// #[derive(cynic::QueryVariables, Debug)]
// #[allow(dead_code)]
// pub struct UserResourcesArguments {
//     pub user_id: String, // The key should match the value provided in the corresponding query fragment
// }
