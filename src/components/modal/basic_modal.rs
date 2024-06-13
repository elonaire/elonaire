use yew::prelude::*;
use yew_icons::{Icon, IconId};
use crate::components::{modal::precursor_modal::PrecursorModal, line_separator::LineSeparator};

#[derive(Properties, PartialEq, Default, Debug, Clone)]
pub struct BasicModalProps {
    pub title: String,
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub use_case: UseCase,
    #[prop_or_default]
    pub on_close: Callback<()>,
    #[prop_or_default]
    pub on_confirm: Callback<()>,
    #[prop_or_default]
    pub on_cancel: Callback<()>,
    #[prop_or_default]
    pub is_open: bool,
}

#[derive(Default, PartialEq, Clone, Copy, Debug)]
pub enum UseCase {
    Error,
    Success,
    Confirmation,
    Info,
    #[default]
    General,
}

#[function_component]
pub fn BasicModal(props: &BasicModalProps) -> Html {

    let on_click_ok = {
        let on_close = props.on_close.clone();
        Callback::from(move |_: MouseEvent| {
            on_close.emit(());
        })
    };
    
    if props.is_open {
        html! {
            <div class="backdrop">
            <div id="modal_host">
            <div class="modal-header">
                {
                    match &props.use_case {
                        UseCase::Error => html!{ <span class="error"><Icon icon_id={IconId::BootstrapExclamationCircleFill} width={"1.5em".to_owned()} height={"1.5em".to_owned()} /></span> },
                        UseCase::Success => html!{ <span class="success"><Icon icon_id={IconId::BootstrapCheckCircleFill} width={"1.5em".to_owned()} height={"1.5em".to_owned()} /></span> },
                        UseCase::Info => html!{ <span class="info"><Icon icon_id={IconId::BootstrapInfoCircleFill} width={"1.5em".to_owned()} height={"1.5em".to_owned()} /></span> },
                        UseCase::Confirmation => html!{ <span class="warning"><Icon icon_id={IconId::BootstrapQuestionCircleFill} width={"1.5em".to_owned()} height={"1.5em".to_owned()} /></span> },
                        UseCase::General => html!{ },
                    }
                
                }<span class="title-text">{ &props.title }</span>
            </div>
            <LineSeparator />
            <div id="modal-content">
                <PrecursorModal>{ props.children.clone() }</PrecursorModal>
            </div>
            <div class="modal-footer">
                { if props.use_case == UseCase::Confirmation { html!{ <button class="button button-outlined">{"Cancel"}</button> } } else { html!() } }
                <button onclick={on_click_ok} class="button button-primary">{"OK"}</button>
            </div>
            </div>
        </div>}
    } else {
        html! {  }
    }
}