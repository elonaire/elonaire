use std::collections::HashMap;

use icondata as IconId;
use leptos::{prelude::*, tachys::view::fragment::IntoFragment, task::spawn_local};
use leptos_icons::Icon;
use leptos_meta::*;

use crate::{
    components::{
        forms::{
            input::{InputField, InputFieldType},
            reactive_form::ReactiveForm,
            select::SelectInput,
        },
        general::{
            badge::Badge,
            button::{BasicButton, ButtonType},
            carousel::Carousel,
            chip::Chip,
            table::pagination::Pagination,
        },
        molecules::{
            blog::{
                blog_post::BlogPostPreview,
                blog_section::BlogSection,
                featured_post::{FeaturedPost, FeaturedPostProps},
            },
            footer::Footer,
        },
        schemas::props::ColorTemperature,
    },
    data::{
        context::{
            shared::{fetch_author_info, fetch_blog_posts},
            store::{AppStateContext, AppStateContextStoreFields},
        },
        models::{
            general::acl::{AuthInfoStoreFields, UserInfoStoreFields},
            graphql::{
                acl::FetchSingleUserVars,
                shared::{BlogPost, BlogStatus, FetchBlogPostsQueryFilters, FetchBlogPostsVars},
            },
        },
    },
};
use reactive_stores::Store;

#[island]
pub fn BlogHome() -> impl IntoView {
    let subscription_form_ref = NodeRef::new();
    let (is_loading, set_is_loading) = signal(false);
    let current_state = expect_context::<Store<AppStateContext>>();
    let (featured_posts, set_featured_posts) = signal(vec![] as Vec<BlogPost>);
    let (other_posts, set_other_posts) = signal(vec![] as Vec<BlogPost>);
    let (other_post_filters, set_other_post_filters) = signal(FetchBlogPostsQueryFilters {
        is_featured: Some(false),
        status: Some(BlogStatus::Published),
        ..Default::default()
    });

    Effect::new(move || {
        set_is_loading.set(true);
        spawn_local(async move {
            let featured_posts_filters = FetchBlogPostsVars {
                filters: FetchBlogPostsQueryFilters {
                    is_featured: Some(true),
                    status: Some(BlogStatus::Published),
                    ..Default::default()
                },
            };

            let fetch_featured_posts_query = r#"
                query FetchBlogPosts($filters: FetchBlogPostsQueryFilters) {
                    fetchBlogPosts(filters: $filters) {
                        data {
                            title
                            shortDescription
                            status
                            thumbnail
                            category
                            link
                            publishedDate
                            isFeatured
                            isPremium
                            createdAt
                            updatedAt
                            id
                            author
                        }
                        metadata {
                            requestId
                            newAccessToken
                        }
                    }
                }
               "#;

            let featured_posts =
                fetch_blog_posts(None, featured_posts_filters, fetch_featured_posts_query).await;

            if let Ok(mut featured_posts) = featured_posts {
                for featured_post in &mut featured_posts {
                    let user_id_vars = FetchSingleUserVars {
                        user_id: featured_post.author.as_ref().unwrap().to_owned(),
                    };
                    let author_details = fetch_author_info(&user_id_vars, None).await;

                    if let Ok(author_details) = author_details {
                        featured_post.full_author_details = Some(author_details);
                    };
                }
                set_featured_posts.set(featured_posts);
            };
        });
    });

    Effect::new(move || {
        set_is_loading.set(true);
        let other_post_filters = other_post_filters.get();
        spawn_local(async move {
            let other_posts_query = r#"
                query FetchBlogPosts($filters: FetchBlogPostsQueryFilters) {
                    fetchBlogPosts(filters: $filters) {
                        data {
                            title
                            shortDescription
                            status
                            thumbnail
                            category
                            link
                            publishedDate
                            isFeatured
                            isPremium
                            createdAt
                            updatedAt
                            id
                            author
                        }
                        metadata {
                            requestId
                            newAccessToken
                        }
                    }
                }
               "#;

            let filters = FetchBlogPostsVars {
                filters: other_post_filters,
            };

            let other_posts = fetch_blog_posts(None, filters, other_posts_query).await;

            if let Ok(other_posts) = other_posts {
                set_other_posts.set(other_posts);
            }
        });
    });

    view! {
        <Title text="Blog Home"/>
        <main>
            <div class="min-h-svh flex flex-col gap-[40px] bg-contrast-white">
                <div class="mx-[5%] md:mx-[10%]">
                    {
                        move || {
                            let featured_posts_val = featured_posts.get();

                            view! {
                                <Carousel>
                                    {
                                            featured_posts_val
                                                .iter()
                                                .map(|blog_post| {
                                                    view! {
                                                        <FeaturedPost
                                                            thumbnail=blog_post.thumbnail.as_ref().unwrap_or(&String::new()).to_owned()
                                                            title=blog_post.title.as_ref().unwrap_or(&String::new()).to_owned()
                                                            short_description=blog_post.short_description.as_ref().unwrap_or(&String::new()).to_owned()
                                                            author_profile_pic=blog_post.full_author_details.as_ref().unwrap().profile_picture.as_ref().unwrap_or(&String::new()).to_owned()
                                                            author_name=blog_post.full_author_details.as_ref().unwrap().full_name.as_ref().unwrap_or(&String::new()).to_owned() link=blog_post.link.as_ref().unwrap_or(&String::new()).to_owned()
                                                        />
                                                    }
                                                })
                                                .collect_view()
                                        }
                                </Carousel>
                            }
                        }
                    }

                </div>
                <div class="mx-[5%] md:mx-[10%] flex flex-col gap-[40px]">
                    <div class="flex flex-col gap-[20px]">
                        <div class="flex items-center gap-[10px]">
                            <div class="flex-1">
                                <BlogSection title="Latest Posts"/>
                            </div>
                            <div class="flex items-center gap-[5px]">
                                <Icon icon=IconId::BsFilter width="1rem" height="1rem" />
                                <Badge text="0" ><span>"Filters"</span></Badge>
                            </div>
                        </div>
                        { move || {
                            let filtered_posts = other_posts.get();
                            filtered_posts
                                .iter()
                                .map(|blog_post| {
                                    view! {
                                        <BlogPostPreview thumbnail=blog_post.thumbnail.as_ref().unwrap_or(&String::new()).to_owned() title=blog_post.title.as_ref().unwrap_or(&String::new()).to_owned() short_description=blog_post.short_description.as_ref().unwrap_or(&String::new()).to_owned() category=blog_post.category.unwrap().to_owned() read_time="2 mins" link=blog_post.link.as_ref().unwrap_or(&String::new()).to_owned() />
                                    }
                                })
                                .collect_view()
                            }
                        }
                        <Pagination pagination_state=Memo::new(|_| (1, 1)) />
                    </div>
                    <div class="flex flex-col gap-[40px]">
                        <div class="flex flex-col gap-[20px]">
                            <BlogSection title="Stay Updated"/>
                            <div class="flex flex-col gap-[20px]">
                                <p>"Receive Notifications whenever new posts are published. No promotional emails will be sent to your inbox."</p>
                                <ReactiveForm form_ref=subscription_form_ref>
                                    <div class="flex flex-col gap-[20px]">
                                        <InputField field_type=InputFieldType::Email label="Email" placeholder="Enter your email" required=true id_attr="email" name="email" />

                                        <BasicButton
                                            button_text="Subscribe"
                                            style_ext="bg-primary text-contrast-white"
                                            button_type=ButtonType::Submit
                                            // disabled=submit_is_disabled
                                        />
                                    </div>
                                </ReactiveForm>
                            </div>
                        </div>
                        <div class="flex flex-col gap-[20px]">
                            <BlogSection title="Categories"/>
                            <div class="flex gap-[16px] flex-wrap">
                                <Chip label="Technology" color=ColorTemperature::Gray removable=false />
                                <Chip label="Design" color=ColorTemperature::Gray removable=false />
                                <Chip label="Development" color=ColorTemperature::Gray removable=false />
                                <Chip label="Marketing" color=ColorTemperature::Gray removable=false />
                                <Chip label="Business" color=ColorTemperature::Gray removable=false />
                                <Chip label="Art" color=ColorTemperature::Gray removable=false />
                                <Chip label="Culture" color=ColorTemperature::Gray removable=false />
                                <Chip label="Science" color=ColorTemperature::Gray removable=false />
                                <Chip label="Health" color=ColorTemperature::Gray removable=false />
                                <Chip label="Fitness" color=ColorTemperature::Gray removable=false />
                            </div>
                        </div>
                    </div>
                </div>
                <div class="mt-auto">
                    <Footer />
                </div>
            </div>
        </main>
    }
}
