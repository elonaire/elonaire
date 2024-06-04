use serde::{Deserialize, Serialize};
use yew::Properties;

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq, Properties)]
pub struct BlogPost {
    pub id: String,
    pub title: String,
    #[serde(rename = "shortDescription")]
    pub short_description: String,
    pub status: Option<String>,
    pub image: String,
    pub category: BlogCategory,
    pub content: String,
    pub link: String,
    #[serde(rename = "publishedDate")]
    pub published_date: Option<String>,
    #[serde(rename = "createdAt")]
    pub created_at: String,
    pub author: String,
}

#[derive(Debug, Deserialize)]
pub struct GetBlogPostsResponse {
    #[serde(rename = "getBlogPosts")]
    pub get_blog_posts: Vec<BlogPost>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct GetSingleBlogPostResponse {
    #[serde(rename = "getSingleBlogPost")]
    pub get_single_blog_post: BlogPost,
}

#[derive(Clone, Debug, Serialize, Deserialize, Copy, Eq, PartialEq)]
pub enum BlogCategory {
    All,
    WebDevelopment,
    MobileDevelopment,
    ArtificialIntelligence,
    Technology,
    Lifestyle,
    Travel,
    LatestRelease,
}

impl BlogCategory {
    pub fn to_string(&self) -> String {
        match self {
            BlogCategory::All => "All".to_string(),
            BlogCategory::WebDevelopment => "Web Development".to_string(),
            BlogCategory::MobileDevelopment => "Mobile Development".to_string(),
            BlogCategory::ArtificialIntelligence => "Artificial Intelligence".to_string(),
            BlogCategory::Technology => "Technology".to_string(),
            BlogCategory::Lifestyle => "Lifestyle".to_string(),
            BlogCategory::Travel => "Travel".to_string(),
            BlogCategory::LatestRelease => "Latest Release".to_string(),
        }
    }
}
