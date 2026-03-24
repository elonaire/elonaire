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
    let (current_error, set_current_error) = signal(None);

    Effect::new(move |_| {
        if let Some(err) = store.error().get() {
            match &err.extensions {
                Some(extensions) if extensions.contains_key("status") => {
                    if let Some(status) = extensions.get("status") {
                        match status.as_str() {
                            "401" => {
                                navigate("/sign-in", Default::default());
                            }
                            "500" => {
                                navigate("/500", Default::default());
                            }
                            _ => {
                                set_current_error.set(Some(err.message));
                                error_modal_is_open.update(|v| *v = true);
                            }
                        }
                    };
                }
                _ => {}
            }
        }
    });

    view! {
        <BasicModal title="Error" is_open=error_modal_is_open use_case=UseCase::Success disable_auto_close=false>
            <div class="p-[10px]">
                <p>{move || current_error.get()}</p>
            </div>
        </BasicModal>
    }
}
