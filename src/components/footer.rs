use chrono::Datelike;
use yew::prelude::*;
use yew_icons::{Icon, IconId};
use crate::components::line_separator::LineSeparator;

#[function_component(Footer)]
pub fn footer() -> Html {
    let year = chrono::Utc::now().year();
    html! {
        <div class="footer">
            <div class="socials">
                <ul class="nav-social-list">
                <li class={classes!("nav-item")}><a href="https://www.facebook.com/elonaire/" rel="noreferrer" target="_blank"><Icon width={"3em".to_owned()} height={"2em".to_owned()} icon_id={IconId::BootstrapFacebook}/></a></li>
                <li class={classes!("nav-item")}><a href="https://twitter.com/elonaire" rel="noreferrer" target="_blank"><Icon width={"2em".to_owned()} height={"2em".to_owned()} icon_id={IconId::BootstrapTwitter}/></a></li>
                <li class={classes!("nav-item")}><a href="https://www.linkedin.com/in/elon-aseneka-elonaire/" rel="noreferrer" target="_blank"><Icon width={"2em".to_owned()} height={"2em".to_owned()} icon_id={IconId::BootstrapLinkedin}/></a></li>
                <li class={classes!("nav-item")}><a href="https://www.instagram.com/elonaire95/" rel="noreferrer" target="_blank"><Icon width={"2em".to_owned()} height={"2em".to_owned()} icon_id={IconId::BootstrapInstagram}/></a></li>
                </ul>        
            </div>
            <div class="separator-container">
                <LineSeparator />
            </div>
            <div class="footer-text">
                <p>{format!("© {} Techie Tenka. All rights reserved.", year)}</p>
            </div>
        </div>
    }
}
