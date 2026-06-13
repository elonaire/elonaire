use detaxine_ui::components::{actions::button::BasicButton, forms::toggle_switch::ToggleSwitch};
use leptos::prelude::*;
use web_sys::{Storage, window};

const COOKIE_PREFS_KEY: &str = "cookie_preferences";

#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct CookiePreferences {
    #[serde(default = "default_true")]
    pub necessary: bool,
    pub analytics: bool,
    pub marketing: bool,
    pub preferences: bool,
}

fn default_true() -> bool {
    true
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
    // Initialise from localStorage — banner is hidden if prefs already exist
    let saved = load_preferences();
    let (visible, set_visible) = signal(saved.is_none());
    let (show_details, set_show_details) = signal(false);

    let analytics = RwSignal::new(saved.as_ref().map(|p| p.analytics).unwrap_or(false));
    let marketing = RwSignal::new(saved.as_ref().map(|p| p.marketing).unwrap_or(false));
    let preferences = RwSignal::new(saved.as_ref().map(|p| p.preferences).unwrap_or(false));

    let on_accept = StoredValue::new(on_accept);
    let on_reject = StoredValue::new(on_reject);

    let save_preferences = move |prefs: &CookiePreferences| {
        if let Some(storage) = get_local_storage() {
            if let Ok(serialized) = serde_json::to_string(prefs) {
                storage.set_item(COOKIE_PREFS_KEY, &serialized).ok();
            }
        }
    };

    let accept_all = move |_| {
        let prefs = CookiePreferences {
            necessary: true,
            analytics: true,
            marketing: true,
            preferences: true,
        };
        save_preferences(&prefs);
        set_visible.set(false);
        on_accept.get_value().run(prefs);
    };

    let reject_all = move |_| {
        let prefs = CookiePreferences {
            necessary: true,
            analytics: false,
            marketing: false,
            preferences: false,
        };
        save_preferences(&prefs);
        set_visible.set(false);
        on_reject.get_value().run(());
    };

    let save_preferences_handler = move |_| {
        let prefs = CookiePreferences {
            necessary: true,
            analytics: analytics.get(),
            marketing: marketing.get(),
            preferences: preferences.get(),
        };
        save_preferences(&prefs);
        set_visible.set(false);
        on_accept.get_value().run(prefs);
    };

    view! {
        <Show when=move || visible.get()>
            <Show when=move || show_details.get()>
                <div
                    class="fixed inset-0 bg-navy/40 dark:bg-navy/60 backdrop-blur-sm z-40"
                    on:click=move |_| set_show_details.set(false)
                />
            </Show>

            <div class="fixed bottom-6 left-1/2 -translate-x-1/2 z-50 w-full max-w-lg px-4">
                <div class="bg-contrast-white dark:bg-navy border border-gray/10 dark:border-mid-gray/20 rounded-[5px] shadow-xl p-5 flex flex-col gap-4">

                    <div class="flex items-center gap-2">
                        <span class="text-xl leading-none">"🍪"</span>
                        <h2 class="text-base font-semibold text-gray dark:text-contrast-white tracking-tight">
                            "We use cookies"
                        </h2>
                    </div>

                    <Show
                        when=move || show_details.get()
                        fallback=move || view! {
                            <p class="text-sm text-gray/60 dark:text-mid-gray leading-relaxed">
                                "We use cookies to personalise content, analyse traffic, and improve
                                your experience. You can choose which categories to allow."
                            </p>
                        }
                    >
                        <p class="text-sm text-gray/60 dark:text-mid-gray leading-relaxed">
                            "Manage your preferences below. Necessary cookies are always active."
                        </p>
                        <div class="flex flex-col divide-y divide-gray/10 dark:divide-mid-gray/20">
                            <CookieRow
                                label="Necessary"
                                description="Required for login sessions and core site functionality. Cannot be disabled."
                                is_active=true
                                readonly=true
                            />
                            {move || {
                                let received_analytics = analytics.get();
                                view!{
                                    <CookieRow
                                        label="Analytics"
                                        description="Helps us understand how visitors interact with the site."
                                        is_active=received_analytics
                                        readonly=false
                                        on:change=move |_| {
                                            analytics.update(|v| *v = !*v);
                                        }
                                    />
                                }
                            }}
                            {move || {
                                let received_marketing = marketing.get();
                                view!{
                                    <CookieRow
                                        label="Marketing"
                                        description="Used to deliver personalised advertisements."
                                        is_active=received_marketing
                                        readonly=false
                                        on:change=move |_| {
                                            marketing.update(|v| *v = !*v);
                                        }
                                    />
                                }
                            }}
                            {move || {
                                let received_preferences = preferences.get();
                                view!{
                                    <CookieRow
                                        label="Preferences"
                                        description="Remembers your settings and personalisation choices."
                                        is_active=received_preferences
                                        readonly=false
                                        on:change=move |_| {
                                            preferences.update(|v| *v = !*v);
                                        }
                                    />
                                }
                            }}

                        </div>
                    </Show>

                    <div class="flex flex-wrap items-center justify-end gap-2 pt-1">
                        <BasicButton
                            style_ext="text-sm text-gray/50 dark:text-mid-gray hover:text-gray dark:hover:text-contrast-white px-3 py-2 transition-colors"
                            on:click=reject_all
                        >
                            "Reject all"
                        </BasicButton>

                        <Show
                            when=move || show_details.get()
                            fallback=move || view! {
                                <BasicButton
                                    style_ext="text-sm px-4 py-2 rounded-[5px] border border-gray/20 dark:border-mid-gray/30 text-gray dark:text-mid-gray hover:bg-secondary/10 dark:hover:bg-mid-gray/10 transition-colors"
                                    on:click=move |_| set_show_details.set(true)
                                >
                                    "Manage preferences"
                                </BasicButton>
                            }
                        >
                            <BasicButton
                                style_ext="text-sm px-4 py-2 rounded-[5px] border border-gray/20 dark:border-mid-gray/30 text-gray dark:text-mid-gray hover:bg-secondary/10 dark:hover:bg-mid-gray/10 transition-colors"
                                on:click=save_preferences_handler
                            >
                                "Save preferences"
                            </BasicButton>
                        </Show>

                        <BasicButton
                            style_ext="text-sm px-4 py-2 rounded-[5px] bg-primary text-contrast-white hover:opacity-90 transition-opacity font-medium"
                            on:click=accept_all
                        >
                            "Accept all"
                        </BasicButton>
                    </div>

                </div>
            </div>
        </Show>
    }.into_any()
}

#[component]
fn CookieRow(
    #[prop(into, optional)] label: String,
    #[prop(into, optional)] description: String,
    is_active: bool,
    readonly: bool,
) -> impl IntoView {
    let id = format!("cookie-toggle-{}", label.to_lowercase());

    view! {
        <div class="flex items-center justify-between gap-4 py-3">
            <div class="flex flex-col gap-0.5">
                <span class="text-sm font-medium text-gray dark:text-contrast-white">{label}</span>
                <span class="text-xs text-gray/60 dark:text-mid-gray leading-snug">{description}</span>
            </div>
            <div class=move || if readonly { "opacity-40 pointer-events-none flex-shrink-0" } else { "flex-shrink-0" }>
                <ToggleSwitch
                    initial_active_state=is_active
                    label_active=""
                    label_inactive=""
                    id_attr=id.clone()
                    readonly=readonly
                />
            </div>
        </div>
    }.into_any()
}

// Helper functions

fn get_local_storage() -> Option<Storage> {
    window()?.local_storage().ok()?
}

fn load_preferences() -> Option<CookiePreferences> {
    let storage = get_local_storage()?;
    let raw = storage.get_item(COOKIE_PREFS_KEY).ok()??;
    serde_json::from_str(&raw).ok()
}
