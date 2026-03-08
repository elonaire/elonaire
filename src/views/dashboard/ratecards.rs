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

use crate::components::forms::select::{CustomSelectInput, SelectOption};
use crate::components::general::spinner::Spinner;
use crate::components::general::table::data_table::TableCellData;
use crate::data::context::shared::{fetch_ratecards, fetch_services};
use crate::data::models::graphql::shared::{
    CreateRatecardResponse, CreateRatecardVars, RatecardInput, RatecardInputMetadata,
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
pub fn Ratecards() -> impl IntoView {
    view! {
        <>
            <Outlet />
        </>
    }
}

#[island]
pub fn RatecardsList() -> impl IntoView {
    let current_state = expect_context::<Store<AppStateContext>>();
    let ratecards = move || current_state.ratecards();
    let (is_loading, set_is_loading) = signal(false);

    let table_data = RwSignal::new((
        vec![
            Column::new("Name", false),
            Column::new("Date of Creation", true),
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
                    current_state.user().auth_info().token().get_untracked()
                ),
            );

            let _response = fetch_ratecards(&current_state, Some(&headers)).await;

            set_is_loading.set(false);
        });
    });

    Effect::new(move || {
        let ratecards_rows: Vec<HashMap<String, TableCellData>> = ratecards()
            .get()
            .iter()
            .map(|ratecard| {
                let mut hash_map_data = HashMap::new();

                // This id is the unique identifier of the table row. and is a MUST for the table to function properly.
                // *Note:* The id is a MUST for the table to function properly. You might be forced to generate a unique id for each row if your data does not have a unique identifier.
                hash_map_data.insert(
                    "id".into(),
                    TableCellData::String(
                        ratecard
                            .id
                            .as_ref()
                            .unwrap_or(&Default::default())
                            .to_owned(),
                    ),
                );

                hash_map_data.insert(
                    "Name".into(),
                    TableCellData::String(
                        ratecard
                            .name
                            .as_ref()
                            .unwrap_or(&Default::default())
                            .to_owned(),
                    ),
                );

                hash_map_data.insert(
                    "Date of Creation".into(),
                    TableCellData::DateTime(
                        ratecard
                            .created_at
                            .as_ref()
                            .unwrap_or(&Default::default())
                            .to_owned(),
                    ),
                );
                hash_map_data
            })
            .collect();

        table_data.update(move |prev| {
            prev.1 = ratecards_rows;
        });
    });

    view! {
        <>
            <Title text="Rate Cards"/>
            <div class="display-constraints">
                <Breadcrumbs custom_route_names=["Home", "Dashboard", "Rate Cards"] />
            </div>
            <Show when=move || is_loading.get()>
                <Spinner />
            </Show>

            <h1 class="display-constraints">Rate Cards</h1>

            <div class="display-constraints flex items-center justify-end">
                <A href="/dashboard/ratecards/create">
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
pub fn CreateRatecard() -> impl IntoView {
    let form_ref = NodeRef::new();
    let (main_form_is_valid, set_main_form_is_valid) = signal(false);
    let selected_services_options = RwSignal::new(vec![] as Vec<String>);
    let submit_is_disabled = Memo::new(move |_| {
        (!main_form_is_valid.get() || selected_services_options.get().is_empty())
    });
    let current_state = expect_context::<Store<AppStateContext>>();
    let services = move || current_state.services();
    let success_modal_is_open = RwSignal::new(false);
    let confirm_modal_is_open = RwSignal::new(false);
    let (is_loading, set_is_loading) = signal(false);
    let services_options = RwSignal::new(vec![] as Vec<SelectOption>);

    let onprimary_handler = Callback::new(move |_| {
        if !selected_services_options.get().is_empty() && main_form_is_valid.get() {
            set_is_loading.set(true);
            spawn_local(async move {
                if let Some(main_form_data) = get_form_data_from_form_ref(&form_ref) {
                    let deserialized_main_form_data = deserialize_form_data_to_struct::<
                        RatecardInput,
                    >(
                        &main_form_data, false, None
                    );

                    if deserialized_main_form_data.is_none() {
                        set_is_loading.set(false);
                        return;
                    }

                    let deserialized_main_form_data = deserialized_main_form_data.unwrap();

                    let input_vars = CreateRatecardVars {
                        ratecard_input: deserialized_main_form_data,
                        ratecard_input_metadata: RatecardInputMetadata {
                            service_ids: selected_services_options.get_untracked(),
                        },
                    };

                    let query = r#"
                           mutation CreateRatecard($ratecardInput: RatecardInput!, $ratecardInputMetadata: RatecardInputMetadata!) {
                                createRatecard(ratecardInput: $ratecardInput, ratecardInputMetadata: $ratecardInputMetadata) {
                                    data {
                                        name
                                        createdAt
                                        updatedAt
                                        id
                                        services {
                                            title
                                            description
                                            thumbnail
                                            id
                                        }
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
                        CreateRatecardResponse,
                        CreateRatecardVars,
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
                                set_main_form_is_valid.set(false);
                            } else {
                            }

                            set_is_loading.set(false);

                            success_modal_is_open.update(|status| *status = true);
                            selected_services_options.set(vec![]);
                        }
                        None => {
                            set_is_loading.set(false);
                        }
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

            let _fetch_services_res = fetch_services(&current_state, Some(&headers)).await;

            set_is_loading.set(false);
        });
    });

    Effect::new(move || {
        services_options.set(
            services()
                .get()
                .iter()
                .map(|service| {
                    SelectOption::new(
                        service.id.as_ref().unwrap_or(&Default::default()),
                        service.title.as_ref().unwrap_or(&Default::default()),
                    )
                })
                .collect(),
        );
    });

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
            <Title text="New Rate Card"/>
            <BasicModal title="Success" is_open=success_modal_is_open use_case=UseCase::Success disable_auto_close=false>
                <div class="p-[10px]">
                    <p>"Rate Card created successfully!"</p>
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
                <Breadcrumbs custom_route_names=["Home", "Dashboard", "Rate Cards", "New"] />
            </div>

            <h1 class="display-constraints">New Rate Card</h1>

            <div class="display-constraints flex flex-col gap-[20px]">
                <h2>Rate Card Metadata</h2>
                <CustomSelectInput
                    label="Services"
                    required=true
                    id_attr="service_ids"
                    options=services_options
                    value=selected_services_options
                    multiple=true
                />
            </div>

            <ReactiveForm on:submit=handle_main_form_submit form_ref=form_ref>
                <div class="display-constraints flex flex-col gap-[20px]">
                    <h2>Rate Card Info</h2>
                    <InputField field_type=InputFieldType::Text label="Name" required=true id_attr="name" name="name" />

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
