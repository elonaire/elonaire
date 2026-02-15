use std::collections::HashMap;

use icondata as IconId;
use leptos::ev::{self, SubmitEvent};
use leptos::prelude::*;
use leptos::task::spawn_local;
use leptos::wasm_bindgen::JsCast;
use leptos_icons::Icon;
use leptos_meta::*;
use leptos_router::components::{A, Outlet};
use reactive_stores::Store;
use web_sys::{HtmlFormElement, HtmlInputElement};

use crate::components::forms::select::{SelectInput, SelectOption};
use crate::components::general::spinner::Spinner;
use crate::components::general::table::data_table::TableCellData;
use crate::data::context::shared::fetch_resume;
use crate::data::models::graphql::shared::{
    CreateResumeItemResponse, ResumeItemInputVars, UserResumeInput, UserResumeSection,
};
use crate::utils::custom_traits::EnumerableEnum;
use crate::utils::graphql_client::{
    perform_mutation_or_query_with_vars, perform_query_without_vars,
};
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
    data::{
        context::store::{AppStateContext, AppStateContextStoreFields},
        models::general::acl::{AuthInfoStoreFields, UserInfoStoreFields},
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
    let current_state = expect_context::<Store<AppStateContext>>();
    let resume = move || current_state.resume();
    let (is_loading, set_is_loading) = signal(false);

    let table_data = RwSignal::new((
        vec![
            Column::new("Title", false),
            Column::new("Start Date", true),
            Column::new("YOE", true),
            Column::new("Section", true),
        ],
        vec![],
    ));

    Effect::new(move || {
        let resume: Vec<HashMap<String, TableCellData>> = resume()
            .get()
            .iter()
            .map(|resume| {
                let mut hash_map_data = HashMap::new();

                // This id is the unique identifier of the table row. and is a MUST for the table to function properly.
                // *Note:* The id is a MUST for the table to function properly. You might be forced to generate a unique id for each row if your data does not have a unique identifier.
                hash_map_data.insert(
                    "id".to_string(),
                    TableCellData::String(resume.id.as_ref().unwrap().to_owned()),
                );

                hash_map_data.insert(
                    "Title".to_string(),
                    TableCellData::String(resume.title.as_ref().unwrap().to_owned()),
                );

                hash_map_data.insert(
                    "YOE".to_string(),
                    TableCellData::Usize(resume.years_of_experience.as_ref().unwrap().to_owned()),
                );

                hash_map_data.insert(
                    "Start Date".to_string(),
                    TableCellData::DateTime(resume.start_date.as_ref().unwrap().to_owned()),
                );

                hash_map_data.insert(
                    "Section".to_string(),
                    TableCellData::String(format!(
                        "{:?}",
                        resume.section.as_ref().unwrap().to_owned()
                    )),
                );
                hash_map_data
            })
            .collect();

        table_data.update(move |prev| {
            prev.1 = resume;
        });
    });

    Effect::new(move || {
        set_is_loading.set(true);
        spawn_local(async move {
            // let mut headers = HashMap::new() as HashMap<String, String>;
            // headers.insert(
            //     "Authorization".into(),
            //     format!(
            //         "Bearer {}",
            //         current_state.user().auth_info().token().get_untracked()
            //     ),
            // );

            let _fetch_resume_res = fetch_resume(&current_state, None).await;

            set_is_loading.set(false);
        });
    });

    view! {
        <>
            <Title text="Resume Items"/>
            <div class="mx-[5%] md:mx-[10%]">
                <Breadcrumbs custom_route_names=["Home", "Dashboard", "Resume Items"] />
            </div>
            <Show when=move || is_loading.get()>
                <Spinner />
            </Show>

            <h1 class="mx-[5%] md:mx-[10%]">Resume Items</h1>

            <div class="mx-[5%] md:mx-[10%] flex items-center justify-end">
                <A href="/dashboard/resume/create">
                    <BasicButton
                        button_text="Create"
                        icon=Some(IconId::BsPlusLg)
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
pub fn CreateResumeItem() -> impl IntoView {
    let form_ref = NodeRef::new();
    let (achievement_field_value, set_achievement_field_value) = signal(String::new());
    let add_is_disabled = Memo::new(move |_| !(achievement_field_value.get().len() > 10));
    let (achievements, set_achievements) = signal(Vec::new() as Vec<String>);
    let (form_is_valid, set_form_is_valid) = signal(false);
    let submit_is_disabled =
        Memo::new(move |_| !form_is_valid.get() || achievements.get().len() == 0);
    let current_state = expect_context::<Store<AppStateContext>>();
    let success_modal_is_open = RwSignal::new(false);
    let confirm_modal_is_open = RwSignal::new(false);
    let init_date = RwSignal::new(None);
    let (is_loading, set_is_loading) = signal(false);

    let resume_sections = RwSignal::new(
        UserResumeSection::variants_slice()
            .iter()
            .map(|section| SelectOption::new(section, section))
            .collect::<Vec<SelectOption>>(),
    );

    let onprimary_handler = Callback::new(move |_| {
        if form_is_valid.get() {
            set_is_loading.set(true);
            spawn_local(async move {
                if let Some(form_data) = get_form_data_from_form_ref(&form_ref) {
                    let deserialized_form_data =
                        deserialize_form_data_to_struct::<UserResumeInput>(&form_data, true, None);

                    if deserialized_form_data.is_none() {
                        set_is_loading.set(false);
                        return;
                    }

                    let deserialized_form_data = deserialized_form_data.unwrap();

                    let input_vars = ResumeItemInputVars {
                        resume_item: deserialized_form_data,
                        achievements: achievements.get_untracked(),
                    };

                    let query = r#"
                           mutation CreateResumeItem($resumeItem: UserResumeInput!, $achievements: [String!]!) {
                                createResumeItem(resumeItem: $resumeItem, achievements: $achievements) {
                                    data {
                                        title
                                        moreInfo
                                        startDate
                                        endDate
                                        link
                                        section
                                        id
                                        yearsOfExperience
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
                            } else {
                            }
                            set_is_loading.set(false);

                            success_modal_is_open.update(|status| *status = true);
                            set_achievements.set(vec![]);
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

    let handle_achievement_input_change = Callback::new(move |ev: ev::Event| {
        let target = ev
            .target()
            .and_then(|t| t.dyn_into::<HtmlInputElement>().ok());

        if let Some(input_el) = target {
            set_achievement_field_value.set(input_el.value());
        }
    });

    let handle_add_button_click = Callback::new(move |_ev: ev::MouseEvent| {
        set_achievements.update(|prev| {
            prev.push(achievement_field_value.get());
            set_achievement_field_value.set(String::new());
        });
    });

    view! {
        <>
            <Title text="New Resume Item"/>
            <BasicModal title="Success" is_open=success_modal_is_open use_case=UseCase::Success disable_auto_close=false>
                <div class="p-[10px]">
                    <p>"Resume Item created successfully!"</p>
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
                <Breadcrumbs custom_route_names=["Home", "Dashboard", "Resume Items", "New"] />
            </div>

            <h1 class="mx-[5%] md:mx-[10%]">New Resume Item</h1>

            <ReactiveForm on:submit=handle_step_form_submit onreset=onreset_handler form_ref=form_ref>
                <div class="mx-[5%] md:mx-[10%] flex flex-col gap-[20px]">
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
                    placeholder="Select Section"
                    options=resume_sections
                    />

                    <div class="flex flex-col gap-[10px]">
                        <h3>Achievements<span class="text-danger">"*"</span></h3>
                        { move || if achievements.get().is_empty() {
                            Some(view!{
                                <div class="flex flex-col text-mid-gray">
                                    <Icon icon=IconId::TbAwardOff />
                                    <p class="text-sm">No achievements added yet.</p>
                                </div>
                            })
                        } else {
                            None
                        }
                        }
                        <ul class="list-disc list-inside">
                            {
                                move || achievements.get().iter().map(|achievement| view!{ <li>{achievement.to_owned()}</li> }).collect::<Vec<_>>()
                            }
                        </ul>
                        <div class="flex flex-row items-center">
                            <InputField field_type=InputFieldType::Text placeholder="Add Achievement" initial_value=achievement_field_value oninput=handle_achievement_input_change id_attr="achievement" ext_wrapper_styles="flex-1" ext_input_styles="rounded-r-none" />
                            <BasicButton
                                button_text="Add"
                                style_ext="bg-primary text-contrast-white rounded-l-none"
                                button_type=ButtonType::Button
                                disabled=add_is_disabled
                                onclick=handle_add_button_click
                            />
                        </div>
                    </div>
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
