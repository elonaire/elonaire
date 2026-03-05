use std::time::Duration;

use icondata as IconId;
use leptos::{prelude::*, task::spawn_local};
use leptos_icons::Icon;
use leptos_meta::*;
use web_sys::{HtmlDivElement, MouseEvent};

use crate::utils::custom_traits::EnumerableEnum;
use crate::{
    components::{
        forms::{
            input::{InputField, InputFieldType},
            reactive_form::ReactiveForm,
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
                blog_post::BlogPostPreview, blog_section::BlogSection, featured_post::FeaturedPost,
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
        models::graphql::{
            acl::FetchSingleUserVars,
            shared::{
                BlogCategory, BlogPost, BlogStatus, FetchBlogPostsQueryFilters, FetchBlogPostsVars,
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
    let search_area_ref = NodeRef::new();
    let (query, set_query) = signal(String::new());
    let (show_overlay, set_show_overlay) = signal(false);
    let (is_loading, set_is_loading) = signal(false);
    let (search_results, set_search_results) = signal(vec![] as Vec<BlogPost>);

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
                            readTime
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
                            readTime
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

    let handle_search_focus = Callback::new(move |_| {
        if let Some(el) = search_area_ref.get() as Option<HtmlDivElement> {
            el.scroll_into_view_with_bool(true);
            set_show_overlay.set(true);
        }
    });

    let search_action = Action::new_local(move |q: &String| {
        let q = q.clone();
        async move {
            let post_filters = FetchBlogPostsQueryFilters {
                search_term: Some(q),
                ..Default::default()
            };
            let other_posts_query = r#"
            query FetchBlogPosts($filters: FetchBlogPostsQueryFilters) {
                fetchBlogPosts(filters: $filters) {
                    data {
                        title
                        category
                        link
                    }
                    metadata {
                        requestId
                        newAccessToken
                    }
                }
            }
           "#;

            let filters = FetchBlogPostsVars {
                filters: post_filters,
            };

            match fetch_blog_posts(None, filters, other_posts_query).await {
                Ok(posts) => posts,
                Err(_) => vec![],
            }
        }
    });

    // Watch query changes
    Effect::new(move |_| {
        let q = query.get();
        if q.is_empty() {
            set_show_overlay.set(false);
            set_search_results.set(vec![]);
            return;
        }
        set_show_overlay.set(true);
        // set_is_loading.set(true);
        search_action.dispatch(q);
    });

    // Update results when action completes
    Effect::new(move |_| {
        if let Some(results) = search_action.value().get() {
            set_search_results.set(results);
            set_is_loading.set(false);
        }
    });

    let handle_blur = Callback::new(move |_| {
        // Small delay so clicks on results register first
        set_timeout(
            move || {
                set_show_overlay.set(false);
                current_state.show_mobile_search().set(false);
            },
            Duration::from_millis(150),
        );
    });

    view! {
        <Title text="Blog Home"/>
        <main>
            <div class="min-h-svh flex flex-col gap-[40px] bg-contrast-white">
                <div class="display-constraints">
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
                <div class="bg-primary/25 hidden md:block">
                    <div class="display-constraints flex flex-col gap-[20px] items-center justify-center h-[284px]" node_ref=search_area_ref>
                        <h1>"Find just what you are looking for"</h1>
                        <p>"Search through our collection of articles"</p>
                        <div class="w-[384px] flex items-center gap-[10px]">
                            <div class="flex-1 relative">
                                <InputField field_type=InputFieldType::Text icon=IconId::BsSearch onfocus=handle_search_focus id_attr="search-input" onblur=handle_blur on:input=move |e| {
                                    set_query.set(event_target_value(&e));
                                } />
                                {move || show_overlay.get().then(|| view! {
                                    <div
                                        class="absolute top-full mt-2 z-50 w-full bg-contrast-white rounded-[5px] shadow-2xl h-[43svh] overflow-y-auto"
                                        on:mousedown=|e: MouseEvent| e.prevent_default()
                                    >
                                        // Loading state
                                        {move || is_loading.get().then(|| view! {
                                            <div class="flex items-center justify-center py-8 text-gray-400 text-sm">
                                                <span>"Searching..."</span>
                                            </div>
                                        })}

                                        // Results list
                                        {move || {
                                            let results = search_results.get();
                                            (!results.is_empty()).then(|| view! {
                                                <ul class="py-2 list-none">
                                                    {results.into_iter().map(|article| view! {
                                                        <li>
                                                        <a

                                                                href=format!("/blog/read/{}", article.link.unwrap_or("".into()))
                                                                class="flex flex-col px-4 py-3 hover:bg-primary/10 \
                                                                       transition-colors cursor-pointer group"
                                                            >
                                                                <span class="text-sm font-medium text-gray-900 \
                                                                             group-hover:text-primary line-clamp-1">
                                                                    {article.title.unwrap_or("".into())}
                                                                </span>
                                                                // Optional: subtitle/category
                                                                <span class="text-xs text-gray-400 mt-0.5">
                                                                    {article.category.unwrap().to_string()}
                                                                </span>
                                                            </a>
                                                        </li>
                                                    }).collect_view()}
                                                </ul>
                                            })
                                        }}

                                        // Empty state
                                        {move || {
                                            let q = query.get();
                                            let results = search_results.get();
                                            (!q.is_empty() && results.is_empty() && !is_loading.get()).then(|| view! {
                                                <div class="py-8 text-center text-sm text-gray-400">
                                                    "No articles found for "
                                                    <span class="font-medium text-gray-600">{q}</span>
                                                </div>
                                            })
                                        }}
                                    </div>
                                })}
                            </div>
                        </div>
                    </div>
                </div>
                // Mobile search overlay
                {move || current_state.show_mobile_search().get().then(|| view! {
                    <div class="fixed inset-0 z-50 bg-black/50 md:hidden"
                        on:mousedown=move |_| {
                            current_state.show_mobile_search().set(false);
                        }
                    >
                        <div class="bg-white w-full px-4 py-3 flex items-center gap-3 shadow-lg"
                            on:mousedown=move |e: MouseEvent| e.stop_propagation()
                        >
                            // Back/close button
                            <button
                                class="text-gray-600 shrink-0"
                                on:click=move |_| {
                                    current_state.show_mobile_search().set(false);
                                }
                            >
                                <Icon icon=IconId::BsArrowLeft width="1.2rem" height="1.2rem" />
                            </button>

                            <div class="flex-1 relative">
                                <InputField
                                    field_type=InputFieldType::Text
                                    icon=IconId::BsSearch
                                    id_attr="mobile-search-input"
                                    placeholder="Search articles..."
                                    on:input=move |e| {
                                        set_query.set(event_target_value(&e));
                                    }
                                    onblur=handle_blur
                                />
                                // Results dropdown
                                {move || {
                                    let results = search_results.get();
                                    let q = query.get();
                                    (!q.is_empty()).then(|| view! {
                                        <div class="absolute top-full mt-2 w-full bg-white rounded-[5px] shadow-2xl max-h-[60svh] overflow-y-auto z-50">
                                            {move || is_loading.get().then(|| view! {
                                                <div class="flex items-center justify-center py-8 text-gray-400 text-sm">
                                                    <span>"Searching..."</span>
                                                </div>
                                            })}
                                            {move || {
                                                let results = search_results.get();
                                                (!results.is_empty()).then(|| view! {
                                                    <ul class="py-2 list-none">
                                                        {results.into_iter().map(|article| view! {
                                                            <li>
                                                                <a
                                                                    href=format!("/blog/read/{}", article.link.unwrap_or("".into()))
                                                                    class="flex flex-col px-4 py-3 hover:bg-primary/10 transition-colors cursor-pointer group"
                                                                >
                                                                    <span class="text-sm font-medium text-gray-900 group-hover:text-primary line-clamp-1">
                                                                        {article.title.unwrap_or("".into())}
                                                                    </span>
                                                                    <span class="text-xs text-gray-400 mt-0.5">
                                                                        {article.category.unwrap().to_string()}
                                                                    </span>
                                                                </a>
                                                            </li>
                                                        }).collect_view()}
                                                    </ul>
                                                })
                                            }}
                                            {move || {
                                                let q = query.get();
                                                let results = search_results.get();
                                                (!q.is_empty() && results.is_empty() && !is_loading.get()).then(|| view! {
                                                    <div class="py-8 text-center text-sm text-gray-400">
                                                        "No articles found for "
                                                        <span class="font-medium text-gray-600">{q}</span>
                                                    </div>
                                                })
                                            }}
                                        </div>
                                    })
                                }}
                            </div>
                        </div>
                    </div>
                })}
                <div class="display-constraints flex flex-col gap-[40px] md:flex-row">
                    <div class="flex flex-col gap-[20px] md:basis-3/4">
                        <div class="flex items-center gap-[10px]">
                            <div class="flex-1">
                                <BlogSection title="Latest Posts"/>
                            </div>
                            <div class="flex items-center gap-[5px]">
                                <Icon icon=IconId::VsSettings width="1rem" height="1rem" />
                                <Badge text="0" ><span>"Filters"</span></Badge>
                            </div>
                        </div>
                        <div class="grid grid-cols-1 md:grid-cols-2 md:grid-rows-5 md:auto-rows-min gap-[20px]">
                            { move || {
                                let filtered_posts = other_posts.get();
                                filtered_posts
                                    .iter()
                                    .map(|blog_post| {
                                        view! {
                                            <BlogPostPreview thumbnail=blog_post.thumbnail.as_ref().unwrap_or(&String::new()).to_owned() title=blog_post.title.as_ref().unwrap_or(&String::new()).to_owned() short_description=blog_post.short_description.as_ref().unwrap_or(&String::new()).to_owned() category=blog_post.category.unwrap().to_owned() read_time=blog_post.read_time.as_ref().unwrap_or(&0).to_owned() link=blog_post.link.as_ref().unwrap_or(&String::new()).to_owned() />
                                        }
                                    })
                                    .collect_view()
                                }
                            }
                        </div>
                        <div class="mt-auto">
                            <Pagination pagination_state=Memo::new(|_| (1, 1)) />
                        </div>
                    </div>
                    <div class="flex flex-col gap-[40px]  md:basis-1/4">
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
                                {
                                    BlogCategory::variants_slice()
                                        .iter()
                                        .map(|category| {
                                            view! {
                                                <Chip label=category.to_string() color=ColorTemperature::Gray removable=false />
                                            }
                                        })
                                        .collect::<Vec<_>>()
                                }
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
