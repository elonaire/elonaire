use serde::Deserialize;

// Pull in the SHARED schema we registered in build.rs
#[cynic::schema("shared")]
mod schema {}

/* This is the beginning of UserPortfolio GraphQL schema */
#[derive(cynic::QueryFragment, Debug)]
#[cynic(
    schema = "shared",
    graphql_type = "Mutation",
    variables = "UserPortfolioInputFields" // these are the query variables for the mutation, and a corresponding struct with the same needs to be defined
)]
pub struct CreatePortfolioItem {
    #[arguments(portfolioItem: $portfolio_item)]
    pub create_portfolio_item: UserPortfolio, // this is the return type expected from the API on success, the key is the resolver name
}

#[derive(cynic::QueryFragment, Debug)]
#[cynic(schema = "shared")]
pub struct UserPortfolio {
    pub id: String,
    pub title: String,
    pub description: String,
    pub start_date: String,
    pub end_date: Option<String>,
    pub link: String,
    pub category: UserPortfolioCategory,
    pub thumbnail: String,
    pub skills: Vec<UserSkill>,
}

#[derive(cynic::QueryVariables, Debug)]
pub struct UserPortfolioInputFields {
    pub portfolio_item: UserPortfolioInput,
}

#[derive(cynic::InputObject, Debug, Clone, PartialEq, Eq, Deserialize)]
#[cynic(schema = "shared")]
pub struct UserPortfolioInput {
    pub title: String,
    pub description: String,
    pub start_date: String,
    pub end_date: Option<String>,
    pub link: String,
    pub category: UserPortfolioCategory,
    pub thumbnail: String,
}

#[derive(cynic::Enum, Clone, Debug, Copy, Eq, PartialEq)]
#[cynic(rename_all = "None", schema = "shared")]
pub enum UserPortfolioCategory {
    JavaScript,
    Rust,
    Database,
    DevOps,
    Cloud,
    Mobile,
}

/* This is the beginning of UserProfessionalInfo GraphQL schema */
#[derive(cynic::QueryFragment, Debug)]
#[cynic(
    schema = "shared",
    graphql_type = "Mutation",
    variables = "ProfessionalDetailsInputFields" // these are the query variables for the mutation, and a corresponding struct with the same needs to be defined
)]
pub struct CreateProfessionalDetails {
    #[arguments(professionalDetails: $professional_details)]
    pub create_professional_details: UserProfessionalInfo, // this is the return type expected from the API on success, the key is the resolver name
}

#[derive(cynic::QueryVariables, Debug)]
pub struct ProfessionalDetailsInputFields {
    pub professional_details: UserProfessionalInfoInput,
}

#[derive(cynic::InputObject, Debug, Clone, PartialEq, Eq, Deserialize)]
#[cynic(schema = "shared")]
pub struct UserProfessionalInfoInput {
    pub occupation: String,
    pub description: String,
    pub active: bool,
    pub start_date: String,
}

#[derive(cynic::QueryFragment, Debug)]
#[cynic(schema = "shared")]
pub struct UserProfessionalInfo {
    pub id: String,
    pub occupation: String,
    pub description: String,
    pub start_date: String,
    pub years_of_experience: Option<i32>,
    pub active: bool,
}

/* This is the beginning of UserService GraphQL schema */
#[derive(cynic::QueryFragment, Debug)]
#[cynic(
    schema = "shared",
    graphql_type = "Mutation",
    variables = "UserServiceInputFields" // these are the query variables for the mutation, and a corresponding struct with the same needs to be defined
)]
pub struct CreateUserService {
    #[arguments(userService: $user_service)]
    pub create_user_service: UserService, // this is the return type expected from the API on success, the key is the resolver name
}

// This struct name should match the variables arg in the cynic macro of the corresponding query fragment
#[derive(cynic::QueryVariables, Debug)]
pub struct UserServiceInputFields {
    pub user_service: UserServiceInput, // The key should match the value provided in the corresponding query fragment
}

#[derive(cynic::InputObject, Debug, Clone, PartialEq, Eq, Deserialize)]
#[cynic(schema = "shared")]
pub struct UserServiceInput {
    pub title: String,
    pub description: String,
    pub thumbnail: String,
}

#[derive(cynic::QueryFragment, Debug)]
#[cynic(schema = "shared")]
pub struct UserService {
    pub id: String,
    pub title: String,
    pub description: String,
    pub thumbnail: String,
}

/* This is the beginning of UserService GraphQL schema */
#[derive(cynic::QueryFragment, Debug)]
#[cynic(
    schema = "shared",
    graphql_type = "Mutation",
    variables = "ResumeItemInputFields" // these are the query variables for the mutation, and a corresponding struct with the same needs to be defined
)]
pub struct CreateResumeItem {
    #[arguments(resumeItem: $resume_item)]
    pub create_resume_item: UserResume, // this is the return type expected from the API on success, the key is the resolver name
}

// This struct name should match the variables arg in the cynic macro of the corresponding query fragment
#[derive(cynic::QueryVariables, Debug)]
pub struct ResumeItemInputFields {
    pub resume_item: UserResumeInput, // The key should match the value provided in the corresponding query fragment
}

#[derive(cynic::InputObject, Debug, Clone, PartialEq, Eq, Deserialize)]
#[cynic(schema = "shared")]
pub struct UserResumeInput {
    pub title: String,
    pub more_info: String,
    pub start_date: String,
    pub end_date: String,
    pub link: String,
    pub section: UserResumeSection,
}

#[derive(cynic::Enum, Clone, Debug, Copy, Eq, PartialEq)]
#[cynic(rename_all = "None", schema = "shared")]
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

#[derive(cynic::QueryFragment, Debug)]
#[cynic(schema = "shared")]
pub struct UserResume {
    pub id: String,
    pub title: String,
    pub more_info: Option<String>,
    pub start_date: String,
    pub end_date: Option<String>,
    pub link: Option<String>,
    pub section: UserResumeSection,
    pub achievements: Vec<ResumeAchievement>,
}

#[derive(cynic::QueryFragment, Debug)]
#[cynic(schema = "shared")]
pub struct ResumeAchievement {
    pub id: String,
    pub description: String,
}

/* This is the beginning of UserSkill GraphQL schema */
#[derive(cynic::QueryFragment, Debug)]
#[cynic(
    schema = "shared",
    graphql_type = "Mutation",
    variables = "UserSkillInputFields" // these are the query variables for the mutation, and a corresponding struct with the same needs to be defined
)]
pub struct CreateSkill {
    #[arguments(skill: $skill)]
    pub create_skill: UserSkill, // this is the return type expected from the API on success, the key is the resolver name
}

// This struct name should match the variables arg in the cynic macro of the corresponding query fragment
#[derive(cynic::QueryVariables, Debug)]
pub struct UserSkillInputFields {
    pub skill: UserSkillInput, // The key should match the value provided in the corresponding query fragment
}

#[derive(cynic::InputObject, Debug, Clone, PartialEq, Eq, Deserialize)]
#[cynic(schema = "shared")]
pub struct UserSkillInput {
    pub name: String,
    #[cynic(rename = "type")]
    pub r#type: UserSkillType,
    pub level: UserSkillLevel,
    pub start_date: String,
    pub thumbnail: String,
}

#[derive(cynic::QueryFragment, Debug)]
#[cynic(schema = "shared")]
pub struct UserSkill {
    pub id: String,
    pub thumbnail: String,
    pub name: String,
    pub level: Option<UserSkillLevel>,
    #[cynic(rename = "type")]
    pub r#type: UserSkillType,
    pub start_date: String,
}

#[derive(cynic::Enum, Clone, Copy, Debug, PartialEq, Eq)]
#[cynic(rename_all = "None", schema = "shared")]
pub enum UserSkillLevel {
    Beginner,
    Intermediate,
    Advanced,
    Expert,
}

#[derive(cynic::Enum, Clone, Copy, Debug, PartialEq, Eq)]
#[cynic(rename_all = "None", schema = "shared")]
pub enum UserSkillType {
    Technical,
    Soft,
}

/* This is the beginning of Blog GraphQL schema */
#[derive(cynic::QueryFragment, Debug)]
#[cynic(
    schema = "shared",
    graphql_type = "Mutation",
    variables = "BlogPostInputFields" // these are the query variables for the mutation, and a corresponding struct with the same needs to be defined
)]
pub struct CreateBlogPost {
    #[arguments(blogPost: $blog_post)]
    pub create_blog_post: BlogPost, // this is the return type expected from the API on success, the key is the resolver name
}

// This struct name should match the variables arg in the cynic macro of the corresponding query fragment
#[derive(cynic::QueryVariables, Debug)]
pub struct BlogPostInputFields {
    pub blog_post: BlogPostInput, // The key should match the value provided in the corresponding query fragment
}

#[derive(cynic::InputObject, Debug, Clone, PartialEq, Eq, Deserialize)]
#[cynic(schema = "shared")]
pub struct BlogPostInput {
    pub title: String,
    pub short_description: String,
    pub status: BlogStatus,
    pub thumbnail: String,
    pub content_file: String,
    pub category: BlogCategory,
    pub is_featured: Option<bool>,
    pub is_premium: Option<bool>,
}

#[derive(cynic::Enum, Clone, Copy, Debug, PartialEq, Eq)]
#[cynic(rename_all = "None", schema = "shared")]
pub enum BlogStatus {
    Draft,
    Published,
    Archived,
}

#[derive(cynic::Enum, Clone, Copy, Debug, PartialEq, Eq)]
#[cynic(rename_all = "None", schema = "shared")]
pub enum BlogCategory {
    WebDevelopment,
    MobileDevelopment,
    ArtificialIntelligence,
    Technology,
    Lifestyle,
}

#[derive(cynic::QueryFragment, Debug)]
#[cynic(schema = "shared")]
pub struct BlogPost {
    pub id: String,
    pub title: String,
    pub short_description: String,
    pub status: Option<BlogStatus>,
    pub thumbnail: String,
    pub content: Option<String>,
    pub category: BlogCategory,
    pub link: String,
    pub published_date: Option<String>,
    pub is_featured: Option<bool>,
    pub is_premium: Option<bool>,
    pub comments: Option<Vec<BlogComment>>,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
    pub content_file: String,
}

#[derive(cynic::QueryFragment, Debug)]
#[cynic(schema = "shared")]
pub struct BlogComment {
    pub content: String,
    pub id: String,
}
