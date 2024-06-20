use yew::prelude::*;
use yew_icons::{Icon, IconId};

use crate::{
    app::AppStateContext,
    components::{nav::Nav, loader::Loader, bottom_svg::BottomSvg, cookie_consent::CookieConsent},
    data::context::{user::get_user_by_id, user_resources::get_user_resources},
};

#[function_component(Home)]
pub fn home() -> Html {
    let current_state = use_context::<AppStateContext>().unwrap();
    let state_clone = current_state.clone();
    let resoures_state_clone = current_state.clone();
    let state_clone_for_effects = current_state.clone();

    use_effect(move || {
        wasm_bindgen_futures::spawn_local(async move {
            let user_id = option_env!("MAIN_USER_ID").expect("MAIN_USER_ID env var not set");

            if state_clone_for_effects.user_details.id.is_none() {
                let _user = get_user_by_id(user_id.to_string(), state_clone).await;
            }
            if state_clone_for_effects
                .user_resources
                .professional_info
                .is_none()
            {
                let _user_resources =
                    get_user_resources(user_id.to_string(), resoures_state_clone)
                        .await;
            }
        }); // Await the async block
        || ()
    });

    html! {
            <>
            <header>
                <Nav />
            </header>
            <main class="home-wrapper">
            { if current_state.user_details.id.is_none() || current_state.active_professional_info.occupation.is_none() { html!{ <Loader /> } } else { html!{ } } }
            <div class="home">
            // <BasicModal title={"Test Modal"} is_open={true} use_case={UseCase::Success} />
            <CookieConsent />
            <div class="left">
            <h1>{ "I'm " } <span class="primary-color-text">{current_state.user_details.first_name.clone().unwrap_or("".to_string()) + " "}</span>{ current_state.user_details.middle_name.clone().unwrap_or("".to_string()) + " " +  &current_state.user_details.last_name.clone().unwrap_or("".to_string())}</h1>
            <img class="profile-image" src="https://imagedelivery.net/fa3SWf5GIAHiTnHQyqU8IQ/c9b133e5-fe4c-4899-4aad-f3a5cefe1400/public" alt="profile-image" />
            <p class="description">{ current_state.user_details.bio.clone().unwrap_or("".to_string()) }</p>
            <div class="button-container">
                <a class="button button-primary glow-on-hover" href="https://drive.google.com/uc?export=download&id=1tz2NyMb9NNhN8u9WgHwJC6IbwZ6Y1RfC">{"Download CV "}<Icon width={"1em".to_owned()} height={"1em".to_owned()} icon_id={IconId::BootstrapDownload}/></a>
            </div>
            </div>

            <div class="right">
                <h2 class="title">{ current_state.active_professional_info.occupation.clone() }</h2>
                <ul class="nav-social-list">
                    <li class={classes!("nav-item")}><a href="https://github.com/elonaire/" rel="noreferrer" target="_blank"><Icon icon_id={IconId::BootstrapGithub}/></a></li>
                    <li class={classes!("nav-item")}><a href="https://www.facebook.com/elonaire/" rel="noreferrer" target="_blank"><Icon icon_id={IconId::BootstrapFacebook}/></a></li>
                    <li class={classes!("nav-item")}><a href="https://twitter.com/elonaire" rel="noreferrer" target="_blank"><Icon icon_id={IconId::BootstrapTwitter}/></a></li>
                    <li class={classes!("nav-item")}><a href="https://www.linkedin.com/in/elon-aseneka-elonaire/" rel="noreferrer" target="_blank"><Icon icon_id={IconId::BootstrapLinkedin}/></a></li>
                    <li class={classes!("nav-item")}><a href="https://www.instagram.com/elonaire95/" rel="noreferrer" target="_blank"><Icon icon_id={IconId::BootstrapInstagram}/></a></li>
                </ul>
            </div>
            <BottomSvg />
            </div>
            </main>
            </>
        }
}
