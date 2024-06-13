use std::ops::Deref;

use crate::{
    components::{
        bottom_svg::BottomSvg,
        input::InputField,
        modal::basic_modal::{BasicModal, BasicModalProps, UseCase},
        nav::Nav,
        select::{SelectInput, SelectOption},
        textarea::TextArea,
    },
    data::{
        context::user_resources::send_message,
        models::user::{Message, Subject},
    },
};

use web_sys::{HtmlInputElement, HtmlSelectElement, HtmlTextAreaElement};
use yew::prelude::*;

#[function_component(HireMe)]
pub fn hire_me() -> Html {
    let subject_ref = use_node_ref();

    let modal_is_open = use_state(|| false as bool);
    let modal_data = use_state(|| BasicModalProps::default());
    let modal_data_rsx = modal_data.clone();
    let modal_is_open_rsx = modal_is_open.clone();
    let contact_form = use_state(|| Message::default());
    let contact_form_clone_rsx = contact_form.clone();
    let send_button_disabled = use_state(|| false);
    let send_button_disabled_rsx = send_button_disabled.clone();

    let oninput = {
        let contact_form_clone = contact_form.clone();
        Callback::from(move |event: InputEvent| {
            let target = event.target_dyn_into::<HtmlInputElement>();

            match target {
                Some(target) => match target.name().as_str() {
                    "full_name" => contact_form_clone.set(Message {
                        sender_name: Some(target.value()),
                        ..contact_form_clone.deref().clone()
                    }),
                    "email" => contact_form_clone.set(Message {
                        sender_email: Some(target.value()),
                        ..contact_form_clone.deref().clone()
                    }),

                    _ => {}
                },
                None => {
                    let target = event.target_dyn_into::<HtmlTextAreaElement>();
                    match target {
                        Some(target) => match target.name().as_str() {
                            "body" => contact_form_clone.set(Message {
                                body: Some(target.value()),
                                ..contact_form_clone.deref().clone()
                            }),
                            _ => {}
                        },
                        None => {}
                    }
                }
            }
        })
    };

    let onchange = {
        let contact_form_clone = contact_form.clone();
        Callback::from(move |event: Event| {
            let target = event.target_dyn_into::<HtmlSelectElement>();

            match target {
                Some(target) => match target.name().as_str() {
                    "subject" => contact_form_clone.set(Message {
                        subject: Some(target.value()),
                        ..contact_form_clone.deref().clone()
                    }),
                    _ => {}
                },
                None => {}
            }
        })
    };

    let on_submit = {
        let contact_form_clone_submit = contact_form.clone();
        Callback::from(move |event: SubmitEvent| {
            event.prevent_default();

            let modal_is_open_clone = modal_is_open.clone();
            let contact_form_clone_submit_f = contact_form_clone_submit.clone();
            let modal_data_clone = modal_data.clone();
            wasm_bindgen_futures::spawn_local(async move {
                let response = send_message(contact_form_clone_submit_f.deref().clone()).await;
                let modal_data_update = match response {
                    Ok(_) => BasicModalProps {
                        title: "Message Sent".to_string(),
                        use_case: UseCase::Success,
                        is_open: true,
                        ..BasicModalProps::default()
                    },
                    Err(_) => BasicModalProps {
                        title: "Message Failed to Send".to_string(),
                        use_case: UseCase::Error,
                        ..BasicModalProps::default()
                    },
                    
                };



                modal_data_clone.set(modal_data_update);
                modal_is_open_clone.set(true);
                contact_form_clone_submit_f.set(Message::default());
            });
        })
    };

    let subject_options = Subject::as_vec()
        .iter()
        .map(|subject| Subject::fmt(subject))
        .collect::<Vec<SelectOption>>();

    let contact_form_clone_deps = contact_form.clone();
    use_effect_with_deps(
        move |_| {
            // set disabled to true if any of the fields are empty
            if contact_form_clone_deps.deref().sender_name.is_none()
                || contact_form_clone_deps.deref().sender_email.is_none()
                || contact_form_clone_deps.deref().body.is_none()
                || contact_form_clone_deps.deref().subject.is_none()
                || contact_form_clone_deps.deref().sender_name.as_ref().unwrap().is_empty()
                || contact_form_clone_deps.deref().sender_email.as_ref().unwrap().is_empty()
                || contact_form_clone_deps.deref().body.as_ref().unwrap().is_empty()
                || contact_form_clone_deps.deref().subject.as_ref().unwrap().is_empty()
            {
                send_button_disabled.set(true);
            } else {
                send_button_disabled.set(false);
            }
        },
        contact_form.clone(),
    );

    html! {
        <>
            <header>
                <Nav />
            </header>
            <main class="hire-wrapper">
                <BasicModal title={modal_data_rsx.title.clone()} is_open={*modal_is_open_rsx} use_case={modal_data_rsx.use_case.clone()} />
                <div class="hire-form">
                    <h2 class="heading">{ "Contact Me" }</h2>
                    <form onsubmit={on_submit}>
                            <SelectInput onchange={onchange.clone()} initial_value={contact_form_clone_rsx.subject.clone()} required={true} input_node_ref={subject_ref} label={"Subject"} options={subject_options} name={"subject"} />
                            <InputField oninput={oninput.clone()} initial_value={contact_form_clone_rsx.sender_name.clone()} required={true} placeholder={"Your Full Name"} label={"Full Name"} field_type={"text"} name={"full_name"} />
                            <InputField oninput={oninput.clone()} initial_value={contact_form_clone_rsx.sender_email.clone()} required={true} placeholder={"Your Email"} label={"Email"} field_type={"email"} name={"email"} />
                            <TextArea oninput={oninput.clone()} initial_value={contact_form_clone_rsx.body.clone()} required={true} placeholder={"Your Message..."} label={"Message"} name={"body"} />
                            <button type="submit" disabled={*send_button_disabled_rsx} class="form-group-button button button-primary">{ "Send" }</button>
                        </form>
                </div>
                <BottomSvg />
            </main>
        </>
    }
}
