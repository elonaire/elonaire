use crate::schemas::general::acl::{AppStateContext, AppStateContextStoreFields};
use leptos::prelude::*;
use leptos_router::hooks::use_navigate;
use reactive_stores::Store;

#[component]
/// This component is a wrapper around the `children` prop. It checks if the user is authenticated.
pub fn ProtectedRoute(mut children: ChildrenFnMut) -> impl IntoView {
    let current_state = expect_context::<Store<AppStateContext>>();
    let user = move || current_state.user(); // Should return ReadSignal<UserInfo>
    let navigate = use_navigate();

    // Effect to handle navigation based on auth status
    Effect::new(move |_| {
        if user().get().auth_info.token.is_empty() {
            // User is not authenticated, redirect to sign-in
            navigate("/sign-in", Default::default());
        }
    });

    view! {
        <>
            {move || {
                if !user().get().auth_info.token.is_empty() {
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
