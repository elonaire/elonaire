use leptos::prelude::*;
use leptos_meta::*;

use crate::components::general::{
    breadcrumbs::Breadcrumbs,
    table::data_table::{Column, DataTable},
};

#[island]
pub fn Portfolio() -> impl IntoView {
    let table_data = RwSignal::new((
        vec![
            Column::new("Title", false),
            Column::new("Description", true),
            Column::new("Start Date", true),
            Column::new("End Date", true),
        ],
        vec![],
    ));

    view! {
        <>
            <Title text="My Portfolio"/>
            <div class="mx-[20px]">
                <Breadcrumbs custom_route_names=["Home", "Dashboard", "Portfolio"] />
            </div>

            <h1 class="mx-[20px]">Portfolio</h1>

            <div class="mx-[20px]">
                <DataTable data=table_data editable=true deletable=true />
            </div>
        </>
    }
}
