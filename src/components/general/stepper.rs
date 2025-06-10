use crate::components::general::button::BasicButton;
use icondata::Icon as IconId;
use leptos::ev;
use leptos::prelude::*;
// use leptos::wasm_bindgen::JsCast;
use leptos_icons::Icon;
use web_sys::CustomEvent;
use web_sys::FormData;
use web_sys::wasm_bindgen::JsValue;
// use web_sys::{HtmlInputElement, HtmlSelectElement, HtmlTextAreaElement};

#[derive(Clone, Debug, Default)]
pub struct StepperLabel {
    pub label: String,
    pub icon: Option<IconId>,
}

impl StepperLabel {
    pub fn new(label: &str, icon: Option<IconId>) -> Self {
        StepperLabel {
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
        ev::MouseEvent,
    >,
    #[prop(into)] step_labels: Vec<StepperLabel>,
    #[prop(optional, default = false)] is_linear: bool,
    // #[prop(optional, default = RwSignal::new(false))] can_proceed: RwSignal<bool>,
) -> impl IntoView {
    let (current_step, set_current_step) = signal(0); // Leptos signal for state
    let (step_form_is_valid, set_step_form_is_valid) = signal(false); // Leptos signal for state
    let step_count = children().nodes.collect_view().len(); // Get number of children

    let onclick_next = Callback::new(move |_| {
        if current_step.get() < step_count - 1 {
            set_current_step.update(|step| *step += 1);
        }
    });

    let onclick_prev = Callback::new(move |_| {
        if current_step.get() > 0 {
            set_current_step.update(|step| *step -= 1);
        }
    });

    let show_form_validity = move |ev: CustomEvent| {
        // Implement logic to show form validity
        leptos::logging::log!("Form validity shown: {:?}", ev.detail().as_bool().unwrap());
        set_step_form_is_valid.set(ev.detail().as_bool().unwrap());
    };

    let next_is_disabled = Memo::new(move |_| !step_form_is_valid.get() && is_linear);

    view! {
        <div class="flex flex-col items-center max-w-full">
            <div class="relative flex items-center w-full mb-4">
                // Line between steps (md+ screens only)
                <div class="hidden md:flex justify-center w-full absolute top-4">
                    <div class="w-full border-t border-gray-300 absolute z-0" />
                </div>
                <div class="relative flex flex-wrap md:flex-nowrap justify-center md:justify-between w-full md:space-x-2">
                    <For
                        each=move || step_labels.clone().into_iter().enumerate()
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
            <div on:show_form_validity=show_form_validity class="mb-4 p-4 border border-gray-300 rounded w-full">
            {
                move || children()
                .nodes
                .into_iter()
                .nth(current_step.get())
                .map(|child| child.into_view())
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
                                button_text="Previous".to_string()
                            />
                        })
                    }
                }
                {
                    move || if current_step.get() == step_count - 1 {
                        view! {
                            <BasicButton
                                onclick=on_click_final_button
                                button_text=final_button_text.clone()
                            />
                        }
                    } else {
                        view! {
                            <BasicButton
                                disabled=next_is_disabled
                                onclick=onclick_next
                                button_text="Next".to_string()
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
pub fn Step(
    children: ChildrenFn,
    // #[prop(optional, default = Callback::new(|_| {}))] form_validity: Callback<bool>,
) -> impl IntoView {
    let form_ref = NodeRef::new();

    view! {
        <form
        node_ref=form_ref
        on:input=move |_| {
            // leptos::logging::log!("input event: {:?}", e.target());
            // let target = e.target().and_then(|t| t.dyn_into::<HtmlInputElement>().ok());
            // if let Some(input) = target {
            //     let value = input.value();
            //     leptos::logging::log!("input value: {:?}", value);
            //     // on_input.run(e)
            // } else {
            //     // Might be a HTMLTextAreaElement
            //     let target = e.target().and_then(|t| t.dyn_into::<HtmlTextAreaElement>().ok());

            //     if let Some(textarea) = target {
            //         let value = textarea.value();
            //         leptos::logging::log!("textarea value: {:?}", value);
            //         // on_input.run(e)
            //     }
            // }

            if let Some(form) = form_ref.get() {
                let valid = form.check_validity();
                if valid {
                    let _form_data = FormData::new_with_form(&form).unwrap();
                    // leptos::logging::log!("gender value: {:?}", form_data.get("gender").as_string());
                };
                let _event = match CustomEvent::new("show_form_validity") {
                    Ok(ev) => {
                        ev.init_custom_event_with_can_bubble_and_cancelable_and_detail("show_form_validity", true, true, &JsValue::from_bool(valid));
                        form.dispatch_event(&ev).unwrap();
                    }
                    Err(_e) => {}
                };
            }

        }
        on:change=move |_| {
            // leptos::logging::log!("change event: {:?}", e);
            // let target = e.target().and_then(|t| t.dyn_into::<HtmlSelectElement>().ok());

            // if let Some(select) = target {
            //     let value = select.value();
            //     leptos::logging::log!("change value: {:?}", value);
            //     // on_input.run(e)
            // } else {
            //     let target = e.target().and_then(|t| t.dyn_into::<HtmlInputElement>().ok());

            //     if let Some(input) = target {
            //         let value = input.value();
            //         leptos::logging::log!("input value: {:?}", value);
            //         // on_input.run(e)
            //     }
            // }

            if let Some(form) = form_ref.get() {
                let valid = form.check_validity();
                if valid {
                    let _form_data = FormData::new_with_form(&form).unwrap();
                    // leptos::logging::log!("gender value: {:?}", form_data.get("gender").as_string());
                };

                let _event = match CustomEvent::new("show_form_validity") {
                    Ok(ev) => {
                        ev.init_custom_event_with_can_bubble_and_cancelable_and_detail("show_form_validity", true, true, &JsValue::from_bool(valid));
                        form.dispatch_event(&ev).unwrap();
                    }
                    Err(_e) => {}
                };
            }
        }
        >
            { children().into_view() }
        </form>
    }
}
