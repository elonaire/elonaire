use leptos::prelude::*;
use leptos_meta::*;
use leptos_router::{
    StaticSegment,
    components::{Route, Router, Routes},
};
use reactive_stores::Store;

use crate::views::home::Home;
use crate::{schemas::general::acl::AppStateContext, views::login::SignIn};

#[component]
pub fn App() -> impl IntoView {
    provide_context(Store::new(AppStateContext::default()));
    provide_meta_context();

    view! {
        // <Link rel="shortcut icon" type_="image/ico" href="/favicon.ico"/>
        <div id="modal-root"></div>
        <Router>
            <Routes fallback=|| "Page not found.">
                // <Route path=StaticSegment("") view=|| view! { <ProtectedRoute><Home /></ProtectedRoute> } />
                <Route path=StaticSegment("") view=Home />
                <Route path=StaticSegment("/sign-in") view=SignIn/>
            </Routes>
        </Router>
    }
}
