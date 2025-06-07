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
// use leptos::ev;
use leptos::prelude::*;
use leptos_icons::Icon;
use leptos_meta::*;

#[island]
pub fn Home() -> impl IntoView {
    let (active, set_active) = signal(false);
    let on_toggle = Callback::new(move |new_active: bool| {
        set_active.set(new_active);
    });
    let (modal_open, set_modal_open) = signal(true);
    let (popover_open, set_popover_open) = signal(false);
    let table_data = RwSignal::new(get_transactions());

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
                <InputField field_type=InputFieldType::Text name="name".into() />
                <DatePicker name="dob".into() />
                <RadioInputField label="Male".into() name="gender".into() id_attr="male".into()><span>"Comeon"</span></RadioInputField>
                <SelectInput
                            initial_value="option1".into()
                            label="Time Zone".into()
                            name="timezone".into()
                            required=true
                            options=vec![
                                SelectOption {
                                    value: "".to_string(),
                                    label: "--Select Timezone".to_string(),
                                },
                                SelectOption {
                                    value: "utc".to_string(),
                                    label: "UTC".to_string(),
                                },
                                SelectOption {
                                    value: "est".to_string(),
                                    label: "EST".to_string(),
                                },
                            ]
                />
                    <Textarea
                                        initial_value="Initial text".into()
                                        label="Description".into()
                                        name="description".into()
                                        required=true
                                        placeholder="Enter your description...".into()
                                        ext_input_styles="bg-gray-100".into()
                                    />
                                    <ToggleSwitch
                                                    active=active
                                                    on_toggle=on_toggle
                                                    label_active="Enabled".into()
                                                    label_inactive="Disabled".into()
                                                    name="status".into()
                                                />

                                                <ButtonGroup style_ext="font-bold bg-primary text-white hover:bg-secondary".to_string()>
                                                    <BasicButton
                                                                    button_text="First".into()
                                                                    icon=Some(IconId::AiCheckCircleOutlined)
                                                                    icon_before=true
                                                                />
                                                                <BasicButton
                                                                    button_text="Second".into()
                                                                    icon=Some(IconId::BsXCircle)
                                                                    icon_before=false
                                                                />
                                                                <BasicButton
                                                                    button_text="Third".into()
                                                                    disabled=Memo::new(move |_| true)
                                                                />
                                                            </ButtonGroup>

                    <Badge text="2".into() ><span>"Notifications"</span></Badge>
                    <LabelTag label="Failed".into() color=ColorTemperature::Danger  />
                    <Accordion title="Elonaire".into() icon=IconId::BsNodePlusFill >
                        <p>"Hey there, I am Mr Elonaire!"</p>
                    </Accordion>
                    <Popover display_item=|| view!{ <p>"Elonaire here"</p> } showing=popover_open on_click_toggle=toggle_popover_handler >
                        <div class="flex flex-row">
                        <span class="text-gray-600">"Tenka"</span>
                            <img src="https://encrypted-tbn0.gstatic.com/images?q=tbn:ANd9GcRij6dtiHizH96qpCOe8WeXXP3yLyQJkPdGVg&s" />
                        </div>
                    </Popover>
                    <DataTable data=table_data editable=true deletable=true />
                    <Stepper step_labels=vec![StepperLabel::new("First", Some(IconId::AiFileAddOutlined)), StepperLabel::new("Second", None), StepperLabel::new("Third", None)] final_button_text="Finish".into()>
                        <Step>
                            <p>"First step"</p>
                            <InputField field_type=InputFieldType::Text name="user_name".into() label="User Name".into() required=true />
                            <InputField field_type=InputFieldType::Email name="email".into() label="Email".into() required=true />

                            <SelectInput
                                        initial_value="".into()
                                        label="Time Zone".into()
                                        name="timezone".into()
                                        required=true
                                        options=vec![
                                            SelectOption {
                                                value: "".into(),
                                                label: "--Select Timezone".into(),
                                            },
                                            SelectOption {
                                                value: "utc".into(),
                                                label: "UTC".into(),
                                            },
                                            SelectOption {
                                                value: "est".into(),
                                                label: "EST".into(),
                                            },
                                        ]
                            />
                            <RadioInputField required=true label="Male".into() name="gender".into() initial_value=Memo::new(move |_|"male".into()) id_attr="male".into() />
                            <RadioInputField required=true label="Female".into() name="gender".into() initial_value=Memo::new(move |_|"female".into()) id_attr="female".into() />
                            <Textarea
                                                initial_value="Initial text".into()
                                                label="Description".into()
                                                name="description".into()
                                                required=true
                                                placeholder="Enter your description...".into()
                                                ext_input_styles="bg-gray-100".into()
                                            />
                        </Step>
                        <Step>
                            <p>"Second step"</p>
                            <DatePicker name="dob".into() required=true />
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
