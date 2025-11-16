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
use crate::data::models::graphql::acl::{
    CreateResourceResponse, CreateResourceVars, FetchDepartmentsResponse,
    FetchOrganizationsResponse, FetchResourcesResponse, ResourceInput, ResourceMetadata,
};
use crate::utils::graphql_client::{
    perform_mutation_or_query_with_vars, perform_query_without_vars,
};
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
    data::models::general::acl::{
        AppStateContext, AppStateContextStoreFields, AuthInfoStoreFields, UserInfoStoreFields,
    },
    utils::forms::{deserialize_form_data_to_struct, get_form_data_from_form_ref},
};

#[island]
pub fn Resources() -> impl IntoView {
    view! {
        <>
            <Outlet />
        </>
    }
}

#[island]
pub fn ResourcesList() -> impl IntoView {
    let current_state = expect_context::<Store<AppStateContext>>();
    let (is_loading, set_is_loading) = signal(false);

    let table_data = RwSignal::new((
        vec![
            Column::new("Resource Name", false),
            Column::new("Date of Creation", true),
        ],
        vec![],
    ));

    Effect::new(move || {
        set_is_loading.set(true);
        spawn_local(async move {
            let fetch_resources_query = r#"
                   query FetchResources {
                        fetchResources {
                           name
                           id
                           createdBy
                           createdAt
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

            let fetch_resources_response = perform_query_without_vars::<FetchResourcesResponse>(
                Some(&headers),
                "http://localhost:8080/api/acl",
                fetch_resources_query,
            )
            .await;

            match fetch_resources_response.get_data() {
                Some(data) => {
                    let resources: Vec<HashMap<String, TableCellData>> = data
                        .fetch_resources
                        .as_ref()
                        .unwrap()
                        .to_vec()
                        .iter()
                        .map(|resource| {
                            let mut hash_map_data = HashMap::new();

                            // This id is the unique identifier of the table row. and is a MUST for the table to function properly.
                            // *Note:* The id is a MUST for the table to function properly. You might be forced to generate a unique id for each row if your data does not have a unique identifier.
                            hash_map_data.insert(
                                "id".to_string(),
                                TableCellData::String(resource.id.as_ref().unwrap().to_owned()),
                            );

                            hash_map_data.insert(
                                "Resource Name".to_string(),
                                TableCellData::String(resource.name.as_ref().unwrap().to_owned()),
                            );

                            hash_map_data.insert(
                                "Date of Creation".to_string(),
                                TableCellData::DateTime(
                                    resource.created_at.as_ref().unwrap().to_owned(),
                                ),
                            );
                            hash_map_data
                        })
                        .collect();

                    table_data.update(move |prev| {
                        prev.1 = resources;
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
            <Title text="Resources"/>
            <div class="mx-[5%] md:mx-[10%]">
                <Breadcrumbs custom_route_names=["Home", "Dashboard", "Resources"] />
            </div>
            <Show when=move || is_loading.get()>
                <Spinner />
            </Show>

            <h1 class="mx-[5%] md:mx-[10%]">Resources</h1>

            <div class="mx-[5%] md:mx-[10%] flex items-center justify-end">
                <A href="/dashboard/resources/create">
                    <BasicButton
                        button_text="Create"
                        icon=Some(IconData::BsPlusLg)
                        icon_before=true
                        style_ext="bg-primary text-white"
                    />
                </A>
            </div>

            <div class="mx-[5%] md:mx-[10%]">
                <DataTable data=table_data editable=true deletable=true />
            </div>
        </>
    }
}

#[island]
pub fn CreateResource() -> impl IntoView {
    let form_ref = NodeRef::new();
    let metadata_form_ref = NodeRef::new();
    let (main_form_is_valid, set_main_form_is_valid) = signal(false);
    let (metadata_form_is_valid, set_metadata_form_is_valid) = signal(false);
    let submit_is_disabled =
        Memo::new(move |_| (!main_form_is_valid.get() || !metadata_form_is_valid.get()));
    let current_state = expect_context::<Store<AppStateContext>>();
    let success_modal_is_open = RwSignal::new(false);
    let confirm_modal_is_open = RwSignal::new(false);
    let (submission_confirmed, set_submission_confirmed) = signal(false);
    let (is_loading, set_is_loading) = signal(false);
    let departments =
        RwSignal::new(vec![SelectOption::new("", "Select Department")] as Vec<SelectOption>);
    let organizations =
        RwSignal::new(vec![SelectOption::new("", "Select Organization")] as Vec<SelectOption>);

    let onprimary_handler = Callback::new(move |_| {
        set_submission_confirmed.set(true);
    });

    Effect::new(move || {
        if submission_confirmed.get() && metadata_form_is_valid.get() && main_form_is_valid.get() {
            set_is_loading.set(true);
            spawn_local(async move {
                if let Some(metadata_form_data) = get_form_data_from_form_ref(&metadata_form_ref) {
                    if let Some(main_form_data) = get_form_data_from_form_ref(&form_ref) {
                        let deserialized_main_form_data = deserialize_form_data_to_struct::<
                            ResourceInput,
                        >(
                            &main_form_data, false, None
                        );
                        let deserialized_metadata_form_data = deserialize_form_data_to_struct::<
                            ResourceMetadata,
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

                        let input_vars = CreateResourceVars {
                            resource_input: deserialized_main_form_data,
                            resource_metadata: deserialized_metadata_form_data,
                        };

                        let query = r#"
                               mutation CreateResource($resourceInput: ResourceInput!, $resourceMetadata: ResourceMetadata!) {
                                    createResource(resourceInput: $resourceInput, resourceMetadata: $resourceMetadata) {
                                        name
                                        id
                                        createdBy
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
                            CreateResourceResponse,
                            CreateResourceVars,
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
                                    set_submission_confirmed.set(false);
                                } else {
                                    set_submission_confirmed.set(false);
                                }

                                if let Some(form) = metadata_form_ref
                                    .get_untracked()
                                    .and_then(|el| el.dyn_into::<HtmlFormElement>().ok())
                                {
                                    form.reset();
                                    set_main_form_is_valid.set(false);
                                    set_submission_confirmed.set(false);
                                } else {
                                    set_submission_confirmed.set(false);
                                }

                                set_is_loading.set(false);

                                success_modal_is_open.update(|status| *status = true);
                            }
                            None => {
                                set_is_loading.set(false);
                                set_submission_confirmed.set(false);
                            }
                        };
                    };
                };
            });
        }
    });

    Effect::new(move || {
        spawn_local(async move {
            let fetch_orgs_query = r#"
                   query FetchOrganizations {
                        fetchOrganizations {
                            orgName
                            id
                        }
                   }
               "#;

            let fetch_deps_query = r#"
                   query FetchDepartments {
                        fetchDepartments {
                            depName
                            id
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

            let fetch_orgs_response = perform_query_without_vars::<FetchOrganizationsResponse>(
                Some(&headers),
                "http://localhost:8080/api/acl",
                fetch_orgs_query,
            )
            .await;

            match fetch_orgs_response.get_data() {
                Some(data) => {
                    organizations.update(move |prev| {
                        let mut orgs = data
                            .fetch_organizations
                            .as_ref()
                            .unwrap()
                            .iter()
                            .map(|org| {
                                SelectOption::new(
                                    org.id.as_ref().unwrap().as_str(),
                                    org.org_name.as_ref().unwrap().as_str(),
                                )
                            })
                            .collect();

                        prev.append(&mut orgs);
                    });
                    set_is_loading.set(false);
                }
                None => {
                    set_is_loading.set(false);
                }
            };

            let fetch_deps_response = perform_query_without_vars::<FetchDepartmentsResponse>(
                Some(&headers),
                "http://localhost:8080/api/acl",
                fetch_deps_query,
            )
            .await;

            match fetch_deps_response.get_data() {
                Some(data) => {
                    departments.update(move |prev| {
                        let mut deps = data
                            .fetch_departments
                            .as_ref()
                            .unwrap()
                            .to_vec()
                            .iter()
                            .map(|dep| {
                                SelectOption::new(
                                    dep.id.as_ref().unwrap().as_str(),
                                    dep.dep_name.as_ref().unwrap().as_str(),
                                )
                            })
                            .collect();
                        prev.append(&mut deps);
                    });
                    set_is_loading.set(false);
                }
                None => {
                    set_is_loading.set(false);
                }
            };
        });
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
            <Title text="New Resource"/>
            <BasicModal title="Success" is_open=success_modal_is_open use_case=UseCase::Success disable_auto_close=false>
                <div>
                    <p>"Resource created successfully!"</p>
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

            <div class="mx-[5%] md:mx-[10%]">
                <Breadcrumbs custom_route_names=["Home", "Dashboard", "Resources", "New"] />
            </div>

            <h1 class="mx-[5%] md:mx-[10%]">New Resource</h1>

            <h2 class="mx-[5%] md:mx-[10%]">Resource Metadata</h2>
            <ReactiveForm on:submit=handle_metadata_form_submit form_ref=metadata_form_ref>
                <div class="mx-[5%] md:mx-[10%] flex flex-col gap-[20px]">
                <SelectInput
                label="Organization"
                name="organization_id"
                id_attr="organization_id"
                options=organizations
                />
                <SelectInput
                label="Department"
                name="department_id"
                id_attr="department_id"
                options=departments
                />
                </div>
            </ReactiveForm>

            <h2 class="mx-[5%] md:mx-[10%]">Resource Info</h2>
            <ReactiveForm on:submit=handle_main_form_submit form_ref=form_ref>
                <div class="mx-[5%] md:mx-[10%] flex flex-col gap-[20px]">
                    <InputField field_type=InputFieldType::Text label="Resource Name" required=true id_attr="name" name="name" />

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
