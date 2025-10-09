use std::collections::HashMap;

use icondata as IconData;
use leptos::ev::SubmitEvent;
use leptos::prelude::*;
use leptos::task::spawn_local;
use leptos::wasm_bindgen::JsCast;
use leptos_meta::*;
use leptos_router::components::{A, Outlet};
use reactive_stores::Store;
use web_sys::{FormData, HtmlFormElement, HtmlInputElement};

use crate::components::forms::toggle_switch::ToggleSwitch;
use crate::components::general::spinner::Spinner;
use crate::data::models::graphql::shared::{
    BlogCategory, BlogStatus, CreateBlogPostResponse, CreateBlogPostVars,
};
use crate::utils::custom_traits::EnumerableEnum;
use crate::utils::graphql_client::perform_mutation_or_query_with_vars;
use crate::{
    components::{
        forms::{
            input::{CustomFileInput, InputField, InputFieldType},
            reactive_form::ReactiveForm,
            select::{SelectInput, SelectOption},
        },
        general::{
            breadcrumbs::Breadcrumbs,
            button::{BasicButton, ButtonType},
            modal::modal::{BasicModal, UseCase},
            table::data_table::{Column, DataTable},
        },
    },
    data::models::{
        general::{
            acl::{
                AppStateContext, AppStateContextStoreFields, AuthInfoStoreFields,
                UserInfoStoreFields,
            },
            files::UploadedFileResponse,
        },
        graphql::shared::BlogPostInput,
    },
    utils::forms::{deserialize_form_data_to_struct, get_form_data_from_form_ref},
};

#[island]
pub fn Blog() -> impl IntoView {
    view! {
        <>
            <Outlet />
        </>
    }
}

#[island]
pub fn BlogList() -> impl IntoView {
    let table_data = RwSignal::new((
        vec![
            Column::new("Title", false),
            Column::new("Status", true),
            Column::new("Category", true),
            Column::new("Date Created", true),
        ],
        vec![],
    ));

    Effect::new(move || {
        spawn_local(async move {});
    });

    view! {
        <>
            <Title text="My Blog Posts"/>
            <div class="mx-[20px]">
                <Breadcrumbs custom_route_names=["Home", "Dashboard", "Blog Posts"] />
            </div>

            <h1 class="mx-[20px]">Blog Posts</h1>

            <div class="mx-[20px] flex items-center justify-end">
                <A href="/dashboard/blog/create">
                    <BasicButton
                        button_text="Create"
                        icon=Some(IconData::BsPlusLg)
                        icon_before=true
                        style_ext="bg-primary text-white"
                    />
                </A>
            </div>

            <div class="mx-[20px]">
                <DataTable data=table_data editable=true deletable=true />
            </div>
        </>
    }
}

#[island]
pub fn CreateBlog() -> impl IntoView {
    let form_ref = NodeRef::new();
    let thumbnail_file_input_ref = NodeRef::new();
    let content_file_input_ref = NodeRef::new();
    let (form_is_valid, set_form_is_valid) = signal(false);
    let submit_is_disabled = Memo::new(move |_| !form_is_valid.get());
    let current_state = expect_context::<Store<AppStateContext>>();
    let success_modal_is_open = RwSignal::new(false);
    let confirm_modal_is_open = RwSignal::new(false);
    let (submission_confirmed, set_submission_confirmed) = signal(false);
    let (is_loading, set_is_loading) = signal(false);
    let blog_statuses = Memo::new(move |_| {
        BlogStatus::variants_slice()
            .iter()
            .map(|status| {
                let mut label = format!("{}", status);
                if label.is_empty() {
                    label = "Select Status".to_string();
                }

                SelectOption::new(format!("{}", status).as_str(), label.as_str())
            })
            .collect::<Vec<SelectOption>>()
    });

    let blog_categories = Memo::new(move |_| {
        BlogCategory::variants_slice()
            .iter()
            .map(|category| {
                let mut label = format!("{}", category);
                if label.is_empty() {
                    label = "Select Category".to_string();
                }

                if category == "WebDevelopment" {
                    label = "Web Development".to_string();
                } else if category == "MobileDevelopment" {
                    label = "Mobile Development".to_string();
                } else if category == "ArtificialIntelligence" {
                    label = "Artificial Intelligence".to_string();
                }

                SelectOption::new(format!("{}", category).as_str(), label.as_str())
            })
            .collect::<Vec<SelectOption>>()
    });

    let onprimary_handler = Callback::new(move |_| {
        set_submission_confirmed.set(true);
    });

    Effect::new(move || {
        if submission_confirmed.get() && form_is_valid.get() {
            set_is_loading.set(true);
            if let Some(thumbnail_file_input) =
                thumbnail_file_input_ref.to_owned().get() as Option<HtmlInputElement>
            {
                if let Some(content_file_input) =
                    content_file_input_ref.to_owned().get() as Option<HtmlInputElement>
                {
                    if let Ok(files_form_data) = FormData::new() {
                        if let Some(thumbnail_filelist) = thumbnail_file_input.files() {
                            for i in 0..thumbnail_filelist.length() {
                                if let Some(file) = thumbnail_filelist.item(i) {
                                    if let Err(e) =
                                        files_form_data.append_with_blob("thumbnail", &file)
                                    {
                                        leptos::logging::error!("Failed to append Blob: {:?}", e);
                                    };
                                }
                            }
                        }

                        if let Some(content_filelist) = content_file_input.files() {
                            for i in 0..content_filelist.length() {
                                if let Some(file) = content_filelist.item(i) {
                                    if let Err(e) =
                                        files_form_data.append_with_blob("content_file", &file)
                                    {
                                        leptos::logging::error!("Failed to append Blob: {:?}", e);
                                    };
                                }
                            }
                        }

                        spawn_local(async move {
                            match gloo_net::http::Request::post(
                                "http://localhost:8080/api/files/upload",
                            )
                            .header(
                                "Authorization",
                                format!(
                                    "Bearer {}",
                                    current_state.user().auth_info().token().get_untracked()
                                )
                                .as_str(),
                            )
                            .body(files_form_data)
                            .unwrap()
                            .send()
                            .await
                            {
                                Ok(response) => {
                                    match response.json::<Vec<UploadedFileResponse>>().await {
                                        Ok(uploaded_files) => {
                                            if let Some(form_data) =
                                                get_form_data_from_form_ref(&form_ref)
                                            {
                                                let thumbnail = uploaded_files
                                                    .iter()
                                                    .find(|file| file.field_name == "thumbnail");
                                                let content_file = uploaded_files
                                                    .iter()
                                                    .find(|file| file.field_name == "content_file");
                                                // Implement logic to handle form data
                                                if let Err(e) = form_data.append_with_str(
                                                    "thumbnail",
                                                    format!(
                                                        "http://localhost:3001/view/{}",
                                                        thumbnail.unwrap().file_name
                                                    )
                                                    .as_str(),
                                                ) {
                                                    leptos::logging::log!(
                                                        "Error appending thumbnail: {:?}",
                                                        e
                                                    );
                                                    return;
                                                };

                                                if let Err(e) = form_data.append_with_str(
                                                    "content_file",
                                                    content_file.unwrap().file_id.as_str(),
                                                ) {
                                                    leptos::logging::log!(
                                                        "Error appending content file: {:?}",
                                                        e
                                                    );
                                                    return;
                                                };

                                                let deserialized_form_data =
                                                    deserialize_form_data_to_struct::<BlogPostInput>(
                                                        &form_data, true,
                                                    );

                                                leptos::logging::log!(
                                                    "Deserialized form data: {:?}",
                                                    deserialized_form_data
                                                );

                                                if deserialized_form_data.is_none() {
                                                    set_submission_confirmed.set(false);
                                                    set_is_loading.set(false);
                                                    return;
                                                }

                                                let deserialized_form_data =
                                                    deserialized_form_data.unwrap();

                                                let input_vars = CreateBlogPostVars {
                                                    blog_post: deserialized_form_data,
                                                };

                                                let query = r#"
                                                       mutation CreateBlogPost($blogPost: BlogPostInput!) {
                                                            createBlogPost(blogPost: $blogPost) {
                                                                id
                                                                shortDescription
                                                                title
                                                                status
                                                                category
                                                                link
                                                                thumbnail
                                                                publishedDate
                                                            }
                                                       }
                                                   "#;

                                                let mut headers =
                                                    HashMap::new() as HashMap<String, String>;
                                                headers.insert(
                                                    "Authorization".into(),
                                                    format!(
                                                        "Bearer {}",
                                                        current_state
                                                            .user()
                                                            .auth_info()
                                                            .token()
                                                            .get_untracked()
                                                    ),
                                                );

                                                let response =
                                                    perform_mutation_or_query_with_vars::<
                                                        CreateBlogPostResponse,
                                                        CreateBlogPostVars,
                                                    >(
                                                        Some(headers),
                                                        "http://localhost:8080/api/shared",
                                                        query,
                                                        input_vars,
                                                    )
                                                    .await;

                                                match response.get_data() {
                                                    Some(_data) => {
                                                        if let Some(form) = form_ref
                                                            .get_untracked()
                                                            .and_then(|el| {
                                                                el.dyn_into::<HtmlFormElement>()
                                                                    .ok()
                                                            })
                                                        {
                                                            form.reset();
                                                            set_form_is_valid.set(false);
                                                            set_submission_confirmed.set(false);
                                                        } else {
                                                            set_submission_confirmed.set(false);
                                                        }
                                                        set_is_loading.set(false);

                                                        success_modal_is_open
                                                            .update(|status| *status = true);
                                                    }
                                                    None => {
                                                        set_is_loading.set(false);
                                                        set_submission_confirmed.set(false);
                                                    }
                                                };
                                            };
                                        }
                                        Err(err) => {
                                            leptos::logging::error!(
                                                "Failed to parse uploaded file response: {:?}",
                                                err
                                            );
                                            set_is_loading.set(false);
                                            set_submission_confirmed.set(false);
                                        }
                                    };
                                }
                                Err(err) => {
                                    leptos::logging::error!("Failed to upload files: {:?}", err);
                                    set_is_loading.set(false);
                                    set_submission_confirmed.set(false);
                                }
                            };
                        });
                    };
                };
            };
        }
    });

    let handle_step_form_submit = move |ev: SubmitEvent| {
        ev.prevent_default();
        ev.stop_propagation();

        // Implement logic to show form validity
        let target = ev
            .target()
            .and_then(|t| t.dyn_into::<HtmlFormElement>().ok());

        if let Some(form) = target {
            set_form_is_valid.set(form.check_validity());

            if let Some(_submitter) = ev.submitter() {
                confirm_modal_is_open.update(|status| *status = true);
            }
        }
    };

    view! {
        <>
            <Title text="Create Blog Post"/>
            <BasicModal title="Success" is_open=success_modal_is_open use_case=UseCase::Success disable_auto_close=false>
                <div>
                    <p>"Blog Post created successfully!"</p>
                </div>
            </BasicModal>
            <BasicModal title="Confirm" on_click_primary=onprimary_handler is_open=confirm_modal_is_open use_case=UseCase::Confirmation disable_auto_close=false>
                <div>
                    <p>"Are you sure that you want to submit?"</p>
                </div>
            </BasicModal>
            <Show when=move || is_loading.get()>
                <Spinner />
            </Show>

            <div class="mx-[20px]">
                <Breadcrumbs custom_route_names=["Home", "Dashboard", "Blog Posts", "New"] />
            </div>

            <h1 class="mx-[20px]">Create New Blog Post</h1>

            <ReactiveForm on:submit=handle_step_form_submit form_ref=form_ref>
                <div class="mx-[20px] flex flex-col gap-[20px]">
                    <InputField field_type=InputFieldType::Text label="Title" required=true id_attr="title" name="title" />
                    <InputField field_type=InputFieldType::Text label="Short Description" required=true id_attr="short_description" name="short_description" />
                    <SelectInput
                    label="Status"
                    name="status"
                    required=true
                    id_attr="status"
                    options=blog_statuses.get_untracked()
                    />
                    <SelectInput
                    label="Category"
                    name="category"
                    required=true
                    id_attr="category"
                    options=blog_categories.get_untracked()
                    />
                    <ToggleSwitch
                       label_active="Premium"
                       label_inactive="Free"
                       name="is_premium"
                       id_attr="is_premium"
                    />

                    <ToggleSwitch
                          label_active="Featured"
                          label_inactive="Not Featured"
                          name="is_featured"
                          id_attr="is_featured"
                    />
                    <CustomFileInput input_node_ref=thumbnail_file_input_ref label="Thumbnail" name="thumbnail" id_attr="thumbnail" accept="image/*" required=true />
                    <CustomFileInput input_node_ref=content_file_input_ref label="Content File" name="content_file" id_attr="content_file" accept="text/markdown" required=true />
                    <BasicButton
                        button_text="Submit"
                        style_ext="bg-primary text-white"
                        button_type=ButtonType::Submit
                        disabled=submit_is_disabled
                    />
                </div>
            </ReactiveForm>
        </>
    }
}
