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

use crate::components::forms::checkbox::{CheckboxGroup, CheckboxOption};
use crate::components::forms::select::{SelectInput, SelectOption};
use crate::components::general::spinner::Spinner;
use crate::components::general::table::data_table::TableCellData;
use crate::components::general::tag::LabelTag;
use crate::components::schemas::props::ColorTemperature;
use crate::data::context::shared::{
    fetch_departments, fetch_organizations, fetch_permissions, fetch_roles,
};
use crate::data::models::graphql::acl::{
    AdminPrivilege, CreateSystemRoleResponse, CreateSystemRoleVars, RoleInput, RoleMetadata,
};
use crate::utils::custom_traits::EnumerableEnum;
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

const ACL_SERVICE_API: Option<&str> = option_env!("ACL_SERVICE_API");

#[component]
pub fn Roles() -> impl IntoView {
    view! {
        <>
            <Outlet />
        </>
    }
}

#[component]
pub fn RolesList() -> impl IntoView {
    let store = expect_context::<Store<AppStateContext>>();
    let roles = move || store.roles();
    let (is_loading, set_is_loading) = signal(false);

    let table_data = RwSignal::new((
        vec![
            Column::new("Role Name", false),
            Column::new("Privilege", true),
        ],
        vec![],
    ));

    Effect::new(move || {
        set_is_loading.set(true);
        spawn_local(async move {
            let mut headers = HashMap::new() as HashMap<String, String>;
            headers.insert(
                "Authorization".into(),
                format!(
                    "Bearer {}",
                    store.user().auth_info().token().get_untracked()
                ),
            );

            let _fetch_roles_response = fetch_roles(&store, Some(&headers)).await;

            set_is_loading.set(false);
        });
    });

    Effect::new(move || {
        let roles: Vec<HashMap<String, TableCellData>> = roles()
            .get()
            .iter()
            .map(|role| {
                let mut hash_map_data = HashMap::new();

                // This id is the unique identifier of the table row. and is a MUST for the table to function properly.
                // *Note:* The id is a MUST for the table to function properly. You might be forced to generate a unique id for each row if your data does not have a unique identifier.
                hash_map_data.insert(
                    "id".to_string(),
                    TableCellData::String(
                        role.id.as_ref().unwrap_or(&Default::default()).to_owned(),
                    ),
                );

                hash_map_data.insert(
                    "Role Name".to_string(),
                    TableCellData::String(
                        role.role_name
                            .as_ref()
                            .unwrap_or(&Default::default())
                            .to_owned(),
                    ),
                );
                let privilege =
                    if role.is_admin.is_some() && role.is_admin.unwrap_or(Default::default()) {
                        ViewFn::from(move || {
                            view! {
                                <LabelTag label="Admin" color=ColorTemperature::Warning />
                            }
                        })
                    } else if role.is_super_admin.is_some()
                        && role.is_super_admin.unwrap_or(Default::default())
                    {
                        ViewFn::from(move || {
                            view! {
                                <LabelTag label="Super Admin" color=ColorTemperature::Danger />
                            }
                        })
                    } else {
                        ViewFn::from(move || {
                            view! {
                                <LabelTag label="None" />
                            }
                        })
                    };
                hash_map_data.insert("Privilege".to_string(), TableCellData::Html(privilege));
                hash_map_data
            })
            .collect();

        table_data.update(move |prev| {
            prev.1 = roles;
        });
    });

    view! {
        <>
            <Title text="Roles"/>
            <div class="display-constraints">
                <Breadcrumbs custom_route_names=["Home", "Dashboard", "Roles"] />
            </div>
            <Show when=move || is_loading.get()>
                <Spinner />
            </Show>

            <h1 class="display-constraints">Roles</h1>

            <div class="display-constraints flex items-center justify-end">
                <A href="/dashboard/roles/create">
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
pub fn CreateRole() -> impl IntoView {
    let form_ref = NodeRef::new();
    let metadata_form_ref = NodeRef::new();
    let (main_form_is_valid, set_main_form_is_valid) = signal(false);
    let (metadata_form_is_valid, set_metadata_form_is_valid) = signal(false);
    let submit_is_disabled =
        Memo::new(move |_| (!main_form_is_valid.get() || !metadata_form_is_valid.get()));
    let store = expect_context::<Store<AppStateContext>>();
    let departments = move || store.departments();
    let organizations = move || store.organizations();
    let permissions = move || store.permissions();
    let success_modal_is_open = RwSignal::new(false);
    let confirm_modal_is_open = RwSignal::new(false);
    let (is_loading, set_is_loading) = signal(false);
    let departments_options = RwSignal::new(vec![] as Vec<SelectOption>);
    let organizations_options = RwSignal::new(vec![] as Vec<SelectOption>);
    let permissions_options = RwSignal::new(vec![] as Vec<CheckboxOption>);

    let admin_privileges = RwSignal::new(
        AdminPrivilege::variants_slice()
            .iter()
            .map(|admin_privilege| {
                SelectOption::new(
                    &format!("{admin_privilege:?}"),
                    &admin_privilege.to_string(),
                )
            })
            .collect::<Vec<SelectOption>>(),
    );

    let onprimary_handler = Callback::new(move |_| {
        if metadata_form_is_valid.get() && main_form_is_valid.get() {
            set_is_loading.set(true);
            spawn_local(async move {
                if let Some(metadata_form_data) = get_form_data_from_form_ref(&metadata_form_ref) {
                    if let Some(main_form_data) = get_form_data_from_form_ref(&form_ref) {
                        let deserialized_main_form_data = deserialize_form_data_to_struct::<
                            RoleInput,
                        >(
                            &main_form_data, false, None
                        );
                        let deserialized_metadata_form_data =
                            deserialize_form_data_to_struct::<RoleMetadata>(
                                &metadata_form_data,
                                false,
                                Some(&["permission_ids"]),
                            );

                        if deserialized_main_form_data.is_none()
                            || deserialized_metadata_form_data.is_none()
                        {
                            set_is_loading.set(false);
                            return;
                        }

                        let deserialized_main_form_data =
                            deserialized_main_form_data.unwrap_or_default();
                        let deserialized_metadata_form_data =
                            deserialized_metadata_form_data.unwrap_or_default();

                        let input_vars = CreateSystemRoleVars {
                            role_input: deserialized_main_form_data,
                            role_metadata: deserialized_metadata_form_data,
                        };

                        let query = r#"
                               mutation CreateSystemRole($roleInput: RoleInput!, $roleMetadata: RoleMetadata!) {
                                    createSystemRole(roleInput: $roleInput, roleMetadata: $roleMetadata) {
                                        data {
                                            roleName
                                            createdAt
                                            isAdmin
                                            isDefault
                                            isSuperAdmin
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
                                store.user().auth_info().token().get_untracked()
                            ),
                        );

                        let Some(acl_service_api) = ACL_SERVICE_API else {
                            return;
                        };

                        let response = perform_mutation_or_query_with_vars::<
                            CreateSystemRoleResponse,
                            CreateSystemRoleVars,
                        >(
                            Some(&headers), acl_service_api, query, input_vars
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
                    store.user().auth_info().token().get_untracked()
                ),
            );

            let _fetch_organizations_res = fetch_organizations(&store, Some(&headers)).await;
            let _fetch_departments_res = fetch_departments(&store, Some(&headers)).await;
            let _fetch_permissions_res = fetch_permissions(&store, Some(&headers)).await;

            set_is_loading.set(false);
        });
    });

    Effect::new(move || {
        organizations_options.set(
            organizations()
                .get()
                .iter()
                .map(|org| {
                    SelectOption::new(
                        org.id.as_ref().unwrap_or(&Default::default()),
                        org.org_name.as_ref().unwrap_or(&Default::default()),
                    )
                })
                .collect(),
        );

        departments_options.set(
            departments()
                .get()
                .iter()
                .map(|dep| {
                    SelectOption::new(
                        dep.id.as_ref().unwrap_or(&Default::default()),
                        dep.dep_name.as_ref().unwrap_or(&Default::default()),
                    )
                })
                .collect(),
        );

        permissions_options.set(
            permissions()
                .get()
                .iter()
                .map(|permission| {
                    CheckboxOption::new(
                        permission.id.as_ref().unwrap_or(&Default::default()),
                        permission.name.as_ref().unwrap_or(&Default::default()),
                        None,
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
            <Title text="New Role"/>
            <BasicModal title="Success" is_open=success_modal_is_open use_case=UseCase::Success disable_auto_close=false>
                <div class="p-[10px]">
                    <p>"Role created successfully!"</p>
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
                <Breadcrumbs custom_route_names=["Home", "Dashboard", "Roles", "New"] />
            </div>

            <h1 class="display-constraints">New Role</h1>

            <h2 class="display-constraints">Role Metadata</h2>
            <ReactiveForm on:submit=handle_metadata_form_submit form_ref=metadata_form_ref>
                <div class="display-constraints flex flex-col gap-[20px]">
                <SelectInput
                label="Admin Privilege"
                name="admin_privilege"
                required=true
                id_attr="admin_privilege"
                placeholder="Select Admin Privilege"
                options=admin_privileges
                />
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
                <CheckboxGroup
                    legend="Permissions"
                    name="permission_ids"
                    options=permissions_options
                />
                </div>
            </ReactiveForm>

            <h2 class="display-constraints">Role Info</h2>
            <ReactiveForm on:submit=handle_main_form_submit form_ref=form_ref>
                <div class="display-constraints flex flex-col gap-[20px]">
                    <InputField field_type=InputFieldType::Text label="Role Name" required=true id_attr="role_name" name="role_name" />

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
