use std::collections::HashMap;

use icondata as IconData;
use leptos::ev::{self, SubmitEvent};
use leptos::prelude::*;
use leptos::task::spawn_local;
use leptos::wasm_bindgen::JsCast;
use leptos_meta::*;
use leptos_router::components::{A, Outlet};
use reactive_stores::Store;
use web_sys::{FormData, HtmlFormElement, HtmlInputElement};

use crate::components::general::spinner::Spinner;
use crate::components::general::table::data_table::TableCellData;
use crate::data::models::graphql::shared::{
    CreatePortfolioItemResponse, FetchSiteResourcesResponse, UserPortfolioCategory,
    UserPortfolioInputVars,
};
use crate::utils::custom_traits::EnumerableEnum;
use crate::utils::graphql_client::{
    perform_mutation_or_query_with_vars, perform_query_without_vars,
};
use crate::{
    components::{
        forms::{
            datepicker::DatePicker,
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
        graphql::shared::UserPortfolioInput,
    },
    utils::forms::{deserialize_form_data_to_struct, get_form_data_from_form_ref},
};

#[island]
pub fn Portfolio() -> impl IntoView {
    view! {
        <>
            <Outlet />
        </>
    }
}

#[island]
pub fn PortfolioList() -> impl IntoView {
    let current_state = expect_context::<Store<AppStateContext>>();
    let (is_loading, set_is_loading) = signal(false);

    let table_data = RwSignal::new((
        vec![
            Column::new("Title", false),
            Column::new("Start Date", true),
            Column::new("YOE", true),
            Column::new("Category", true),
        ],
        vec![],
    ));

    Effect::new(move || {
        set_is_loading.set(true);
        spawn_local(async move {
            let fetch_roles_query = r#"
                   query FetchSiteResources {
                        fetchSiteResources {
                            portfolio {
                                title
                                description
                                startDate
                                endDate
                                link
                                category
                                thumbnail
                                id
                                yearsOfExperience
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

            let fetch_roles_response = perform_query_without_vars::<FetchSiteResourcesResponse>(
                Some(&headers),
                "http://localhost:8080/api/shared",
                fetch_roles_query,
            )
            .await;

            match fetch_roles_response.get_data() {
                Some(data) => {
                    let roles: Vec<HashMap<String, TableCellData>> = data
                        .fetch_site_resources
                        .as_ref()
                        .unwrap()
                        .portfolio
                        .as_ref()
                        .unwrap()
                        .to_vec()
                        .iter()
                        .map(|portfolio| {
                            let mut hash_map_data = HashMap::new();

                            // This id is the unique identifier of the table row. and is a MUST for the table to function properly.
                            // *Note:* The id is a MUST for the table to function properly. You might be forced to generate a unique id for each row if your data does not have a unique identifier.
                            hash_map_data.insert(
                                "id".to_string(),
                                TableCellData::String(portfolio.id.as_ref().unwrap().to_owned()),
                            );

                            hash_map_data.insert(
                                "Title".to_string(),
                                TableCellData::String(portfolio.title.as_ref().unwrap().to_owned()),
                            );

                            hash_map_data.insert(
                                "YOE".to_string(),
                                TableCellData::Usize(
                                    portfolio.years_of_experience.as_ref().unwrap().to_owned(),
                                ),
                            );

                            hash_map_data.insert(
                                "Start Date".to_string(),
                                TableCellData::DateTime(
                                    portfolio.start_date.as_ref().unwrap().to_owned(),
                                ),
                            );

                            hash_map_data.insert(
                                "Category".to_string(),
                                TableCellData::String(format!(
                                    "{:?}",
                                    portfolio.category.as_ref().unwrap().to_owned()
                                )),
                            );
                            hash_map_data
                        })
                        .collect();

                    table_data.update(move |prev| {
                        prev.1 = roles;
                    });

                    set_is_loading.set(false);
                }
                None => {
                    set_is_loading.set(false);
                }
            };
        });
    });

    view! {
        <>
            <Title text="My Portfolio"/>
            <div class="mx-[20px]">
                <Breadcrumbs custom_route_names=["Home", "Dashboard", "Portfolio"] />
            </div>
            <Show when=move || is_loading.get()>
                <Spinner />
            </Show>

            <h1 class="mx-[20px]">Portfolio</h1>

            <div class="mx-[20px] flex items-center justify-end">
                <A href="/dashboard/portfolio/create">
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
pub fn CreatePortfolio() -> impl IntoView {
    let form_ref = NodeRef::new();
    let file_input_ref = NodeRef::new();
    let (form_is_valid, set_form_is_valid) = signal(false);
    let submit_is_disabled = Memo::new(move |_| !form_is_valid.get());
    let current_state = expect_context::<Store<AppStateContext>>();
    let success_modal_is_open = RwSignal::new(false);
    let confirm_modal_is_open = RwSignal::new(false);
    let (submission_confirmed, set_submission_confirmed) = signal(false);
    let init_date = RwSignal::new(None);
    let (is_loading, set_is_loading) = signal(false);
    let portfolio_categories = RwSignal::new(
        UserPortfolioCategory::variants_slice()
            .iter()
            .map(|category| {
                let mut label = format!("{}", category);
                if label.is_empty() {
                    label = "Select Category".to_string();
                }
                SelectOption::new(format!("{}", category).as_str(), label.as_str())
            })
            .collect::<Vec<SelectOption>>(),
    );

    let onprimary_handler = Callback::new(move |_| {
        set_submission_confirmed.set(true);
    });

    let onreset_handler = Callback::new(move |_ev: ev::Event| {
        init_date.set(None);
    });

    Effect::new(move || {
        if submission_confirmed.get() && form_is_valid.get() {
            set_is_loading.set(true);
            if let Some(file_input) = file_input_ref.to_owned().get() as Option<HtmlInputElement> {
                if let Ok(files_form_data) = FormData::new() {
                    if let Some(filelist) = file_input.files() {
                        for i in 0..filelist.length() {
                            if let Some(file) = filelist.item(i) {
                                if let Err(e) = files_form_data.append_with_blob("file", &file) {
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
                                            // Implement logic to handle form data
                                            if let Ok(_) = form_data.append_with_str(
                                                "thumbnail",
                                                format!(
                                                    "http://localhost:3001/view/{}",
                                                    uploaded_files[0].file_name
                                                )
                                                .as_str(),
                                            ) {
                                                let deserialized_form_data =
                                                    deserialize_form_data_to_struct::<
                                                        UserPortfolioInput,
                                                    >(
                                                        &form_data, false
                                                    );

                                                if deserialized_form_data.is_none() {
                                                    set_is_loading.set(false);
                                                    return;
                                                }

                                                let deserialized_form_data =
                                                    deserialized_form_data.unwrap();

                                                let input_vars = UserPortfolioInputVars {
                                                    portfolio_item: deserialized_form_data,
                                                };

                                                let query = r#"
                                                       mutation CreatePortfolioItem($portfolioItem: UserPortfolioInput!) {
                                                            createPortfolioItem(portfolioItem: $portfolioItem) {
                                                                id
                                                                title
                                                                description
                                                                startDate
                                                                category
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
                                                        CreatePortfolioItemResponse,
                                                        UserPortfolioInputVars,
                                                    >(
                                                        Some(&headers),
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
            <Title text="Create Portfolio"/>
            <BasicModal title="Success" is_open=success_modal_is_open use_case=UseCase::Success disable_auto_close=false>
                <div>
                    <p>"Portfolio Project created successfully!"</p>
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
                <Breadcrumbs custom_route_names=["Home", "Dashboard", "Portfolio", "New"] />
            </div>

            <h1 class="mx-[20px]">Create New Portfolio Project</h1>

            <ReactiveForm on:submit=handle_step_form_submit onreset=onreset_handler form_ref=form_ref>
                <div class="mx-[20px] flex flex-col gap-[20px]">
                    <InputField field_type=InputFieldType::Text label="Title" required=true id_attr="title" name="title" />
                    <InputField field_type=InputFieldType::Text label="Description" required=true id_attr="description" name="description" />
                    <DatePicker label="Start Date" required=true id_attr="start_date" initial_value=init_date name="start_date" />
                    <DatePicker label="End Date" required=true id_attr="end_date" initial_value=init_date name="end_date" />
                    <InputField field_type=InputFieldType::Text label="Link" required=true id_attr="link" name="link" />
                    <SelectInput
                    label="Category"
                    name="category"
                    required=true
                    id_attr="category"
                    options=portfolio_categories
                    />
                    <CustomFileInput input_node_ref=file_input_ref label="Thumbnail" name="thumbnail" id_attr="thumbnail" accept="image/*" required=true />
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
