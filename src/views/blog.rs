use yew::prelude::*;

use crate::components::{blog_nav::BlogNav, blog::{blog_section::BlogSection, main_banner::MainBanner, blog_post_card::BlogPostCardProps}, line_separator::LineSeparator, ad::AdComponent};

#[derive(Debug, PartialEq, Clone)]
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

#[function_component(Blog)]
pub fn blog() -> Html {
    let posts: Vec<BlogPostCardProps> = vec![
        BlogPostCardProps {
            image_url: "img/unsplash10mwi2uawfg@2x.png".to_string(),
            title: "Miami Dolphins won the match and officially qualified for the final".to_string(),
            description: "Lorem ipsum dolor sit amet, consectetur adipiscing elit".to_string(),
            is_hot_topic: true,
            pub_date: "2021-01-01".to_string(),
            category: BlogCategory::LatestRelease,
        },
        BlogPostCardProps {
            image_url: "img/unsplash10mwi2uawfg@2x.png".to_string(),
            title: "Miami Dolphins won the match and officially qualified for the final".to_string(),
            description: "Lorem ipsum dolor sit amet, consectetur adipiscing elit".to_string(),
            is_hot_topic: false,
            pub_date: "2021-01-01".to_string(),
            category: BlogCategory::WebDevelopment,
        },
        BlogPostCardProps {
            image_url: "img/unsplash10mwi2uawfg@2x.png".to_string(),
            title: "Miami Dolphins won the match and officially qualified for the final".to_string(),
            description: "Lorem ipsum dolor sit amet, consectetur adipiscing elit".to_string(),
            is_hot_topic: false,
            pub_date: "2021-01-01".to_string(),
            category: BlogCategory::MobileDevelopment,
        },
    ];
    html! {
        <>
        <header>
            <BlogNav />
        </header>
        <LineSeparator />
        <main class="blog">
            <MainBanner
                title="\"In the world of code, the best debugging tool is a fresh perspective."
                subtitle="~Chat GPT"
                background_url="img/bg.jpg"
            />
            <BlogSection category={BlogCategory::LatestRelease} posts={posts.clone()} />
            <AdComponent />
            <BlogSection category={BlogCategory::WebDevelopment} posts={posts.clone()} />
            <AdComponent />
            <BlogSection category={BlogCategory::MobileDevelopment} posts={posts.clone()} />
            <AdComponent />
            <BlogSection category={BlogCategory::ArtificialIntelligence} posts={posts.clone()} />
            <AdComponent />
            <BlogSection category={BlogCategory::Technology} posts={posts.clone()} />
            <AdComponent />
            <BlogSection category={BlogCategory::Lifestyle} posts={posts.clone()} />
            <AdComponent />
            <BlogSection category={BlogCategory::Travel} posts={posts} />
        </main>
        </>
    }
}
