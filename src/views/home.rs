use crate::{
    components::{
        forms::{
            datepicker::DatePicker,
            input::{InputField, InputFieldType},
            radio_input::RadioInputField,
            select::{SelectInput, SelectOption},
            textarea::Textarea,
            toggle_switch::ToggleSwitch,
        },
        general::{
            badge::Badge,
            button::{BasicButton, ButtonGroup},
            collapse::{Collapse, Panel, PanelInfo},
            modal::modal::{BasicModal, UseCase},
            popover::Popover,
            stepper::{Step, StepInfo, Stepper},
            table::data_table::DataTable,
            tabs::{Tab, TabInfo, Tabs},
            tag::LabelTag,
        },
        schemas::{mock_data::database::get_transactions, props::ColorTemperature},
    },
    utils::forms::{deserialize_form_data_to_struct, get_form_data_from_form_ref},
};
use icondata as IconId;
use leptos::{html::Form, prelude::*};
use leptos_meta::*;
use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
struct FirstForm {
    pub user_name: String,
    pub email: String,
    pub timezone: String,
    pub gender: String,
    pub description: String,
}

#[island]
pub fn Home() -> impl IntoView {
    let modal_open = RwSignal::new(true);
    let popover_open = RwSignal::new(false);
    let table_data = RwSignal::new(get_transactions());
    let switch_active = RwSignal::new(true);
    let kitchen_switch_active = RwSignal::new(false);
    let stepper_form_refs = RwSignal::new(Vec::new());
    let panel_is_open = RwSignal::new(true);

    let handle_received_form_refs = Callback::new(move |form_refs: Vec<NodeRef<Form>>| {
        stepper_form_refs.update(|prev| *prev = form_refs);
    });

    view! {
        <Title text="Techie Tenka"/>
        <main>
            <div class="min-h-screen m-2">
        <BasicModal title="Can I confirm this?" is_open=modal_open use_case=UseCase::Confirmation disable_auto_close=false ><div><p>"Hey I am just a Nerd tryna make it. Have pity on me Rust."</p></div></BasicModal>
                <div class="flex flex-col m-auto">
                <InputField field_type=InputFieldType::Text name="name" />
                <DatePicker name="dob_lone" />
                <RadioInputField label="Male" id_attr="male_lone"><span>"Comeon"</span></RadioInputField>
                <SelectInput
                            initial_value=""
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
                                                    name="status"
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

                    <Badge text="2" ><span>"Notifications"</span></Badge>
                    <LabelTag label="Failed" color=ColorTemperature::Danger  />
                    <Panel is_open=panel_is_open title="Elonaire" icon=IconId::BsNodePlusFill >
                        <p>"Hey there, I am Mr Elonaire!"</p>
                    </Panel>
                    <Collapse is_accordion=true panel_items=RwSignal::new(vec![
                        PanelInfo::new("title 1", None, RwSignal::new(false), ViewFn::from(move || view!{ <p>"Panel content"</p> })),
                        PanelInfo::new("title 2", None, RwSignal::new(false), ViewFn::from(move || view!{ <p>"Panel content"</p> })),
                        PanelInfo::new("title 3", None, RwSignal::new(false), ViewFn::from(move || view!{ <p>"Panel content"</p> })),
                        PanelInfo::new("title 4", None, RwSignal::new(false), ViewFn::from(move || view!{ <p>"Panel content"</p> }))
                    ]) />
                    <Popover display_item=|| view!{ <p>"Elonaire here"</p> } showing=popover_open>
                        <div class="flex flex-row">
                        <span class="text-gray-600">"Tenka"</span>
                            <img src="https://encrypted-tbn0.gstatic.com/images?q=tbn:ANd9GcRij6dtiHizH96qpCOe8WeXXP3yLyQJkPdGVg&s" />
                        </div>
                    </Popover>
                    <DataTable data=table_data editable=true deletable=true />
                    <Stepper step_labels=RwSignal::new(vec![StepInfo::new("First", Some(IconId::AiFileAddOutlined)), StepInfo::new("Second", None), StepInfo::new("Third", None)]) send_all_form_refs=handle_received_form_refs is_linear=true final_button_text="Finish">
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
                                            name="kitchen_lights"
                                            id_attr="kitchen_lights"
                                            required=true
                                        />
                            <DatePicker id_attr="step2_dob" name="dob" label="Date of Birth" required=true />
                        </Step>
                        <Step>
                            <p>"Third step"</p>
                            { move || {
                                if let Some(first_form_ref) = stepper_form_refs.get().get(0) {
                                    let form_data = get_form_data_from_form_ref(first_form_ref).unwrap();
                                    let data = deserialize_form_data_to_struct::<FirstForm>(&form_data).unwrap();
                                    Some(view! {
                                        <h2 class="text-lg">"First Step Verification"</h2>
                                        <p><strong>"Username: "</strong>{data.user_name}</p>
                                    })
                                } else {
                                    None
                                }
                            }
                            }
                        </Step>
                    </Stepper>

                    <Tabs tab_labels=RwSignal::new(vec![TabInfo::new("First", None), TabInfo::new("Second", None)])>
                        <Tab>
                            <p>"First tab"</p>
                        </Tab>
                        <Tab>
                            <p>"Second tab"</p>
                        </Tab>
                    </Tabs>
                </div>
            </div>
        </main>
    }
}
