use leptos::prelude::*;
use leptos_meta::*;

use crate::components::general::breadcrumbs::Breadcrumbs;

#[island]
pub fn CreatePortfolio() -> impl IntoView {
    view! {
        <>
            <Title text="Create Portfolio"/>

            <div class="mx-[20px]">
                <Breadcrumbs custom_route_names=["Home", "Create Portfolio"] />
            </div>


        </>
    }
}
