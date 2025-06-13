use leptos::{html::Form, prelude::*};

use crate::utils::forms::fire_bubbled_and_cancelable_event;

#[component]
pub fn ReactiveForm(
    form_ref: NodeRef<Form>,
    #[prop(into, optional)] ext_styles: String,
    children: Children,
) -> impl IntoView {
    view! {
        <form
            node_ref=form_ref
            class=ext_styles
            on:input=move |_| {
                if let Some(form) = form_ref.get() {
                    if form.check_validity() {
                        fire_bubbled_and_cancelable_event("submit", true, true, form);
                    }
                }
            }
            on:change=move |_| {
                if let Some(form) = form_ref.get() {
                    if form.check_validity() {
                        fire_bubbled_and_cancelable_event("submit", true, true, form);
                    }
                }
            }
        >
            {children()}
        </form>
    }
}
