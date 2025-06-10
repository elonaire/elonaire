use crate::components::{
    forms::{
        datepicker::DatePicker,
        input::{InputField, InputFieldType},
        radio_input::RadioInputField,
        select::{SelectInput, SelectOption},
        textarea::Textarea,
        toggle_switch::ToggleSwitch,
    },
    general::{
        accordion::Accordion,
        badge::Badge,
        button::{BasicButton, ButtonGroup},
        popover::Popover,
        stepper::{Step, Stepper, StepperLabel},
        table::data_table::DataTable,
        tag::LabelTag,
    },
    modal::modal::{BasicModal, UseCase},
    schemas::{mock_data::database::get_transactions, props::ColorTemperature},
};
use icondata as IconId;
use leptos::prelude::*;
use leptos_meta::*;

#[island]
pub fn Home() -> impl IntoView {
    let (modal_open, set_modal_open) = signal(true);
    let (popover_open, set_popover_open) = signal(false);
    let table_data = RwSignal::new(get_transactions());
    let switch_active = RwSignal::new(true);
    let kitchen_switch_active = RwSignal::new(false);

    let onclick_primary = Callback::new(move |_| {
        set_modal_open.set(false);
    });

    let on_cancel = Callback::new(move |value: bool| {
        set_modal_open.set(value);
    });

    let toggle_popover_handler = Callback::new(move |value: bool| {
        set_popover_open.set(value);
    });

    view! {
        <Title text="Techie Tenka"/>
        <main>
            <div class="min-h-screen m-2">
        <BasicModal title="Can I confirm this?".to_string() is_open=modal_open use_case=UseCase::Confirmation on_click_primary=onclick_primary on_cancel=on_cancel disable_auto_close=false ><div><p>"Hey I am just a Nerd tryna make it. Have pity on me Rust."</p></div></BasicModal>
                <div class="flex flex-col m-auto">
                <InputField field_type=InputFieldType::Text name="name" />
                <DatePicker name="dob_lone" />
                <RadioInputField label="Male" id_attr="male_lone"><span>"Comeon"</span></RadioInputField>
                <SelectInput
                            initial_value="est"
                            label="Time Zone"
                            name="timezone"
                            required=true
                            options=vec![
                                SelectOption::new("", "--Select Timezone"),
                                SelectOption::new("utc", "UTC"),
                                SelectOption::new("est", "EST"),
                            ]
                />
                    <Textarea
                                        initial_value="Initial text"
                                        label="Description"
                                        name="description"
                                        required=true
                                        placeholder="Enter your description..."
                                        ext_input_styles="bg-gray-100"
                                    />
                                    <ToggleSwitch
                                                    active=switch_active
                                                    label_active="Enabled"
                                                    label_inactive="Disabled"
                                                    name="status".into()
                                                />

                                                <ButtonGroup style_ext="font-bold bg-primary text-white hover:bg-secondary".to_string()>
                                                    <BasicButton
                                                                    button_text="First"
                                                                    icon=Some(IconId::AiCheckCircleOutlined)
                                                                    icon_before=true
                                                                />
                                                                <BasicButton
                                                                    button_text="Second"
                                                                    icon=Some(IconId::BsXCircle)
                                                                    icon_before=false
                                                                />
                                                                <BasicButton
                                                                    button_text="Third"
                                                                    disabled=true
                                                                />
                                                            </ButtonGroup>

                    <Badge text="2".into() ><span>"Notifications"</span></Badge>
                    <LabelTag label="Failed" color=ColorTemperature::Danger  />
                    <Accordion title="Elonaire" icon=IconId::BsNodePlusFill >
                        <p>"Hey there, I am Mr Elonaire!"</p>
                    </Accordion>
                    <Popover display_item=|| view!{ <p>"Elonaire here"</p> } showing=popover_open on_click_toggle=toggle_popover_handler >
                        <div class="flex flex-row">
                        <span class="text-gray-600">"Tenka"</span>
                            <img src="https://encrypted-tbn0.gstatic.com/images?q=tbn:ANd9GcRij6dtiHizH96qpCOe8WeXXP3yLyQJkPdGVg&s" />
                        </div>
                    </Popover>
                    <DataTable data=table_data editable=true deletable=true />
                    <Stepper step_labels=vec![StepperLabel::new("First", Some(IconId::AiFileAddOutlined)), StepperLabel::new("Second", None), StepperLabel::new("Third", None)] is_linear=true final_button_text="Finish">
                        <Step>
                            <p>"First step"</p>
                            <InputField field_type=InputFieldType::Text name="user_name" label="User Name" required=true />
                            <InputField field_type=InputFieldType::Email name="email" label="Email" autocomplete="on" required=true />

                            <SelectInput
                                        initial_value="utc"
                                        label="Time Zone"
                                        name="timezone"
                                        required=true
                                        options=vec![
                                            SelectOption::new("", "--Select Timezone"),
                                            SelectOption::new("utc", "UTC"),
                                            SelectOption::new("est", "EST"),
                                        ]
                            />
                            <RadioInputField required=true label="Male" name="gender" initial_value="male" id_attr="male" />
                            <RadioInputField required=true label="Female" name="gender" initial_value="female" id_attr="female" />
                            <Textarea
                                                initial_value="Initial text"
                                                label="Description"
                                                name="description"
                                                required=true
                                                placeholder="Enter your description..."
                                                ext_input_styles="bg-gray-100"
                                            />
                        </Step>
                        <Step>
                            <p>"Second step"</p>
                            <InputField field_type=InputFieldType::Text name="first_name" label="First Name" required=true />
                            <ToggleSwitch
                                            label="Toggle Kitchen Lights"
                                            active=kitchen_switch_active
                                            name="kitchen_lights".into()
                                            id_attr="kitchen_lights"
                                            required=true
                                        />
                            <DatePicker id_attr="step2_dob" name="dob" label="Date of Birth" required=true />
                        </Step>
                        <Step>
                            <p>"Third step"</p>
                        </Step>
                    </Stepper>
                </div>
            </div>
        </main>
    }
}
