use icondata as IconId;
use leptos::prelude::*;
use leptos_meta::*;
use leptos_router::{
    StaticSegment,
    components::{Route, Router, Routes},
};
use reactive_stores::Store;

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
        general::button::{BasicButton, ButtonGroup},
        hocs::protected_route::ProtectedRoute,
    },
    schemas::general::acl::AppStateContext,
    views::login::SignIn,
};

#[component]
pub fn App() -> impl IntoView {
    provide_context(Store::new(AppStateContext::default()));
    provide_meta_context();

    view! {
        // <Link rel="shortcut icon" type_="image/ico" href="/favicon.ico"/>
        <Router>
            <Routes fallback=|| "Page not found.">
                <Route path=StaticSegment("") view=|| view! { <ProtectedRoute><Home /></ProtectedRoute> } />
                <Route path=StaticSegment("/sign-in") view=SignIn/>
            </Routes>
        </Router>
    }
}

#[island]
fn Home() -> impl IntoView {
    let (value, set_value) = signal(0);
    let (active, set_active) = signal(false);
    let on_toggle = Callback::new(move |new_active: bool| {
        set_active.set(new_active);
    });

    view! {
        <Title text="Techie Tenka"/>
        <main>
            <div class="font-mono flex flex-col min-h-screen">
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
                                                    active={active}
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
                </div>
            </div>
        </main>
    }
}
