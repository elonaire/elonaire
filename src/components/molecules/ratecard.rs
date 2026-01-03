use icondata as IconId;
use leptos::task::spawn_local;
use leptos::wasm_bindgen::JsCast;
use leptos::{ev, prelude::*};
use web_sys::{HtmlFormElement, HtmlSelectElement, SubmitEvent};

use crate::components::forms::checkbox::CheckboxInputField;
use crate::components::forms::reactive_form::ReactiveForm;
use crate::components::general::button::ButtonType;
use crate::components::{
    forms::select::{SelectInput, SelectOption},
    general::button::BasicButton,
};
use crate::data::context::shared::fetch_billing_rate;
use crate::data::models::graphql::shared::{
    BillingInterval, BillingIntervalForm, FetchBillingRateVars, ServiceIdsForm, UserService,
};
use crate::utils::custom_traits::EnumerableEnum;
use crate::utils::forms::{deserialize_form_data_to_struct, get_form_data_from_form_ref};

#[component]
pub fn RatecardComponent(
    #[prop(into)] name: RwSignal<String>,
    #[prop(into)] services: RwSignal<Vec<UserService>>,
) -> impl IntoView {
    let services_form_ref = NodeRef::new();
    let billing_interval_form_ref = NodeRef::new();
    let (services_form_is_valid, set_services_form_is_valid) = signal(false);
    let (billing_interval_form_is_valid, set_billing_interval_form_is_valid) = signal(false);
    let (amount, set_amount) = signal(None as Option<f64>);
    let submit_is_disabled =
        Memo::new(move |_| !services_form_is_valid.get() || !billing_interval_form_is_valid.get());
    let success_modal_is_open = RwSignal::new(false);
    let confirm_modal_is_open = RwSignal::new(false);
    let (submission_confirmed, set_submission_confirmed) = signal(false);
    let (is_loading, set_is_loading) = signal(false);

    let billing_interval = RwSignal::new(
        BillingInterval::variants_slice()
            .iter()
            .map(|billing_interval| SelectOption::new(billing_interval, billing_interval))
            .collect::<Vec<SelectOption>>(),
    );
    let (selected_billing_interval, set_selected_billing_interval) = signal("hr");

    Effect::new(move || {
        if services_form_is_valid.get() && billing_interval_form_is_valid.get() {
            spawn_local(async move {
                if let Some(billing_interval_form_data) =
                    get_form_data_from_form_ref(&billing_interval_form_ref)
                {
                    if let Some(services_form_data) =
                        get_form_data_from_form_ref(&services_form_ref)
                    {
                        let deserialized_billing_interval_form_data =
                            deserialize_form_data_to_struct::<BillingIntervalForm>(
                                &billing_interval_form_data,
                                false,
                                None,
                            );
                        let deserialized_services_form_data =
                            deserialize_form_data_to_struct::<ServiceIdsForm>(
                                &services_form_data,
                                false,
                                Some(&["service_ids"]),
                            );

                        if let Some(billing_interval) = deserialized_billing_interval_form_data {
                            if let Some(services) = deserialized_services_form_data {
                                let vars = FetchBillingRateVars {
                                    billing_interval: billing_interval.billing_interval,
                                    service_ids: services.service_ids,
                                };

                                let billing_rate = fetch_billing_rate(vars, None).await;

                                if let Ok(amount_str) = billing_rate {
                                    // Process ratecards data here
                                    set_amount.set(Some(amount_str.parse().unwrap_or(0.0)));
                                }
                            };
                        };
                    };
                };
            });
        };
    });

    Effect::new(move || {
        let target: Option<HtmlFormElement> = billing_interval_form_ref.get();

        if let Some(form) = target {
            set_billing_interval_form_is_valid.set(form.check_validity());
        }
    });

    let handle_services_form_submit = move |ev: SubmitEvent| {
        ev.prevent_default();
        ev.stop_propagation();

        leptos::logging::log!("SubmitEvent fired");

        // Implement logic to show form validity
        let target = ev
            .target()
            .and_then(|t| t.dyn_into::<HtmlFormElement>().ok());

        if let Some(form) = target {
            set_services_form_is_valid.set(form.check_validity());

            if let Some(_submitter) = ev.submitter() {
                confirm_modal_is_open.update(|status| *status = true);
            }
        }
    };

    let handle_billing_interval_form_submit = move |ev: SubmitEvent| {
        ev.prevent_default();
        ev.stop_propagation();

        leptos::logging::log!("SubmitEvent fired");

        // Implement logic to show form validity
        let target = ev
            .target()
            .and_then(|t| t.dyn_into::<HtmlFormElement>().ok());

        if let Some(form) = target {
            set_billing_interval_form_is_valid.set(form.check_validity());

            if let Some(_submitter) = ev.submitter() {
                confirm_modal_is_open.update(|status| *status = true);
            }
        }
    };

    view! {
        <div class="flex flex-col gap-[20px] border-[0.5px] border-light-gray rounded-[5px] text-light-gray min-h-[564px] min-w-[400px]">
            <div class="border-b-[0.5px]">
                <div class="p-[10px] flex flex-row justify-between items-center">
                    <div class="flex flex-col">
                        <h4 class="text-light-gray">{move || name.get()}</h4>
                        <p class="text-primary font-bold text-2xl"><sup class="text-sm text-light-gray">$</sup>{ move ||
                            if let Some(amount) = amount.get() {
                                format!("{:.2}", amount)
                            } else {
                                "_ _".into()
                            }
                        }/{move || selected_billing_interval.get()}</p>
                    </div>
                    <div class="basis-1/3">
                        <ReactiveForm on:submit=handle_billing_interval_form_submit form_ref=billing_interval_form_ref>
                            <SelectInput
                            id_attr="billing_interval"
                            name="billing_interval"
                            options=billing_interval
                            required=true
                            initial_value=RwSignal::new("Hourly".into())
                            ext_input_styles="text-light-gray"
                            onchange=Callback::new(move |ev: ev::Event| {
                                let target = ev
                                    .target()
                                    .and_then(|t| t.dyn_into::<HtmlSelectElement>().ok());

                                if let Some(input_el) = target {
                                    let short_name = match input_el.value().as_str() {
                                        "Monthly" => "mo",
                                        "Hourly" => "hr",
                                        "Weekly" => "wk",
                                        "Annual" => "yr",
                                        _ => "_ _",
                                    };
                                    set_selected_billing_interval.set(short_name);
                                }
                            })
                            />
                        </ReactiveForm>
                    </div>
                </div>
            </div>

            <div class="p-[10px] flex flex-col gap-[10px] text-light-gray text-md">
                <ReactiveForm on:submit=handle_services_form_submit form_ref=services_form_ref>
                    <For
                        each=move || services.get()
                        key=|service| service.id.as_ref().unwrap().clone()
                            children=move |service| {
                            view! {
                                <CheckboxInputField initial_value=RwSignal::new(service.id.as_ref().unwrap().clone()) label=service.title.as_ref().unwrap().clone() id_attr=format!("service-{}", service.id.as_ref().unwrap().clone()) name="service_ids" />
                            }
                        }
                    />
                </ReactiveForm>
            </div>
            <div class="p-[10px] mt-auto">
                <BasicButton button_text="Request Service" icon=Some(IconId::BsArrowRight) style_ext="bg-primary text-contrast-white" disabled=submit_is_disabled />
            </div>
        </div>
    }
}
