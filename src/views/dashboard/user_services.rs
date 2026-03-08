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

use crate::components::forms::input::CustomFileInput;
use crate::components::forms::textarea::Textarea;
use crate::components::general::spinner::Spinner;
use crate::components::general::table::data_table::TableCellData;
use crate::data::models::graphql::shared::{CreateUserServiceResponse, UserServiceInputVars};
use crate::data::{
    context::shared::fetch_services,
    models::{general::files::UploadedFileResponse, graphql::shared::UserServiceInput},
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
        models::{
            general::acl::{AuthInfoStoreFields, UserInfoStoreFields},
            graphql::shared::UserServiceStoreFields,
        },
    },
    utils::forms::{deserialize_form_data_to_struct, get_form_data_from_form_ref},
};

#[island]
pub fn UserService() -> impl IntoView {
    view! {
        <>
            <Outlet />
        </>
    }
}

#[island]
pub fn UserServicesList() -> impl IntoView {
    let current_state = expect_context::<Store<AppStateContext>>();
    let services = move || current_state.services();
    let (is_loading, set_is_loading) = signal(false);

    let table_data = RwSignal::new((
        vec![
            Column::new("Title", false),
            Column::new("Description", true),
        ],
        vec![],
    ));

    Effect::new(move || {
        let services: Vec<HashMap<String, TableCellData>> = services()
            .get()
            .iter()
            .map(|service| {
                let mut hash_map_data = HashMap::new();

                // This id is the unique identifier of the table row. and is a MUST for the table to function properly.
                // *Note:* The id is a MUST for the table to function properly. You might be forced to generate a unique id for each row if your data does not have a unique identifier.
                hash_map_data.insert(
                    "id".to_string(),
                    TableCellData::String(
                        service
                            .id
                            .as_ref()
                            .unwrap_or(&Default::default())
                            .to_owned(),
                    ),
                );

                hash_map_data.insert(
                    "Title".to_string(),
                    TableCellData::String(
                        service
                            .title
                            .as_ref()
                            .unwrap_or(&Default::default())
                            .to_owned(),
                    ),
                );

                hash_map_data.insert(
                    "Description".to_string(),
                    TableCellData::String(
                        service
                            .description
                            .as_ref()
                            .unwrap_or(&Default::default())
                            .to_owned(),
                    ),
                );

                hash_map_data
            })
            .collect();

        table_data.update(move |prev| {
            prev.1 = services;
        });
    });

    Effect::new(move || {
        set_is_loading.set(true);
        spawn_local(async move {
            let _fetch_services_res = fetch_services(&current_state, None).await;
            set_is_loading.set(false);
        });
    });

    view! {
        <>
            <Title text="User Services"/>
            <div class="display-constraints">
                <Breadcrumbs custom_route_names=["Home", "Dashboard", "Services"] />
            </div>
            <Show when=move || is_loading.get()>
                <Spinner />
            </Show>

            <h1 class="display-constraints">User Services</h1>

            <div class="display-constraints flex items-center justify-end">
                <A href="/dashboard/services/create">
                    <BasicButton
                        button_text="Create Service"
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
pub fn CreateUserService() -> impl IntoView {
    let form_ref = NodeRef::new();
    let file_input_ref = NodeRef::new();
    let (form_is_valid, set_form_is_valid) = signal(false);
    let submit_is_disabled = Memo::new(move |_| !form_is_valid.get());
    let current_state = expect_context::<Store<AppStateContext>>();
    let success_modal_is_open = RwSignal::new(false);
    let confirm_modal_is_open = RwSignal::new(false);
    let (is_loading, set_is_loading) = signal(false);

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
                        let Ok(request) =
                            gloo_net::http::Request::post("http://localhost:8080/api/files/upload")
                                .header(
                                    "Authorization",
                                    format!(
                                        "Bearer {}",
                                        current_state.user().auth_info().token().get_untracked()
                                    )
                                    .as_str(),
                                )
                                .body(files_form_data)
                        else {
                            set_is_loading.set(false);
                            return;
                        };

                        let response = match request.send().await {
                            Ok(r) => r,
                            Err(err) => {
                                leptos::logging::error!("Failed to upload files: {:?}", err);
                                set_is_loading.set(false);
                                return;
                            }
                        };

                        let uploaded_files =
                            match response.json::<Vec<UploadedFileResponse>>().await {
                                Ok(f) => f,
                                Err(err) => {
                                    leptos::logging::error!(
                                        "Failed to parse uploaded file response: {:?}",
                                        err
                                    );
                                    set_is_loading.set(false);
                                    return;
                                }
                            };

                        let Some(form_data) = get_form_data_from_form_ref(&form_ref) else {
                            set_is_loading.set(false);
                            return;
                        };

                        if let Err(e) = form_data.append_with_str(
                            "thumbnail",
                            format!("http://localhost:3001/view/{}", uploaded_files[0].file_name)
                                .as_str(),
                        ) {
                            leptos::logging::log!("Error appending thumbnail: {:?}", e);
                            set_is_loading.set(false);
                            return;
                        }

                        let Some(deserialized_form_data) =
                            deserialize_form_data_to_struct::<UserServiceInput>(
                                &form_data, false, None,
                            )
                        else {
                            set_is_loading.set(false);
                            return;
                        };

                        let input_vars = UserServiceInputVars {
                            user_service: deserialized_form_data,
                        };

                        let query = r#"
                            mutation CreateUserService($userService: UserServiceInput!) {
                                createUserService(userService: $userService) {
                                    data {
                                        title
                                        description
                                        thumbnail
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
                                current_state.user().auth_info().token().get_untracked()
                            ),
                        );

                        let response = perform_mutation_or_query_with_vars::<
                            CreateUserServiceResponse,
                            UserServiceInputVars,
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
            <Title text="New Service"/>
            <BasicModal title="Success" is_open=success_modal_is_open use_case=UseCase::Success disable_auto_close=false>
                <div class="p-[10px]">
                    <p>"Service created successfully!"</p>
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
                <Breadcrumbs custom_route_names=["Home", "Dashboard", "Services", "New Service"] />
            </div>

            <h1 class="display-constraints">New Service</h1>

            <ReactiveForm on:submit=handle_step_form_submit form_ref=form_ref>
                <div class="display-constraints flex flex-col gap-[20px]">
                    <InputField field_type=InputFieldType::Text label="Title" required=true id_attr="title" name="title" />
                    <Textarea label="Description" required=true id_attr="description" name="description" />
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
