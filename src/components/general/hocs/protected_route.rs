use std::collections::HashMap;

use crate::data::{
    context::{
        shared::check_auth,
        store::{AppStateContext, AppStateContextStoreFields},
    },
    models::general::acl::{AuthInfoStoreFields, UserInfoStoreFields},
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
    let current_state = expect_context::<Store<AppStateContext>>();
    let user = move || current_state.user(); // Should return ReadSignal<UserInfo>
    let navigate = use_navigate();

    let is_authenticated = Memo::new(move |_| !user().auth_info().token().get().is_empty());

    // Effect to handle navigation based on auth status
    Effect::new(move |_| {
        if !is_authenticated.get() {
            // User is not authenticated, try checking if they have an active session
            let navigate = navigate.clone();
            let current_state = current_state.clone();
            spawn_local(async move {
                let mut headers = HashMap::new() as HashMap<String, String>;
                headers.insert(
                    "Authorization".into(),
                    format!(
                        "Bearer {}",
                        current_state.user().auth_info().token().get_untracked()
                    ),
                );

                let check_auth = check_auth(Some(&headers)).await;

                match check_auth {
                    Ok(auth) => {
                        *current_state.user().auth_info().token().write() = auth
                            .new_access_token
                            .as_ref()
                            .unwrap_or(&String::new())
                            .to_owned();
                    }
                    Err(_) => {
                        // User is not authenticated, and server failed to verify session.
                        navigate("/sign-in", Default::default());
                    }
                }
            });
        }
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
