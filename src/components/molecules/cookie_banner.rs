use leptos::prelude::*;

use crate::components::forms::toggle_switch::ToggleSwitch;

#[derive(Clone, Debug, PartialEq)]
pub struct CookiePreferences {
    pub necessary: bool,
    pub analytics: bool,
    pub marketing: bool,
    pub preferences: bool,
}

impl Default for CookiePreferences {
    fn default() -> Self {
        Self {
            necessary: true,
            analytics: false,
            marketing: false,
            preferences: false,
        }
    }
}

/// Cookie consent banner with optional granular preferences panel.
///
/// # Example
/// ```rust
/// <CookieBanner
///     on_accept=Callback::new(move |prefs| log!("accepted: {:?}", prefs))
///     on_reject=Callback::new(move || log!("rejected"))
/// />
/// ```
#[component]
pub fn CookieBanner(
    #[prop(optional, default = Callback::new(|_| {}))] on_accept: Callback<CookiePreferences>,
    #[prop(optional, default = Callback::new(|_| {}))] on_reject: Callback<()>,
) -> impl IntoView {
    let (visible, set_visible) = signal(true);
    let (show_details, set_show_details) = signal(false);

    let analytics = RwSignal::new(false);
    let marketing = RwSignal::new(false);
    let preferences = RwSignal::new(false);

    let on_accept = StoredValue::new(on_accept);
    let on_reject = StoredValue::new(on_reject);

    let accept_all = move |_| {
        set_visible.set(false);
        on_accept.get_value().run(CookiePreferences {
            necessary: true,
            analytics: true,
            marketing: true,
            preferences: true,
        });
    };

    let reject_all = move |_| {
        set_visible.set(false);
        on_reject.get_value().run(());
    };

    let save_preferences = move |_| {
        set_visible.set(false);
        on_accept.get_value().run(CookiePreferences {
            necessary: true,
            analytics: analytics.get(),
            marketing: marketing.get(),
            preferences: preferences.get(),
        });
    };

    view! {
        <Show when=move || visible.get()>
            // Backdrop — visible only behind the expanded panel
            <Show when=move || show_details.get()>
                <div
                    class="fixed inset-0 bg-black/30 backdrop-blur-sm z-40"
                    on:click=move |_| set_show_details.set(false)
                />
            </Show>

            <div class="fixed bottom-6 left-1/2 -translate-x-1/2 z-50 w-full max-w-lg px-4">
                <div class="bg-white dark:bg-zinc-900 border border-zinc-200 dark:border-zinc-700 rounded-2xl shadow-xl p-5 flex flex-col gap-4">

                    // Header
                    <div class="flex items-center gap-2">
                        <span class="text-xl leading-none">"🍪"</span>
                        <h2 class="text-base font-semibold text-zinc-900 dark:text-zinc-100 tracking-tight">
                            "We use cookies"
                        </h2>
                    </div>

                    // Summary or detail body
                    <Show
                        when=move || show_details.get()
                        fallback=move || view! {
                            <p class="text-sm text-zinc-500 dark:text-zinc-400 leading-relaxed">
                                "We use cookies to personalise content, analyse traffic, and improve
                                your experience. You can choose which categories to allow."
                            </p>
                        }
                    >
                        <p class="text-sm text-zinc-500 dark:text-zinc-400 leading-relaxed">
                            "Manage your preferences below. Necessary cookies are always active."
                        </p>
                        <div class="flex flex-col divide-y divide-zinc-100 dark:divide-zinc-800">
                            <CookieRow
                                label="Necessary"
                                description="Required for login sessions and core site functionality. Cannot be disabled."
                                is_active=RwSignal::new(true)
                                readonly=true
                            />
                            <CookieRow
                                label="Analytics"
                                description="Helps us understand how visitors interact with the site."
                                is_active=analytics
                                readonly=false
                            />
                            <CookieRow
                                label="Marketing"
                                description="Used to deliver personalised advertisements."
                                is_active=marketing
                                readonly=false
                            />
                            <CookieRow
                                label="Preferences"
                                description="Remembers your settings and personalisation choices."
                                is_active=preferences
                                readonly=false
                            />
                        </div>
                    </Show>

                    // Actions
                    <div class="flex flex-wrap items-center justify-end gap-2 pt-1">
                        <button
                            class="text-sm text-zinc-400 hover:text-zinc-600 dark:hover:text-zinc-300 px-3 py-2 transition-colors"
                            on:click=reject_all
                        >
                            "Reject all"
                        </button>

                        <Show
                            when=move || show_details.get()
                            fallback=move || view! {
                                <button
                                    class="text-sm px-4 py-2 rounded-lg border border-zinc-200 dark:border-zinc-700 text-zinc-700 dark:text-zinc-300 hover:bg-zinc-50 dark:hover:bg-zinc-800 transition-colors"
                                    on:click=move |_| set_show_details.set(true)
                                >
                                    "Manage preferences"
                                </button>
                            }
                        >
                            <button
                                class="text-sm px-4 py-2 rounded-lg border border-zinc-200 dark:border-zinc-700 text-zinc-700 dark:text-zinc-300 hover:bg-zinc-50 dark:hover:bg-zinc-800 transition-colors"
                                on:click=save_preferences
                            >
                                "Save preferences"
                            </button>
                        </Show>

                        <button
                            class="text-sm px-4 py-2 rounded-lg bg-zinc-900 dark:bg-zinc-100 text-white dark:text-zinc-900 hover:opacity-90 transition-opacity font-medium"
                            on:click=accept_all
                        >
                            "Accept all"
                        </button>
                    </div>

                </div>
            </div>
        </Show>
    }
}

#[component]
fn CookieRow(
    #[prop(into, optional)] label: String,
    #[prop(into, optional)] description: String,
    is_active: RwSignal<bool>,
    readonly: bool,
) -> impl IntoView {
    let id = format!("cookie-toggle-{}", label.to_lowercase());

    view! {
        <div class="flex items-center justify-between gap-4 py-3">
            <div class="flex flex-col gap-0.5">
                <span class="text-sm font-medium text-zinc-800 dark:text-zinc-200">{label}</span>
                <span class="text-xs text-zinc-400 dark:text-zinc-500 leading-snug">{description}</span>
            </div>
            <div class=move || if readonly { "opacity-40 pointer-events-none flex-shrink-0" } else { "flex-shrink-0" }>
                <ToggleSwitch
                    active=is_active
                    label_active=""
                    label_inactive=""
                    id_attr=id.clone()
                    on:change=move |_| {
                        if !readonly {
                            is_active.update(|v| *v = !*v);
                        }
                    }
                    readonly=readonly
                />
            </div>
        </div>
    }
}
