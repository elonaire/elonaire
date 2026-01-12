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

use crate::components::general::spinner::Spinner;
use crate::components::general::table::data_table::TableCellData;
use crate::components::general::tag::LabelTag;
use crate::components::schemas::props::ColorTemperature;
use crate::data::models::graphql::acl::{
    AccountStatus, FetchUsersResponse, SignUpResponse, SignUpVars, UserInput,
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
    data::{
        context::store::{AppStateContext, AppStateContextStoreFields},
        models::general::acl::{AuthInfoStoreFields, UserInfoStoreFields},
    },
    utils::forms::{deserialize_form_data_to_struct, get_form_data_from_form_ref},
};

#[island]
pub fn Users() -> impl IntoView {
    view! {
        <>
            <Outlet />
        </>
    }
}

#[island]
pub fn UsersList() -> impl IntoView {
    let current_state = expect_context::<Store<AppStateContext>>();
    let (is_loading, set_is_loading) = signal(false);

    let table_data = RwSignal::new((
        vec![
            Column::new("Full Name", false),
            Column::new("Email", true),
            Column::new("OAuth Client", true),
            Column::new("Status", true),
        ],
        vec![],
    ));

    Effect::new(move || {
        set_is_loading.set(true);
        spawn_local(async move {
            let fetch_users_query = r#"
                   query FetchUsers {
                        fetchUsers {
                            id
                            email
                            status
                            oauthClient
                            fullName
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

            let fetch_users_response = perform_query_without_vars::<FetchUsersResponse>(
                Some(&headers),
                "http://localhost:8080/api/acl",
                fetch_users_query,
            )
            .await;

            match fetch_users_response.get_data() {
                Some(data) => {
                    let users: Vec<HashMap<String, TableCellData>> = data
                        .fetch_users
                        .as_ref()
                        .unwrap()
                        .to_vec()
                        .iter()
                        .map(|user| {
                            let mut hash_map_data = HashMap::new();

                            // This id is the unique identifier of the table row. and is a MUST for the table to function properly.
                            // *Note:* The id is a MUST for the table to function properly. You might be forced to generate a unique id for each row if your data does not have a unique identifier.
                            hash_map_data.insert(
                                "id".to_string(),
                                TableCellData::String(user.id.as_ref().unwrap().to_owned()),
                            );

                            hash_map_data.insert(
                                "Full Name".to_string(),
                                TableCellData::String(
                                    user.full_name.as_ref().unwrap_or(&String::new()).to_owned(),
                                ),
                            );
                            hash_map_data.insert(
                                "Email".to_string(),
                                TableCellData::String(user.email.to_owned()),
                            );
                            let oauth_client = match user.oauth_client {
                                Some(client) => format!("{:?}", client),
                                None => String::from("None")
                            };
                            hash_map_data.insert(
                                "OAuth Client".to_string(),
                                TableCellData::String(
                                    oauth_client,
                                ),
                            );
                            let status = match user.status.as_ref().unwrap() {
                                AccountStatus::Active => ViewFn::from(move || view! {
                                    <LabelTag label="Active" color=ColorTemperature::Success />
                                }),
                                AccountStatus::Inactive => ViewFn::from(move || view! {
                                    <LabelTag label="InActive" color=ColorTemperature::Info />
                                }),
                                AccountStatus::Suspended => ViewFn::from(move || view! {
                                    <LabelTag label="Suspended" color=ColorTemperature::Warning />
                                }),
                                AccountStatus::Deleted => ViewFn::from(move || view! {
                                    <LabelTag label="Deleted" color=ColorTemperature::Danger />
                                }),
                            };
                            hash_map_data.insert(
                                "Status".to_string(),
                                TableCellData::Html(status),
                            );
                            hash_map_data
                        })
                        .collect();

                    table_data.update(move |prev| {
                        prev.1 = users;
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
            <Title text="Users"/>
            <div class="mx-[5%] md:mx-[10%]">
                <Breadcrumbs custom_route_names=["Home", "Dashboard", "Users"] />
            </div>
            <Show when=move || is_loading.get()>
                <Spinner />
            </Show>

            <h1 class="mx-[5%] md:mx-[10%]">Users</h1>

            <div class="mx-[5%] md:mx-[10%] flex items-center justify-end">
                <A href="/dashboard/users/create">
                    <BasicButton
                        button_text="Create"
                        icon=Some(IconData::BsPlusLg)
                        icon_before=true
                        style_ext="bg-primary text-contrast-white"
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
pub fn CreateUser() -> impl IntoView {
    let form_ref = NodeRef::new();
    let (form_is_valid, set_form_is_valid) = signal(false);
    let submit_is_disabled = Memo::new(move |_| !form_is_valid.get());
    let current_state = expect_context::<Store<AppStateContext>>();
    let success_modal_is_open = RwSignal::new(false);
    let confirm_modal_is_open = RwSignal::new(false);
    let (is_loading, set_is_loading) = signal(false);

    let onprimary_handler = Callback::new(move |_| {
        if form_is_valid.get() {
            set_is_loading.set(true);
            spawn_local(async move {
                if let Some(form_data) = get_form_data_from_form_ref(&form_ref) {
                    let deserialized_form_data =
                        deserialize_form_data_to_struct::<UserInput>(&form_data, true, None);

                    if deserialized_form_data.is_none() {
                        set_is_loading.set(false);
                        return;
                    }

                    let deserialized_form_data = deserialized_form_data.unwrap();

                    let input_vars = SignUpVars {
                        user: deserialized_form_data,
                    };

                    let query = r#"
                           mutation SignUp($user: UserInput!) {
                                signUp(user: $user) {
                                    id
                                    fullName
                                    email
                                    status
                                    oauthClient
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

                    let response =
                        perform_mutation_or_query_with_vars::<SignUpResponse, SignUpVars>(
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
                                set_form_is_valid.set(false);
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
            <Title text="New User"/>
            <BasicModal title="Success" is_open=success_modal_is_open use_case=UseCase::Success disable_auto_close=false>
                <div class="p-[10px]">
                    <p>"User created successfully!"</p>
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

            <div class="mx-[5%] md:mx-[10%]">
                <Breadcrumbs custom_route_names=["Home", "Dashboard", "Users", "New"] />
            </div>

            <h1 class="mx-[5%] md:mx-[10%]">New User</h1>

            <ReactiveForm on:submit=handle_step_form_submit form_ref=form_ref>
                <div class="mx-[5%] md:mx-[10%] flex flex-col gap-[20px]">
                    <InputField field_type=InputFieldType::Email label="Email" required=true id_attr="email" name="email" />
                    <InputField field_type=InputFieldType::Password label="Password" required=true id_attr="password" name="password" />

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
