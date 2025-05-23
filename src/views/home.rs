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
    },
    modal::modal::{BasicModal, UseCase},
};
use icondata as IconId;
use leptos::ev;
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

    let onclick_primary = Callback::new(move |_: ev::MouseEvent| {
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
            <div class="font-mono flex flex-col min-h-screen">
        <BasicModal title="Can I confirm this?".to_string() is_open=modal_open use_case=UseCase::Confirmation on_click_primary=onclick_primary on_cancel=on_cancel disable_auto_close=false ><div><p>"Hey I am just a Nerd tryna make it. Have pity on me Rust."</p></div></BasicModal>
                <div class="flex flex-col m-auto">
                <InputField field_type={InputFieldType::Text} name={"name".to_string()} />
                <DatePicker name={"dob".to_string()} />
                <RadioInputField label={"Male".to_string()} name={"gender".to_string()} id_attr={"male".to_string()}><span>"Comeon"</span></RadioInputField>
                <SelectInput
                            initial_value={"option1".to_string()}
                            label={"Time Zone".to_string()}
                            name={"timezone".to_string()}
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
                                        initial_value="Initial text".to_string()
                                        label="Description".to_string()
                                        name="description".to_string()
                                        // input_node_ref=Some(input_ref)
                                        required=true
                                        placeholder="Enter your description...".to_string()
                                        // oninput=Some(oninput)
                                        ext_input_styles="bg-gray-100".to_string()
                                    />
                                    <ToggleSwitch
                                                    active=active
                                                    on_toggle=on_toggle
                                                    label_active="Enabled".to_string()
                                                    label_inactive="Disabled".to_string()
                                                />

                                                <ButtonGroup style_ext="font-bold bg-primary text-white hover:bg-secondary".to_string()>
                                                    <BasicButton
                                                                    button_text="First".to_string()
                                                                    // style_ext="bg-blue-600 hover:bg-red-800".to_string()
                                                                    // onclick=onclick.clone()
                                                                    icon=Some(IconId::AiCheckCircleOutlined)
                                                                    icon_before=true
                                                                />
                                                                <BasicButton
                                                                    button_text="Second".to_string()
                                                                    // style_ext="bg-blue-600 hover:bg-blue-800".to_string()
                                                                    // onclick=onclick.clone()
                                                                    icon=Some(IconId::BsXCircle)
                                                                    icon_before=false
                                                                />
                                                                <BasicButton
                                                                    button_text="Third".to_string()
                                                                    // style_ext="bg-blue-600 hover:bg-blue-800".to_string()
                                                                    // onclick=onclick
                                                                    disabled=true
                                                                />
                                                            </ButtonGroup>

                    <Badge text="2".to_string() ><span>"Notifications"</span></Badge>
                    <Accordion title="Elonaire".to_string() icon=|| view! {<Icon icon=IconId::BsNodePlusFill />} >
                        <p>"Hey there, I am Mr Elonaire!"</p>
                    </Accordion>
                    <Popover display_item=|| view!{ <p>"Elonaire here"</p> } showing=popover_open on_click_toggle=toggle_popover_handler >
                        <div class="flex flex-row">
                        <span class="text-gray-600">"Tenka"</span>
                            <img src="https://encrypted-tbn0.gstatic.com/images?q=tbn:ANd9GcRij6dtiHizH96qpCOe8WeXXP3yLyQJkPdGVg&s" />
                        </div>
                    </Popover>
                </div>
            </div>
        </main>
    }
}
