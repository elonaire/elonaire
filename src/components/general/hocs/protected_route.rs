use std::collections::HashMap;

use crate::data::{
    context::{
        shared::{check_auth, fetch_single_user},
        store::{AppStateContext, AppStateContextStoreFields},
    },
    models::{
        general::acl::{AuthInfo, AuthInfoStoreFields, UserInfoStoreFields},
        graphql::acl::FetchSingleUserVars,
    },
};
use leptos::{prelude::*, task::spawn_local};
use leptos_router::hooks::use_navigate;
use reactive_stores::Store;

/// This is a higher-order component that returns a view if the user is authenticated or redirected to the sign-in page if not.
/// The component is a wrapper around the `children` prop.
/// Example usage:
/// ```
/// <Route path=StaticSegment("") view=|| view! { <ProtectedRoute><Home /></ProtectedRoute> } />
/// ```
#[component]
pub fn ProtectedRoute(children: ChildrenFn) -> impl IntoView {
    let store = expect_context::<Store<AppStateContext>>();
    let navigate = use_navigate();
    let (is_authenticated, set_is_authenticated) = signal(false);

    // Effect to handle navigation based on auth status
    Effect::new(move |_| {
        let navigate = navigate.clone();
        let store = store.clone();
        spawn_local(async move {
            let mut headers = HashMap::new() as HashMap<String, String>;
            headers.insert(
                "Authorization".into(),
                format!(
                    "Bearer {}",
                    store.user().auth_info().token().get_untracked()
                ),
            );

            let check_auth = check_auth(Some(&headers)).await;

            match check_auth {
                Ok(auth) => {
                    set_is_authenticated.set(true);
                    store.user().auth_info().set(AuthInfo {
                        token: auth
                            .new_access_token
                            .as_ref()
                            .unwrap_or(&String::new())
                            .to_owned(),
                        current_role: auth.current_role.clone(),
                        current_role_permissions: auth.current_role_permissions,
                    });
                    let user_id_vars = FetchSingleUserVars {
                        user_id: auth.sub.clone(),
                    };

                    let fetch_user_info_query = r#"
                        query FetchSingleUser($userId: String!) {
                            fetchSingleUser(userId: $userId) {
                                data {
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
                                    requestId
                                    newAccessToken
                                }
                            }
                        }
                       "#;

                    if let Ok(user_profile) =
                        fetch_single_user(&user_id_vars, None, fetch_user_info_query).await
                    {
                        store.user().user_profile().set(user_profile);
                    };
                }
                Err(_) => {
                    // User is not authenticated, and server failed to verify session.
                    navigate("/sign-in", Default::default());
                }
            }
        });
    });

    view! {
        <>
            {move || {
                if is_authenticated.get() {
                    // User is authenticated, render children
                    Some(children().into_view())
                } else {
                    // User is not authenticated, render nothing (navigation happens in effect)
                    None
                }
            }}
        </>
    }
}
