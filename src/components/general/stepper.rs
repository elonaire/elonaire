use crate::components::general::button::BasicButton;
use crate::utils::forms::fire_bubbled_and_cancelable_event;
use icondata::Icon as IconId;
use leptos::html::Form;
use leptos::prelude::*;
use leptos::wasm_bindgen::JsCast;
use leptos_icons::Icon;
use web_sys::HtmlFormElement;
use web_sys::SubmitEvent;

#[derive(Clone, Debug, Default)]
pub struct StepInfo {
    pub label: String,
    pub icon: Option<IconId>,
}

impl StepInfo {
    pub fn new(label: &str, icon: Option<IconId>) -> Self {
        StepInfo {
            label: label.to_string(),
            icon,
        }
    }
}

// Stepper Component
#[component]
pub fn Stepper(
    mut children: ChildrenFragmentMut, // Children passed as a function
    #[prop(into, optional)] final_button_text: String,
    #[prop(optional, default = Callback::new(|_| {}))] on_click_final_button: Callback<
        Vec<NodeRef<Form>>,
    >,
    #[prop(into)] step_labels: RwSignal<Vec<StepInfo>>,
    #[prop(optional, default = false)] is_linear: bool,
    #[prop(optional, default = Callback::new(|_| {}))] send_all_form_refs: Callback<
        Vec<NodeRef<Form>>,
    >,
    #[prop(into, optional)] ext_wrapper_styles: String,
) -> impl IntoView {
    let (current_step, set_current_step) = signal(0); // Leptos signal for state
    let (step_form_is_valid, set_step_form_is_valid) = signal(false); // Leptos signal for state
    let step_count = children().nodes.collect_view().len(); // Get number of children
    let form_refs = RwSignal::new(
        (0..step_count)
            .map(|_| NodeRef::<Form>::new())
            .collect::<Vec<_>>(),
    );

    let onclick_next = Callback::new(move |_| {
        if current_step.get() < step_count - 1 {
            set_current_step.update(|step| *step += 1);
        }

        // if second last, send all form_refs to parent in a callback
        if current_step.get() == step_count - 1 {
            let form_refs = form_refs.get();
            send_all_form_refs.run(form_refs);
        }
    });

    let onclick_prev = Callback::new(move |_| {
        if current_step.get() > 0 {
            set_current_step.update(|step| *step -= 1);
        }
    });

    let handle_final_button_click = Callback::new(move |_| {
        let form_refs = form_refs.get();
        on_click_final_button.run(form_refs);
    });

    let handle_step_form_submit = move |ev: SubmitEvent| {
        ev.prevent_default();
        // Implement logic to show form validity
        let target = ev
            .target()
            .and_then(|t| t.dyn_into::<HtmlFormElement>().ok());

        if let Some(form) = target {
            // let form_data = FormData::new_with_form(&form).unwrap();
            let is_valid = form.check_validity();
            set_step_form_is_valid.set(is_valid);
        }
    };

    let next_is_disabled = Memo::new(move |_| !step_form_is_valid.get() && is_linear);

    // A workaround for updating the next step's form's validity state when navigating to the next step or previous step
    Effect::new(move || {
        let form_ref = form_refs.get().get(current_step.get()).unwrap().to_owned();

        if let Some(form) = form_ref.get() as Option<HtmlFormElement> {
            fire_bubbled_and_cancelable_event("submit", true, true, form);
        }
    });

    view! {
        <div class="flex flex-col items-center max-w-full">
            <div class="relative flex items-center w-full mb-4">
                // Line between steps (md+ screens only)
                <div class="hidden md:flex justify-center w-full absolute top-4">
                    <div class="w-full border-t border-gray-300 absolute z-0" />
                </div>
                <div class="relative flex flex-wrap md:flex-nowrap justify-center md:justify-between w-full md:space-x-2">
                    <For
                        each=move || step_labels.get().into_iter().enumerate()
                        key=|(index, _)| *index
                        let:((index, step_label))
                    >
                        {
                            let is_current = move || index == current_step.get();
                            view! {
                                <div on:click=move |_| {
                                    if next_is_disabled.get() {
                                        return;
                                    }
                                    set_current_step.update(|step| *step = index);
                                } class=move || {
                                    format!(
                                        "relative flex items-center cursor-pointer bg-white space-x-2 px-4 mb-2 z-9 {}",
                                        if !is_current() { "hidden md:flex" } else { "" }
                                    )
                                }>
                                    <div class=move || {
                                        format!(
                                            "w-8 h-8 flex items-center justify-center rounded-full text-sm {}",
                                            if is_current() {
                                                "bg-primary text-white"
                                            } else {
                                                "bg-gray-200 text-gray-800"
                                            }
                                        )
                                    }>
                                        {
                                            if step_label.icon.is_none() {
                                                Some(index + 1)
                                            } else {
                                                None
                                            }
                                        }
                                        {
                                            if step_label.icon.is_some() {
                                                Some(view!{ <Icon icon={step_label.icon.unwrap()} /> })
                                            } else {
                                                None
                                            }
                                        }
                                    </div>
                                    <div class=move || {format!(
                                        "text-sm {}",
                                        if is_current() {
                                            "font-bold text-primary"
                                        } else {
                                            "text-gray-800"
                                        }
                                    )}>
                                        { step_label.label.clone() }
                                    </div>
                                </div>
                            }
                        }
                    </For>
                </div>
            </div>
            <div on:submit=handle_step_form_submit class=format!("mb-4 p-4 border border-gray-300 rounded w-full {}", ext_wrapper_styles)>
            {
                    move || {
                        let current = current_step.get();
                        children()
                            .nodes
                            .into_iter()
                            .enumerate()
                            .map(|(i, child)| {
                                let form_ref = form_refs.get().get(i).unwrap().to_owned();

                                view! {
                                    <form
                                        node_ref=form_ref
                                        class=move || if current == i { "block" } else { "hidden" }
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
                                        { child.into_view() }
                                    </form>
                                }
                            }).collect_view()
                    }
                }
            </div>
            <div class="flex w-full justify-start gap-4">
                {
                    move || if current_step.get() == 0 {
                        None
                    } else {
                        Some(view! {
                            <BasicButton
                                onclick=onclick_prev
                                button_text="Previous"
                            />
                        })
                    }
                }
                {
                    move || if current_step.get() == step_count - 1 {
                        view! {
                            <BasicButton
                                onclick=handle_final_button_click
                                button_text=final_button_text.clone()
                            />
                        }
                    } else {
                        view! {
                            <BasicButton
                                disabled=next_is_disabled
                                onclick=onclick_next
                                button_text="Next"
                            />
                        }
                    }
                }
            </div>
        </div>
    }
}

// Step Component
#[component]
pub fn Step(children: ChildrenFn) -> impl IntoView {
    view! {

            { children().into_view() }

    }
}
