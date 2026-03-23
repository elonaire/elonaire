use std::collections::HashMap;

use icondata as IconId;
use leptos::task::spawn_local;
use leptos::wasm_bindgen::JsCast;
use leptos::{ev, prelude::*};
use reactive_stores::Store;
use web_sys::{FormData, HtmlFormElement, HtmlInputElement, HtmlSelectElement, SubmitEvent};

use crate::components::forms::checkbox::CheckboxInputField;
use crate::components::forms::datepicker::DatePicker;
use crate::components::forms::input::{CustomFileInput, InputField, InputFieldType};
use crate::components::forms::reactive_form::ReactiveForm;
use crate::components::forms::textarea::Textarea;
use crate::components::general::modal::modal::{BasicModal, UseCase};
use crate::components::general::spinner::Spinner;
use crate::components::{
    forms::select::{SelectInput, SelectOption},
    general::button::BasicButton,
};
use crate::data::context::shared::fetch_billing_rate;
use crate::data::context::store::{AppStateContext, AppStateContextStoreFields};
use crate::data::models::general::{
    acl::{AuthInfoStoreFields, UserInfoStoreFields},
    files::UploadedFileResponse,
};
use crate::data::models::graphql::shared::{
    BillingInterval, BillingIntervalForm, CreateServiceRequestResponse, CreateServiceRequestVars,
    FetchBillingRateVars, ServiceIdsForm, ServiceRequestInput, ServiceRequestInputMetadata,
    UserService,
};
use crate::utils::custom_traits::EnumerableEnum;
use crate::utils::forms::{deserialize_form_data_to_struct, get_form_data_from_form_ref};
use crate::utils::graphql_client::perform_mutation_or_query_with_vars;

const FILES_SERVICE_API: Option<&str> = option_env!("FILES_SERVICE_API");
const SHARED_SERVICE_API: Option<&str> = option_env!("SHARED_SERVICE_API");

#[component]
pub fn RatecardComponent(
    #[prop(into)] name: RwSignal<String>,
    #[prop(into)] services: RwSignal<Vec<UserService>>,
) -> impl IntoView {
    let services_form_ref = NodeRef::new();
    let service_request_form_ref = NodeRef::new();
    let service_request_metadata_form_ref = NodeRef::new();
    let billing_interval_form_ref = NodeRef::new();
    let file_input_ref = NodeRef::new();
    let (services_form_is_valid, set_services_form_is_valid) = signal(false);
    let (service_request_form_is_valid, set_service_request_form_is_valid) = signal(false);
    let (service_request_metadata_form_is_valid, set_service_request_metadata_form_is_valid) =
        signal(false);
    let (billing_interval_form_is_valid, set_billing_interval_form_is_valid) = signal(false);
    let (amount, set_amount) = signal(None as Option<f64>);
    let submit_is_disabled =
        Memo::new(move |_| !services_form_is_valid.get() || !billing_interval_form_is_valid.get());
    let modal_primary_is_disabled = Memo::new(move |_| {
        !service_request_form_is_valid.get() || !service_request_metadata_form_is_valid.get()
        // || !services_form_is_valid.get()
    });
    let success_modal_is_open = RwSignal::new(false);
    let service_request_modal_is_open = RwSignal::new(false);
    let confirm_modal_is_open = RwSignal::new(false);
    let (is_loading, set_is_loading) = signal(false);
    let init_date = RwSignal::new(None);
    let current_state = expect_context::<Store<AppStateContext>>();

    let billing_interval = RwSignal::new(
        BillingInterval::variants_slice()
            .iter()
            .map(|billing_interval| {
                SelectOption::new(
                    &format!("{billing_interval:?}"),
                    &billing_interval.to_string(),
                )
            })
            .collect::<Vec<SelectOption>>(),
    );
    let (selected_billing_interval, set_selected_billing_interval) = signal("hr");

    // This is to force the form to reset the date input
    let onreset_handler = Callback::new(move |_ev: ev::Event| {
        init_date.set(None);
    });

    Effect::new(move || {
        let target: Option<HtmlFormElement> = billing_interval_form_ref.get();

        if let Some(form) = target {
            set_billing_interval_form_is_valid.set(form.check_validity());
        }

        if services_form_is_valid.get() && billing_interval_form_is_valid.get() {
            spawn_local(async move {
                if let Some(billing_interval_form_data) =
                    get_form_data_from_form_ref(&billing_interval_form_ref)
                {
                    if let Some(services_form_data) =
                        get_form_data_from_form_ref(&services_form_ref)
                    {
                        let deserialized_billing_interval_form_data =
                            deserialize_form_data_to_struct::<BillingIntervalForm>(
                                &billing_interval_form_data,
                                false,
                                None,
                            );
                        let deserialized_services_form_data =
                            deserialize_form_data_to_struct::<ServiceIdsForm>(
                                &services_form_data,
                                false,
                                Some(&["service_ids"]),
                            );

                        if let Some(billing_interval) = deserialized_billing_interval_form_data {
                            if let Some(services) = deserialized_services_form_data {
                                let vars = FetchBillingRateVars {
                                    billing_interval: billing_interval.billing_interval,
                                    service_ids: services.service_ids,
                                };

                                let billing_rate = fetch_billing_rate(vars, None).await;

                                if let Ok(amount_str) = billing_rate {
                                    // Process ratecards data here
                                    set_amount.set(Some(amount_str.parse().unwrap_or(0.0)));
                                }
                            };
                        };
                    };
                };
            });
        };
    });

    let handle_services_form_submit = move |ev: SubmitEvent| {
        ev.prevent_default();
        ev.stop_propagation();

        // Implement logic to show form validity
        let target = ev
            .target()
            .and_then(|t| t.dyn_into::<HtmlFormElement>().ok());

        if let Some(form) = target {
            set_services_form_is_valid.set(form.check_validity());
        }
    };

    let handle_billing_interval_form_submit = move |ev: SubmitEvent| {
        ev.prevent_default();
        ev.stop_propagation();

        // Implement logic to show form validity
        let target = ev
            .target()
            .and_then(|t| t.dyn_into::<HtmlFormElement>().ok());

        if let Some(form) = target {
            set_billing_interval_form_is_valid.set(form.check_validity());
        }
    };

    let handle_service_request_form_submit = move |ev: SubmitEvent| {
        ev.prevent_default();
        ev.stop_propagation();

        // Implement logic to show form validity
        let target = ev
            .target()
            .and_then(|t| t.dyn_into::<HtmlFormElement>().ok());

        if let Some(form) = target {
            set_service_request_form_is_valid.set(form.check_validity());

            if let Some(_submitter) = ev.submitter() {
                confirm_modal_is_open.update(|status| *status = true);
            }
        }
    };

    let handle_service_request_metadata_form_submit = move |ev: SubmitEvent| {
        ev.prevent_default();
        ev.stop_propagation();

        // Implement logic to show form validity
        let target = ev
            .target()
            .and_then(|t| t.dyn_into::<HtmlFormElement>().ok());

        if let Some(form) = target {
            set_service_request_metadata_form_is_valid.set(form.check_validity());
        }
    };

    let handle_service_request_modal_primary_click = Callback::new(move |_| {
        confirm_modal_is_open.update(|status| *status = true);
    });

    let onprimary_confirm_handler = Callback::new(move |_| {
        if service_request_form_is_valid.get()
            && service_request_metadata_form_is_valid.get()
            && services_form_is_valid.get()
        {
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
                            gloo_net::http::Request::post(&format!("{files_service_api}upload"))
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

                        let (
                            Some(service_request_form_data),
                            Some(request_metadata_form_data),
                            Some(services_form_data),
                        ) = (
                            get_form_data_from_form_ref(&service_request_form_ref),
                            get_form_data_from_form_ref(&service_request_metadata_form_ref),
                            get_form_data_from_form_ref(&services_form_ref),
                        )
                        else {
                            set_is_loading.set(false);
                            return;
                        };

                        uploaded_files.iter().for_each(|uploaded_file| {
                            request_metadata_form_data
                                .append_with_str("supporting_docs_file_ids", &uploaded_file.file_id)
                                .ok();
                        });

                        let Some(deserialized_services_form_data) =
                            deserialize_form_data_to_struct::<ServiceIdsForm>(
                                &services_form_data,
                                false,
                                Some(&["service_ids"]),
                            )
                        else {
                            set_is_loading.set(false);
                            return;
                        };

                        deserialized_services_form_data
                            .service_ids
                            .iter()
                            .for_each(|service_id| {
                                request_metadata_form_data
                                    .append_with_str("service_ids", service_id)
                                    .ok();
                            });

                        let (
                            Some(deserialized_service_request_form_data),
                            Some(deserialized_service_request_metadata_form_data),
                        ) = (
                            deserialize_form_data_to_struct::<ServiceRequestInput>(
                                &service_request_form_data,
                                false,
                                None,
                            ),
                            deserialize_form_data_to_struct::<ServiceRequestInputMetadata>(
                                &request_metadata_form_data,
                                false,
                                Some(&["service_ids", "supporting_docs_file_ids"]),
                            ),
                        )
                        else {
                            set_is_loading.set(false);
                            return;
                        };

                        let input_vars = CreateServiceRequestVars {
                            service_request_input: deserialized_service_request_form_data,
                            service_request_input_metadata:
                                deserialized_service_request_metadata_form_data,
                        };

                        let query = r#"
                            mutation CreateServiceRequest(
                                $serviceRequestInput: ServiceRequestInput,
                                $serviceRequestInputMetadata: ServiceRequestInputMetadata
                            ) {
                                createServiceRequest(
                                    serviceRequestInput: $serviceRequestInput,
                                    serviceRequestInputMetadata: $serviceRequestInputMetadata
                                ) {
                                    description
                                    startDate
                                    engagementLength
                                    createdAt
                                    updatedAt
                                    id
                                    supportingDocs {
                                        id
                                        fileId
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

                        let Some(shared_service_api) = SHARED_SERVICE_API else {
                            return;
                        };

                        let response = perform_mutation_or_query_with_vars::<
                            CreateServiceRequestResponse,
                            CreateServiceRequestVars,
                        >(
                            Some(&headers), shared_service_api, query, input_vars
                        )
                        .await;

                        match response.get_data() {
                            Some(_data) => {
                                if let Some(form) = service_request_form_ref
                                    .get_untracked()
                                    .and_then(|el| el.dyn_into::<HtmlFormElement>().ok())
                                {
                                    form.reset();
                                    set_service_request_form_is_valid.set(false);
                                }
                                set_is_loading.set(false);
                                success_modal_is_open.update(|status| *status = true);
                                service_request_modal_is_open.update(|status| *status = false);
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

    view! {
        <div class="flex flex-col gap-[20px] border-[0.5px] border-light-gray rounded-[5px] text-light-gray min-h-[564px] max-w-[400px] flex-1">
            <BasicModal title="Service Request" is_open=service_request_modal_is_open use_case=UseCase::General disable_auto_close=false primary_button_text="Submit" disable_primary_close=true on_click_primary=handle_service_request_modal_primary_click primary_is_disabled=modal_primary_is_disabled>
                <>
                <Show when=move || is_loading.get()>
                    <Spinner />
                </Show>
                <ReactiveForm on:submit=handle_service_request_form_submit form_ref=service_request_form_ref onreset=onreset_handler>
                    <div class="p-[10px] flex flex-col gap-[20px]">
                        <Textarea label="Description" required=true id_attr="description" name="description" />
                        <DatePicker label="Start Date" required=true id_attr="start_date" initial_value=init_date name="start_date" />
                        <InputField label="Engagement Length" min="1" field_type=InputFieldType::Number required=true id_attr="engagement_length" name="engagement_length" />
                    </div>
                </ReactiveForm>
                <ReactiveForm on:submit=handle_service_request_metadata_form_submit form_ref=service_request_metadata_form_ref>
                    <div class="p-[10px] flex flex-col gap-[20px]">
                        <CustomFileInput input_node_ref=file_input_ref label="Supporting Documents" name="supporting_documents" id_attr="supporting_documents" accept="image/*, .pdf, .docx, .txt, .odt, .md" required=true />
                    </div>
                </ReactiveForm>
                </>
            </BasicModal>
                <BasicModal title="Success" is_open=success_modal_is_open use_case=UseCase::Success disable_auto_close=false>
                    <div class="p-[10px]">
                        <p>"Service Request submitted successfully!"</p>
                        <p>"Elon will reach out to you shortly."</p>
                    </div>
                </BasicModal>
                <BasicModal title="Confirm" on_click_primary=onprimary_confirm_handler is_open=confirm_modal_is_open use_case=UseCase::Confirmation disable_auto_close=false stack_number=1>
                    <div class="p-[10px]">
                        <p>"Are you sure that you want to submit?"</p>
                    </div>
                </BasicModal>
            <div class="border-b-[0.5px]">
                <div class="p-[10px] flex flex-row justify-between items-center">
                    <div class="flex flex-col">
                        <h4 class="text-light-gray">{move || name.get()}</h4>
                        <p class="text-primary font-bold text-2xl"><sup class="text-sm text-light-gray">$</sup>{ move ||
                            if let Some(amount) = amount.get() {
                                format!("{:.2}", amount)
                            } else {
                                "_ _".into()
                            }
                        }/{move || selected_billing_interval.get()}</p>
                    </div>
                    <div class="basis-1/3">
                        <ReactiveForm on:submit=handle_billing_interval_form_submit form_ref=billing_interval_form_ref>
                            <SelectInput
                            id_attr="billing_interval"
                            name="billing_interval"
                            options=billing_interval
                            required=true
                            initial_value=RwSignal::new("Hourly".into())
                            ext_input_styles="text-light-gray"
                            on:change=move |ev: ev::Event| {
                                let target = ev
                                    .target()
                                    .and_then(|t| t.dyn_into::<HtmlSelectElement>().ok());

                                if let Some(input_el) = target {
                                    let short_name = match input_el.value().as_str() {
                                        "Monthly" => "mo",
                                        "Hourly" => "hr",
                                        "Weekly" => "wk",
                                        "Annual" => "yr",
                                        "Milestone" => "mi",
                                        _ => "_ _",
                                    };
                                    set_selected_billing_interval.set(short_name);
                                }
                            }
                            />
                        </ReactiveForm>
                    </div>
                </div>
            </div>

            <ReactiveForm on:submit=handle_services_form_submit form_ref=services_form_ref>
                <div class="p-[10px] flex flex-col gap-[10px] text-light-gray text-md">
                    <For
                        each=move || services.get()
                        key=|service| service.id.as_ref().unwrap_or(&String::new()).clone()
                            children=move |service| {
                            view! {
                                <CheckboxInputField initial_value=RwSignal::new(service.id.as_ref().unwrap_or(&String::new()).clone()) label=service.title.as_ref().unwrap_or(&String::new()).clone() id_attr=format!("service-{}", service.id.as_ref().unwrap_or(&String::new()).clone()) name="service_ids" />
                            }
                        }
                    />
                </div>
            </ReactiveForm>
            <div class="p-[10px] mt-auto">
                // <BasicButton button_text="Request Service" icon=Some(IconId::BsArrowRight) style_ext="bg-primary text-contrast-white" disabled=submit_is_disabled onclick=Callback::new(move |_| { service_request_modal_is_open.set(true); }) />
            </div>
        </div>
    }
}
