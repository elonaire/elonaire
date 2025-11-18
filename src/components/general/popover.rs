use leptos::html::*;
use leptos::prelude::*;

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

    let onclick_toggle_handler = move |_| {
        showing.update(|val| *val = !*val);
    };

    // Determine the classes for the position and arrow
    let (position_class, _arrow_class) = match position {
        Position::Top => ("bottom-full mb-2", "border-b-gray-800"),
        Position::Bottom => ("top-full mt-2", "border-t-gray-800"),
    };

    // Dynamic classes based on popover state
    let open_classes = Memo::new(move |_| {
        format!(
            "absolute {} md:min-w-48 bg-slate-50 border-[1px] border-gray-200 shadow-lg text-white text-sm rounded transition-all duration-300 z-50 right-0 {}",
            position_class, style_ext
        )
    });

    view! {
        <>
            <div on:click=onclick_toggle_handler class="cursor-pointer">
                {display_item.run()}
            </div>
            <Show when=move || showing.get() fallback=|| ()>
                <div
                    on:click=onclick_toggle_handler
                    class="fixed inset-0 z-40 bg-transparent"
                ></div>
                <div class="relative flex justify-center items-center">


                    <div
                    class=open_classes
                    >
                        {move || children.get().map(|child| child())}
                    </div>
                </div>
            </Show>
        </>
    }
}
