use crate::components::forms::reactive_form::ReactiveForm;
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

/// This component is used to create a stepper UI.
/// It allows users to navigate through a series of steps, each containing a form.
/// The stepper provides navigation buttons to move between steps and a final button to submit the entire form.
/// Form are validated as long as fields use the required attribute.
/// Example usage:
/// ```
/// // Use this to track form references changes in order to handle form submission
/// let stepper_form_refs = RwSignal::new(Vec::new());
///
/// let handle_received_form_refs = Callback::new(move |form_refs: Vec<NodeRef<Form>>| {
///     stepper_form_refs.update(|prev| *prev = form_refs);
/// });
/// <Stepper step_labels=RwSignal::new(vec![StepInfo::new("First", Some(IconId::AiFileAddOutlined)), StepInfo::new("Second", None), StepInfo::new("Third", None)]) send_all_form_refs=handle_received_form_refs is_linear=true final_button_text="Finish">
///    <Step>
///        <p>"First step"</p>
///        <InputField field_type=InputFieldType::Text name="user_name" label="Username" required=true />
///        <InputField field_type=InputFieldType::Email name="email" label="Email" autocomplete="on" required=true />
///
///        <SelectInput
///           initial_value=""
///           label="Time Zone"
///           name="timezone"
///           required=true
///           options=vec![
///               SelectOption::new("", "--Select Timezone"),
///               SelectOption::new("utc", "UTC"),
///               SelectOption::new("est", "EST"),
///           ]
///        />
///        <RadioInputField required=true label="Male" name="gender" initial_value="male" id_attr="male" />
///        <RadioInputField required=true label="Female" name="gender" initial_value="female" id_attr="female" />
///        <Textarea
///            initial_value="Initial text"
///            label="Description"
///            name="description"
///            required=true
///            placeholder="Enter your description..."
///            ext_input_styles="bg-gray-100"
///        />
///    </Step>
///    <Step>
///        <p>"Second step"</p>
///        <InputField field_type=InputFieldType::Text name="first_name" label="First Name" required=true />
///        <ToggleSwitch
///            label="Accept Terms of Service"
///            active=accepted
///            name="tos"
///            id_attr="tos-step2"
///            required=true
///        />
///        <DatePicker id_attr="step2_dob" name="dob" label="Date of Birth" required=true />
///    </Step>
///    <Step>
///        <p>"Third step"</p>
///        { move || {
///            if let Some(first_form_ref) = stepper_form_refs.get().get(0) {
///                let form_data = get_form_data_from_form_ref(first_form_ref).unwrap_or_default();
///                let data = deserialize_form_data_to_struct::<FirstForm>(&form_data).unwrap_or_default();
///                Some(view! {
///                    <h2 class="text-lg">"First Step Verification"</h2>
///                    <p><strong>"Username: "</strong>{data.user_name}</p>
///                })
///            } else {
///                None
///            }
///        }
///        }
///    </Step>
/// </Stepper>
/// ```
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
        ev.stop_propagation();
        // Implement logic to show form validity
        let target = ev
            .target()
            .and_then(|t| t.dyn_into::<HtmlFormElement>().ok());

        if let Some(form) = target {
            // let form_data = FormData::new_with_form(&form).unwrap_or_default();
            let is_valid = form.check_validity();
            set_step_form_is_valid.set(is_valid);
        }
    };

    let next_is_disabled = Memo::new(move |_| !step_form_is_valid.get() && is_linear);

    // A workaround for updating the next step's form's validity state when navigating to the next step or previous step
    Effect::new(move || {
        if let Some(form_ref) = form_refs.get().get(current_step.get()) {
            if let Some(form) = form_ref.get() as Option<HtmlFormElement> {
                fire_bubbled_and_cancelable_event("submit", true, true, &form);
            }
        }
    });

    view! {
        <div class="flex flex-col items-center max-w-full">
            <div class="relative flex items-center w-full mb-4">
                // Line between steps (md+ screens only)
                <div class="hidden md:flex justify-center w-full absolute top-4">
                    <div class="w-full border-t border-mid-gray absolute z-0" />
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
                                        "relative flex items-center cursor-pointer bg-contrast-white space-x-2 px-4 mb-2 z-9 {}",
                                        if !is_current() { "hidden md:flex" } else { "" }
                                    )
                                }>
                                    <div class=move || {
                                        format!(
                                            "w-8 h-8 flex items-center justify-center rounded-full text-sm {}",
                                            if is_current() {
                                                "bg-primary text-contrast-white"
                                            } else {
                                                "bg-light-gray"
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
                                            if let Some(icon) = step_label.icon {
                                                Some(view!{ <Icon icon=icon /> })
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
                                            ""
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
            <div on:submit=handle_step_form_submit class=format!("mb-4 p-4 border border-mid-gray rounded w-full {}", ext_wrapper_styles)>
            {
                    move || {
                        let current = current_step.get();
                        children()
                            .nodes
                            .into_iter()
                            .enumerate()
                            .map(|(i, child)| {
                                if let Some(form_ref) = form_refs.get().get(i) {
                                    let dynamic_class = move || if current == i { "block" } else { "hidden" };

                                    Some(view! {
                                        <ReactiveForm form_ref=form_ref.to_owned() ext_styles=dynamic_class()>
                                            { child.into_view() }
                                        </ReactiveForm>
                                    })
                                } else {
                                    None
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
pub fn Step(children: Children) -> impl IntoView {
    view! {
        { children() }
    }
}
