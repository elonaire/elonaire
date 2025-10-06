use serde::Deserialize;

// Pull in the SHARED schema we registered in build.rs
#[cynic::schema("shared")]
mod schema {}

/* This is the beginning of UserPortfolio GraphQL schema */
#[derive(cynic::QueryFragment, Debug)]
#[cynic(
    schema = "shared",
    graphql_type = "Mutation",
    variables = "UserPortfolioInputArguments" // these are the query variables for the mutation, and a corresponding struct with the same needs to be defined
)]
#[allow(dead_code)]
pub struct CreatePortfolioItem {
    #[arguments(portfolioItem: $portfolio_item)]
    pub create_portfolio_item: UserPortfolio, // this is the return type expected from the API on success, the key is the resolver name
}

#[derive(cynic::QueryFragment, Debug)]
#[cynic(schema = "shared")]
#[allow(dead_code)]
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
pub struct UserPortfolioInputArguments {
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
    variables = "ProfessionalDetailsInputArguments" // these are the query variables for the mutation, and a corresponding struct with the same needs to be defined
)]
#[allow(dead_code)]
pub struct CreateProfessionalDetails {
    #[arguments(professionalDetails: $professional_details)]
    pub create_professional_details: UserProfessionalInfo, // this is the return type expected from the API on success, the key is the resolver name
}

#[derive(cynic::QueryVariables, Debug)]
pub struct ProfessionalDetailsInputArguments {
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
#[allow(dead_code)]
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
    variables = "UserServiceInputArguments" // these are the query variables for the mutation, and a corresponding struct with the same needs to be defined
)]
#[allow(dead_code)]
pub struct CreateUserService {
    #[arguments(userService: $user_service)]
    pub create_user_service: UserService, // this is the return type expected from the API on success, the key is the resolver name
}

// This struct name should match the variables arg in the cynic macro of the corresponding query fragment
#[derive(cynic::QueryVariables, Debug)]
pub struct UserServiceInputArguments {
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
#[allow(dead_code)]
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
    variables = "ResumeItemInputArguments" // these are the query variables for the mutation, and a corresponding struct with the same needs to be defined
)]
#[allow(dead_code)]
pub struct CreateResumeItem {
    #[arguments(resumeItem: $resume_item)]
    pub create_resume_item: UserResume, // this is the return type expected from the API on success, the key is the resolver name
}

// This struct name should match the variables arg in the cynic macro of the corresponding query fragment
#[derive(cynic::QueryVariables, Debug)]
pub struct ResumeItemInputArguments {
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
#[allow(dead_code)]
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
#[allow(dead_code)]
pub struct ResumeAchievement {
    pub id: String,
    pub description: String,
}

/* This is the beginning of UserSkill GraphQL schema */
#[derive(cynic::QueryFragment, Debug)]
#[cynic(
    schema = "shared",
    graphql_type = "Mutation",
    variables = "UserSkillInputArguments" // these are the query variables for the mutation, and a corresponding struct with the same needs to be defined
)]
#[allow(dead_code)]
pub struct CreateSkill {
    #[arguments(skill: $skill)]
    pub create_skill: UserSkill, // this is the return type expected from the API on success, the key is the resolver name
}

// This struct name should match the variables arg in the cynic macro of the corresponding query fragment
#[derive(cynic::QueryVariables, Debug)]
pub struct UserSkillInputArguments {
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
#[allow(dead_code)]
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
    variables = "BlogPostInputArguments" // these are the query variables for the mutation, and a corresponding struct with the same needs to be defined
)]
#[allow(dead_code)]
pub struct CreateBlogPost {
    #[arguments(blogPost: $blog_post)]
    pub create_blog_post: BlogPost, // this is the return type expected from the API on success, the key is the resolver name
}

// This struct name should match the variables arg in the cynic macro of the corresponding query fragment
#[derive(cynic::QueryVariables, Debug)]
pub struct BlogPostInputArguments {
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
#[allow(dead_code)]
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
#[allow(dead_code)]
pub struct BlogComment {
    pub content: String,
    pub id: String,
}

/* This is a Query for UserResources */
#[derive(cynic::QueryFragment, Debug)]
#[cynic(
    graphql_type = "Query",
    schema = "shared",
    variables = "UserResourcesArguments"
)]
#[allow(dead_code)]
pub struct FetchUserResources {
    #[arguments(userId: $user_id)]
    pub fetch_user_resources: UserResources,
}

#[derive(cynic::QueryFragment, Debug)]
#[cynic(schema = "shared")]
#[allow(dead_code)]
pub struct UserResources {
    blog_posts: Vec<BlogPost>,
    professional_info: Vec<UserProfessionalInfo>,
    portfolio: Vec<UserPortfolio>,
    resume: Vec<UserResume>,
    skills: Vec<UserSkill>,
    services: Vec<UserService>,
}

#[derive(cynic::QueryVariables)]
pub struct UserResourcesVars {
    should_include: bool,
}

// This struct name should match the variables arg in the cynic macro of the corresponding query fragment
#[derive(cynic::QueryVariables, Debug)]
#[allow(dead_code)]
pub struct UserResourcesArguments {
    pub user_id: String, // The key should match the value provided in the corresponding query fragment
}
