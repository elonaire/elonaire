use icondata::{self, Icon as IconId};
use leptos::html::*;
use leptos::prelude::*;
use leptos_icons::Icon;
use web_sys::CustomEvent;

use crate::utils::forms::fire_custom_bubbled_and_cancelable_event;

#[derive(Clone)]
pub struct PanelInfo {
    pub title: String,
    pub icon: Option<IconId>,
    pub is_open: RwSignal<bool>,
    pub children: ViewFn,
}

impl PanelInfo {
    pub fn new(
        title: &str,
        icon: Option<IconId>,
        is_open: RwSignal<bool>,
        children: ViewFn,
    ) -> Self {
        Self {
            title: title.to_string(),
            icon,
            is_open,
            children,
        }
    }
}

impl std::fmt::Debug for PanelInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Column")
            .field("title", &self.title)
            .field("icon", &self.icon)
            .field("is_open", &self.is_open)
            .field("children", &"<ViewFn>")
            .finish()
    }
}

/// This is a component for displaying a panel with a title and content.
/// Example usage:
/// ```
/// // This example uses a single panel
/// <Panel is_open=panel_is_open title="Elonaire" icon=IconId::BsNodePlusFill >
///     <p>"Hey there, I am Mr Elonaire!"</p>
/// </Panel>
///
/// // You can also group multiple panels by using the Collapse component
/// // is_accordion prop enables only one panel to be open at a time. It's optional and the default is false.
/// <Collapse is_accordion=true panel_items=RwSignal::new(vec![
///    PanelInfo::new("title 1", None, RwSignal::new(false), ViewFn::from(move || view!{ <p>"Panel content"</p> })),
///    PanelInfo::new("title 2", None, RwSignal::new(false), ViewFn::from(move || view!{ <p>"Panel content"</p> })),
///    PanelInfo::new("title 3", None, RwSignal::new(false), ViewFn::from(move || view!{ <p>"Panel content"</p> })),
///    PanelInfo::new("title 4", None, RwSignal::new(false), ViewFn::from(move || view!{ <p>"Panel content"</p> }))
/// ]) />
/// ```
#[component]
pub fn Panel(
    #[prop(into)] title: String,
    #[prop(optional)] children: Option<Children>,
    #[prop(optional)] icon: Option<IconId>,
    #[prop(into)] is_open: RwSignal<bool>,
    #[prop(optional)] is_accordion: bool,
    #[prop(into, optional)] ext_panel_title_styles: String,
) -> impl IntoView {
    let panel_ref = NodeRef::new();
    let toggle_content = move |_| {
        if let Some(panel_element) = panel_ref.get() {
            fire_custom_bubbled_and_cancelable_event("togglepanel", true, true, panel_element);
        }

        if !is_accordion {
            is_open.update(|value| *value = !*value);
        }
    };

    view! {
        <div node_ref=panel_ref>
            <span
                on:click=toggle_content
                class=format!("flex flex-row items-center justify-between gap-2 mb-2 p-2 rounded cursor-pointer hover:bg-primary hover:text-white {}", ext_panel_title_styles)
            >
                <span class="flex flex-row items-center gap-2">
                    {
                        if icon.is_some() {
                            Some(view!{ <Icon icon=icon.unwrap() /> })
                        } else {
                            None
                        }
                    }
                    <span>{title}</span>
                </span>
                {
                    move || {
                        let icon_id = if is_open.get() {
                            icondata::BsDashLg
                        } else {
                            icondata::BsPlusLg
                        };
                        view!{ <Icon icon=icon_id /> }

                    }
                }
            </span>
            <div
                class=move || {
                    if is_open.get() {
                        "transition-max-height duration-700 ease-in-out overflow-hidden max-h-svh p-2 ml-2"
                    } else {
                        "overflow-hidden h-0 transition-max-height duration-700 ease-in-out"
                    }
                }
            >
                {children.map(|c| c())}
            </div>
        </div>
    }
}

#[component]
pub fn Collapse(
    #[prop(into)] panel_items: RwSignal<Vec<PanelInfo>>,
    #[prop(default = false)] is_accordion: bool,
) -> impl IntoView {
    let handle_panel_toggle = move |index| {
        if is_accordion {
            panel_items.update(|panels| {
                for (i, panel) in panels.iter().enumerate() {
                    if i == index {
                        panel.is_open.update(|val| *val = !*val);
                    } else {
                        panel.is_open.update(|val| *val = false);
                    }
                }
            });
        }
    };

    view! {
        <div class="flex flex-col">
            <For
                each=move || panel_items.get().into_iter().enumerate()
                key=|(index, _)| *index
                let:((index, panel_item))
            >
                {
                    view! {
                        <Panel on:togglepanel=move |ev: CustomEvent| {
                            ev.stop_propagation();
                            handle_panel_toggle(index)
                        } title=panel_item.title.clone() is_open=panel_item.is_open is_accordion=is_accordion>
                            {panel_item.children.run()}
                        </Panel>
                    }
                }
            </For>
        </div>
    }
}
