use std::collections::HashMap;

use icondata as IconData;
use leptos::ev::SubmitEvent;
use leptos::prelude::*;
use leptos::task::spawn_local;
use leptos::wasm_bindgen::JsCast;
use leptos_meta::*;
use leptos_router::components::{A, Outlet};
use reactive_stores::Store;
use web_sys::HtmlFormElement;

use crate::components::forms::select::{SelectInput, SelectOption};
use crate::components::general::spinner::Spinner;
use crate::components::general::table::data_table::TableCellData;
use crate::data::context::shared::{fetch_departments, fetch_organizations};
use crate::data::models::graphql::acl::{
    CreateDepartmentResponse, CreateDepartmentVars, DepartmentInput, DepartmentMetadata,
};
use crate::utils::graphql_client::perform_mutation_or_query_with_vars;
use crate::{
    components::{
        forms::{
            input::{InputField, InputFieldType},
            reactive_form::ReactiveForm,
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
        models::general::acl::{AuthInfoStoreFields, UserInfoStoreFields},
    },
    utils::forms::{deserialize_form_data_to_struct, get_form_data_from_form_ref},
};

#[island]
pub fn Departments() -> impl IntoView {
    view! {
        <>
            <Outlet />
        </>
    }
}

#[island]
pub fn DepartmentsList() -> impl IntoView {
    let current_state = expect_context::<Store<AppStateContext>>();
    let departments = move || current_state.departments();
    let (is_loading, set_is_loading) = signal(false);

    let table_data = RwSignal::new((
        vec![
            Column::new("Name", false),
            Column::new("Date of Creation", true),
        ],
        vec![],
    ));

    Effect::new(move || {
        let departments_data: Vec<HashMap<String, TableCellData>> = departments()
            .get()
            .iter()
            .map(|department| {
                let mut hash_map_data = HashMap::new();

                // This id is the unique identifier of the table row. and is a MUST for the table to function properly.
                // *Note:* The id is a MUST for the table to function properly. You might be forced to generate a unique id for each row if your data does not have a unique identifier.
                hash_map_data.insert(
                    "id".to_string(),
                    TableCellData::String(department.id.as_ref().unwrap().to_owned()),
                );

                hash_map_data.insert(
                    "Name".to_string(),
                    TableCellData::String(department.dep_name.as_ref().unwrap().to_owned()),
                );

                hash_map_data.insert(
                    "Date of Creation".to_string(),
                    TableCellData::DateTime(department.created_at.as_ref().unwrap().to_owned()),
                );
                hash_map_data
            })
            .collect();

        table_data.update(move |prev| {
            prev.1 = departments_data;
        });
    });

    Effect::new(move || {
        set_is_loading.set(true);
        spawn_local(async move {
            let mut headers = HashMap::new() as HashMap<String, String>;
            headers.insert(
                "Authorization".into(),
                format!(
                    "Bearer {}",
                    current_state.user().auth_info().token().get_untracked()
                ),
            );

            let _fetch_departments_res = fetch_departments(&current_state, Some(&headers)).await;

            set_is_loading.set(false);
        });
    });

    view! {
        <>
            <Title text="Departments"/>
            <div class="display-constraints">
                <Breadcrumbs custom_route_names=["Home", "Dashboard", "Departments"] />
            </div>
            <Show when=move || is_loading.get()>
                <Spinner />
            </Show>

            <h1 class="display-constraints">Departments</h1>

            <div class="display-constraints flex items-center justify-end">
                <A href="/dashboard/departments/create">
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
pub fn CreateDepartment() -> impl IntoView {
    let form_ref = NodeRef::new();
    let metadata_form_ref = NodeRef::new();
    let (main_form_is_valid, set_main_form_is_valid) = signal(false);
    let (metadata_form_is_valid, set_metadata_form_is_valid) = signal(false);
    let submit_is_disabled =
        Memo::new(move |_| !main_form_is_valid.get() || !metadata_form_is_valid.get());
    let current_state = expect_context::<Store<AppStateContext>>();
    let departments = move || current_state.departments();
    let organizations = move || current_state.organizations();
    let success_modal_is_open = RwSignal::new(false);
    let confirm_modal_is_open = RwSignal::new(false);
    let (is_loading, set_is_loading) = signal(false);
    let departments_options = RwSignal::new(vec![] as Vec<SelectOption>);
    let organizations_options = RwSignal::new(vec![] as Vec<SelectOption>);

    let onprimary_handler = Callback::new(move |_| {
        if metadata_form_is_valid.get() && main_form_is_valid.get() {
            set_is_loading.set(true);
            spawn_local(async move {
                if let Some(metadata_form_data) = get_form_data_from_form_ref(&metadata_form_ref) {
                    if let Some(main_form_data) = get_form_data_from_form_ref(&form_ref) {
                        let deserialized_main_form_data = deserialize_form_data_to_struct::<
                            DepartmentInput,
                        >(
                            &main_form_data, false, None
                        );
                        let deserialized_metadata_form_data = deserialize_form_data_to_struct::<
                            DepartmentMetadata,
                        >(
                            &metadata_form_data, false, None
                        );

                        if deserialized_main_form_data.is_none()
                            || deserialized_metadata_form_data.is_none()
                        {
                            set_is_loading.set(false);
                            return;
                        }

                        let deserialized_main_form_data = deserialized_main_form_data.unwrap();
                        let deserialized_metadata_form_data =
                            deserialized_metadata_form_data.unwrap();

                        let input_vars = CreateDepartmentVars {
                            department_input: deserialized_main_form_data,
                            department_metadata: deserialized_metadata_form_data,
                        };

                        let query = r#"
                               mutation CreateDepartment($departmentInput: DepartmentInput!, $departmentMetadata: DepartmentMetadata!) {
                                    createDepartment(departmentInput: $departmentInput, departmentMetadata: $departmentMetadata) {
                                        data {
                                            depName
                                            createdAt
                                            updatedAt
                                            id
                                            createdBy
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
                                current_state.user().auth_info().token().get_untracked()
                            ),
                        );

                        let response = perform_mutation_or_query_with_vars::<
                            CreateDepartmentResponse,
                            CreateDepartmentVars,
                        >(
                            Some(&headers),
                            "http://localhost:8080/api/acl",
                            query,
                            input_vars,
                        )
                        .await;

                        match response.get_data() {
                            Some(_data) => {
                                if let Some(form) = form_ref
                                    .get_untracked()
                                    .and_then(|el| el.dyn_into::<HtmlFormElement>().ok())
                                {
                                    form.reset();
                                    set_main_form_is_valid.set(false);
                                } else {
                                }

                                if let Some(form) = metadata_form_ref
                                    .get_untracked()
                                    .and_then(|el| el.dyn_into::<HtmlFormElement>().ok())
                                {
                                    form.reset();
                                    set_main_form_is_valid.set(false);
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
                };
            });
        }
    });

    Effect::new(move || {
        spawn_local(async move {
            let mut headers = HashMap::new() as HashMap<String, String>;
            headers.insert(
                "Authorization".into(),
                format!(
                    "Bearer {}",
                    current_state.user().auth_info().token().get_untracked()
                ),
            );

            let _fetch_orgs = fetch_organizations(&current_state, Some(&headers)).await;

            let _fetch_departments_res = fetch_departments(&current_state, Some(&headers)).await;

            set_is_loading.set(false);
        });
    });

    Effect::new(move || {
        departments_options.set(
            departments()
                .get()
                .iter()
                .map(|dep| {
                    SelectOption::new(dep.id.as_ref().unwrap(), dep.dep_name.as_ref().unwrap())
                })
                .collect(),
        );

        organizations_options.set(
            organizations()
                .get()
                .iter()
                .map(|org| {
                    SelectOption::new(
                        org.id.as_ref().unwrap().as_str(),
                        org.org_name.as_ref().unwrap().as_str(),
                    )
                })
                .collect(),
        );
    });

    let handle_metadata_form_submit = move |ev: SubmitEvent| {
        ev.prevent_default();
        ev.stop_propagation();

        // Implement logic to show form validity
        let target = ev
            .target()
            .and_then(|t| t.dyn_into::<HtmlFormElement>().ok());

        if let Some(form) = target {
            set_metadata_form_is_valid.set(form.check_validity());
        }
    };

    let handle_main_form_submit = move |ev: SubmitEvent| {
        ev.prevent_default();
        ev.stop_propagation();

        // Implement logic to show form validity
        let target = ev
            .target()
            .and_then(|t| t.dyn_into::<HtmlFormElement>().ok());

        if let Some(form) = target {
            set_main_form_is_valid.set(form.check_validity());

            if let Some(_submitter) = ev.submitter() {
                confirm_modal_is_open.update(|status| *status = true);
            }
        }
    };

    view! {
        <>
            <Title text="New Department"/>
            <BasicModal title="Success" is_open=success_modal_is_open use_case=UseCase::Success disable_auto_close=false>
                <div class="p-[10px]">
                    <p>"Department created successfully!"</p>
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
                <Breadcrumbs custom_route_names=["Home", "Dashboard", "Departments", "New"] />
            </div>

            <h1 class="display-constraints">New Department</h1>

            <h2 class="display-constraints">Department Metadata</h2>
            <ReactiveForm on:submit=handle_metadata_form_submit form_ref=metadata_form_ref>
                <div class="display-constraints flex flex-col gap-[20px]">
                <SelectInput
                label="Organization"
                name="organization_id"
                id_attr="organization_id"
                placeholder="Select Organization"
                options=organizations_options
                />
                <SelectInput
                label="Department"
                name="department_id"
                id_attr="department_id"
                placeholder="Select Department"
                options=departments_options
                />
                </div>
            </ReactiveForm>

            <h2 class="display-constraints">Department Info</h2>
            <ReactiveForm on:submit=handle_main_form_submit form_ref=form_ref>
                <div class="display-constraints flex flex-col gap-[20px]">
                    <InputField field_type=InputFieldType::Text label="Department Name" required=true id_attr="dep_name" name="dep_name" />

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
