use std::collections::HashMap;

use leptos::{prelude::*, task::spawn_local};
use leptos_meta::*;
use reactive_stores::Store;

use crate::{
    components::molecules::{headline::Headline, ratecard::RatecardComponent, top_nav::TopNav},
    data::{
        context::{
            shared::fetch_ratecards,
            store::{AppStateContext, AppStateContextStoreFields},
        },
        models::general::acl::{AuthInfoStoreFields, UserInfoStoreFields},
    },
};

#[component]
pub fn Ratecard() -> impl IntoView {
    let store = expect_context::<Store<AppStateContext>>();
    let ratecards = move || store.ratecards();

    Effect::new(move || {
        spawn_local(async move {
            let mut headers = HashMap::new() as HashMap<String, String>;
            headers.insert(
                "Authorization".into(),
                format!(
                    "Bearer {}",
                    store.user().auth_info().token().get_untracked()
                ),
            );

            let _ratecards_res = fetch_ratecards(&store, None).await;
        });
    });

    view! {
        <Title text="My Ratecard"/>
        <main>
            <div class="min-h-svh flex flex-col gap-[40px]">
                <div class="sticky top-0 z-10 bg-contrast-white dark:bg-navy">
                    <TopNav />
                </div>
                <Headline title="My Ratecard" description="How much do I charge?" />
                <div class="display-constraints flex flex-col md:flex-row md:justify-center gap-[10px]">
                    <For
                        each=move || ratecards().get()
                        key=|ratecard| ratecard.id.clone()
                        children=move |ratecard| {
                            view! {
                                <RatecardComponent
                                    name=RwSignal::new(ratecard.name.as_ref().unwrap_or(&Default::default()).clone())
                                    services=RwSignal::new(ratecard.services.as_ref().unwrap_or(&Default::default()).to_vec())
                                />
                            }
                        }
                    />
                </div>
            </div>
        </main>
    }
}
