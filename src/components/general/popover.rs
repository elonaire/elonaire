use leptos::html::*;
use leptos::{portal::Portal, prelude::*};
use web_sys::HtmlDivElement;

#[derive(Clone, PartialEq)]
#[allow(dead_code)]
pub enum Position {
    Top,
    Bottom,
}

/// The Popover component is a reusable UI element that displays a popover with customizable content and positioning.
/// ```
/// <Popover display_item=|| view!{ <p>"Elonaire here"</p> } showing=popover_open on_click_toggle=toggle_popover_handler >
///    <div class="flex flex-row">
///    <span class="text-gray-600">"Tenka"</span>
///        <img src="https://encrypted-tbn0.gstatic.com/images?q=tbn:ANd9GcRij6dtiHizH96qpCOe8WeXXP3yLyQJkPdGVg&s" />
///    </div>
/// </Popover>
/// ```
#[component]
pub fn Popover(
    #[prop(optional)] children: Option<ChildrenFn>,
    #[prop(into)] display_item: ViewFn,
    #[prop(default = Position::Bottom, optional)] position: Position,
    #[prop(into, optional)] style_ext: String,
    // #[prop(default = Callback::new(|_| {}), optional)] on_click_toggle: Callback<bool>,
    #[prop(into)] showing: RwSignal<bool>,
) -> impl IntoView {
    let (children, _set_children) = signal(children);
    let trigger_ref = NodeRef::new();

    let onclick_toggle_handler = move |_| {
        showing.update(|val| *val = !*val);
    };

    // Determine the classes for the position and arrow
    let (position_class, _arrow_class) = match position {
        Position::Top => ("bottom-full mb-2", "border-b-gray"),
        Position::Bottom => ("top-full mt-2", "border-t-gray"),
    };

    // Dynamic classes based on popover state
    let open_classes = Memo::new(move |_| {
        format!(
            "absolute {} md:min-w-48 bg-contrast-white border-[1px] border-light-gray shadow-lg text-contrast-white text-sm rounded transition-all duration-300 z-30 {}",
            position_class, style_ext
        )
    });

    view! {
        <>
            <div node_ref=trigger_ref on:click=onclick_toggle_handler class="cursor-pointer">
                {display_item.run()}
            </div>
            <Show when=move || showing.get() fallback=|| ()>
                <div
                    on:click=onclick_toggle_handler
                    class="fixed inset-0 z-20 bg-transparent"
                ></div>
                <Portal>
                    <div
                    class=open_classes
                    style=move || {
                                                // Basic example: get bounding rect and position manually
                                                if let Some(el) = trigger_ref.get() {
                                                    let rect = el.get_bounding_client_rect();

                                                    let style = format!(
                                                        "left: {}px; top: {}px;",
                                                        rect.left() - 20.0,
                                                        rect.bottom() - 10.0, // below trigger
                                                    );

                                                    style
                                                } else {
                                                    "".to_string()
                                                }
                                            }
                    >
                        {move || children.get().map(|child| child())}
                    </div>
                </Portal>
            </Show>
        </>
    }
}
