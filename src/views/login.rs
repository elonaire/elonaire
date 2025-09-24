use icondata as IconId;
use leptos::ev;
use leptos::html::Form;
use leptos::prelude::*;
use leptos::task::spawn_local;
use leptos_meta::*;
use leptos_router::components::A;
use leptos_router::hooks::use_navigate;
use leptos_router::hooks::use_query;
use reactive_stores::Store;
use web_sys::window;

use crate::components::forms::input::InputField;
use crate::components::forms::input::InputFieldType;
use crate::components::forms::reactive_form::ReactiveForm;
// use crate::components::general::breadcrumbs::Breadcrumbs;
use crate::components::general::button::BasicButton;
use crate::schemas::general::acl::AppStateContext;
use crate::schemas::general::acl::AppStateContextStoreFields;
use crate::schemas::general::acl::AuthCode;
use crate::schemas::general::acl::AuthDetailsRest;
use crate::schemas::general::acl::{AuthInfoStoreFields, UserInfoStoreFields};
use crate::schemas::graphql::acl::{
    OauthClientName, SignInMutation, UserLoginsInput, UserLoginsInputFields,
};
use cynic::{MutationBuilder, http::ReqwestExt};

#[island]
pub fn SignIn() -> impl IntoView {
    let login_form_ref = NodeRef::<Form>::new();
    let current_state = expect_context::<Store<AppStateContext>>();

    let query = use_query::<AuthCode>();
    Effect::new(move || {
        let navigate = use_navigate();

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

                        navigate("/dashboard", Default::default());
                    };
                });
            }
            None => {}
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

    view! {
        <Title text="Sign In"/>

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

                        <ReactiveForm form_ref=login_form_ref ext_styles="w-full max-w-md">
                            <div class="mb-6">
                                <InputField
                                    label="Email/Username"
                                    field_type=InputFieldType::Text
                                    name="username"
                                    required=true
                                    placeholder="Enter your email or username"
                                    id_attr="username"
                                    ext_wrapper_styles="mb-4"
                                    ext_label_styles="block text-gray-700 text-sm font-bold mb-2"
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
                                    ext_wrapper_styles="mb-4"
                                    ext_label_styles="block text-gray-700 text-sm font-bold mb-2"
                                    ext_input_styles="focus:ring-secondary"
                                    autocomplete="on"
                                />
                            </div>
                            <div class="flex items-center justify-between mb-2">
                                // <p class="text-sm text-red-500">{ (*error).clone() }</p>
                            </div>
                            <div class="flex items-center justify-between mb-6">
                                <a class="text-sm text-blue-500 hover:text-blue-700" href="#">{ "Forgot Password?" }</a>
                            </div>
                            <BasicButton
                                button_text="Sign In"
                                style_ext="bg-primary text-white px-4 py-2 hover:bg-secondary transition duration-300 ease-in-out hover:shadow-md hover:-translate-y-1 hover:z-10 text-white w-full"
                                // disabled={!*login_form_is_valid}
                                icon_before=true // if you have an icon before the button text, set it to true
                            />
                            <div class="flex items-center justify-center mt-6 text-sm text-blue-500 hover:text-blue-400">
                                <A href="/signup">"Don't have an account? Sign up"</A>
                            </div>
                        </ReactiveForm>
        </div>
    }
}
