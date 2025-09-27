use icondata as IconData;
use leptos::ev::SubmitEvent;
use leptos::prelude::*;
use leptos::task::spawn_local;
use leptos::wasm_bindgen::JsCast;
use leptos_meta::*;
use leptos_router::components::A;
use leptos_router::components::Outlet;
use reactive_stores::Store;
use web_sys::{FormData, HtmlFormElement, HtmlInputElement};

use crate::components::forms::datepicker::DatePicker;
use crate::components::forms::input::CustomFileInput;
use crate::components::forms::input::InputField;
use crate::components::forms::input::InputFieldType;
use crate::components::forms::reactive_form::ReactiveForm;
use crate::components::forms::select::SelectInput;
use crate::components::forms::select::SelectOption;
use crate::components::general::button::ButtonType;
use crate::components::general::modal::modal::BasicModal;
use crate::components::general::modal::modal::UseCase;
use crate::components::general::{
    breadcrumbs::Breadcrumbs,
    button::BasicButton,
    table::data_table::{Column, DataTable},
};
use crate::schemas::general::acl::AuthInfoStoreFields;
use crate::schemas::general::acl::UserInfoStoreFields;
use crate::schemas::general::acl::{AppStateContext, AppStateContextStoreFields};
use crate::schemas::general::files::UploadedFileResponse;
use crate::schemas::graphql::shared::AddPortfolioItem;
use crate::schemas::graphql::shared::UserPortfolioInput;
use crate::schemas::graphql::shared::UserPortfolioInputFields;
use crate::utils::forms::{deserialize_form_data_to_struct, get_form_data_from_form_ref};
use cynic::{MutationBuilder, http::ReqwestExt};

#[island]
pub fn Portfolio() -> impl IntoView {
    view! {
        <>
            <Outlet />
        </>
    }
}

#[island]
pub fn PortfolioList() -> impl IntoView {
    let table_data = RwSignal::new((
        vec![
            Column::new("Title", false),
            Column::new("Description", true),
            Column::new("Start Date", true),
            Column::new("End Date", true),
        ],
        vec![],
    ));

    view! {
        <>
            <Title text="My Portfolio"/>
            <div class="mx-[20px]">
                <Breadcrumbs custom_route_names=["Home", "Dashboard", "Portfolio"] />
            </div>

            <h1 class="mx-[20px]">Portfolio</h1>

            <div class="mx-[20px] flex items-center justify-end">
                <A href="/dashboard/portfolio/create">
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
pub fn CreatePortfolio() -> impl IntoView {
    let form_ref = NodeRef::new();
    let file_input_ref = NodeRef::new();
    let (form_is_valid, set_form_is_valid) = signal(false);
    let submit_is_disabled = Memo::new(move |_| !form_is_valid.get());
    let current_state = expect_context::<Store<AppStateContext>>();
    let success_modal_is_open = RwSignal::new(false);
    let confirm_modal_is_open = RwSignal::new(false);
    let (submission_confirmed, set_submission_confirmed) = signal(false);

    let onprimary_handler = Callback::new(move |_| {
        set_submission_confirmed.set(true);
    });

    Effect::new(move || {
        if submission_confirmed.get() && form_is_valid.get() {
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
                        match gloo_net::http::Request::post(
                            "http://localhost:8080/api/files/upload",
                        )
                        .header(
                            "Authorization",
                            format!(
                                "Bearer {}",
                                current_state.user().auth_info().token().get_untracked()
                            )
                            .as_str(),
                        )
                        .body(files_form_data)
                        .unwrap()
                        .send()
                        .await
                        {
                            Ok(response) => {
                                match response.json::<UploadedFileResponse>().await {
                                    Ok(uploaded_file) => {
                                        if let Some(form_data) =
                                            get_form_data_from_form_ref(&form_ref)
                                        {
                                            // Implement logic to handle form data
                                            if let Ok(_) = form_data.append_with_str(
                                                "thumbnail",
                                                format!(
                                                    "http://localhost:3001/view/{}",
                                                    uploaded_file.file_name
                                                )
                                                .as_str(),
                                            ) {
                                                let deserialized_form_data =
                                                    deserialize_form_data_to_struct::<
                                                        UserPortfolioInput,
                                                    >(
                                                        &form_data
                                                    );

                                                let operation = AddPortfolioItem::build(
                                                    UserPortfolioInputFields {
                                                        portfolio_item: deserialized_form_data
                                                            .unwrap(),
                                                    },
                                                );

                                                let response = reqwest::Client::new()
                                                    .post("http://localhost:8080/api/shared")
                                                    .header(
                                                        "Authorization",
                                                        format!(
                                                            "Bearer {}",
                                                            current_state
                                                                .user()
                                                                .auth_info()
                                                                .token()
                                                                .get_untracked()
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
                                                            .and_then(|el| {
                                                                el.dyn_into::<HtmlFormElement>()
                                                                    .ok()
                                                            })
                                                        {
                                                            form.reset();
                                                            set_form_is_valid
                                                                .set(form.check_validity());
                                                        }

                                                        success_modal_is_open
                                                            .update(|status| *status = true);
                                                    }
                                                    None => {
                                                        leptos::logging::error!(
                                                            "Failed to add portfolio item: {:?}",
                                                            response.errors
                                                        );
                                                    }
                                                };
                                            };
                                        };
                                    }
                                    Err(err) => {
                                        leptos::logging::error!(
                                            "Failed to parse uploaded file response: {:?}",
                                            err
                                        );
                                    }
                                };
                            }
                            Err(err) => {
                                leptos::logging::error!("Failed to upload files: {:?}", err);
                            }
                        };
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
            <Title text="Create Portfolio"/>
            <BasicModal title="Success" is_open=success_modal_is_open use_case=UseCase::Success disable_auto_close=false>
                <div>
                    <p>"Portfolio Project created successfully!"</p>
                </div>
            </BasicModal>
            <BasicModal title="Confirm" on_click_primary=onprimary_handler is_open=confirm_modal_is_open use_case=UseCase::Confirmation disable_auto_close=false>
                <div>
                    <p>"Are you sure that you want to submit?"</p>
                </div>
            </BasicModal>

            <div class="mx-[20px]">
                <Breadcrumbs custom_route_names=["Home", "Dashboard", "Portfolio", "Create"] />
            </div>

            <h1 class="mx-[20px]">Create New Portfolio Project</h1>

            <ReactiveForm on:submit=handle_step_form_submit form_ref=form_ref>
                <div class="mx-[20px] flex flex-col gap-[20px]">
                    <InputField field_type=InputFieldType::Text label="Title" required=true id_attr="title" name="title" />
                    <InputField field_type=InputFieldType::Text label="Description" required=true id_attr="description" name="description" />
                    <DatePicker label="Start Date" required=true id_attr="start_date" name="start_date" />
                    <DatePicker label="End Date" required=true id_attr="end_date" name="end_date" />
                    <InputField field_type=InputFieldType::Text label="Link" required=true id_attr="link" name="link" />
                    <SelectInput
                    label="Category"
                    name="category"
                    required=true
                    id_attr="category"
                    options=vec![
                        SelectOption::new("", "Select Category"),
                        SelectOption::new("JavaScript", "JavaScript")
                    ]
                    />
                    <CustomFileInput input_node_ref=file_input_ref label="Thumbnail" name="thumbnail" id_attr="thumbnail" required=true />
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
