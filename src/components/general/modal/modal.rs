use icondata as IconId;
use leptos::{control_flow::Show, ev, portal::Portal, prelude::*};
use leptos_icons::Icon;

use crate::components::general::button::BasicButton;

#[derive(Clone, PartialEq, Copy, Debug, Default, Eq)]
#[allow(dead_code)]
pub enum UseCase {
    Error,
    Success,
    Confirmation,
    Info,
    #[default]
    General,
}

/// This is a basic modal component that can be used to display information to the user.
/// It can be used for various use cases such as error, success, confirmation, info, and general use cases.
/// It can be customized with different icons and colors based on the use case.
/// Example usage:
/// ```
/// <BasicModal title="Can I confirm this?" is_open=modal_open use_case=UseCase::Confirmation on_click_primary=onclick_primary on_cancel=on_cancel disable_auto_close=false>
///     <div>
///         <p>"Hey, please confirm this."</p>
///     </div>
/// </BasicModal>
/// ```
#[component]
pub fn BasicModal(
    #[prop(into)] title: String,
    #[prop(optional)] children: Option<ChildrenFn>,
    #[prop(default = UseCase::General, optional)] use_case: UseCase,
    #[prop(default = Callback::new(|_| {}), optional)] on_click_primary: Callback<()>,
    #[prop(default = Callback::new(|_| {}), optional)] on_cancel: Callback<()>,
    #[prop(default = RwSignal::new(false), into, optional)] is_open: RwSignal<bool>,
    #[prop(into, default = "OK".to_string())] primary_button_text: String,
    #[prop(default = true, optional)] disable_auto_close: bool,
) -> impl IntoView {
    let (title, _set_title) = signal(title);
    let (primary_button_text, _set_primary_button_text) = signal(primary_button_text);
    let (children, _set_children) = signal(children);

    let oncancel_handler = move |_| {
        Callback::new(move |e: ev::MouseEvent| {
            e.stop_propagation();
            is_open.update(|val| *val = false);
            on_cancel.run(());
        })
    };

    let onclick_primary_handler = move || {
        Callback::new(move |e: ev::MouseEvent| {
            e.stop_propagation();
            is_open.update(|val| *val = false);
            on_click_primary.run(());
        })
    };

    let handle_backdrop_click = move |e: ev::MouseEvent| {
        e.stop_propagation();
        if !disable_auto_close {
            is_open.update(|val| *val = false);
            on_cancel.run(());
        };
    };

    view! {
        <>
            <Show when=move || is_open.get() fallback=|| ()>
                <Portal mount=document().get_element_by_id("modal-root").unwrap()>
                    <div class="fixed inset-0 bg-gray-900 opacity-50 z-10"></div>
                        <div on:click=handle_backdrop_click class="fixed inset-0 flex items-center justify-center bg-transparent z-11">
                        <div on:click=move |e| e.stop_propagation() class="bg-slate-50 rounded shadow-lg -translate-y-1/4 w-full max-w-md m-2">
                            <div class="flex items-center mb-4 border-gray-300 border-b p-4">
                                {
                                    move || match use_case {
                                        UseCase::Error => Some(view! {
                                            <span class="text-danger mr-2">
                                                <Icon width="2rem" height="2rem" icon=IconId::BiErrorSolid />
                                            </span>
                                        }),
                                        UseCase::Success => Some(view! {
                                            <span class="text-success mr-2">
                                                <Icon width="2rem" height="2rem" icon=IconId::BiCheckCircleRegular />
                                            </span>
                                        }),
                                        UseCase::Info => Some(view! {
                                            <span class="text-blue-500 mr-2">
                                                <Icon width="2rem" height="2rem" icon=IconId::AiInfoCircleOutlined />
                                            </span>
                                        }),
                                        UseCase::Confirmation => Some(view! {
                                            <span class="text-warning mr-2">
                                                <Icon width="2rem" height="2rem" icon=IconId::AiQuestionCircleOutlined />
                                            </span>
                                        }),
                                        UseCase::General => None,
                                    }
                                }
                                <span class="font-semibold text-lg">{move || title.get()}</span>
                            </div>
                            <div class="mb-4 p-4">
                                {move || children.get().map(|c| c())}
                            </div>
                            <div class="flex justify-end space-x-2 p-4">
                                                    {move || {
                                                        if use_case == UseCase::Confirmation {
                                                            Some(view! {
                                                                    <BasicButton
                                                                        button_text="Cancel".to_string()
                                                                        style_ext="bg-gray-400 text-white".to_string()
                                                                        onclick=oncancel_handler(false)
                                                                    />
                                                            })
                                                        } else {
                                                            None
                                                        }
                                                    }}
                                                    <BasicButton
                                                        button_text=primary_button_text.get()
                                                        style_ext="bg-blue-500 text-white".to_string()
                                                        onclick=onclick_primary_handler()
                                                    />
                                                </div>
                        </div>
                        </div>
                </Portal>
            </Show>
        </>
    }
}
