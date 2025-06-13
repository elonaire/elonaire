use icondata as IconId;
use leptos::{control_flow::Show, ev, portal::Portal, prelude::*};
use leptos_icons::Icon;

use crate::components::general::button::BasicButton;

#[derive(Clone, PartialEq, Copy, Debug, Default)]
#[allow(dead_code)]
pub enum UseCase {
    Error,
    Success,
    Confirmation,
    Info,
    #[default]
    General,
}

#[component]
pub fn BasicModal(
    title: String,
    #[prop(optional)] children: Option<ChildrenFn>,
    #[prop(default = UseCase::General, optional)] use_case: UseCase,
    #[prop(default = Callback::new(|_| {}), optional)] on_click_primary: Callback<()>,
    #[prop(default = Callback::new(|_| {}), optional)] on_cancel: Callback<bool>,
    #[prop(default = Signal::derive(move || false), into, optional)] is_open: Signal<bool>,
    #[prop(default = "OK".to_string())] primary_button_text: String,
    #[prop(default = true, optional)] disable_auto_close: bool,
) -> impl IntoView {
    let (title, _set_title) = signal(title);
    let (primary_button_text, _set_primary_button_text) = signal(primary_button_text);
    let (children, _set_children) = signal(children);

    let oncancel_handler = move |value: bool| {
        Callback::new(move |e: ev::MouseEvent| {
            e.stop_propagation();
            on_cancel.run(value);
        })
    };

    let onclick_primary_handler = move || {
        Callback::new(move |e: ev::MouseEvent| {
            e.stop_propagation();
            on_click_primary.run(());
        })
    };

    let handle_backdrop_click = move |e: ev::MouseEvent| {
        e.stop_propagation();
        if !disable_auto_close {
            on_cancel.run(false);
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
                                                <Icon icon=IconId::BiErrorSolid />
                                            </span>
                                        }),
                                        UseCase::Success => Some(view! {
                                            <span class="text-success mr-2">
                                                <Icon icon=IconId::BiCheckCircleRegular />
                                            </span>
                                        }),
                                        UseCase::Info => Some(view! {
                                            <span class="text-blue-500 mr-2">
                                                <Icon icon=IconId::AiInfoCircleOutlined />
                                            </span>
                                        }),
                                        UseCase::Confirmation => Some(view! {
                                            <span class="text-warning mr-2">
                                                <Icon icon=IconId::AiQuestionCircleOutlined />
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
