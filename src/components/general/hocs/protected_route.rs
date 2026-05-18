use crate::data::{
    context::store::{AppStateContext, AppStateContextStoreFields},
    models::general::acl::{AuthInfoStoreFields, UserInfoStoreFields},
};
use leptos::prelude::*;
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
    let token = store.user().auth_info().token();

    // Wait for auth check to complete, then gate
    let is_authenticated = Memo::new(move |_| !token.get().is_empty());

    Effect::new(move |_| {
        if
        /* auth check is done and */
        !is_authenticated.get() {
            navigate("/sign-in", Default::default());
        }
    });

    view! {
        {move || is_authenticated.get().then(|| children().into_view())}
    }
}
