use icondata as IconData;
use leptos::ev::{self, SubmitEvent};
use leptos::prelude::*;
use leptos::task::spawn_local;
use leptos::wasm_bindgen::JsCast;
use leptos_meta::*;
use leptos_router::components::{A, Outlet};
use reactive_stores::Store;
use web_sys::HtmlFormElement;

use crate::{
    components::{
        forms::{
            datepicker::DatePicker,
            input::{InputField, InputFieldType},
            radio_input::RadioInputField,
            reactive_form::ReactiveForm,
        },
        general::{
            breadcrumbs::Breadcrumbs,
            button::{BasicButton, ButtonType},
            modal::modal::{BasicModal, UseCase},
            table::data_table::{Column, DataTable},
        },
    },
    schemas::{
        general::acl::{
            AppStateContext, AppStateContextStoreFields, AuthInfoStoreFields, UserInfoStoreFields,
        },
        graphql::shared::{
            AddProfessionalDetails, ProfessionalDetailsInputFields, UserProfessionalInfoInput,
        },
    },
    utils::forms::{deserialize_form_data_to_struct, get_form_data_from_form_ref},
};
use cynic::{MutationBuilder, http::ReqwestExt};

#[island]
pub fn ProfessionalDetails() -> impl IntoView {
    view! {
        <>
            <Outlet />
        </>
    }
}

#[island]
pub fn ProfessionalDetailsList() -> impl IntoView {
    let table_data = RwSignal::new((
        vec![
            Column::new("Occupation", false),
            Column::new("Description", true),
            Column::new("Status", true),
            Column::new("Start Date", true),
        ],
        vec![],
    ));

    view! {
        <>
            <Title text="My Portfolio"/>
            <div class="mx-[20px]">
                <Breadcrumbs custom_route_names=["Home", "Dashboard", "Professions"] />
            </div>

            <h1 class="mx-[20px]">Professional Details</h1>

            <div class="mx-[20px] flex items-center justify-end">
                <A href="/dashboard/professional-details/create">
                    <BasicButton
                        button_text="Create Project"
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
pub fn CreateProfessionalDetail() -> impl IntoView {
    let form_ref = NodeRef::new();
    let (form_is_valid, set_form_is_valid) = signal(false);
    let submit_is_disabled = Memo::new(move |_| !form_is_valid.get());
    let current_state = expect_context::<Store<AppStateContext>>();
    let success_modal_is_open = RwSignal::new(false);
    let confirm_modal_is_open = RwSignal::new(false);
    let (submission_confirmed, set_submission_confirmed) = signal(false);
    let init_date = RwSignal::new(None);

    let onprimary_handler = Callback::new(move |_| {
        set_submission_confirmed.set(true);
    });

    let onreset_handler = Callback::new(move |_ev: ev::Event| {
        init_date.set(None);
    });

    Effect::new(move || {
        if submission_confirmed.get() && form_is_valid.get() {
            spawn_local(async move {
                if let Some(form_data) = get_form_data_from_form_ref(&form_ref) {
                    let deserialized_form_data = deserialize_form_data_to_struct::<
                        UserProfessionalInfoInput,
                    >(&form_data, true);

                    leptos::logging::log!("deserialized_form_data: {:?}", deserialized_form_data);

                    let operation = AddProfessionalDetails::build(ProfessionalDetailsInputFields {
                        professional_details: deserialized_form_data.unwrap(),
                    });

                    let response = reqwest::Client::new()
                        .post("http://localhost:8080/api/shared")
                        .header(
                            "Authorization",
                            format!(
                                "Bearer {}",
                                current_state.user().auth_info().token().get_untracked()
                            )
                            .as_str(),
                        )
                        .run_graphql(operation)
                        .await
                        .unwrap();

                    match response.data {
                        Some(_data) => {
                            if let Some(form) = form_ref
                                .get_untracked()
                                .and_then(|el| el.dyn_into::<HtmlFormElement>().ok())
                            {
                                form.reset();
                                set_form_is_valid.set(false);
                                set_submission_confirmed.set(false);
                            } else {
                                set_submission_confirmed.set(false);
                            }

                            success_modal_is_open.update(|status| *status = true);
                        }
                        None => {
                            leptos::logging::error!(
                                "Failed to add portfolio item: {:?}",
                                response.errors
                            );
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
            <Title text="New Profession"/>
            <BasicModal title="Success" is_open=success_modal_is_open use_case=UseCase::Success disable_auto_close=false>
                <div>
                    <p>"Profession created successfully!"</p>
                </div>
            </BasicModal>
            <BasicModal title="Confirm" on_click_primary=onprimary_handler is_open=confirm_modal_is_open use_case=UseCase::Confirmation disable_auto_close=false>
                <div>
                    <p>"Are you sure that you want to submit?"</p>
                </div>
            </BasicModal>

            <div class="mx-[20px]">
                <Breadcrumbs custom_route_names=["Home", "Dashboard", "Professions", "New Profession"] />
            </div>

            <h1 class="mx-[20px]">New Profession</h1>

            <ReactiveForm on:submit=handle_step_form_submit onreset=onreset_handler form_ref=form_ref>
                <div class="mx-[20px] flex flex-col gap-[20px]">
                    <InputField field_type=InputFieldType::Text label="Occupation" required=true id_attr="occupation" name="occupation" />
                    <InputField field_type=InputFieldType::Text label="Description" required=true id_attr="description" name="description" />

                    <DatePicker label="Start Date" required=true id_attr="start_date" initial_value=init_date name="start_date" />
                    <RadioInputField label="Active" required=true id_attr="active_lone" name="active" initial_value=Signal::derive(|| "true") />
                    <RadioInputField label="Inactive" required=true id_attr="inactive_lone" name="active" initial_value=Signal::derive(|| "false") />
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
