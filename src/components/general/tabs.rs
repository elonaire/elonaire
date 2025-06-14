use icondata::Icon as IconId;
use leptos::prelude::*;
use leptos_icons::Icon;

#[derive(Clone, Debug, Default)]
pub struct TabInfo {
    pub label: String,
    pub icon: Option<IconId>,
}

impl TabInfo {
    pub fn new(label: &str, icon: Option<IconId>) -> Self {
        TabInfo {
            label: label.to_string(),
            icon,
        }
    }
}

#[component]
pub fn Tabs(
    mut children: ChildrenFragmentMut,
    #[prop(into)] tab_labels: RwSignal<Vec<TabInfo>>,
) -> impl IntoView {
    let (current_tab, set_current_tab) = signal(0);

    view! {
        <div>
            <div class="flex items-center gap-6 w-full border-b border-gray-300">
                {move || {
                    let labels = tab_labels.get();
                    let current = current_tab.get();

                    labels.iter().enumerate().map(|(index, label)| {
                        let dynamic_class = move || {
                            if current == index {
                                "border-blue-500 text-blue-500"
                            } else {
                                "border-transparent text-gray-500"
                            }
                        };

                        view! {
                            <div class="flex items-center gap-2 cursor-pointer transition-all duration-300 ease-in-out">
                                <span
                                    class=format!("border-b-4 transition-all duration-200 ease-in-out pb-1 {}", dynamic_class())
                                    on:click=move |_| set_current_tab.set(index)
                                >
                                    {
                                        if let Some(icon) = label.icon {
                                            Some(view! {
                                                <Icon icon=icon />
                                            })
                                        } else {
                                            None
                                        }
                                    }
                                    <span>{label.label.clone()}</span>
                                </span>
                            </div>
                        }
                    }).collect::<Vec<_>>()
                }}
            </div>

            <div class="relative min-h-[150px] mt-4"> // Ensure container has min-height
                {
                    move || {
                        let current = current_tab.get();
                        children()
                            .nodes
                            .into_iter()
                            .enumerate()
                            .map(|(i, child)| {
                                let dynamic_class = move || if current == i { "block" } else { "hidden" };

                                view! {
                                    <div class=dynamic_class()>
                                        { child.into_view() }
                                    </div>
                                }
                            }).collect_view()
                    }
                }
            </div>
        </div>
    }
}

// Tab Component
#[component]
pub fn Tab(children: Children) -> impl IntoView {
    view! {
        { children() }
    }
}
