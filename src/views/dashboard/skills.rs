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

use crate::components::forms::textarea::Textarea;
use crate::components::general::spinner::Spinner;
use crate::components::general::table::data_table::TableCellData;
use crate::data::context::shared::fetch_skills;
use crate::data::models::general::shared::RestResponse;
use crate::data::models::graphql::shared::{
    CreateUserSkillResponse, CreateUserSkillVars, UserSkillLevel, UserSkillType,
};
use crate::utils::custom_traits::EnumerableEnum;
use crate::utils::errors::unwrap_rest_response;
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
        context::store::{AppStateContext, AppStateContextStoreFields},
        models::{
            general::{
                acl::{AuthInfoStoreFields, UserInfoStoreFields},
                files::UploadedFileResponse,
            },
            graphql::shared::UserSkillInput,
        },
    },
    utils::forms::{deserialize_form_data_to_struct, get_form_data_from_form_ref},
};

const FILES_SERVICE_API: Option<&str> = option_env!("FILES_SERVICE_API");
const SHARED_SERVICE_API: Option<&str> = option_env!("SHARED_SERVICE_API");

#[component]
pub fn Skills() -> impl IntoView {
    view! {
        <>
            <Outlet />
        </>
    }
}

#[component]
pub fn SkillsList() -> impl IntoView {
    let store = expect_context::<Store<AppStateContext>>();
    let skills = move || store.skills();
    let (is_loading, set_is_loading) = signal(false);

    let table_data = RwSignal::new((
        vec![
            Column::new("Name", false),
            Column::new("Type", true),
            Column::new("Level", true),
            Column::new("Start Date", true),
        ],
        vec![],
    ));

    Effect::new(move || {
        let skills: Vec<HashMap<String, TableCellData>> = skills()
            .get()
            .iter()
            .map(|skill| {
                let mut hash_map_data = HashMap::new();

                // This id is the unique identifier of the table row. and is a MUST for the table to function properly.
                // *Note:* The id is a MUST for the table to function properly. You might be forced to generate a unique id for each row if your data does not have a unique identifier.
                hash_map_data.insert(
                    "id".to_string(),
                    TableCellData::String(
                        skill.id.as_ref().unwrap_or(&Default::default()).to_owned(),
                    ),
                );

                hash_map_data.insert(
                    "Name".to_string(),
                    TableCellData::String(
                        skill
                            .name
                            .as_ref()
                            .unwrap_or(&Default::default())
                            .to_owned(),
                    ),
                );

                hash_map_data.insert(
                    "YOE".to_string(),
                    TableCellData::Usize(
                        skill
                            .years_of_experience
                            .as_ref()
                            .unwrap_or(&Default::default())
                            .to_owned(),
                    ),
                );

                hash_map_data.insert(
                    "Start Date".to_string(),
                    TableCellData::DateTime(
                        skill
                            .start_date
                            .as_ref()
                            .unwrap_or(&Default::default())
                            .to_owned(),
                    ),
                );

                hash_map_data.insert(
                    "Level".to_string(),
                    TableCellData::String(format!(
                        "{:?}",
                        skill
                            .level
                            .as_ref()
                            .unwrap_or(&UserSkillLevel::Beginner)
                            .to_owned()
                    )),
                );
                hash_map_data.insert(
                    "Type".to_string(),
                    TableCellData::String(format!(
                        "{:?}",
                        skill
                            .r#type
                            .as_ref()
                            .unwrap_or(&UserSkillType::Technical)
                            .to_owned()
                    )),
                );
                hash_map_data
            })
            .collect();

        table_data.update(move |prev| {
            prev.1 = skills;
        });
    });

    Effect::new(move || {
        set_is_loading.set(true);
        spawn_local(async move {
            let _fetch_skills = fetch_skills(&store, None).await;

            set_is_loading.set(false);
        });
    });

    view! {
        <>
            <Title text="My Skills"/>
            <div class="display-constraints">
                <Breadcrumbs custom_route_names=["Home", "Dashboard", "Skills"] />
            </div>
            <Show when=move || is_loading.get()>
                <Spinner />
            </Show>

            <h1 class="display-constraints">My Skills</h1>

            <div class="display-constraints flex items-center justify-end">
                <A href="/dashboard/skills/create">
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

#[component]
pub fn CreateSkill() -> impl IntoView {
    let form_ref = NodeRef::new();
    let file_input_ref = NodeRef::new();
    let (form_is_valid, set_form_is_valid) = signal(false);
    let submit_is_disabled = Memo::new(move |_| !form_is_valid.get());
    let store = expect_context::<Store<AppStateContext>>();
    let success_modal_is_open = RwSignal::new(false);
    let confirm_modal_is_open = RwSignal::new(false);
    let init_date = RwSignal::new(None);
    let (is_loading, set_is_loading) = signal(false);
    let user_skill_levels = RwSignal::new(
        UserSkillLevel::variants_slice()
            .iter()
            .map(|level| SelectOption::new(&format!("{level:?}"), &level.to_string()))
            .collect::<Vec<SelectOption>>(),
    );
    let user_skill_types = RwSignal::new(
        UserSkillType::variants_slice()
            .iter()
            .map(|skill_type| {
                SelectOption::new(&format!("{skill_type:?}"), &skill_type.to_string())
            })
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

                    let Some(files_service_api) = FILES_SERVICE_API else {
                        return;
                    };

                    spawn_local(async move {
                        let Ok(request) =
                            gloo_net::http::Request::post(&format!("{files_service_api}/upload"))
                                .header(
                                    "Authorization",
                                    format!(
                                        "Bearer {}",
                                        store.user().auth_info().token().get_untracked()
                                    )
                                    .as_str(),
                                )
                                .body(files_form_data)
                        else {
                            set_is_loading.set(false);
                            return;
                        };

                        let body = match request.send().await {
                            Ok(r) => r,
                            Err(err) => {
                                leptos::logging::error!("Failed to upload files: {:?}", err);
                                set_is_loading.set(false);
                                return;
                            }
                        };

                        let body =
                            match body.json::<RestResponse<Vec<UploadedFileResponse>>>().await {
                                Ok(b) => b,
                                Err(err) => {
                                    leptos::logging::error!(
                                        "Failed to parse upload response: {:?}",
                                        err
                                    );
                                    set_is_loading.set(false);
                                    return;
                                }
                            };

                        let Some(uploaded_files) = unwrap_rest_response(body, &store, None) else {
                            set_is_loading.set(false);
                            return;
                        };

                        let Some(form_data) = get_form_data_from_form_ref(&form_ref) else {
                            set_is_loading.set(false);
                            return;
                        };

                        let Some(files_service_api) = FILES_SERVICE_API else {
                            return;
                        };

                        if let Err(e) = form_data.append_with_str(
                            "thumbnail",
                            format!("{files_service_api}/view/{}", uploaded_files[0].file_name)
                                .as_str(),
                        ) {
                            leptos::logging::log!("Error appending thumbnail: {:?}", e);
                            set_is_loading.set(false);
                            return;
                        }

                        let Some(deserialized_form_data) =
                            deserialize_form_data_to_struct::<UserSkillInput>(
                                &form_data, false, None,
                            )
                        else {
                            set_is_loading.set(false);
                            return;
                        };

                        let input_vars = CreateUserSkillVars {
                            skill: deserialized_form_data,
                        };

                        let query = r#"
                            mutation CreateSkill($skill: UserSkillInput!) {
                                createSkill(skill: $skill) {
                                    data {
                                        thumbnail
                                        name
                                        description
                                        level
                                        type
                                        startDate
                                        id
                                    }
                                    metadata {
                                        newAccessToken
                                        requestId
                                    }
                                }
                            }
                        "#;

                        let mut headers = HashMap::new() as HashMap<String, String>;
                        headers.insert(
                            "Authorization".into(),
                            format!(
                                "Bearer {}",
                                store.user().auth_info().token().get_untracked()
                            ),
                        );

                        let Some(shared_service_api) = SHARED_SERVICE_API else {
                            return;
                        };

                        let response = perform_mutation_or_query_with_vars::<
                            CreateUserSkillResponse,
                            CreateUserSkillVars,
                        >(
                            Some(&headers), shared_service_api, query, input_vars
                        )
                        .await;

                        match response.get_data() {
                            Some(_data) => {
                                if let Some(form) = form_ref
                                    .get_untracked()
                                    .and_then(|el| el.dyn_into::<HtmlFormElement>().ok())
                                {
                                    form.reset();
                                    set_form_is_valid.set(false);
                                }
                                set_is_loading.set(false);
                                success_modal_is_open.update(|status| *status = true);
                            }
                            None => {
                                set_is_loading.set(false);
                            }
                        }
                    });
                };
            };
        }
    });

    // This is to force the form to reset the date input
    let onreset_handler = Callback::new(move |_ev: ev::Event| {
        init_date.set(None);
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
            <Title text="Create Skill"/>
            <BasicModal title="Success" is_open=success_modal_is_open use_case=UseCase::Success disable_auto_close=false>
                <div class="p-[10px]">
                    <p>"Skill created successfully!"</p>
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
                <Breadcrumbs custom_route_names=["Home", "Dashboard", "Skills", "New"] />
            </div>

            <h1 class="display-constraints">Create New Skill</h1>

            <ReactiveForm on:submit=handle_step_form_submit onreset=onreset_handler form_ref=form_ref>
                <div class="display-constraints flex flex-col gap-[20px]">
                    <InputField field_type=InputFieldType::Text label="Name" required=true id_attr="name" name="name" />
                    <Textarea label="Description" required=true id_attr="description" name="description" />
                    <SelectInput
                    label="Type"
                    name="type"
                    required=true
                    id_attr="type"
                    placeholder="Select Type"
                    options=user_skill_types
                    />
                    <SelectInput
                    label="Level"
                    name="level"
                    required=true
                    id_attr="level"
                    placeholder="Select Level"
                    options=user_skill_levels
                    />
                    <DatePicker label="Start Date" required=true id_attr="start_date" initial_value=init_date name="start_date" />
                    <CustomFileInput input_node_ref=file_input_ref label="Thumbnail" name="thumbnail" id_attr="thumbnail" accept="image/*" required=true />
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
