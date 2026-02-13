use std::collections::HashMap;

use icondata as IconId;
use leptos::task::spawn_local;
use leptos::wasm_bindgen::JsCast;
use leptos::{ev, prelude::*};
use leptos_icons::Icon;
use leptos_meta::*;
use leptos_router::components::A;
use leptos_router::hooks::use_params_map;
use reactive_stores::Store;
use web_sys::{HtmlDivElement, HtmlFormElement};

use crate::components::general::button::ButtonType;
use crate::components::general::modal::modal::{BasicModal, UseCase};
use crate::components::general::richtext_editor::ExtraFormatingOption;
use crate::components::general::spinner::Spinner;
use crate::components::{
    forms::reactive_form::ReactiveForm,
    general::{button::BasicButton, richtext_editor::RichTextEditor},
    molecules::{
        blog::{blog_comment::BlogComment, blog_post_metadata::BlogDetailMetadata},
        footer::Footer,
    },
};
use crate::data::context::shared::{fetch_author_info, fetch_single_blog_post};
use crate::data::models::graphql::acl::FetchSingleUserVars;
use crate::data::models::graphql::shared::{
    BlogCommentInput, CreateBlogCommentResponse, CreateBlogCommentVars, FetchSingleBlogPostVars,
};
use crate::data::{
    context::store::{AppStateContext, AppStateContextStoreFields},
    models::general::acl::{AuthInfoStoreFields, UserInfoStoreFields},
};
use crate::utils::forms::{deserialize_form_data_to_struct, get_form_data_from_form_ref};
use crate::utils::graphql_client::perform_mutation_or_query_with_vars;

#[island]
pub fn BlogPostDetail() -> impl IntoView {
    let menu_visible = RwSignal::new(true);
    let comments_ref = NodeRef::new();
    let comment_form_ref = NodeRef::new();
    let (comment_form_is_valid, set_comment_form_is_valid) = signal(false);
    let submit_is_disabled = Memo::new(move |_| !comment_form_is_valid.get());
    let (is_loading, set_is_loading) = signal(false);
    let params = use_params_map();
    let (blog_post, set_blog_post) = signal(None);
    let success_modal_is_open = RwSignal::new(false);
    let confirm_modal_is_open = RwSignal::new(false);
    let current_state = expect_context::<Store<AppStateContext>>();

    // Add scroll event listener to toggle menu visibility
    window_event_listener(ev::scroll, move |_| {
        if let Some(comments_el) = comments_ref.get() as Option<HtmlDivElement> {
            let rect = comments_el.get_bounding_client_rect();
            if let Ok(window_height) = window().inner_height() {
                let window_height = window_height.as_f64().unwrap_or(0.0);
                let is_visible = rect.top() < window_height;
                menu_visible.set(!is_visible); // Hide when comments are in view
            };
        }
    });

    let handle_on_input = Callback::new(move |comment: String| {
        leptos::logging::log!("val: {}", comment);
    });

    let handle_comment_form_submit = move |ev: ev::SubmitEvent| {
        ev.prevent_default();
        ev.stop_propagation();

        leptos::logging::log!("comment_form valid");

        // Implement logic to show form validity
        let target = ev
            .target()
            .and_then(|t| t.dyn_into::<HtmlFormElement>().ok());

        if let Some(form) = target {
            set_comment_form_is_valid.set(form.check_validity());

            if let Some(_submitter) = ev.submitter() {
                confirm_modal_is_open.update(|status| *status = true);
            }
        }
    };

    Effect::new(move || {
        set_is_loading.set(true);
        let slug_fn = move || params.read().get("slug");

        if let Some(slug) = slug_fn() {
            let vars = FetchSingleBlogPostVars {
                blog_id_or_slug: slug,
            };

            spawn_local(async move {
                let blog_post = fetch_single_blog_post(None, vars).await;

                if let Ok(mut blog_post) = blog_post {
                    if let Some(comments) = &mut blog_post.comments {
                        for comment in comments {
                            let user_id_vars = FetchSingleUserVars {
                                user_id: comment.author.as_ref().unwrap().to_owned(),
                            };
                            let author_details = fetch_author_info(&user_id_vars, None).await;

                            if let Ok(author_details) = author_details {
                                comment.full_author_details = Some(author_details);
                            };
                        }
                    };

                    let user_id_vars = FetchSingleUserVars {
                        user_id: blog_post.author.as_ref().unwrap().to_owned(),
                    };
                    let author_details = fetch_author_info(&user_id_vars, None).await;

                    if let Ok(author_details) = author_details {
                        blog_post.full_author_details = Some(author_details);
                    };

                    set_blog_post.set(Some(blog_post));
                }

                set_is_loading.set(false);
            });
        };
    });

    let onprimary_handler = Callback::new(move |_| {
        if comment_form_is_valid.get() && blog_post.get().is_some() {
            set_is_loading.set(true);
            spawn_local(async move {
                if let Some(main_form_data) = get_form_data_from_form_ref(&comment_form_ref) {
                    let deserialized_main_form_data = deserialize_form_data_to_struct::<
                        BlogCommentInput,
                    >(
                        &main_form_data, false, None
                    );

                    if deserialized_main_form_data.is_none() {
                        leptos::logging::log!("Something failed to deserialize");
                        set_is_loading.set(false);
                        return;
                    }

                    let deserialized_main_form_data = deserialized_main_form_data.unwrap();

                    let input_vars = CreateBlogCommentVars {
                        blog_comment: deserialized_main_form_data,
                        blog_post_id: blog_post.get_untracked().unwrap().id.unwrap(),
                    };

                    let query = r#"
                        mutation AddCommentToBlogPost($blogComment: BlogCommentInput!, $blogPostId: String!) {
                            addCommentToBlogPost(blogComment: $blogComment, blogPostId: $blogPostId) {
                                data {
                                    content
                                    createdAt
                                    updatedAt
                                    id
                                }
                                metadata {
                                    requestId
                                    newAccessToken
                                }
                            }
                        }
                       "#;

                    let mut headers = HashMap::new() as HashMap<String, String>;
                    headers.insert(
                        "Authorization".into(),
                        format!(
                            "Bearer {}",
                            current_state.user().auth_info().token().get_untracked()
                        ),
                    );

                    let response = perform_mutation_or_query_with_vars::<
                        CreateBlogCommentResponse,
                        CreateBlogCommentVars,
                    >(
                        Some(&headers),
                        "http://localhost:8080/api/shared",
                        query,
                        input_vars,
                    )
                    .await;

                    match response.get_data() {
                        Some(_data) => {
                            if let Some(form) = comment_form_ref
                                .get_untracked()
                                .and_then(|el| el.dyn_into::<HtmlFormElement>().ok())
                            {
                                form.reset();
                                set_comment_form_is_valid.set(false);
                            } else {
                            }

                            set_is_loading.set(false);

                            success_modal_is_open.update(|status| *status = true);
                        }
                        None => {
                            set_is_loading.set(false);
                        }
                    };
                };
            });
        }
    });

    view! {
        <Title text="Blog Detail"/>

        <main>
            <div class="min-h-svh flex flex-col gap-[40px] bg-contrast-white relative">
            <BasicModal title="Success" is_open=success_modal_is_open use_case=UseCase::Success disable_auto_close=false>
                <div class="p-[10px]">
                    <p>"Rate Card created successfully!"</p>
                </div>
            </BasicModal>
            <BasicModal title="Confirm" on_click_primary=onprimary_handler is_open=confirm_modal_is_open use_case=UseCase::Confirmation disable_auto_close=false>
                <div class="p-[10px]">
                    <p>"Are you sure that you want to submit?"</p>
                </div>
            </BasicModal>
            <Show when=move || is_loading.get()>
                <Spinner />
            </Show>
                {
                    move || {
                        let blog_post = blog_post.get();
                        if let Some(blog_post) = blog_post {
                            let blog_comments_ref = blog_post.comments.as_ref();

                            Some(
                                view! {
                                    <article class="flex flex-col gap-[20px] mx-[5%] md:mx-[30%]">
                                        <h1>{blog_post.title.as_ref().unwrap().to_owned()}</h1>
                                        <div class="flex items-center">
                                            {
                                                match &blog_post.full_author_details {
                                                    Some(author_details) => {
                                                        Some(
                                                            view! {
                                                                <BlogDetailMetadata date_of_creation={blog_post.created_at.as_ref().unwrap().to_owned()} read_time="2 mins read" author_profile_pic={author_details.profile_picture.as_ref().unwrap().to_owned()} author_name={author_details.full_name.as_ref().unwrap().to_owned()} />
                                                            }
                                                        )
                                                    }
                                                    None => None
                                                }
                                            }
                                        </div>
                                        <img src={blog_post.thumbnail.as_ref().unwrap().to_owned()} alt="Blog Post Image" class="w-full h-auto rounded-[5px] object-cover" />
                                        // Blog post content goes here
                                        <div class="flex flex-col gap-[20px] leading-relaxed text-base"  inner_html={blog_post.content.as_ref().unwrap().to_owned()} />
                                    </article>

                                    // floating reaction menu
                                    <div class=move || format!("flex items-center justify-between rounded-full bg-contrast-white border-[1px] border-light-gray absolute fixed bottom-[3%] left-1/2 z-10 -translate-x-1/2 shadow-lg px-[32px] py-[5px] mx-auto transition-all transition-discrete duration-300 {}", if menu_visible.get() {
                                                "translate-y-0 block"
                                            } else {
                                                "translate-y-[-100%] hidden"
                                            })>
                                        <BasicButton button_text="2K" icon=Some(IconId::LuThumbsUp) icon_before=true />
                                        <BasicButton button_text="300" icon=Some(IconId::BiBookmarkRegular) icon_before=true />
                                        <BasicButton button_text="123" icon=Some(IconId::FaCommentRegular) icon_before=true />
                                        <BasicButton button_text="567" icon=Some(IconId::BiShareAltRegular) icon_before=true />
                                    </div>

                                    <div class="flex flex-col gap-[20px] mx-[5%] md:mx-[30%]" node_ref=comments_ref>
                                        <h3>{format!("Comments({})", blog_comments_ref.unwrap_or(&vec![]).len())}</h3>
                                        // WYSIWYG editor goes here
                                        <ReactiveForm on:submit=handle_comment_form_submit form_ref=comment_form_ref>
                                            <div class="flex flex-col gap-[20px]">
                                                <RichTextEditor name="content" on_input=handle_on_input extra_formating_options=vec![ExtraFormatingOption::InlineCode, ExtraFormatingOption::CodeBlock] />
                                                <div class="flex gap-[20px] items-center justify-end">
                                                    <BasicButton
                                                        button_text="Submit"
                                                        style_ext="bg-primary text-contrast-white"
                                                        button_type=ButtonType::Submit
                                                        disabled=submit_is_disabled
                                                    />
                                                </div>
                                            </div>
                                        </ReactiveForm>
                                    </div>

                                    <div class="flex flex-col gap-[20px] py-[20px] mx-[5%] md:mx-[30%] border-y border-light-gray">
                                    {
                                        match blog_comments_ref {
                                            Some(comments) => {
                                                Some(
                                                    view! {
                                                        {
                                                            comments.into_iter()
                                                            .map(|comment| {
                                                                view! {
                                                                    <BlogComment comment_id=comment.id.as_ref().unwrap_or(&String::new()).to_owned() content=comment.content.as_ref().unwrap_or(&String::new()).to_owned() date_of_creation=comment.created_at.as_ref().unwrap_or(&String::new()).to_owned() author_name=comment.full_author_details.as_ref().unwrap().full_name.as_ref().unwrap().to_owned() author_avatar=comment.full_author_details.as_ref().unwrap().profile_picture.as_ref().unwrap().to_owned() reply_count=comment.reply_count.as_ref().unwrap_or(&0).to_owned() />
                                                                }
                                                            })
                                                            .collect_view()
                                                        }
                                                    }
                                                )
                                            }
                                            None => None
                                        }
                                    }

                                    </div>
                                    <div class="flex flex-col gap-[20px] mx-[5%] md:mx-[30%]">
                                        <h3>Meet the Author</h3>
                                        {
                                            match &blog_post.full_author_details {
                                                Some(author_details) => {
                                                    Some(
                                                        view! {
                                                            <div class="flex py-[20px] gap-[20px] border-y border-light-gray">
                                                                <img src={author_details.profile_picture.as_ref().unwrap().to_owned()} alt="Author Image" class="w-[100px] h-[100px] rounded-full" />
                                                                <div class="flex flex-col gap-[15px]">
                                                                    <p class="font-bold">{author_details.full_name.as_ref().unwrap().to_owned()}</p>
                                                                    <p>{author_details.bio.as_ref().unwrap().to_owned()}</p>
                                                                    // Socials
                                                                    <div class="flex gap-[15px]">
                                                                        <A href="#" target="_blank">
                                                                            <Icon icon=IconId::BsLinkedin />
                                                                        </A>
                                                                        <A href="#" target="_blank">
                                                                            <Icon icon=IconId::BsTwitterX />
                                                                        </A>
                                                                        <A href="#" target="_blank">
                                                                            <Icon icon=IconId::BsGithub />
                                                                        </A>
                                                                    </div>
                                                                </div>
                                                            </div>
                                                        }
                                                    )
                                                }
                                                None => None
                                            }
                                        }
                                    </div>
                                }
                            )
                        } else {
                            None
                        }
                    }
                }
                <div class="mt-auto">
                    <Footer />
                </div>
            </div>
        </main>
    }
}
