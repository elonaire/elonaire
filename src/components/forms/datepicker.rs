use crate::components::forms::input::{InputField, InputFieldType};
use chrono::{Datelike, Duration, Local, NaiveDate, Weekday};
use icondata as IconId;
use leptos::ev;
use leptos::prelude::*;
use leptos_icons::Icon;

#[component]
pub fn DatePicker(
    #[prop(default = "".to_string())] label: String,
    name: String,
    #[prop(default = false)] required: bool,
    #[prop(default = Local::now().date_naive())] initial_value: NaiveDate,
    #[prop(default = None)] onchange: Option<Callback<NaiveDate>>,
) -> impl IntoView {
    let (show_calendar, set_show_calendar) = signal(false);
    let (selected_date, set_selected_date) = signal(initial_value);

    let toggle_calendar = Callback::new(move |_| {
        set_show_calendar.update(|val| *val = !*val);
    });

    let select_date = Callback::new(move |date: NaiveDate| {
        set_selected_date.set(date);
        if let Some(cb) = onchange {
            cb.run(date);
        }
        set_show_calendar.set(false);
    });

    view! {
        <div class="mb-2">
            <label for={name.clone()} class="block text-gray-700 text-sm font-bold mb-2">
                {label}
                {move || if required {
                    Some(view! { <span class="text-red-500">"*"</span> })
                } else {
                    None
                }}
            </label>
            <div class="relative">
                <InputField
                    readonly=true
                    onclick=Some(Callback::new(move |ev: ev::MouseEvent| toggle_calendar.run(ev)))
                    initial_value={selected_date.get().format("%b %0e %Y").to_string()}
                    name={name.clone()}
                    field_type={InputFieldType::Text}
                />
                <div
                    class="absolute inset-y-0 right-0 pr-3 flex items-center cursor-pointer"
                    on:click={move |ev| toggle_calendar.run(ev)}
                >
                    <span class="text-gray-500"><Icon icon=IconId::BsCalendar2Date /></span>
                </div>
                {move || if show_calendar.get() {
                    Some(view! {
                        <div class="absolute bg-slate-50 border mt-1 rounded shadow-lg z-10 max-h-[400px] overflow-auto">
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
fn Calendar(#[prop(into)] select_date: Callback<NaiveDate>) -> impl IntoView {
    let today = chrono::Local::now().naive_local();
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
                    <button
                        class="flex text-xs border-none rounded m-1 hover:bg-blue-200 cursor-pointer"
                        on:click={move |_| change_year.run(year)}
                    >
                        {year}
                    </button>
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

        // Log calendar adjustment
        leptos::logging::log!("Calendar adjustment: {}", calendar_adjustment);

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
                        Some(NaiveDate::from_ymd_opt(current_year.get(), current_month.get(), day as u32).unwrap())
                    };
                    let select_date = select_date.clone();
                    view! {
                        <button
                            class="flex text-xs items-center justify-center border-none rounded m-1 hover:bg-blue-200 cursor-pointer"
                            on:click=move |_| {
                                if let Some(date) = date {
                                    select_date.run(date);
                                }
                            }
                        >
                            {if is_blank { "".to_string() } else { day.to_string() }}
                        </button>
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
                            <button on:click={move |_| prev_year_page.run(())}>
                                <span class="text-gray-500 cursor-pointer"><Icon icon=IconId::BiChevronLeftRegular /></span>
                            </button>
                            <span class="cursor-pointer">"Years"</span>
                            <button on:click={move |_| next_year_page.run(())}>
                                <span class="text-gray-500 cursor-pointer"><Icon icon=IconId::BiChevronRightRegular /></span>
                            </button>
                        </div>
                        <div class="grid grid-cols-4 gap-1 bg-white -none rounded p-2">
                            {render_years()}
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
                            <button
                                on:click={move |_| {
                                    set_current_month.update(|m| {
                                        if *m == 1 {
                                            set_current_year.update(|y| *y -= 1);
                                            *m = 12;
                                        } else {
                                            *m -= 1;
                                        }
                                    });
                                }}
                            >
                                <span class="text-gray-500 cursor-pointer"><Icon icon=IconId::BiChevronLeftRegular /></span>
                            </button>
                            <span
                                on:click={move |_| toggle_viewing_years.run(())}
                                class="cursor-pointer"
                            >
                                {format!("{:?} {:?}", current_year.get(), chrono::Month::try_from(u8::try_from(current_month.get()).unwrap()).unwrap())}
                            </span>
                            <button
                                on:click={move |_| {
                                    set_current_month.update(|m| {
                                        if *m == 12 {
                                            set_current_year.update(|y| *y += 1);
                                            *m = 1;
                                        } else {
                                            *m += 1;
                                        }
                                    });
                                }}
                            >
                                <span class="text-gray-500 cursor-pointer"><Icon icon=IconId::BiChevronRightRegular /></span>
                            </button>
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
