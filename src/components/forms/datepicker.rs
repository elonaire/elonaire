use crate::components::forms::input::{InputField, InputFieldType};
use crate::components::general::button::BasicButton;
use chrono::Local;
use chrono::{DateTime, Datelike, Duration, NaiveDate, TimeZone, Weekday};
use icondata as IconId;
use leptos::ev;
use leptos::prelude::*;
use leptos_icons::Icon;
use web_sys::{Event, HtmlInputElement};

#[component]
pub fn DatePicker(
    #[prop(into, optional)] label: String,
    #[prop(into, optional)] name: String,
    #[prop(default = false, optional)] required: bool,
    #[prop(into, default = Signal::derive(move || Local::now()), optional)] initial_value: Signal<
        DateTime<Local>,
    >,
    #[prop(default = Callback::new(|_| {}), optional)] onchange: Callback<DateTime<Local>>,
    #[prop(into, optional)] id_attr: String,
) -> impl IntoView {
    let (show_calendar, set_show_calendar) = signal(false);
    let (selected_date, set_selected_date) = signal(initial_value.get());
    let date_input_ref = NodeRef::new();

    let selected_date_value = Memo::new(move |_| selected_date.get().to_rfc3339());

    let selected_date_display_value =
        Memo::new(move |_| selected_date.get().format("%b %0e %Y").to_string());

    let toggle_calendar = Callback::new(move |_| {
        set_show_calendar.update(|val| *val = !*val);
    });

    let select_date = Callback::new(move |date: DateTime<Local>| {
        set_selected_date.set(date);
        onchange.run(date);
        set_show_calendar.set(false);

        // Fire a bubbling Change event so that the form can capture changes
        date_input_ref.on_load(|i: HtmlInputElement| {
            let _event = match Event::new("change") {
                Ok(ev) => {
                    ev.init_event_with_bubbles("change", true);
                    i.dispatch_event(&ev).unwrap();
                }
                Err(_e) => {}
            };
        });
    });

    view! {
        <div class="mb-2">
            <div class="relative">
                <InputField
                    initial_value=selected_date_value
                    name=name
                    label=label
                    field_type=InputFieldType::Text
                    required=required
                    ext_input_styles="sr-only"
                    id_attr=id_attr
                    input_node_ref=date_input_ref
                />
                <InputField
                    readonly=true
                    onclick=Callback::new(move |ev: ev::MouseEvent| toggle_calendar.run(ev))
                    initial_value=selected_date_display_value
                    field_type=InputFieldType::Text
                />
                <div
                    class="absolute inset-y-0 right-0 pr-3 flex items-center cursor-pointer"
                    on:click={move |ev| toggle_calendar.run(ev)}
                >
                    <span class="text-gray-500"><Icon icon=IconId::BsCalendar2Date /></span>
                </div>
                {move || if show_calendar.get() {
                    Some(view! {
                        <div class="absolute bg-slate-50 border mt-1 rounded shadow-lg z-10 w-[300px] max-h-[400px] overflow-auto">
                            <Calendar select_date={select_date.clone()} />
                        </div>
                    })
                } else {
                    None
                }}
            </div>
        </div>
    }
}

#[component]
fn Calendar(#[prop(into)] select_date: Callback<DateTime<Local>>) -> impl IntoView {
    let today: DateTime<Local> = Local::now();
    let default_month = today.month();
    let default_year = today.year();
    let (current_month, set_current_month) = signal(default_month);
    let (current_year, set_current_year) = signal(default_year);
    let (viewing_years, set_viewing_years) = signal(false);
    let (year_page, set_year_page) = signal(0);

    let years_per_page = 16; // Number of years to display per page

    let toggle_viewing_years = Callback::new(move |_| {
        set_viewing_years.update(|val| *val = !*val);
    });

    let change_year = Callback::new(move |year: i32| {
        set_current_year.set(year);
        set_viewing_years.set(false);
    });

    let render_years = move || {
        let start_year = (default_year - 60).max(1);
        let end_year = default_year + 12;
        let total_years: Vec<i32> = (start_year..end_year).collect();
        let pages = total_years.chunks(years_per_page).collect::<Vec<&[i32]>>();

        if pages.is_empty() {
            return vec![None];
        }

        let current_page = year_page.get() % pages.len();
        pages[current_page]
            .iter()
            .map(|&year| {
                Some(view! {
                    <BasicButton onclick=Callback::new(move |_| {
                        change_year.run(year);
                    }) style_ext="flex text-xs border-none rounded m-1 hover:bg-blue-200 cursor-pointer" button_text=year.to_string() />
                })
            })
            .collect::<Vec<_>>()
    };

    let next_year_page = Callback::new(move |_| {
        set_year_page.update(|val| *val += 1);
    });

    let prev_year_page = Callback::new(move |_| {
        set_year_page.update(|val| {
            if *val > 0 {
                *val -= 1;
            }
        });
    });

    fn last_day_of_month(date: NaiveDate) -> NaiveDate {
        let year = if date.month() == 12 {
            date.year() + 1
        } else {
            date.year()
        };
        let month = if date.month() == 12 {
            1
        } else {
            date.month() + 1
        };
        let first_of_next_month = NaiveDate::from_ymd_opt(year, month, 1).expect("Invalid date");
        first_of_next_month - Duration::days(1)
    }

    let days_in_month = move || {
        let first_date = NaiveDate::from_ymd_opt(current_year.get(), current_month.get(), 1)
            .expect("Invalid date");
        last_day_of_month(first_date).day()
    };

    let render_days = move || {
        let days_in_month = days_in_month();
        let first_date =
            NaiveDate::from_ymd_opt(current_year.get(), current_month.get(), 1).unwrap();
        let calendar_adjustment = match first_date.weekday() {
            Weekday::Sun => 0,
            Weekday::Mon => 1,
            Weekday::Tue => 2,
            Weekday::Wed => 3,
            Weekday::Thu => 4,
            Weekday::Fri => 5,
            Weekday::Sat => 6,
        };

        view! {
            <For
                each=move || (0..(calendar_adjustment + days_in_month)).enumerate()
                key=|&(i, _)| i
                children=move |(i, _)| {
                    let is_blank = (i as u32) < calendar_adjustment;
                    let day = if is_blank { 0 } else { (i as u32) - calendar_adjustment + 1 };
                    let date = if is_blank {
                        None
                    } else {
                        Some(Local.with_ymd_and_hms(current_year.get(), current_month.get(), day as u32, 0, 0, 0).unwrap())
                    };
                    let select_date = select_date.clone();
                    view! {
                        <BasicButton onclick=Callback::new(move |_| {
                            if let Some(date) = date {
                                select_date.run(date);
                            }
                        }) style_ext="flex text-xs items-center justify-center border-none rounded m-1 hover:bg-blue-200 cursor-pointer" button_text={if is_blank { "".to_string() } else { day.to_string() }} />
                    }
                }
            />
        }
    };

    view! {
        <div class="w-full max-w-md bg-white border-none rounded">
            {move || if viewing_years.get() {
                Some(view! {
                    <div>
                        <div class="flex justify-between items-center mb-2">
                            <BasicButton onclick=prev_year_page icon=Some(IconId::BiChevronLeftRegular) />
                            <span class="cursor-pointer">"Years"</span>
                            <BasicButton onclick=next_year_page icon=Some(IconId::BiChevronRightRegular) />
                        </div>
                        <div class="grid grid-cols-4 gap-1 bg-white -none rounded p-2">
                            {move || render_years()}
                        </div>
                    </div>
                })
            } else {
                None
            }}

            {move || if !viewing_years.get() {
                let days_of_week = ["S", "M", "T", "W", "T", "F", "S"];
                Some(view! {
                    <div>
                        <div class="flex justify-between items-center mb-2">
                            <BasicButton onclick=Callback::new(
                                move |_| {
                                    set_current_month.update(|m| {
                                        if *m == 1 {
                                            set_current_year.update(|y| *y -= 1);
                                            *m = 12;
                                        } else {
                                            *m -= 1;
                                        }
                                    });
                                }
                            ) icon=Some(IconId::BiChevronLeftRegular) />
                            <span
                                on:click={move |_| toggle_viewing_years.run(())}
                                class="cursor-pointer"
                            >
                                {move || format!("{:?} {:?}", current_year.get(), chrono::Month::try_from(u8::try_from(current_month.get()).unwrap()).unwrap())}
                            </span>
                            <BasicButton onclick=Callback::new(
                                move |_| {
                                    set_current_month.update(|m| {
                                        if *m == 12 {
                                            set_current_year.update(|y| *y += 1);
                                            *m = 1;
                                        } else {
                                            *m += 1;
                                        }
                                    });
                                }
                            ) icon=Some(IconId::BiChevronRightRegular) />
                        </div>
                        <div class="grid grid-cols-7 gap-1 text-gray-500 bg-white border-none rounded p-2">
                            {days_of_week.iter().map(|&day| view! { <div class="font-bold text-center text-sm">{day}</div> }).collect::<Vec<_>>()}
                            {render_days()}
                        </div>
                    </div>
                })
            } else {
                None
            }}
        </div>
    }
}
