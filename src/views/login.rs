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

use crate::components::{
    forms::{
        input::{InputField, InputFieldType},
        reactive_form::ReactiveForm,
    },
    general::{
        button::{BasicButton, ButtonType},
        spinner::Spinner,
    },
};
use crate::data::models::general::acl::OauthClientName;
use crate::data::models::graphql::acl::SignInResponse;
use crate::data::models::graphql::acl::SignInVars;
use crate::data::{
    context::store::{AppStateContext, AppStateContextStoreFields},
    models::{
        general::acl::{AuthCode, AuthDetails, AuthInfoStoreFields, UserInfoStoreFields},
        graphql::acl::UserLoginsInput,
    },
};
use crate::utils::forms::{deserialize_form_data_to_struct, get_form_data_from_form_ref};
use crate::utils::graphql_client::perform_mutation_or_query_with_vars;

const ACL_SERVICE_API: Option<&str> = option_env!("ACL_SERVICE_API");

#[component]
pub fn SignIn() -> impl IntoView {
    let login_form_ref = NodeRef::new();
    let current_state = expect_context::<Store<AppStateContext>>();
    let (form_is_valid, set_form_is_valid) = signal(false);
    let submit_is_disabled = Memo::new(move |_| !form_is_valid.get());
    let (is_loading, set_is_loading) = signal(false);
    let is_authenticated = RwSignal::new(false);
    let navigate = use_navigate();

    let query = use_query::<AuthCode>();
    Effect::new(move || {
        match query
            .read()
            .as_ref()
            .ok()
            .and_then(|params| params.auth_code.clone())
        {
            Some(auth_code) => {
                set_is_loading.set(true);
                spawn_local(async move {
                    let auth_code_body = AuthCode {
                        auth_code: Some(auth_code),
                    };

                    let Some(acl_service_api) = ACL_SERVICE_API else {
                        return;
                    };

                    if let Ok(response) = reqwest::Client::new()
                        .post(&format!("{acl_service_api}/social-sign-in"))
                        .json(&auth_code_body)
                        .send()
                        .await
                    {
                        if let Ok(auth_status) = response.json::<AuthDetails>().await {
                            current_state
                                .user()
                                .auth_info()
                                .token()
                                .set(auth_status.token.unwrap_or_default());
                            is_authenticated.set(true);
                            set_is_loading.set(false);
                        };
                    };
                });
            }
            None => {}
        }
    });

    Effect::new(move || {
        let redirect_to = current_state.redirect_to().get();
        // User is authenticated, redirect to the previous page or dashboard
        if is_authenticated.get() {
            if let Some(redirect_to) = redirect_to {
                current_state.error().set(None);
                navigate(&redirect_to, Default::default());
            } else {
                navigate("/dashboard", Default::default());
            }
        }
    });

    let open_url = |url: &str| {
        if let Some(window) = window() {
            let _ = window.open_with_url_and_target(url, "_self");
        }
    };

    let onsocial_sign_in = move |client: OauthClientName| {
        Callback::new(move |_e: ev::MouseEvent| {
            let user_logins = SignInVars {
                raw_user_details: UserLoginsInput {
                    user_name: None,
                    password: None,
                    oauth_client: Some(client),
                },
            };

            let query = r#"
                   mutation SignIn($rawUserDetails: UserLoginsInput!) {
                       signIn(rawUserDetails: $rawUserDetails) {
                           data {
                               url
                           }
                           metadata {
                               newAccessToken
                               requestId
                           }
                       }
                   }
               "#;

            let Some(acl_service_api) = ACL_SERVICE_API else {
                return;
            };

            set_is_loading.set(true);
            spawn_local(async move {
                let login_res = perform_mutation_or_query_with_vars::<SignInResponse, SignInVars>(
                    None,
                    acl_service_api,
                    query,
                    user_logins,
                )
                .await;

                match login_res.get_data() {
                    Some(data) => {
                        match &data.sign_in {
                            Some(auth_details) => {
                                open_url(
                                    auth_details
                                        .get_data()
                                        .url
                                        .as_ref()
                                        .unwrap_or(&"/sign-in".into())
                                        .as_str(),
                                );
                            }
                            None => {
                                set_is_loading.set(false);
                            }
                        };
                    }
                    None => {
                        set_is_loading.set(false);
                        // login_res.get_error()
                    }
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
                                    &form_data, true, None,
                                );

                            if deserialized_form_data.is_none() {
                                set_is_loading.set(false);
                                return;
                            }

                            let deserialized_form_data = deserialized_form_data.unwrap();

                            let user_logins = SignInVars {
                                raw_user_details: deserialized_form_data,
                            };

                            let query = r#"
                                   mutation SignIn($rawUserDetails: UserLoginsInput!) {
                                       signIn(rawUserDetails: $rawUserDetails) {
                                            data {
                                                token
                                            }
                                            metadata {
                                                newAccessToken
                                                requestId
                                            }
                                       }
                                   }
                               "#;

                            let Some(acl_service_api) = ACL_SERVICE_API else {
                                return;
                            };

                            let login_res = perform_mutation_or_query_with_vars::<
                                SignInResponse,
                                SignInVars,
                            >(
                                None, acl_service_api, query, user_logins
                            )
                            .await;

                            match login_res.get_data() {
                                Some(data) => {
                                    match &data.sign_in {
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
                                            is_authenticated.set(true);
                                            set_is_loading.set(false);

                                            current_state.user().auth_info().token().set(
                                                auth_details
                                                    .get_data()
                                                    .token
                                                    .as_ref()
                                                    .unwrap_or(&String::new())
                                                    .to_owned(),
                                            );
                                        }
                                        None => {
                                            set_is_loading.set(false);
                                        }
                                    };
                                }
                                None => {
                                    set_is_loading.set(false);
                                    // login_res.get_error()
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

        <div class="flex flex-col items-center justify-center p-8 bg-contrast-white min-h-svh">
            // <Breadcrumbs custom_route_names=["Home", "Sign In"] />
                        <h1 class="text-4xl font-bold my-4">{"Sign In"}</h1>
                        <div class="w-full max-w-md flex flex-col items-center gap-2 md:flex-row md:justify-between my-4">
                                                <BasicButton
                                                    button_text="Sign in with Google"
                                                    style_ext="bg-danger hover:bg-danger transition-all duration-300 ease-in-out hover:shadow-md hover:-translate-y-1 hover:z-10 text-contrast-white w-full"
                                                    onclick=onsocial_sign_in(OauthClientName::Google)
                                                    icon=Some(IconId::AiGoogleOutlined) // Assuming you have icons for Google
                                                    icon_before=true
                                                />
                                                <BasicButton
                                                    button_text="Sign in with GitHub"
                                                    style_ext="bg-gray hover:bg-gray transition-all duration-300 ease-in-out hover:shadow-md hover:-translate-y-1 hover:z-10 text-contrast-white w-full"
                                                    onclick=onsocial_sign_in(OauthClientName::Github)
                                                    icon=Some(IconId::AiGithubOutlined) // Assuming you have icons for GitHub
                                                    icon_before=true
                                                />
                        </div>
                        <div class="w-full max-w-md flex items-center my-6">
                            <hr class="flex-grow border-t border-mid-gray"/>
                            <span class="mx-4 text-mid-gray">{"OR"}</span>
                            <hr class="flex-grow border-t border-mid-gray"/>
                        </div>

                        <ReactiveForm form_ref=login_form_ref on:submit=handle_step_form_submit ext_styles="w-full max-w-md">
                            <div class="mb-6">
                                <InputField
                                    label="Email/Username"
                                    field_type=InputFieldType::Text
                                    name="user_name"
                                    required=true
                                    placeholder="Enter your email or username"
                                    id_attr="user_name"
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
                                <a class="text-sm text-secondary hover:secondary" href="#">{ "Forgot Password?" }</a>
                            </div>
                            <BasicButton
                                button_text="Sign In"
                                style_ext="bg-primary text-contrast-white px-4 py-2 hover:bg-primary transition duration-300 ease-in-out hover:shadow-md hover:-translate-y-1 hover:z-10 text-contrast-white w-full"
                                button_type=ButtonType::Submit
                                disabled=submit_is_disabled
                            />
                            <div class="flex items-center justify-center mt-6 text-sm text-secondary hover:text-secondary">
                                <A href="/signup">"Don't have an account? Sign up"</A>
                            </div>
                        </ReactiveForm>
        </div>
    }
}
