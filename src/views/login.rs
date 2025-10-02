use icondata as IconId;
use leptos::ev;
use leptos::prelude::*;
use leptos::task::spawn_local;
use leptos::wasm_bindgen::JsCast;
use leptos_meta::*;
use leptos_router::components::A;
use leptos_router::hooks::use_navigate;
use leptos_router::hooks::use_query;
use reactive_stores::Store;
use web_sys::HtmlFormElement;
use web_sys::window;

use crate::components::forms::input::InputField;
use crate::components::forms::input::InputFieldType;
use crate::components::forms::reactive_form::ReactiveForm;
// use crate::components::general::breadcrumbs::Breadcrumbs;
use crate::components::general::button::BasicButton;
use crate::components::general::button::ButtonType;
use crate::components::general::spinner::Spinner;
use crate::schemas::general::acl::AppStateContext;
use crate::schemas::general::acl::AppStateContextStoreFields;
use crate::schemas::general::acl::AuthCode;
use crate::schemas::general::acl::AuthDetailsRest;
use crate::schemas::general::acl::{AuthInfoStoreFields, UserInfoStoreFields};
use crate::schemas::graphql::acl::{
    OauthClientName, SignInMutation, UserLoginsInput, UserLoginsInputFields,
};
use crate::utils::forms::deserialize_form_data_to_struct;
use crate::utils::forms::get_form_data_from_form_ref;
use cynic::{MutationBuilder, http::ReqwestExt};

#[island]
pub fn SignIn() -> impl IntoView {
    let login_form_ref = NodeRef::new();
    let current_state = expect_context::<Store<AppStateContext>>();
    let (form_is_valid, set_form_is_valid) = signal(false);
    let submit_is_disabled = Memo::new(move |_| !form_is_valid.get());
    let success_modal_is_open = RwSignal::new(false);
    let (is_loading, set_is_loading) = signal(false);

    let query = use_query::<AuthCode>();
    Effect::new(move || {
        match query
            .read()
            .as_ref()
            .ok()
            .and_then(|params| params.auth_code.clone())
        {
            Some(auth_code) => {
                spawn_local(async move {
                    let auth_code_body = AuthCode {
                        auth_code: Some(auth_code),
                    };

                    let response = reqwest::Client::new()
                        .post("http://localhost:8080/api/acl/social-sign-in")
                        .json(&auth_code_body)
                        .send()
                        .await
                        .unwrap();

                    if let Ok(auth_status) = response.json::<AuthDetailsRest>().await {
                        *current_state.user().auth_info().token().write() =
                            auth_status.token.unwrap();
                    };
                });
            }
            None => {}
        }
    });

    Effect::new(move || {
        let leptos_navigate = use_navigate();

        if !current_state.user().get().auth_info.token.is_empty() {
            // User is authenticated, redirect to dashboard
            leptos_navigate("/dashboard", Default::default());
        }
    });

    let navigate = |url: &str| {
        if let Some(window) = window() {
            let _ = window.open_with_url_and_target(url, "_self");
        }
    };

    let onsocial_sign_in = move |client: OauthClientName| {
        Callback::new(move |_e: ev::MouseEvent| {
            let operation = SignInMutation::build(UserLoginsInputFields {
                raw_user_details: UserLoginsInput {
                    user_name: None,
                    password: None,
                    oauth_client: Some(client),
                },
            });

            spawn_local(async move {
                let response = reqwest::Client::new()
                    .post("http://localhost:8080/api/acl")
                    .run_graphql(operation)
                    .await
                    .unwrap();

                match response.data {
                    Some(data) => {
                        match data.sign_in {
                            Some(auth_details) => {
                                navigate(auth_details.url.unwrap().as_str());
                            }
                            None => {}
                        };
                    }
                    None => {}
                };
            });
        })
    };

    let handle_step_form_submit = move |ev: ev::SubmitEvent| {
        ev.prevent_default();
        ev.stop_propagation();

        // Implement logic to show form validity
        let target = ev
            .target()
            .and_then(|t| t.dyn_into::<HtmlFormElement>().ok());

        if let Some(form) = target {
            set_form_is_valid.set(form.check_validity());

            if form_is_valid.get() {
                if let Some(_submitter) = ev.submitter() {
                    set_is_loading.set(true);
                    spawn_local(async move {
                        if let Some(form_data) = get_form_data_from_form_ref(&login_form_ref) {
                            let deserialized_form_data =
                                deserialize_form_data_to_struct::<UserLoginsInput>(
                                    &form_data, true,
                                );

                            if deserialized_form_data.is_none() {
                                set_is_loading.set(false);
                                return;
                            }

                            let operation = SignInMutation::build(UserLoginsInputFields {
                                raw_user_details: deserialized_form_data.unwrap(),
                            });

                            let response = reqwest::Client::new()
                                .post("http://localhost:8080/api/acl")
                                .run_graphql(operation)
                                .await
                                .unwrap();

                            match response.data {
                                Some(data) => {
                                    match data.sign_in {
                                        Some(auth_details) => {
                                            if let Some(form) =
                                                login_form_ref.get_untracked().and_then(|el| {
                                                    el.dyn_into::<HtmlFormElement>().ok()
                                                })
                                            {
                                                form.reset();
                                                set_form_is_valid.set(false);
                                            } else {
                                            }
                                            set_is_loading.set(false);

                                            success_modal_is_open.update(|status| *status = true);

                                            *current_state.user().auth_info().token().write() =
                                                auth_details.token.unwrap();
                                        }
                                        None => {
                                            set_is_loading.set(false);
                                        }
                                    };
                                }
                                None => {
                                    set_is_loading.set(false);
                                }
                            };
                        };
                    });
                }
            }
        }
    };

    view! {
        <Title text="Sign In"/>

        <Show when=move || is_loading.get()>
            <Spinner />
        </Show>

        <div class="flex flex-col items-center justify-center p-8 bg-white">
            // <Breadcrumbs custom_route_names=["Home", "Sign In"] />
                        <h1 class="text-4xl font-bold my-4">{"Sign In"}</h1>
                        <div class="w-full max-w-md flex flex-col items-center gap-2 md:flex-row md:justify-between my-4">
                                                <BasicButton
                                                    button_text="Sign in with Google"
                                                    style_ext="bg-red-500 hover:bg-red-400 transition-all duration-300 ease-in-out hover:shadow-md hover:-translate-y-1 hover:z-10 text-white w-full"
                                                    onclick=onsocial_sign_in(OauthClientName::Google)
                                                    icon=Some(IconId::AiGoogleOutlined) // Assuming you have icons for Google
                                                    icon_before=true
                                                />
                                                <BasicButton
                                                    button_text="Sign in with GitHub"
                                                    style_ext="bg-gray-700 hover:bg-gray-600 transition-all duration-300 ease-in-out hover:shadow-md hover:-translate-y-1 hover:z-10 text-white w-full"
                                                    onclick=onsocial_sign_in(OauthClientName::Github)
                                                    icon=Some(IconId::AiGithubOutlined) // Assuming you have icons for GitHub
                                                    icon_before=true
                                                />
                        </div>
                        <div class="w-full max-w-md flex items-center my-6">
                            <hr class="flex-grow border-t border-gray-300"/>
                            <span class="mx-4 text-gray-500">{"OR"}</span>
                            <hr class="flex-grow border-t border-gray-300"/>
                        </div>

                        <ReactiveForm form_ref=login_form_ref on:submit=handle_step_form_submit ext_styles="w-full max-w-md">
                            <div class="mb-6">
                                <InputField
                                    label="Email/Username"
                                    field_type=InputFieldType::Text
                                    name="user_name"
                                    required=true
                                    placeholder="Enter your email or username"
                                    id_attr="username"
                                    ext_input_styles="focus:ring-secondary"
                                    autocomplete="on"
                                />
                            </div>
                            <div class="mb-6">
                                <InputField
                                    label="Password"
                                    field_type=InputFieldType::Password
                                    name="password"
                                    required=true
                                    placeholder="Enter your password"
                                    id_attr="password"
                                    ext_input_styles="focus:ring-secondary"
                                    autocomplete="on"
                                />
                            </div>

                            <div class="flex items-center justify-between mb-6">
                                <a class="text-sm text-blue-500 hover:text-blue-700" href="#">{ "Forgot Password?" }</a>
                            </div>
                            <BasicButton
                                button_text="Sign In"
                                style_ext="bg-primary text-white px-4 py-2 hover:bg-secondary transition duration-300 ease-in-out hover:shadow-md hover:-translate-y-1 hover:z-10 text-white w-full"
                                button_type=ButtonType::Submit
                                disabled=submit_is_disabled
                            />
                            <div class="flex items-center justify-center mt-6 text-sm text-blue-500 hover:text-blue-400">
                                <A href="/signup">"Don't have an account? Sign up"</A>
                            </div>
                        </ReactiveForm>
        </div>
    }
}
