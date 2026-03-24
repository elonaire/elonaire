use leptos::prelude::*;
use leptos_icons::Icon;
use reactive_stores::Store;

use crate::components::general::button::BasicButton;
use crate::data::{
    context::store::{AppStateContext, AppStateContextStoreFields},
    models::general::acl::UserInfoStoreFields,
};
use crate::utils::formatters::Pipe;
use crate::utils::formatters::PipeOption;
use icondata::{BsCamera, BsPencil};

#[component]
pub fn ProfilePage() -> impl IntoView {
    let current_state = expect_context::<Store<AppStateContext>>();
    // let user_profile = current_state.user().user_profile();
    // let auth_status = current_state.user().auth_info();

    view! {
        <div class="min-h-svh bg-contrast-white flex flex-col gap-[40px] display-constraints">

            {
                move || {
                    let user = current_state.user().user_profile().get();
                    let auth_status = current_state.user().auth_info().get();
                    view! {
                        // Avatar card
                        <div class="bg-white rounded-[5px] shadow-sm p-6 mb-4 flex items-center gap-5">
                            <div class="relative w-20 h-20 shrink-0">
                                <img
                                    src=format!("{}?width=300", user.profile_picture.unwrap_or_default())
                                    class="w-20 h-20 rounded-full object-cover border-4 border-white shadow"
                                />
                                <button class="absolute bottom-0 right-0 bg-primary text-white rounded-full p-1.5 shadow flex items-center justify-center">
                                    <Icon icon=BsCamera />
                                </button>
                            </div>
                            <div>
                                <h2 class="text-lg font-semibold">{user.full_name.text(None)}</h2>
                                <p class="text-sm">{auth_status.current_role.text(None)}</p>
                                <p class="text-sm">"Leeds, United Kingdom"</p>
                            </div>
                        </div>

                        // Personal Information card
                        <div class="bg-white rounded-[5px] shadow-sm p-6 mb-4">
                            <div class="flex items-center justify-between mb-4">
                                <h3 class="text-base font-semibold">"Personal Information"</h3>
                                <BasicButton style_ext="bg-primary text-white text-sm font-medium px-4 py-2 rounded-lg hover:bg-secondary transition-colors">
                                    <Icon icon=BsPencil />
                                </BasicButton>
                            </div>
                            <div class="border-t border-light-gray pt-4 grid grid-cols-1 sm:grid-cols-3 gap-6">
                                <div>
                                    <p class="text-xs mb-1">"First Name"</p>
                                    <p class="text-sm font-semibold">{user.first_name.text(None)}</p>
                                </div>
                                <div>
                                    <p class="text-xs mb-1">"Last Name"</p>
                                    <p class="text-sm font-semibold">{user.last_name.text(None)}</p>
                                </div>
                                <div>
                                    <p class="text-xs mb-1">"Date of Birth"</p>
                                    <p class="text-sm font-semibold">{user.dob.text(None)}</p>
                                </div>
                                <div>
                                    <p class="text-xs mb-1">"Email Address"</p>
                                    <p class="text-sm font-semibold">{user.email.text(None)}</p>
                                </div>
                                <div>
                                    <p class="text-xs mb-1">"Phone Number"</p>
                                    <p class="text-sm font-semibold">{user.phone.text(None)}</p>
                                </div>
                                <div>
                                    <p class="text-xs mb-1">"User Role"</p>
                                    <p class="text-sm font-semibold">"Admin"</p>
                                </div>
                            </div>
                        </div>

                        // Address card
                        <div class="bg-white rounded-[5px] shadow-sm p-6">
                            <div class="flex items-center justify-between mb-4">
                                <h3 class="text-base font-semibold">"Address"</h3>
                                <BasicButton style_ext="border border-light-gray text-sm font-medium px-4 py-2 rounded-[5px] hover:bg-gray/20 transition-colors">
                                    <Icon icon=BsPencil />
                                </BasicButton>
                            </div>
                            <div class="border-t border-light-gray pt-4 grid grid-cols-1 sm:grid-cols-3 gap-6">
                                <div>
                                    <p class="text-xs mb-1">"Country"</p>
                                    <p class="text-sm font-semibold">{user.country.text(None)}</p>
                                </div>
                                <div>
                                    <p class="text-xs mb-1">"City"</p>
                                    <p class="text-sm font-semibold">"Leeds, East London"</p>
                                </div>
                                <div>
                                    <p class="text-xs mb-1">"Postal Code"</p>
                                    <p class="text-sm font-semibold">"ERT 1254"</p>
                                </div>
                            </div>
                        </div>
                    }
                }
            }

        </div>
    }
}
