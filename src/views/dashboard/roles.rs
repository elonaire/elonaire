use std::collections::HashMap;

use icondata as IconData;
use leptos::ev::{self, SubmitEvent};
use leptos::prelude::*;
use leptos::task::spawn_local;
use leptos::wasm_bindgen::JsCast;
use leptos_meta::*;
use leptos_router::components::{A, Outlet};
use reactive_stores::Store;
use web_sys::HtmlFormElement;

use crate::components::forms::checkbox::{CheckboxGroup, CheckboxOption};
use crate::components::forms::select::{SelectInput, SelectOption};
use crate::components::general::spinner::Spinner;
use crate::data::models::graphql::acl::{
    AdminPermission, CreateSystemRoleResponse, CreateSystemRoleVars, FetchDepartmentsResponse,
    FetchOrganizationsResponse, RoleInput, RoleMetadata, RoleType,
};
use crate::utils::custom_traits::EnumerableEnum;
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
pub fn Roles() -> impl IntoView {
    view! {
        <>
            <Outlet />
        </>
    }
}

#[island]
pub fn RolesList() -> impl IntoView {
    let table_data = RwSignal::new((
        vec![
            Column::new("Role Name", false),
            Column::new("Privilege", true),
        ],
        vec![],
    ));

    Effect::new(move || {
        spawn_local(async move {});
    });

    view! {
        <>
            <Title text="Roles"/>
            <div class="mx-[20px]">
                <Breadcrumbs custom_route_names=["Home", "Dashboard", "Roles"] />
            </div>

            <h1 class="mx-[20px]">Roles</h1>

            <div class="mx-[20px] flex items-center justify-end">
                <A href="/dashboard/roles/create">
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
pub fn CreateRole() -> impl IntoView {
    let form_ref = NodeRef::new();
    let metadata_form_ref = NodeRef::new();
    let (main_form_is_valid, set_main_form_is_valid) = signal(false);
    let (metadata_form_is_valid, set_metadata_form_is_valid) = signal(false);
    let submit_is_disabled =
        Memo::new(move |_| (!main_form_is_valid.get() && !metadata_form_is_valid.get()));
    let current_state = expect_context::<Store<AppStateContext>>();
    let success_modal_is_open = RwSignal::new(false);
    let confirm_modal_is_open = RwSignal::new(false);
    let (submission_confirmed, set_submission_confirmed) = signal(false);
    let (is_loading, set_is_loading) = signal(false);
    let departments =
        RwSignal::new(vec![SelectOption::new("", "Select Department")] as Vec<SelectOption>);
    let organizations =
        RwSignal::new(vec![SelectOption::new("", "Select Organization")] as Vec<SelectOption>);

    let role_types = RwSignal::new(
        RoleType::variants_slice()
            .iter()
            .map(|role_type| {
                let mut label = format!("{}", role_type);
                if label.is_empty() {
                    label = "Select Role Type".to_string();
                }
                SelectOption::new(format!("{}", role_type).as_str(), label.as_str())
            })
            .collect::<Vec<SelectOption>>(),
    );

    let admin_permissions = RwSignal::new(
        AdminPermission::variants_slice()
            .iter()
            .map(|admin_permission| {
                let mut label = format!("{}", admin_permission);
                if label.is_empty() {
                    label = "Select Admin Permission".to_string();
                }
                CheckboxOption::new(
                    format!("{}", admin_permission).as_str(),
                    label.as_str(),
                    None,
                )
            })
            .collect::<Vec<CheckboxOption>>(),
    );

    let onprimary_handler = Callback::new(move |_| {
        set_submission_confirmed.set(true);
    });

    Effect::new(move || {
        if submission_confirmed.get() && metadata_form_is_valid.get() && main_form_is_valid.get() {
            set_is_loading.set(true);
            spawn_local(async move {
                if let Some(metadata_form_data) = get_form_data_from_form_ref(&metadata_form_ref) {
                    if let Some(main_form_data) = get_form_data_from_form_ref(&form_ref) {
                        let deserialized_main_form_data =
                            deserialize_form_data_to_struct::<RoleInput>(&main_form_data, false);
                        let deserialized_metadata_form_data = deserialize_form_data_to_struct::<
                            RoleMetadata,
                        >(
                            &metadata_form_data, false
                        );

                        leptos::logging::log!(
                            "deserialized_main_form_data: {:?}",
                            deserialized_main_form_data
                        );
                        leptos::logging::log!(
                            "deserialized_metadata_form_data: {:?}",
                            deserialized_metadata_form_data
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

                        let input_vars = CreateSystemRoleVars {
                            role_input: deserialized_main_form_data,
                            role_metadata: deserialized_metadata_form_data,
                        };

                        let query = r#"
                               mutation CreateSystemRole($roleInput: RoleInput!, $roleMetadata: RoleMetadata!) {
                                    createSystemRole(roleInput: $roleInput, roleMetadata: $roleMetadata) {
                                        roleName
                                        createdAt
                                        isAdmin
                                        isDefault
                                        isSuperAdmin
                                        updatedAt
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
                            CreateSystemRoleResponse,
                            CreateSystemRoleVars,
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

        leptos::logging::log!("SubmitEvent triggered: handle_metadata_form_submit");

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

        leptos::logging::log!("SubmitEvent triggered: handle_main_form_submit");

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
            <Title text="New Role"/>
            <BasicModal title="Success" is_open=success_modal_is_open use_case=UseCase::Success disable_auto_close=false>
                <div>
                    <p>"Role created successfully!"</p>
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
                <Breadcrumbs custom_route_names=["Home", "Dashboard", "Roles", "New"] />
            </div>

            <h1 class="mx-[20px]">New Role</h1>

            <h2 class="mx-[20px]">Role Metadata</h2>
            <ReactiveForm on:submit=handle_metadata_form_submit form_ref=metadata_form_ref>
                <div class="mx-[20px] flex flex-col gap-[20px]">
                <SelectInput
                label="Role Type"
                name="role_type"
                required=true
                id_attr="role_type"
                options=role_types
                />
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

            <h2 class="mx-[20px]">Role Info</h2>
            <ReactiveForm on:submit=handle_main_form_submit form_ref=form_ref>
                <div class="mx-[20px] flex flex-col gap-[20px]">
                    <InputField field_type=InputFieldType::Text label="Role Name" required=true id_attr="role_name" name="role_name" />
                    <CheckboxGroup
                        legend="Admin Permissions"
                        name="admin_permissions"
                        options=admin_permissions
                    />

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
