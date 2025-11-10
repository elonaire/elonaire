use icondata::Icon as IconId;
use leptos::prelude::*;
use leptos_icons::Icon;

#[derive(Clone, Debug)]
#[allow(dead_code)]
pub enum TimelineStatus {
    Warning,
    Info,
    Danger,
    Success,
}

#[derive(Clone)]
pub struct TimelineItem {
    pub title: String,
    pub description: Option<String>,
    pub icon_head: Option<IconId>,
    pub image_head: Option<String>,
    pub content: ViewFn,
    pub status: TimelineStatus,
}

impl TimelineItem {
    pub fn new(
        title: &str,
        description: Option<&str>,
        icon_head: Option<IconId>,
        image_head: Option<&str>,
        status: TimelineStatus,
        content: ViewFn,
    ) -> Self {
        Self {
            title: title.to_string(),
            description: description.map(|s| s.to_string()),
            icon_head,
            image_head: image_head.map(|s| s.to_string()),
            status,
            content,
        }
    }
}

/// This is a component that displays a timeline of steps.
/// There are 3 ways to display the header of each step.
/// One way is to use icons, another way is to use images, and a third way is to use the default circle.
/// Example usage:
/// ```
/// let steps = RwSignal::new(vec![
///    TimelineItem::new(
///        "Step 1",
///        Some("Initialize project"),
///        None,
///        Some("https://encrypted-tbn0.gstatic.com/images?q=tbn:ANd9GcTiZ17_7VUm-SR8l9Z7ksl7n7SfjUTTNK5NWA&s"),
///        TimelineStatus::Success,
///        ViewFn::from(|| view! { <p>"Project created with Leptos!"</p> }),
///    ),
///    TimelineItem::new(
///        "Step 2",
///        Some("Build UI components"),
///        None,
///        Some(
///            "https://img.icons8.com/?size=100&id=4PiNHtUJVbLs&format=png&color=000000",
///        ),
///        TimelineStatus::Info,
///        ViewFn::from(|| view! { <p>"Using Tailwind and component slots."</p> }),
///    ),
///    TimelineItem::new(
///        "Step 3",
///        Some("Deploy to production"),
///        None,
///        Some("https://img.icons8.com/color/512/amazon-web-services.png"),
///        TimelineStatus::Info,
///        ViewFn::from(|| view! { <p>"Vercel, Deno or your own server."</p> }),
///    ),
///
/// // The view is as shown below
/// <Timeline steps=steps />
/// ]);

#[component]
pub fn Timeline(#[prop(into)] steps: RwSignal<Vec<TimelineItem>>) -> impl IntoView {
    view! {
        <div class="relative">
            <For
                each=move || steps.get().into_iter().enumerate()
                key=|(i, _)| *i
                let:((_i, item))
            >
                {
                    let bg_status_classes = match item.status {
                        TimelineStatus::Warning => "bg-warning/20 text-warning",
                        TimelineStatus::Info => "bg-info/20 text-info",
                        TimelineStatus::Success => "bg-success/20 text-success",
                        TimelineStatus::Danger => "bg-danger/20 text-danger",
                    };

                    view! {
                        <div class="relative flex">
                            <div class="flex flex-col">
                                {
                                    if let Some(icon_head) = &item.icon_head {
                                        Some(view!{
                                            <span class="relative flex size-8 cursor-pointer">
                                                <span class=format!("absolute inline-flex h-full w-full rounded-full {} {}", match &item.status { TimelineStatus::Info => "animate-ping", _ => "" }, bg_status_classes)></span>
                                                <span class=format!("relative inline-flex items-center justify-center size-8 rounded-full {}", bg_status_classes)>
                                                    <Icon width="50%" height="50%" icon=icon_head.to_owned() />
                                                </span>
                                            </span>
                                        })
                                    } else {
                                        None
                                    }
                                }
                                {
                                    if let Some(image_head) = &item.image_head {
                                        Some(view!{
                                            <span class="relative flex size-8 cursor-pointer">
                                                <span class=format!("absolute inline-flex h-full w-full rounded-full {} {}", match &item.status { TimelineStatus::Info => "animate-ping", _ => "" }, bg_status_classes)></span>
                                                <span class=format!("relative inline-flex items-center justify-center size-8 rounded-full {}", bg_status_classes)>
                                                    <img alt="timeline-head" src=image_head.to_owned() class="w-full h-full rounded-full object-contain saturate-200" />
                                                </span>
                                            </span>
                                        })
                                    } else {
                                        None
                                    }
                                }
                                {
                                    if item.image_head.is_none() && item.icon_head.is_none() {
                                        Some(view!{
                                            <span class="relative flex size-3 cursor-pointer">
                                                <span class=format!("absolute inline-flex h-full w-full rounded-full {} {}", match &item.status { TimelineStatus::Info => "animate-ping", _ => "" }, bg_status_classes)></span>
                                                <span class=format!("relative inline-flex size-3 rounded-full {}", bg_status_classes)></span>
                                            </span>
                                        })
                                    } else {
                                        None
                                    }
                                }
                                <div class="flex justify-center flex-1">
                                    <div class="border border-gray-300"></div>
                                </div>
                            </div>
                            <div class="ml-4 mb-4">
                                <div class="flex items-center gap-2">
                                    <h4>{item.title}</h4>
                                </div>
                                {
                                    item.description.as_ref().map(|desc| view! {
                                        <p class="text-sm text-gray-400">{desc.to_owned()}</p>
                                    })
                                }
                                <div class="mt-2 text-gray-600">
                                    {item.content.run()}
                                </div>
                            </div>
                        </div>
                    }
                }
            </For>
        </div>
    }
}
