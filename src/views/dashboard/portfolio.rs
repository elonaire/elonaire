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

use crate::components::forms::select::CustomSelectInput;
use crate::components::forms::textarea::Textarea;
use crate::components::general::spinner::Spinner;
use crate::components::general::table::data_table::TableCellData;
use crate::data::context::shared::fetch_portfolio;
use crate::data::models::graphql::shared::{
    CreatePortfolioItemResponse, UserPortfolioCategory, UserPortfolioInputVars,
};
use crate::utils::custom_traits::EnumerableEnum;
use crate::utils::graphql_client::perform_mutation_or_query_with_vars;
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
    data::{
        context::{
            shared::fetch_skills,
            store::{AppStateContext, AppStateContextStoreFields},
        },
        models::{
            general::{
                acl::{AuthInfoStoreFields, UserInfoStoreFields},
                files::UploadedFileResponse,
            },
            graphql::shared::UserPortfolioInput,
        },
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
    let portfolio = move || current_state.portfolio();
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
        let portfolio_data: Vec<HashMap<String, TableCellData>> = portfolio()
            .get()
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
                    TableCellData::DateTime(portfolio.start_date.as_ref().unwrap().to_owned()),
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
            prev.1 = portfolio_data;
        });
    });

    Effect::new(move || {
        set_is_loading.set(true);
        spawn_local(async move {
            // let mut headers = HashMap::new() as HashMap<String, String>;
            // headers.insert(
            //     "Authorization".into(),
            //     format!(
            //         "Bearer {}",
            //         current_state.user().auth_info().token().get_untracked()
            //     ),
            // );

            let _portfolio_res = fetch_portfolio(&current_state, None).await;

            set_is_loading.set(false);
        });
    });

    view! {
        <>
            <Title text="My Portfolio"/>
            <div class="display-constraints">
                <Breadcrumbs custom_route_names=["Home", "Dashboard", "Portfolio"] />
            </div>
            <Show when=move || is_loading.get()>
                <Spinner />
            </Show>

            <h1 class="display-constraints">Portfolio</h1>

            <div class="display-constraints flex items-center justify-end">
                <A href="/dashboard/portfolio/create">
                    <BasicButton
                        button_text="Create"
                        icon=Some(IconData::BsPlusLg)
                        icon_before=true
                        style_ext="bg-primary text-contrast-white"
                    />
                </A>
            </div>

            <div class="display-constraints">
                <DataTable data=table_data editable=true deletable=true />
            </div>
        </>
    }
}

#[island]
pub fn CreatePortfolio() -> impl IntoView {
    let form_ref = NodeRef::new();
    let file_input_ref = NodeRef::new();
    let applied_skills = RwSignal::new(Vec::new() as Vec<String>);
    let (form_is_valid, set_form_is_valid) = signal(false);
    let submit_is_disabled =
        Memo::new(move |_| !form_is_valid.get() || applied_skills.get().is_empty());
    let current_state = expect_context::<Store<AppStateContext>>();
    let skills = move || current_state.skills();
    let skills_select_options = RwSignal::new(vec![] as Vec<SelectOption>);
    let success_modal_is_open = RwSignal::new(false);
    let confirm_modal_is_open = RwSignal::new(false);
    let init_date = RwSignal::new(None);
    let (is_loading, set_is_loading) = signal(false);

    let portfolio_categories = RwSignal::new(
        UserPortfolioCategory::variants_slice()
            .iter()
            .map(|category| SelectOption::new(&format!("{category:?}"), &category.to_string()))
            .collect::<Vec<SelectOption>>(),
    );

    let onprimary_handler = Callback::new(move |_| {
        if form_is_valid.get() {
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
                                                        &form_data, false, None
                                                    );

                                                if deserialized_form_data.is_none() {
                                                    set_is_loading.set(false);
                                                    return;
                                                }

                                                let deserialized_form_data =
                                                    deserialized_form_data.unwrap();

                                                let input_vars = UserPortfolioInputVars {
                                                    portfolio_item: deserialized_form_data,
                                                    skills: applied_skills.get_untracked(),
                                                };

                                                let query = r#"
                                                       mutation CreatePortfolioItem($portfolioItem: UserPortfolioInput!, $skills: [String!]!) {
                                                            createPortfolioItem(portfolioItem: $portfolioItem, skills: $skills) {
                                                                data {
                                                                    id
                                                                    title
                                                                    description
                                                                    link
                                                                    startDate
                                                                    category
                                                                    thumbnail
                                                                    skills {
                                                                        id
                                                                        thumbnail
                                                                        name
                                                                    }
                                                                }
                                                                metadata {
                                                                    newAccessToken
                                                                    requestId
                                                                }
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
                                                        } else {
                                                        }
                                                        set_is_loading.set(false);

                                                        success_modal_is_open
                                                            .update(|status| *status = true);
                                                    }
                                                    None => {
                                                        set_is_loading.set(false);
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
                                    }
                                };
                            }
                            Err(err) => {
                                leptos::logging::error!("Failed to upload files: {:?}", err);
                                set_is_loading.set(false);
                            }
                        };
                    });
                };
            };
        }
    });

    let onreset_handler = Callback::new(move |_ev: ev::Event| {
        init_date.set(None);
    });

    Effect::new(move || {
        skills().get().iter().for_each(|skill| {
            let skill_option = SelectOption {
                value: skill.id.as_ref().unwrap().clone(),
                label: skill.name.as_ref().unwrap().clone(),
            };

            skills_select_options.write().push(skill_option);
        });
    });

    Effect::new(move || {
        set_is_loading.set(true);
        spawn_local(async move {
            let _fetch_skills_res = fetch_skills(&current_state, None).await;

            set_is_loading.set(false);
        });
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
                <div class="p-[10px]">
                    <p>"Portfolio Project created successfully!"</p>
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

            <div class="display-constraints">
                <Breadcrumbs custom_route_names=["Home", "Dashboard", "Portfolio", "New"] />
            </div>

            <h1 class="display-constraints">Create New Portfolio Project</h1>

            <ReactiveForm on:submit=handle_step_form_submit onreset=onreset_handler form_ref=form_ref>
                <div class="display-constraints flex flex-col gap-[20px]">
                    <InputField field_type=InputFieldType::Text label="Title" required=true id_attr="title" name="title" />
                    <Textarea label="Description" required=true id_attr="description" name="description" />
                    <DatePicker label="Start Date" required=true id_attr="start_date" initial_value=init_date name="start_date" />
                    <DatePicker label="End Date" required=true id_attr="end_date" initial_value=init_date name="end_date" />
                    <InputField field_type=InputFieldType::Text label="Link" required=true id_attr="link" name="link" />
                    <SelectInput
                    label="Category"
                    name="category"
                    required=true
                    id_attr="category"
                    placeholder="Select Category"
                    options=portfolio_categories
                    />
                    <CustomFileInput input_node_ref=file_input_ref label="Thumbnail" name="thumbnail" id_attr="thumbnail" accept="image/*" required=true />
                    <div class="flex flex-col gap-[10px]">
                        <h3>Applied Skills</h3>
                        <div class="flex flex-row items-center">
                        <CustomSelectInput
                                label="Skills"
                                id_attr="skills"
                                multiple=true
                                required=true
                                options=skills_select_options
                                value=applied_skills
                            />
                        </div>
                    </div>
                    <BasicButton
                        button_text="Submit"
                        style_ext="bg-primary text-contrast-white"
                        button_type=ButtonType::Submit
                        disabled=submit_is_disabled
                    />
                </div>
            </ReactiveForm>
        </>
    }
}
