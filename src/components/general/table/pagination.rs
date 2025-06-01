use icondata as IconId;
use leptos::prelude::*;
use std::collections::HashMap;

use crate::components::general::{
    button::{BasicButton, ButtonGroup},
    table::data_table::TableCellData,
};

#[component]
pub fn Pagination(
    /// pagination_state: (current_page, total_pages, current_data)
    pagination_state: Memo<(usize, usize, Vec<HashMap<String, TableCellData>>)>,
    on_page_change: Callback<usize>,
) -> impl IntoView {
    let current_page = Memo::new(move |_| pagination_state.get().0);
    let total_pages = Memo::new(move |_| pagination_state.get().1);

    let next_page = Memo::new(move |_| {
        if current_page.get() < total_pages.get() {
            Some(current_page.get() + 1)
        } else {
            None
        }
    });

    let prev_page = Memo::new(move |_| {
        if current_page.get() > 1 {
            Some(current_page.get() - 1)
        } else {
            None
        }
    });

    let on_prev_click = Callback::new(move |_| {
        let page = prev_page.get().unwrap_or(current_page.get());
        on_page_change.run(page);
    });

    let on_next_click = Callback::new(move |_| {
        let page = next_page.get().unwrap_or(current_page.get());
        on_page_change.run(page);
    });

    let on_first_click = Callback::new(move |_| {
        on_page_change.run(1);
    });

    let on_last_click = Callback::new(move |_| {
        on_page_change.run(total_pages.get());
    });

    let is_first_page = Memo::new(move |_| current_page.get() <= 1);
    let is_last_page = Memo::new(move |_| current_page.get() >= total_pages.get());
    let can_go_to_prev = Memo::new(move |_| current_page.get() == 1);
    let can_go_to_next =
        Memo::new(move |_| total_pages.get() <= 1 || current_page.get() == total_pages.get());

    view! {
        <div class="flex flex-col">
            <div class="flex items-center justify-end">
                <span class="text-xs mr-2">
                    {move || format!("{} of {}", current_page.get(), pagination_state.get().1)}
                </span>
                <ButtonGroup style_ext="font-bold bg-primary text-white hover:bg-secondary".to_string()>
                    <BasicButton onclick=on_first_click disabled=is_first_page icon=Some(IconId::BsChevronBarLeft) />
                    <BasicButton onclick=on_prev_click disabled=can_go_to_prev icon=Some(IconId::BsChevronLeft) />
                    <BasicButton onclick=on_next_click disabled=can_go_to_next icon=Some(IconId::BsChevronRight) />
                    <BasicButton onclick=on_last_click disabled=is_last_page icon=Some(IconId::BsChevronBarRight) />
                </ButtonGroup>
            </div>
        </div>
    }
}
