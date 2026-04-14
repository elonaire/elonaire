use crate::components::general::stepper::{Step, StepInfo, Stepper};
use crate::data::models::general::shared::RestResponse;
use crate::utils::errors::{handle_graphql_errors, unwrap_rest_response};
use crate::utils::formatters::{Pipe, PipeOption};
use crate::views::public::error_handler::ErrorHandler;
use std::collections::HashMap;

use chrono::Local;
use icondata::{AiFilePdfOutlined, AiReadOutlined, BsArrowRight, MdiFileDocumentEditOutline};
use leptos::html::Form;
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
    let billing_interval_form_ref = NodeRef::new();
    let billing_interval_field_ref = NodeRef::new();
    let file_input_ref = NodeRef::new();
    let (services_form_is_valid, set_services_form_is_valid) = signal(false);
    let (billing_interval_form_is_valid, set_billing_interval_form_is_valid) = signal(false);
    let (amount, set_amount) = signal(None as Option<f64>);
    let submit_is_disabled =
        Memo::new(move |_| !services_form_is_valid.get() || !billing_interval_form_is_valid.get());

    let success_modal_is_open = RwSignal::new(false);
    let service_request_modal_is_open = RwSignal::new(false);
    let confirm_modal_is_open = RwSignal::new(false);
    let (is_loading, set_is_loading) = signal(false);
    let current_state = expect_context::<Store<AppStateContext>>();

    let stepper_form_refs = RwSignal::new(Vec::new());

    let modal_primary_is_disabled = Memo::new(move |_| {
        let refs = stepper_form_refs.get();

        if refs.is_empty() {
            return true;
        }

        refs.iter().any(|form_ref: &NodeRef<Form>| {
            form_ref
                .get()
                .map(|form| !form.check_validity())
                .unwrap_or(true) // if form ref not yet mounted, treat as invalid
        })
    });

    let handle_received_form_refs = Callback::new(move |form_refs: Vec<NodeRef<Form>>| {
        stepper_form_refs.update(|prev| *prev = form_refs);
    });

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

                                let billing_rate =
                                    fetch_billing_rate(vars, None, &current_state).await;

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

    let handle_service_request_modal_primary_click = Callback::new(move |_| {
        confirm_modal_is_open.update(|status| *status = true);
    });

    let onprimary_confirm_handler = Callback::new(move |_| {
        if !modal_primary_is_disabled.get() {
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
                                        current_state.user().auth_info().token().get_untracked()
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

                        let Some(uploaded_files) = unwrap_rest_response(body, &current_state)
                        else {
                            set_is_loading.set(false);
                            return;
                        };

                        let Some(service_request_form_ref) =
                            stepper_form_refs.get_untracked().first().cloned()
                        else {
                            set_is_loading.set(false);
                            return;
                        };
                        let Ok(request_metadata_form_data) = FormData::new() else {
                            set_is_loading.set(false);
                            return;
                        };

                        let (Some(service_request_form_data), Some(services_form_data)) = (
                            get_form_data_from_form_ref(&service_request_form_ref),
                            get_form_data_from_form_ref(&services_form_ref),
                        ) else {
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
                            $serviceRequestInput: ServiceRequestInput!,
                            $serviceRequestInputMetadata: ServiceRequestInputMetadata!
                        ) {
                            createServiceRequest(
                                serviceRequestInput: $serviceRequestInput,
                                serviceRequestInputMetadata: $serviceRequestInputMetadata
                            ) {
                                data {
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
                                metadata {
                                    requestId
                                    newAccessToken
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
                                }
                                set_is_loading.set(false);
                                success_modal_is_open.update(|status| *status = true);
                                service_request_modal_is_open.update(|status| *status = false);
                            }
                            None => {
                                let _handle_errors =
                                    handle_graphql_errors(&response, &current_state, None);
                                set_is_loading.set(false);
                            }
                        }
                    });
                };
            };
        }
    });

    let handle_stepper_on_cleanup = Callback::new(move |_| {
        stepper_form_refs.update(|refs| refs.clear());
    });

    view! {
        <ErrorHandler />
        <div class="flex flex-col gap-[20px] border-[0.5px] border-light-gray rounded-[5px] min-h-[564px] max-w-[400px] flex-1">
            <BasicModal title="Service Request" is_open=service_request_modal_is_open use_case=UseCase::General disable_auto_close=false container_style_ext="md:w-[70%] h-[70svh]" show_footer=false>
                <>
                <Show when=move || is_loading.get()>
                    <Spinner />
                </Show>
                <Stepper step_labels=RwSignal::new(vec![StepInfo::new("Basic Information", Some(MdiFileDocumentEditOutline)), StepInfo::new("Supporting Documents", Some(AiFilePdfOutlined)), StepInfo::new("Review", Some(AiReadOutlined))]) send_all_form_refs=handle_received_form_refs is_linear=true final_button_text="Submit" ext_wrapper_styles="h-full overflow-y-auto" on_click_final_button=handle_service_request_modal_primary_click final_button_is_disabled=modal_primary_is_disabled handle_on_cleanup=handle_stepper_on_cleanup>
                   <Step>
                       <div class="flex flex-col gap-[20px] p-2">
                           <Textarea label="Description" required=true id_attr="description" name="description" placeholder="Describe your request" />
                           <DatePicker label="Start Date" required=true id_attr="start_date" name="start_date" min=Local::now() />
                           {
                               move || {
                                   let mut interval_str = "";
                                   if let Some(input_el) = billing_interval_field_ref.get() as Option<HtmlSelectElement> {
                                       interval_str = match input_el.value().as_str() {
                                           "Monthly" => "months",
                                           "Hourly" => "hours",
                                           "Weekly" => "weeks",
                                           "Annual" => "years",
                                           "Milestone" => "milestones",
                                           _ => "_ _",
                                       };
                                   };
                                   view! {
                                       <InputField label=format!("Engagement Length ({})", interval_str) min="1" field_type=InputFieldType::Number required=true id_attr="engagement_length" name="engagement_length" placeholder="e.g. 1" />
                                   }
                               }
                           }
                       </div>
                   </Step>
                   <Step>
                       <div class="flex flex-col gap-[20px] p-2">
                           <CustomFileInput input_node_ref=file_input_ref label="Supporting Documents" name="supporting_documents" id_attr="supporting_documents" accept="image/*, .pdf, .docx, .txt, .odt, .md" required=true multiple=true />
                       </div>
                   </Step>
                   <Step>
                        <div class="flex flex-col gap-[20px]">
                            { move || if let Some(first_form_ref) = stepper_form_refs.get().get(0) {
                                let Some(form_data) = get_form_data_from_form_ref(first_form_ref) else { return None };
                                let Some(data) = deserialize_form_data_to_struct::<ServiceRequestInput>(&form_data, false, None) else { return None };
                                Some(view! {
                                    <h4>"Basic Information"</h4>
                                    <table class="border-collapse border border-light-gray dark:border-mid-gray">
                                        <tr>
                                            <td class="border-collapse border border-light-gray dark:border-mid-gray px-4 py-2"><strong>"Description"</strong></td>
                                            <td class="border-collapse border border-light-gray dark:border-mid-gray px-4 py-2">{data.description.text(None)}</td>
                                        </tr>
                                        <tr>
                                            <td class="border-collapse border border-light-gray dark:border-mid-gray px-4 py-2"><strong>"Start Date"</strong></td>
                                            <td class="border-collapse border border-light-gray dark:border-mid-gray px-4 py-2">{data.start_date.date("%b %e %Y", None)}</td>
                                        </tr>
                                        <tr>
                                            <td class="border-collapse border border-light-gray dark:border-mid-gray px-4 py-2"><strong>"Engagement Length"</strong></td>
                                            <td class="border-collapse border border-light-gray dark:border-mid-gray px-4 py-2">{data.engagement_length.int(None)}</td>
                                        </tr>
                                    </table>
                                })
                            } else {
                                None
                            }
                            }
                            { move ||
                                if let Some(_second_form_ref) = stepper_form_refs.get().get(1) {
                                    let Some(file_input) = file_input_ref.to_owned().get() else { return None };

                                    let files = file_input.files();
                                    let file_list = files.map(|fl| {
                                        (0..fl.length())
                                            .filter_map(|i| fl.item(i))
                                            .collect::<Vec<_>>()
                                    }).unwrap_or_default();

                                    Some(view! {
                                        <h4>"Supporting Documents"</h4>
                                        {if file_list.is_empty() {
                                            view! {
                                                <p class="text-light-gray dark:text-mid-gray italic">"No documents uploaded."</p>
                                            }.into_any()
                                        } else {
                                            view! {
                                                <table class="border-collapse border border-light-gray dark:border-mid-gray">
                                                    <thead>
                                                        <tr>
                                                            <th class="border-collapse border border-light-gray dark:border-mid-gray px-4 py-2 text-left"><strong>"File Name"</strong></th>
                                                            <th class="border-collapse border border-light-gray dark:border-mid-gray px-4 py-2 text-left"><strong>"Size"</strong></th>
                                                        </tr>
                                                    </thead>
                                                    <tbody>
                                                        {file_list.into_iter().map(|file| {
                                                            let name = file.name();
                                                            let size = file.size();

                                                            let size_display = if size < 1024.0 {
                                                                format!("{:.0} B", size)
                                                            } else if size < 1024.0 * 1024.0 {
                                                                format!("{:.1} KB", size / 1024.0)
                                                            } else {
                                                                format!("{:.1} MB", size / (1024.0 * 1024.0))
                                                            };

                                                            view! {
                                                                <tr>
                                                                    <td class="border-collapse border border-light-gray dark:border-mid-gray px-4 py-2">{name}</td>
                                                                    <td class="border-collapse border border-light-gray dark:border-mid-gray px-4 py-2">{size_display}</td>
                                                                </tr>
                                                            }
                                                        }).collect::<Vec<_>>()}
                                                    </tbody>
                                                </table>
                                            }.into_any()
                                        }}
                                    })
                                } else {
                                    None
                                }
                            }
                       </div>
                   </Step>
                </Stepper>
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
                        <h4>{move || name.get()}</h4>
                        <p class="text-primary font-bold text-2xl"><sup class="text-sm">$</sup>{ move ||
                            {
                                let amount_val = amount.get();
                                amount_val.float(Some(2), Some("_ _"))
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
                            ext_input_styles=""
                            input_node_ref=billing_interval_field_ref
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

            <ReactiveForm
            on:submit=handle_services_form_submit
            form_ref=services_form_ref
            on:change=move |_| {
                if let Some(form) = services_form_ref.get() {
                    let has_checked = form
                        .query_selector_all("input[name='service_ids']:checked")
                        .map(|nodes| nodes.length() > 0)
                        .unwrap_or(false);

                    set_services_form_is_valid.set(has_checked);
                }
            }
            >
                <div class="p-[10px] flex flex-col gap-[10px] text-md">
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
                <BasicButton button_text="Request Service" icon=Some(BsArrowRight) style_ext="bg-primary text-contrast-white" disabled=submit_is_disabled onclick=Callback::new(move |_| { service_request_modal_is_open.set(true); }) />
            </div>
        </div>
    }
}
