use std::fmt::{self, Display};

use reactive_stores::Store;
use serde::{Deserialize, Serialize};

use crate::{
    data::models::{general::shared::ApiResponse, graphql::acl::User},
    utils::custom_traits::EnumerableEnum,
};

/* This is the beginning of UserPortfolio GraphQL schema */
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize, Default)]
#[allow(dead_code)]
pub struct CreatePortfolioItemResponse {
    #[serde(rename = "createPortfolioItem")]
    pub create_portfolio_item: Option<ApiResponse<UserPortfolio>>, // this is the return type expected from the API on success, the key is the resolver name
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize, Default)]
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
    pub skills: Vec<String>,
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
    fn variants_slice() -> Vec<Self> {
        vec![
            Self::JavaScript,
            Self::Rust,
            Self::Database,
            Self::DevOps,
            Self::Cloud,
            Self::Mobile,
        ]
    }
}

impl Display for UserPortfolioCategory {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            any_other => write!(f, "{any_other:?}"),
        }
    }
}

/* This is the beginning of UserProfessionalInfo GraphQL schema */
#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[allow(dead_code)]
pub struct CreateProfessionalDetailsResponse {
    #[serde(rename = "createProfessionalDetails")]
    pub create_professional_details: Option<ApiResponse<UserProfessionalInfo>>, // this is the return type expected from the API on success, the key is the resolver name
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

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize, Default)]
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
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize, Default)]
#[allow(dead_code)]
pub struct CreateUserServiceResponse {
    #[serde(rename = "createUserService")]
    pub create_user_service: Option<ApiResponse<UserService>>, // this is the return type expected from the API on success, the key is the resolver name
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

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize, Store, Default)]
#[allow(dead_code)]
pub struct UserService {
    pub id: Option<String>,
    pub title: Option<String>,
    pub description: Option<String>,
    pub thumbnail: Option<String>,
}

/* This is the beginning of Resume GraphQL schema */
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize, Default)]
#[allow(dead_code)]
pub struct CreateResumeItemResponse {
    #[serde(rename = "createResumeItem")]
    pub create_resume_item: Option<ApiResponse<UserResume>>, // this is the return type expected from the API on success, the key is the resolver name
}

// This struct name should match the variables arg in the cynic macro of the corresponding query fragment
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct ResumeItemInputVars {
    #[serde(rename = "resumeItem")]
    pub resume_item: UserResumeInput, // The key should match the value provided in the corresponding query fragment
    pub achievements: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct UserResumeInput {
    pub title: String,
    #[serde(rename = "moreInfo", alias = "more_info")]
    pub more_info: Option<String>,
    #[serde(rename = "startDate", alias = "start_date")]
    pub start_date: String,
    #[serde(rename = "endDate", alias = "end_date")]
    pub end_date: Option<String>,
    pub link: Option<String>,
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
    fn variants_slice() -> Vec<Self> {
        vec![
            Self::Education,
            Self::Experience,
            Self::Achievements,
            Self::Projects,
            Self::Certifications,
            Self::Volunteer,
            Self::Publications,
            Self::Languages,
            Self::Interests,
            Self::References,
        ]
    }
}

impl Display for UserResumeSection {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            any_other => write!(f, "{any_other:?}"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize, Default)]
#[allow(dead_code)]
pub struct UserResume {
    pub id: Option<String>,
    pub title: Option<String>,
    #[serde(rename = "moreInfo", alias = "more_info")]
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

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize, Default)]
#[allow(dead_code)]
pub struct ResumeAchievement {
    pub id: Option<String>,
    pub description: Option<String>,
}

/* This is the beginning of UserSkill GraphQL schema */
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize, Default)]
#[allow(dead_code)]
pub struct CreateUserSkillResponse {
    #[serde(rename = "createSkill")]
    pub create_skill: Option<ApiResponse<UserSkill>>, // this is the return type expected from the API on success, the key is the resolver name
}

// This struct name should match the variables arg in the cynic macro of the corresponding query fragment
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct CreateUserSkillVars {
    pub skill: UserSkillInput, // The key should match the value provided in the corresponding query fragment
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct UserSkillInput {
    pub name: String,
    pub description: String,
    #[serde(rename = "type")]
    pub r#type: UserSkillType,
    pub level: UserSkillLevel,
    #[serde(rename = "startDate", alias = "start_date")]
    pub start_date: String,
    pub thumbnail: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize, Default)]
#[allow(dead_code)]
pub struct UserSkill {
    pub id: Option<String>,
    pub thumbnail: Option<String>,
    pub name: Option<String>,
    pub description: Option<String>,
    pub level: Option<UserSkillLevel>,
    #[serde(rename = "type")]
    pub r#type: Option<UserSkillType>,
    #[serde(rename = "startDate")]
    pub start_date: Option<String>,
    #[serde(rename = "yearsOfExperience", alias = "years_of_experience")]
    pub years_of_experience: Option<usize>,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub enum UserSkillLevel {
    Beginner,
    Intermediate,
    Advanced,
    Expert,
}

impl EnumerableEnum for UserSkillLevel {
    fn variants_slice() -> Vec<Self> {
        vec![
            Self::Beginner,
            Self::Intermediate,
            Self::Advanced,
            Self::Expert,
        ]
    }
}

impl Display for UserSkillLevel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            any_other => write!(f, "{any_other:?}"),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub enum UserSkillType {
    Technical,
    Soft,
}

impl EnumerableEnum for UserSkillType {
    fn variants_slice() -> Vec<Self> {
        vec![Self::Technical, Self::Soft]
    }
}

impl Display for UserSkillType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            any_other => write!(f, "{any_other:?}"),
        }
    }
}

/* This is the beginning of Blog GraphQL schema */
#[derive(Debug, Deserialize, Serialize, Clone, Default)]
pub struct CreateBlogPostResponse {
    #[serde(rename = "createBlogPost")]
    pub create_blog_post: Option<ApiResponse<BlogPost>>, // this is the return type expected from the API on success
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
    pub content: String,
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
    fn variants_slice() -> Vec<Self> {
        vec![Self::Draft, Self::Published, Self::Archived]
    }
}

impl Display for BlogStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            any_other => write!(f, "{any_other:?}"),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub enum BlogCategory {
    WebDevelopment,
    MobileDevelopment,
    ArtificialIntelligence,
    Technology,
    Lifestyle,
    Science,
    Health,
    EmbeddedSystems,
    IoT,
    UpCloseAndCandid,
    Commentary,
    CyberSecurity,
    Programming,
}

impl EnumerableEnum for BlogCategory {
    fn variants_slice() -> Vec<Self> {
        vec![
            Self::WebDevelopment,
            Self::MobileDevelopment,
            Self::ArtificialIntelligence,
            Self::Technology,
            Self::Lifestyle,
            Self::Science,
            Self::Health,
            Self::EmbeddedSystems,
            Self::IoT,
            Self::UpCloseAndCandid,
            Self::Commentary,
            Self::CyberSecurity,
            Self::Programming,
        ]
    }
}

impl Display for BlogCategory {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ArtificialIntelligence => write!(f, "Artificial Intelligence"),
            Self::WebDevelopment => write!(f, "Web Development"),
            Self::MobileDevelopment => write!(f, "Mobile Development"),
            Self::EmbeddedSystems => write!(f, "Embedded Systems"),
            Self::UpCloseAndCandid => write!(f, "Up Close and Candid"),
            Self::CyberSecurity => write!(f, "Cyber Security"),
            any_other => write!(f, "{any_other:?}"),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
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
    pub author: Option<String>,
    #[serde(skip_serializing)]
    pub full_author_details: Option<User>,
    #[serde(rename = "readTime")]
    pub read_time: Option<u32>,
    #[serde(rename = "reactionCount")]
    pub reaction_count: Option<u32>,
    #[serde(rename = "currentUserReaction")]
    pub current_user_reaction: Option<Reaction>,
    #[serde(rename = "bookmarksCount")]
    pub bookmarks_count: Option<u32>,
    #[serde(rename = "sharesCount")]
    pub shares_count: Option<u32>,
    #[serde(rename = "currentUserBookmarked")]
    pub current_user_bookmarked: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[allow(dead_code)]
pub struct BlogComment {
    pub content: Option<String>,
    pub id: Option<String>,
    #[serde(skip_serializing)]
    pub full_author_details: Option<User>,
    #[serde(rename = "replyCount")]
    pub reply_count: Option<u32>,
    pub author: Option<String>,
    #[serde(rename = "createdAt")]
    pub created_at: Option<String>,
    #[serde(rename = "updatedAt")]
    pub updated_at: Option<String>,
    #[serde(rename = "currentUserReaction")]
    pub current_user_reaction: Option<Reaction>,
    #[serde(rename = "reactionCount")]
    pub reaction_count: Option<u32>,
}

/* This is a Query for UserResources */
#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct FetchSiteResourcesResponse {
    #[serde(rename = "fetchSiteResources")]
    pub fetch_site_resources: Option<ApiResponse<PublicSiteResources>>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
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

#[derive(Debug, Deserialize, Serialize, Clone, Default)]
pub struct FetchRatecardsResponse {
    #[serde(rename = "fetchRatecards")]
    pub fetch_ratecards: Option<ApiResponse<Vec<Ratecard>>>, // this is the return type expected from the API on success
}

#[derive(Clone, Debug, Serialize, Deserialize, Eq, PartialEq, Store, Default)]
pub struct Ratecard {
    pub id: Option<String>,
    pub name: Option<String>,
    pub services: Option<Vec<UserService>>,
    #[serde(rename = "createdAt")]
    pub created_at: Option<String>,
    #[serde(rename = "updatedAt")]
    pub updated_at: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RatecardInput {
    pub name: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RatecardInputMetadata {
    #[serde(rename = "serviceIds")]
    pub service_ids: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateRatecardVars {
    #[serde(rename = "ratecardInput")]
    pub ratecard_input: RatecardInput,
    #[serde(rename = "ratecardInputMetadata")]
    pub ratecard_input_metadata: RatecardInputMetadata,
}

#[derive(Debug, Deserialize, Serialize, Clone, Default)]
pub struct CreateRatecardResponse {
    #[serde(rename = "createRatecard")]
    pub create_ratecard: Option<ApiResponse<Ratecard>>, // this is the return type expected from the API on success
}

#[derive(Debug, Deserialize, Serialize, Clone, Default)]
pub struct FetchBillingRateResponse {
    #[serde(rename = "fetchBillingRate")]
    pub fetch_billing_rate: Option<ApiResponse<String>>, // this is the return type expected from the API on success
}

#[derive(Clone, Debug, Serialize, Deserialize, Copy, Eq, PartialEq)]
pub enum BillingInterval {
    Hourly,
    Weekly,
    Monthly,
    Annual,
    Milestone,
}

impl EnumerableEnum for BillingInterval {
    fn variants_slice() -> Vec<Self> {
        vec![
            Self::Hourly,
            Self::Weekly,
            Self::Monthly,
            Self::Annual,
            Self::Milestone,
        ]
    }
}

impl Display for BillingInterval {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            any_other => write!(f, "{any_other:?}"),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FetchBillingRateVars {
    #[serde(rename = "billingInterval")]
    pub billing_interval: BillingInterval,
    #[serde(rename = "serviceIds")]
    pub service_ids: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BillingIntervalForm {
    pub billing_interval: BillingInterval,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ServiceIdsForm {
    pub service_ids: Vec<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ServiceRequestInput {
    pub description: String,
    #[serde(rename = "startDate", alias = "start_date")]
    pub start_date: String,
    #[serde(rename = "engagementLength", alias = "engagement_length")]
    pub engagement_length: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ServiceRequestInputMetadata {
    #[serde(rename = "supportingDocsFileIds", alias = "supporting_docs_file_ids")]
    pub supporting_docs_file_ids: Vec<String>,
    #[serde(rename = "serviceIds", alias = "service_ids")]
    pub service_ids: Vec<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize, Eq, PartialEq, Store)]
pub struct UploadedFileId {
    pub id: String,
    #[serde(rename = "fileId")]
    pub file_id: String,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq, Store)]
pub struct ServiceRequest {
    pub id: Option<String>,
    pub description: Option<String>,
    #[serde(rename = "supportingDocs")]
    pub supporting_docs: Option<Vec<UploadedFileId>>,
    #[serde(rename = "startDate")]
    pub start_date: Option<String>,
    #[serde(rename = "engagementLength")]
    pub engagement_length: Option<String>,
    #[serde(rename = "createdAt")]
    pub created_at: Option<String>,
    #[serde(rename = "updatedAt")]
    pub updated_at: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateServiceRequestVars {
    #[serde(rename = "serviceRequestInput")]
    pub service_request_input: ServiceRequestInput,
    #[serde(rename = "serviceRequestInputMetadata")]
    pub service_request_input_metadata: ServiceRequestInputMetadata,
}

#[derive(Debug, Deserialize, Serialize, Clone, Default)]
pub struct CreateServiceRequestResponse {
    #[serde(rename = "createServiceRequest")]
    pub create_service_request: Option<ApiResponse<ServiceRequest>>, // this is the return type expected from the API on success
}

#[derive(Debug, Deserialize, Serialize, Clone, Default)]
pub struct FetchServiceRequestsResponse {
    #[serde(rename = "fetchServiceRequests")]
    pub fetch_service_requests: Option<ApiResponse<Vec<ServiceRequest>>>, // this is the return type expected from the API on success
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ServiceRateInput {
    #[serde(rename = "baseRate", alias = "base_rate")]
    pub base_rate: String,
    #[serde(rename = "hourWeek", alias = "hour_week")]
    pub hour_week: Option<u8>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ServiceRateInputMetadata {
    #[serde(rename = "serviceId")]
    pub service_id: String,
    #[serde(rename = "currencyId")]
    pub currency_id: String,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq, Store)]
pub struct ServiceRate {
    pub id: Option<String>,
    pub service: Option<UserService>,
    #[serde(rename = "baseRate")]
    pub base_rate: Option<String>,
    #[serde(rename = "hourWeek")]
    pub hour_week: Option<u8>,
    #[serde(rename = "currencyId")]
    pub currency_id: Option<CurrencyId>,
    #[serde(rename = "createdAt")]
    pub created_at: Option<String>,
    #[serde(rename = "updatedAt")]
    pub updated_at: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq, Store)]
pub struct CurrencyId {
    pub id: Option<String>,
    #[serde(rename = "currencyId")]
    pub currency_id: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateServiceRateVars {
    #[serde(rename = "serviceRateInput")]
    pub service_rate_input: ServiceRateInput,
    #[serde(rename = "serviceRateInputMetadata")]
    pub service_rate_input_metadata: ServiceRateInputMetadata,
}

#[derive(Debug, Deserialize, Serialize, Clone, Default)]
pub struct CreateServiceRateResponse {
    #[serde(rename = "createServiceRate")]
    pub create_service_rate: Option<ApiResponse<ServiceRate>>, // this is the return type expected from the API on success
}

#[derive(Debug, Deserialize, Serialize, Clone, Default)]
pub struct FetchServiceRatesResponse {
    #[serde(rename = "fetchServiceRates")]
    pub fetch_service_rates: Option<ApiResponse<Vec<ServiceRate>>>, // this is the return type expected from the API on success
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq, Store)]
pub struct Currency {
    pub id: Option<String>,
    pub code: Option<String>,
    pub numeric: Option<String>,
    pub name: Option<String>,
    pub symbol: Option<String>,
    #[serde(rename = "createdAt")]
    pub created_at: Option<String>,
    #[serde(rename = "updatedAt")]
    pub updated_at: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, Clone, Default)]
pub struct FetchCurrenciesResponse {
    #[serde(rename = "fetchCurrencies")]
    pub fetch_currencies: Option<ApiResponse<Vec<Currency>>>, // this is the return type expected from the API on success
}

#[derive(Debug, Deserialize, Serialize, Clone, Default)]
pub struct FetchBlogPostsResponse {
    #[serde(rename = "fetchBlogPosts")]
    pub fetch_blog_posts: Option<ApiResponse<Vec<BlogPost>>>, // this is the return type expected from the API on success
}

#[derive(Debug, Deserialize, Serialize, Clone, Default)]
pub struct FetchSingleBlogPostResponse {
    #[serde(rename = "fetchSingleBlogPost")]
    pub fetch_single_blog_post: Option<ApiResponse<BlogPost>>, // this is the return type expected from the API on success
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FetchSingleBlogPostVars {
    #[serde(rename = "blogIdOrSlug")]
    pub blog_id_or_slug: String,
}

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct FetchBlogPostsQueryFilters {
    pub status: Option<BlogStatus>,
    #[serde(rename = "isFeatured")]
    pub is_featured: Option<bool>,
    // pub is_premium: Option<bool>,
    #[serde(rename = "sortConfigs")]
    pub sort_configs: Option<SortConfigs>,
    #[serde(rename = "searchTerm")]
    pub search_term: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize, Copy, Eq, PartialEq)]
pub enum BlogPostsFilterSortBy {
    DateOfCreation,
    Reads,
}

#[derive(Clone, Debug, Serialize, Deserialize, Copy, Eq, PartialEq)]
pub enum SortOrder {
    Asc,
    Desc,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SortConfigs {
    #[serde(rename = "sortBy")]
    pub sort_by: BlogPostsFilterSortBy,
    #[serde(rename = "sortOrder")]
    pub sort_order: SortOrder,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FetchBlogPostsVars {
    pub filters: FetchBlogPostsQueryFilters,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ReactionInput {
    pub r#type: ReactionType,
}

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct Reaction {
    pub id: String,
    pub r#type: ReactionType,
}

// enum for ReactionType
#[derive(Clone, Debug, Serialize, Deserialize, Copy, Eq, PartialEq, Default)]
pub enum ReactionType {
    #[default]
    Like,
    Dislike,
    Love,
    Haha,
    Wow,
    Sad,
    Angry,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct BlogCommentInput {
    pub content: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateBlogCommentVars {
    #[serde(rename = "blogComment")]
    pub blog_comment: BlogCommentInput,
    #[serde(rename = "blogPostId")]
    pub blog_post_id: String,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct CreateBlogCommentResponse {
    #[serde(rename = "addCommentToBlogPost")]
    pub add_comment_to_blog_post: Option<ApiResponse<BlogComment>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ReactToBlogPostVars {
    pub reaction: ReactionInput,
    #[serde(rename = "blogPostId")]
    pub blog_post_id: String,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ReactToBlogPostResponse {
    #[serde(rename = "reactToBlogPost")]
    pub react_to_blog_post: Option<ApiResponse<Reaction>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BookmarkBlogPostVars {
    #[serde(rename = "blogPostId")]
    pub blog_post_id: String,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct BookmarkBlogPostResponse {
    #[serde(rename = "bookmarkBlogPost")]
    pub bookmark_blog_post: Option<ApiResponse<bool>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateBlogPostShareCountVars {
    #[serde(rename = "blogPostId")]
    pub blog_post_id: String,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct UpdateBlogPostShareCountResponse {
    #[serde(rename = "updateBlogPostShareCount")]
    pub update_blog_post_share_count: Option<ApiResponse<u32>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ReactToBlogCommentVars {
    pub reaction: ReactionInput,
    #[serde(rename = "commentId")]
    pub comment_id: String,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ReactToBlogCommentResponse {
    #[serde(rename = "reactToBlogComment")]
    pub react_to_blog_comment: Option<ApiResponse<Reaction>>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MessageInput {
    pub subject: Subject,
    pub body: String,
    #[serde(rename = "senderName", alias = "sender_name")]
    pub sender_name: String,
    #[serde(rename = "senderEmail", alias = "sender_email")]
    pub sender_email: String,
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

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Message {
    pub id: Option<String>,
    pub subject: Option<Subject>,
    pub body: Option<String>,
    #[serde(rename = "senderName")]
    pub sender_name: Option<String>,
    #[serde(rename = "senderEmail")]
    pub sender_email: Option<String>,
    #[serde(rename = "createdAt")]
    pub created_at: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SendMessageVars {
    pub message: MessageInput,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct SendMessageResponse {
    #[serde(rename = "sendMessage")]
    pub send_message: Option<ApiResponse<Message>>,
}
