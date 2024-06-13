use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct ModalProps {
    #[prop_or_default]
    pub children: Children,
}

#[function_component]
pub fn PrecursorModal(props: &ModalProps) -> Html {
    let modal_host = gloo::utils::document()
        .get_element_by_id("modal-content")
        .expect("Expected to find a #modal-content element");

    create_portal(
        html!{ {for props.children.iter()} },
        modal_host.into(),
    )
}