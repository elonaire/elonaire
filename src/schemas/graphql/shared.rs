use serde::Deserialize;

// Pull in the SHARED schema we registered in build.rs
#[cynic::schema("shared")]
mod schema {}

#[derive(cynic::QueryFragment, Debug)]
#[cynic(
    schema = "shared",
    graphql_type = "Mutation",
    variables = "UserPortfolioInputFields" // these are the query variables for the mutation, and a corresponding struct with the same needs to be defined
)]
pub struct AddPortfolioItem {
    #[arguments(portfolioItem: $portfolio_item)]
    pub add_portfolio_item: Vec<UserPortfolio>, // this is the return type expected from the API on success
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

#[derive(cynic::Enum, Clone, Copy, Debug)]
#[cynic(rename_all = "None", schema = "shared")]
pub enum UserSkillLevel {
    Beginner,
    Intermediate,
    Advanced,
    Expert,
}

#[derive(cynic::Enum, Clone, Copy, Debug)]
#[cynic(rename_all = "None", schema = "shared")]
pub enum UserSkillType {
    Technical,
    Soft,
}
