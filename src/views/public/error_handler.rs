use leptos::prelude::*;
use leptos_router::hooks::use_navigate;
use reactive_stores::Store;

use crate::{
    components::general::modal::modal::{BasicModal, UseCase},
    data::context::store::{AppStateContext, AppStateContextStoreFields},
};

#[component]
pub fn ErrorHandler() -> impl IntoView {
    let store = expect_context::<Store<AppStateContext>>();
    let navigate = use_navigate();
    let error_modal_is_open = RwSignal::new(false);
    let (current_error, set_current_error) = signal(None::<String>);

    Effect::new(move |_| {
        if let Some(err) = store.error().get() {
            if err.is_unauthorized() {
                store.error().set(None);
                navigate("/sign-in", Default::default());
            } else if err.is_internal() {
                store.error().set(None);
                navigate("/500", Default::default());
            } else {
                set_current_error.set(Some(err.message().to_owned()));
                error_modal_is_open.set(true);
                store.error().set(None);
            }
        }
    });

    view! {
        <BasicModal title="Error" is_open=error_modal_is_open use_case=UseCase::Error disable_auto_close=false>
            <div class="p-[10px]">
                <p>{move || current_error.get()}</p>
            </div>
        </BasicModal>
    }
}
