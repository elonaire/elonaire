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

use crate::components::forms::radio_input::RadioOption;
use crate::components::forms::textarea::Textarea;
use crate::components::general::spinner::Spinner;
use crate::components::general::table::data_table::TableCellData;
use crate::components::schemas::props::ColorTemperature;
use crate::data::context::shared::fetch_professions;
use crate::data::models::graphql::shared::{
    CreateProfessionalDetailsResponse, ProfessionalDetailsInputVars,
};
use crate::utils::graphql_client::perform_mutation_or_query_with_vars;
use crate::{
    components::{
        forms::{
            datepicker::DatePicker,
            input::{InputField, InputFieldType},
            radio_input::RadioInputGroup,
            reactive_form::ReactiveForm,
        },
        general::{
            breadcrumbs::Breadcrumbs,
            button::{BasicButton, ButtonType},
            modal::modal::{BasicModal, UseCase},
            table::data_table::{Column, DataTable},
            tag::LabelTag,
        },
    },
    data::{
        context::store::{AppStateContext, AppStateContextStoreFields},
        models::{
            general::acl::{AuthInfoStoreFields, UserInfoStoreFields},
            graphql::shared::UserProfessionalInfoInput,
        },
    },
    utils::forms::{deserialize_form_data_to_struct, get_form_data_from_form_ref},
};

const SHARED_SERVICE_API: Option<&str> = option_env!("SHARED_SERVICE_API");

#[component]
pub fn ProfessionalDetails() -> impl IntoView {
    view! {
        <>
            <Outlet />
        </>
    }
}

#[component]
pub fn ProfessionalDetailsList() -> impl IntoView {
    let store = expect_context::<Store<AppStateContext>>();
    let professions = move || store.professions();
    let (is_loading, set_is_loading) = signal(false);

    let table_data = RwSignal::new((
        vec![
            Column::new("Occupation", false),
            Column::new("Status", true),
            Column::new("Start Date", true),
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

            let _fetch_professions_res = fetch_professions(&store, Some(&headers)).await;

            set_is_loading.set(false);
        });
    });

    Effect::new(move || {
        let roles: Vec<HashMap<String, TableCellData>> = professions()
            .get()
            .iter()
            .map(|profession| {
                let mut hash_map_data = HashMap::new();

                // This id is the unique identifier of the table row. and is a MUST for the table to function properly.
                // *Note:* The id is a MUST for the table to function properly. You might be forced to generate a unique id for each row if your data does not have a unique identifier.
                hash_map_data.insert(
                    "id".to_string(),
                    TableCellData::String(
                        profession
                            .id
                            .as_ref()
                            .unwrap_or(&Default::default())
                            .to_owned(),
                    ),
                );

                hash_map_data.insert(
                    "Occupation".to_string(),
                    TableCellData::String(
                        profession
                            .occupation
                            .as_ref()
                            .unwrap_or(&Default::default())
                            .to_owned(),
                    ),
                );

                let status = if profession.active.is_some() && profession.active.unwrap_or_default()
                {
                    ViewFn::from(move || {
                        view! {
                            <LabelTag label="Active" color=ColorTemperature::Success />
                        }
                    })
                } else {
                    ViewFn::from(move || {
                        view! {
                            <LabelTag label="Inactive" color=ColorTemperature::Warning />
                        }
                    })
                };
                hash_map_data.insert("Status".to_string(), TableCellData::Html(status));

                hash_map_data.insert(
                    "Start Date".to_string(),
                    TableCellData::DateTime(
                        profession
                            .start_date
                            .as_ref()
                            .unwrap_or(&Default::default())
                            .to_owned(),
                    ),
                );
                hash_map_data
            })
            .collect();

        table_data.update(move |prev| {
            prev.1 = roles;
        });
    });

    view! {
        <>
            <Title text="My Portfolio"/>
            <div class="display-constraints">
                <Breadcrumbs custom_route_names=["Home", "Dashboard", "Professions"] />
            </div>
            <Show when=move || is_loading.get()>
                <Spinner />
            </Show>

            <h1 class="display-constraints">Professional Details</h1>

            <div class="display-constraints flex items-center justify-end">
                <A href="/dashboard/professional-details/create">
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
pub fn CreateProfessionalDetail() -> impl IntoView {
    let form_ref = NodeRef::new();
    let (form_is_valid, set_form_is_valid) = signal(false);
    let submit_is_disabled = Memo::new(move |_| !form_is_valid.get());
    let store = expect_context::<Store<AppStateContext>>();
    let success_modal_is_open = RwSignal::new(false);
    let confirm_modal_is_open = RwSignal::new(false);
    let init_date = RwSignal::new(None);
    let (is_loading, set_is_loading) = signal(false);

    let onprimary_handler = Callback::new(move |_| {
        if form_is_valid.get() {
            set_is_loading.set(true);
            spawn_local(async move {
                if let Some(form_data) = get_form_data_from_form_ref(&form_ref) {
                    let deserialized_form_data = deserialize_form_data_to_struct::<
                        UserProfessionalInfoInput,
                    >(&form_data, true, None);

                    if deserialized_form_data.is_none() {
                        set_is_loading.set(false);
                        return;
                    }

                    let deserialized_form_data = deserialized_form_data.unwrap();

                    let input_vars = ProfessionalDetailsInputVars {
                        professional_details: deserialized_form_data,
                    };

                    let query = r#"
                           mutation CreateProfessionalDetails($professionalDetails: UserProfessionalInfoInput!) {
                                createProfessionalDetails(professionalDetails: $professionalDetails) {
                                    data {
                                        description
                                        active
                                        occupation
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
                        CreateProfessionalDetailsResponse,
                        ProfessionalDetailsInputVars,
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
            <Title text="New Profession"/>
            <BasicModal title="Success" is_open=success_modal_is_open use_case=UseCase::Success disable_auto_close=false>
                <div class="p-[10px]">
                    <p>"Profession created successfully!"</p>
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
                <Breadcrumbs custom_route_names=["Home", "Dashboard", "Professions", "New"] />
            </div>

            <h1 class="display-constraints">New Profession</h1>

            <ReactiveForm on:submit=handle_step_form_submit onreset=onreset_handler form_ref=form_ref>
                <div class="display-constraints flex flex-col gap-[20px]">
                    <InputField field_type=InputFieldType::Text label="Occupation" required=true id_attr="occupation" name="occupation" />
                    <Textarea label="Description" required=true id_attr="description" name="description" />

                    <DatePicker label="Start Date" required=true id_attr="start_date" initial_value=init_date name="start_date" />
                    <RadioInputGroup
                        legend="Select Status"
                        name="active"
                        required=true
                        options=vec![
                            RadioOption::new("true", "Active", None),
                            RadioOption::new("false", "InActive", None),
                        ]
                    />
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
