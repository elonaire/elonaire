use icondata::Icon as IconId;
use leptos::ev;
use leptos::prelude::*;
use leptos_icons::Icon; // Adjust based on your icon set (e.g., icondata::BsIcon)

// Define the BasicButton component
#[component]
pub fn BasicButton(
    #[prop(default = "".to_string())] button_text: String,
    #[prop(default = "".to_string())] style_ext: String,
    #[prop(default = Callback::new(|_| {}))] onclick: Callback<ev::MouseEvent>,
    #[prop(default = None)] icon: Option<IconId>,
    #[prop(default = false)] disabled: bool,
    #[prop(default = "button".to_string())] button_type: String,
    #[prop(default = true)] icon_before: bool,
) -> impl IntoView {
    let button_text_styles = button_text.clone();
    let button_content_styles = move || {
        if button_text_styles.is_empty() {
            ""
        } else if icon_before {
            "gap-2"
        } else {
            "gap-2 flex-row-reverse"
        }
    };

    view! {
        <button
            type={button_type}
            class=move || format!(
                "font-bold py-2 px-4 cursor-pointer rounded-md disabled:opacity-50 disabled:cursor-not-allowed {}",
                style_ext
            )
            on:click=move |ev| onclick.run(ev)
            disabled={disabled}
        >
            <span class=move || format!("flex flex-row items-center justify-center {}", button_content_styles())>
                {move || match icon {
                    Some(button_icon) => view! {
                        <Icon width="1em" height="1em" icon=button_icon />
                    }.into(),
                    None => None,
                }}
                <span>{button_text}</span>
            </span>
        </button>
    }
}

// Define the ButtonGroup component
#[component]
pub fn ButtonGroup(
    /// `style_ext` property will extend styles for the buttons. `N/B:` All buttons share the same styles(These styles will affect all the buttons).
    #[prop(default = "".to_string())]
    style_ext: String,
    mut children: ChildrenFragmentMut,
) -> impl IntoView {
    view! {
        <div class="flex" role="group">
            {
                let style_ext_view = style_ext.clone();
                let children_len = children().nodes.iter().collect::<Vec<_>>().len();
                children()
                .nodes
                .into_iter()
                .enumerate()
                .map(|(index, child)| {
                    let style_ext_view = style_ext_view.clone();
                    let class_name = move || {
                        let mut base = format!(
                            "font-bold py-2 px-4 border border-gray-200 border-l-0 cursor-pointer disabled:opacity-50 disabled:cursor-not-allowed {}",
                            style_ext_view
                        );
                        if index == 0 {
                            base.push_str(" rounded-l-md");
                        }

                        if index == children_len - 1 {
                            base.push_str(" rounded-r-md");
                        }
                        base
                    };
                    view! {
                        {child.attr("class", class_name())}
                    }
                })
                .collect::<Vec<_>>()}
        </div>
    }
}
