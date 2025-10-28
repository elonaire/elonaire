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

use crate::components::forms::select::{SelectInput, SelectOption};
use crate::components::general::spinner::Spinner;
use crate::data::models::graphql::shared::{
    CreateResumeItemResponse, ResumeItemInputVars, UserResumeInput, UserResumeSection,
};
use crate::utils::custom_traits::EnumerableEnum;
use crate::utils::graphql_client::perform_mutation_or_query_with_vars;
use crate::{
    components::{
        forms::{
            datepicker::DatePicker,
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
pub fn Resume() -> impl IntoView {
    view! {
        <>
            <Outlet />
        </>
    }
}

#[island]
pub fn ResumeItemsList() -> impl IntoView {
    let table_data = RwSignal::new((
        vec![
            Column::new("Title", false),
            Column::new("Start Date", true),
            Column::new("End Date", true),
            Column::new("Section", true),
        ],
        vec![],
    ));

    Effect::new(move || {
        spawn_local(async move {});
    });

    view! {
        <>
            <Title text="Resume Items"/>
            <div class="mx-[20px]">
                <Breadcrumbs custom_route_names=["Home", "Dashboard", "Resume Items"] />
            </div>

            <h1 class="mx-[20px]">Resume Items</h1>

            <div class="mx-[20px] flex items-center justify-end">
                <A href="/dashboard/resume/create">
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
pub fn CreateResumeItem() -> impl IntoView {
    let form_ref = NodeRef::new();
    let (form_is_valid, set_form_is_valid) = signal(false);
    let submit_is_disabled = Memo::new(move |_| !form_is_valid.get());
    let current_state = expect_context::<Store<AppStateContext>>();
    let success_modal_is_open = RwSignal::new(false);
    let confirm_modal_is_open = RwSignal::new(false);
    let (submission_confirmed, set_submission_confirmed) = signal(false);
    let init_date = RwSignal::new(None);
    let (is_loading, set_is_loading) = signal(false);
    let resume_sections = RwSignal::new(
        UserResumeSection::variants_slice()
            .iter()
            .map(|section| {
                let mut label = format!("{}", section);
                if label.is_empty() {
                    label = "Select Section".to_string();
                }
                SelectOption::new(format!("{}", section).as_str(), label.as_str())
            })
            .collect::<Vec<SelectOption>>(),
    );

    let onprimary_handler = Callback::new(move |_| {
        set_submission_confirmed.set(true);
    });

    let onreset_handler = Callback::new(move |_ev: ev::Event| {
        init_date.set(None);
    });

    Effect::new(move || {
        if submission_confirmed.get() && form_is_valid.get() {
            set_is_loading.set(true);
            spawn_local(async move {
                if let Some(form_data) = get_form_data_from_form_ref(&form_ref) {
                    let deserialized_form_data =
                        deserialize_form_data_to_struct::<UserResumeInput>(&form_data, true);

                    if deserialized_form_data.is_none() {
                        set_is_loading.set(false);
                        return;
                    }

                    let deserialized_form_data = deserialized_form_data.unwrap();

                    let input_vars = ResumeItemInputVars {
                        resume_item: deserialized_form_data,
                    };

                    let query = r#"
                           mutation CreateResumeItem($resumeItem: UserResumeInput!) {
                                createResumeItem(resumeItem: $resumeItem) {
                                    title
                                    moreInfo
                                    startDate
                                    endDate
                                    link
                                    section
                                    id
                                    yearsOfExperience
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
                        CreateResumeItemResponse,
                        ResumeItemInputVars,
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
            <Title text="New Resume Item"/>
            <BasicModal title="Success" is_open=success_modal_is_open use_case=UseCase::Success disable_auto_close=false>
                <div>
                    <p>"Resume Item created successfully!"</p>
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
                <Breadcrumbs custom_route_names=["Home", "Dashboard", "Resume Items", "New"] />
            </div>

            <h1 class="mx-[20px]">New Resume Item</h1>

            <ReactiveForm on:submit=handle_step_form_submit onreset=onreset_handler form_ref=form_ref>
                <div class="mx-[20px] flex flex-col gap-[20px]">
                    <InputField field_type=InputFieldType::Text label="Title" required=true id_attr="title" name="title" />
                    <InputField field_type=InputFieldType::Text label="More Info" id_attr="more_info" name="more_info" />

                    <DatePicker label="Start Date" required=true id_attr="start_date" initial_value=init_date name="start_date" />
                    <DatePicker label="End Date" id_attr="end_date" initial_value=init_date name="end_date" />
                    <InputField field_type=InputFieldType::Text label="Link" id_attr="link" name="link" />
                    <SelectInput
                    label="Section"
                    name="section"
                    required=true
                    id_attr="section"
                    options=resume_sections
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
