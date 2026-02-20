use std::collections::HashMap;

use leptos::prelude::*;
use leptos::task::spawn_local;
use leptos_meta::*;
use leptos_router::components::Outlet;
use reactive_stores::Store;

use crate::components::general::spinner::Spinner;
use crate::components::general::table::data_table::TableCellData;
use crate::data::context::shared::fetch_service_requests;
use crate::{
    components::general::{
        breadcrumbs::Breadcrumbs,
        table::data_table::{Column, DataTable},
    },
    data::{
        context::store::{AppStateContext, AppStateContextStoreFields},
        models::general::acl::{AuthInfoStoreFields, UserInfoStoreFields},
    },
};

#[island]
pub fn ServiceRequests() -> impl IntoView {
    view! {
        <>
            <Outlet />
        </>
    }
}

#[island]
pub fn ServiceRequestsList() -> impl IntoView {
    let current_state = expect_context::<Store<AppStateContext>>();
    let service_requests = move || current_state.service_requests();
    let (is_loading, set_is_loading) = signal(false);

    let table_data = RwSignal::new((
        vec![
            Column::new("Description", false),
            Column::new("Start Date", true),
        ],
        vec![],
    ));

    Effect::new(move || {
        set_is_loading.set(true);
        spawn_local(async move {
            let mut headers = HashMap::new() as HashMap<String, String>;
            headers.insert(
                "Authorization".into(),
                format!(
                    "Bearer {}",
                    current_state.user().auth_info().token().get_untracked()
                ),
            );

            let _response = fetch_service_requests(&current_state, Some(&headers)).await;

            set_is_loading.set(false);
        });
    });

    Effect::new(move || {
        let service_requests_rows: Vec<HashMap<String, TableCellData>> = service_requests()
            .get()
            .iter()
            .map(|service_request| {
                let mut hash_map_data = HashMap::new();

                // This id is the unique identifier of the table row. and is a MUST for the table to function properly.
                // *Note:* The id is a MUST for the table to function properly. You might be forced to generate a unique id for each row if your data does not have a unique identifier.
                hash_map_data.insert(
                    "id".into(),
                    TableCellData::String(service_request.id.as_ref().unwrap().to_owned()),
                );

                hash_map_data.insert(
                    "Description".into(),
                    TableCellData::String(service_request.description.as_ref().unwrap().to_owned()),
                );

                hash_map_data.insert(
                    "Start Date".into(),
                    TableCellData::DateTime(
                        service_request.start_date.as_ref().unwrap().to_owned(),
                    ),
                );
                hash_map_data
            })
            .collect();

        table_data.update(move |prev| {
            prev.1 = service_requests_rows;
        });
    });

    view! {
        <>
            <Title text="Service Requests"/>
            <div class="display-constraints">
                <Breadcrumbs custom_route_names=["Home", "Dashboard", "Service Requests"] />
            </div>
            <Show when=move || is_loading.get()>
                <Spinner />
            </Show>

            <h1 class="display-constraints">Service Requests</h1>

            <div class="display-constraints">
                <DataTable data=table_data editable=true deletable=true />
            </div>
        </>
    }
}
