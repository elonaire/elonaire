use icondata::{AiGithubOutlined, AiGoogleOutlined};
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

use crate::components::general::modal::modal::BasicModal;
use crate::components::general::modal::modal::UseCase;
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
use crate::data::models::general::acl::AuthCode;
use crate::data::models::general::acl::AuthDetails;
use crate::data::models::general::acl::OauthClientName;
use crate::data::models::general::shared::RestResponse;
use crate::data::models::graphql::acl::SignUpResponse;
use crate::data::models::graphql::acl::SignUpVars;
use crate::data::models::graphql::acl::UserInput;
use crate::data::models::graphql::acl::{SignInResponse, SignInVars};
use crate::data::{
    context::store::{AppStateContext, AppStateContextStoreFields},
    models::{
        general::acl::{AuthInfoStoreFields, UserInfoStoreFields},
        graphql::acl::UserLoginsInput,
    },
};
use crate::utils::errors::handle_graphql_errors;
use crate::utils::forms::{deserialize_form_data_to_struct, get_form_data_from_form_ref};
use crate::utils::graphql_client::perform_mutation_or_query_with_vars;

const ACL_SERVICE_API: Option<&str> = option_env!("ACL_SERVICE_API");

#[component]
pub fn SignUp() -> impl IntoView {
    let signup_form_ref = NodeRef::new();
    let store = expect_context::<Store<AppStateContext>>();
    let (form_is_valid, set_form_is_valid) = signal(false);
    let (confirm_password_value, set_confirm_password_value) = signal(None as Option<String>);
    let (password_is_matching, set_password_is_matching) = signal(false);
    let submit_is_disabled =
        Memo::new(move |_| !form_is_valid.get() || !password_is_matching.get());
    let (is_loading, set_is_loading) = signal(false);
    let is_authenticated = RwSignal::new(false);
    let navigate = use_navigate();
    let success_modal_is_open = RwSignal::new(false);

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
                        if let Ok(auth_status) = response.json::<RestResponse<AuthDetails>>().await
                        {
                            store.user().auth_info().token().set(
                                auth_status
                                    .data
                                    .map(|auth_detail| auth_detail.token.unwrap_or_default())
                                    .unwrap_or_default(),
                            );
                            is_authenticated.set(true);
                            set_is_loading.set(false);
                        };
                    };
                });
            }
            None => {}
        }
    });

    let navigate_effect = navigate.clone();
    let navigate_submit = navigate.clone();

    Effect::new(move || {
        if is_authenticated.get() {
            let redirect_to = store.redirect_to().get();
            if let Some(redirect_to) = redirect_to {
                store.error().set(None);
                navigate_effect(&redirect_to, Default::default());
            } else {
                navigate_effect("/dashboard", Default::default());
            }
        }
    });

    let open_url = |url: &str| {
        if let Some(window) = window() {
            let _ = window.open_with_url_and_target(url, "_self");
        }
    };

    let onsocial_sign_up = move |client: OauthClientName| {
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
                let res = perform_mutation_or_query_with_vars::<SignInResponse, SignInVars>(
                    None,
                    acl_service_api,
                    query,
                    user_logins,
                )
                .await;

                match res.get_data() {
                    Some(data) => match &data.sign_in {
                        Some(auth_details) => {
                            open_url(
                                auth_details
                                    .get_data()
                                    .url
                                    .as_ref()
                                    .unwrap_or(&"/sign-up".into())
                                    .as_str(),
                            );
                        }
                        None => set_is_loading.set(false),
                    },
                    None => set_is_loading.set(false),
                };
            });
        })
    };

    let handle_signup_submit = move |ev: ev::SubmitEvent| {
        ev.prevent_default();
        ev.stop_propagation();
        // let navigate = navigate_submit.clone();

        let target = ev
            .target()
            .and_then(|t| t.dyn_into::<HtmlFormElement>().ok());

        if let Some(form) = target {
            set_form_is_valid.set(form.check_validity());

            if form_is_valid.get() {
                if let Some(_submitter) = ev.submitter() {
                    set_is_loading.set(true);
                    spawn_local(async move {
                        let Some(form_data) = get_form_data_from_form_ref(&signup_form_ref) else {
                            set_is_loading.set(false);
                            return;
                        };

                        let Some(deserialized) =
                            deserialize_form_data_to_struct::<UserInput>(&form_data, true, None)
                        else {
                            set_is_loading.set(false);
                            return;
                        };

                        let vars = SignUpVars { user: deserialized };

                        let query = r#"
                            mutation SignUp($user: UserInput!) {
                                signUp(user: $user) {
                                    data {
                                        userName
                                        firstName
                                        middleName
                                        lastName
                                        gender
                                        dob
                                        email
                                        country
                                        phone
                                        createdAt
                                        updatedAt
                                        status
                                        oauthClient
                                        oauthUserId
                                        profilePicture
                                        bio
                                        website
                                        address
                                        id
                                        fullName
                                        age
                                    }
                                    metadata {
                                        newAccessToken
                                        requestId
                                    }
                                }
                            }
                        "#;

                        let Some(acl_service_api) = ACL_SERVICE_API else {
                            set_is_loading.set(false);
                            return;
                        };

                        let res =
                            perform_mutation_or_query_with_vars::<SignUpResponse, SignUpVars>(
                                None,
                                acl_service_api,
                                query,
                                vars,
                            )
                            .await;

                        match res.get_data() {
                            Some(data) => match &data.sign_up {
                                Some(_user) => {
                                    if let Some(form_el) = signup_form_ref
                                        .get_untracked()
                                        .and_then(|el| el.dyn_into::<HtmlFormElement>().ok())
                                    {
                                        form_el.reset();
                                        set_form_is_valid.set(false);
                                    }

                                    set_is_loading.set(false);
                                    set_confirm_password_value.set(None);
                                    success_modal_is_open.set(true);
                                    // navigate("/sign-in", Default::default())
                                }
                                None => set_is_loading.set(false),
                            },
                            None => {
                                let _handle_errors = handle_graphql_errors(&res, &store, None);
                                set_is_loading.set(false);
                            }
                        };
                    });
                }
            }
        }
    };

    Effect::new(move || {
        let Some(confirmed_password) = confirm_password_value.get() else {
            return;
        };
        let Some(form_data) = get_form_data_from_form_ref(&signup_form_ref) else {
            return;
        };

        let Some(deserialized) =
            deserialize_form_data_to_struct::<UserInput>(&form_data, true, None)
        else {
            return;
        };

        if confirmed_password == deserialized.password {
            set_password_is_matching.set(true);
        } else {
            set_password_is_matching.set(false);
        };
    });

    view! {
        <Title text="Sign Up"/>

        <Show when=move || is_loading.get()>
            <Spinner />
        </Show>

        <BasicModal
            title="Sign Up Successful"
            is_open=success_modal_is_open
            use_case=UseCase::Success
            disable_auto_close=false
        >
            <div class="p-[10px]">
                <p>"Your registration was successful!"</p>
                <p>"A confirmation email was sent to your inbox. Confirm your email for your account to be activated."</p>
            </div>
        </BasicModal>

        <div class="flex flex-col items-center justify-center p-8 min-h-svh">
            <h1 class="text-4xl font-bold my-4">"Create Account"</h1>

            // Social sign-up
            <div class="w-full max-w-md flex flex-col items-center gap-2 my-4">
                <BasicButton
                    button_text="Continue with Google"
                    style_ext="border-[1px] border-danger hover:bg-danger transition-all duration-300 ease-in-out hover:shadow-md hover:-translate-y-1 hover:z-10 hover:text-contrast-white text-danger w-full"
                    onclick=onsocial_sign_up(OauthClientName::Google)
                    icon=Some(AiGoogleOutlined)
                    icon_before=true
                />
                <BasicButton
                    button_text="Continue with GitHub"
                    style_ext="border-[1px] border-gray hover:bg-gray transition-all duration-300 ease-in-out hover:shadow-md hover:-translate-y-1 hover:z-10 hover:text-contrast-white w-full"
                    onclick=onsocial_sign_up(OauthClientName::Github)
                    icon=Some(AiGithubOutlined)
                    icon_before=true
                />
            </div>

            <div class="w-full max-w-md flex items-center my-6">
                <hr class="flex-grow border-t border-mid-gray"/>
                <span class="mx-4">"OR"</span>
                <hr class="flex-grow border-t border-mid-gray"/>
            </div>

            // Sign-up form
            <ReactiveForm form_ref=signup_form_ref on:submit=handle_signup_submit ext_styles="w-full max-w-md">
                <div class="mb-6">
                    <InputField
                        label="Email"
                        field_type=InputFieldType::Email
                        name="email"
                        required=true
                        placeholder="Enter your email"
                        id_attr="email"
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
                        placeholder="Choose a password"
                        id_attr="password"
                        ext_input_styles="focus:ring-secondary"
                    />
                </div>
                <div class="mb-6">
                    <InputField
                        label="Confirm Password"
                        field_type=InputFieldType::Password
                        required=true
                        placeholder="Repeat your password"
                        id_attr="confirm_password"
                        ext_input_styles="focus:ring-secondary"
                        on:input=move |e| set_confirm_password_value.set(Some(event_target_value(&e)))
                    />
                    <p class=move || format!("text-xs py-[5px] h-[30px] {}", if password_is_matching.get() { "text-success" } else { "text-danger" })>{move || if confirm_password_value.get().is_some() {
                        Some(
                            format!("{}", if password_is_matching.get() { "Your passwords are matching!" } else { "Your passwords are not matching!" })
                        )
                    } else {None}}</p>
                </div>
                <BasicButton
                    button_text="Create Account"
                    style_ext="bg-primary text-contrast-white px-4 py-2 hover:bg-primary transition duration-300 ease-in-out hover:shadow-md hover:-translate-y-1 hover:z-10 text-contrast-white w-full"
                    button_type=ButtonType::Submit
                    disabled=submit_is_disabled
                />
                <div class="flex items-center justify-center mt-6 text-sm text-secondary">
                    <A href="/sign-in">"Already have an account? Sign in"</A>
                </div>
            </ReactiveForm>
        </div>
    }
}
