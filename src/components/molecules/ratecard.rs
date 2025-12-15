use icondata as IconId;
use leptos::prelude::*;

use crate::components::{
    forms::select::{SelectInput, SelectOption},
    general::button::BasicButton,
};

#[component]
pub fn RatecardComponent() -> impl IntoView {
    let time_bounds = RwSignal::new(vec![
        SelectOption::new("Monthly", "monthly"),
        SelectOption::new("Hourly", "hourly"),
    ] as Vec<SelectOption>);

    view! {
        <div class="flex flex-col gap-[20px] border-[0.5px] border-light-gray rounded-[5px] text-light-gray min-h-[564px] min-w-[400px]">
            <div class="border-b-[0.5px]">
                <div class="p-[10px] flex flex-row justify-between items-center">
                    <div class="flex flex-col">
                        <h4 class="text-light-gray">Software Engineering</h4>
                        <p class="text-primary font-bold text-2xl"><sup class="text-sm text-light-gray">$</sup>900/mo</p>
                    </div>
                    <div class="basis-1/3">
                        <SelectInput
                        label=""
                        id_attr="time_bounds"
                        name="time_bounds"
                        options=time_bounds
                        ext_input_styles="text-light-gray"
                        />
                    </div>
                </div>
            </div>
            <div class="p-[10px]">
            </div>
            <div class="p-[10px]">
                <BasicButton button_text="Request Service" icon=Some(IconId::BsArrowRight) style_ext="bg-primary text-white" />
            </div>
        </div>
    }
}
