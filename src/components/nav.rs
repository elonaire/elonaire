use yew::prelude::*;
use yew_icons::{Icon, IconId};
use yew_router::prelude::Link;

use crate::app::Route;

struct NavItem {
    link: Route,
    label: String,
    is_active: bool,
    id: u32,
}

#[function_component(Nav)]
pub fn nav() -> Html {
    let nav_items = use_state(|| {
        vec![
            NavItem {
                link: Route::About,
                label: "About".to_owned(),
                is_active: false,
                id: 0,
            },
            NavItem {
                link: Route::Resume,
                label: "Resume".to_owned(),
                is_active: false,
                id: 1,
            },
            NavItem {
                link: Route::PortfolioRoot,
                label: "Portfolio".to_owned(),
                is_active: false,
                id: 2,
            },
        ]
    });

    html! {
        <nav class="nav">
        <Link<Route> classes={classes!("logo")} to={Route::Home}>
        <img class={classes!("logo-img")} src="img/logo-black.png" alt="logo" />
        </Link<Route>>
        <ul class="nav-list">
        {
            nav_items.iter().map(|nav_item| {
                html!{<li key={nav_item.id} class={classes!("nav-item", if nav_item.is_active { "active" } else { "" })}>
                <Link<Route> to={nav_item.link.clone()}>{nav_item.label.clone()}</Link<Route>>
                    </li>}
            }).collect::<Html>()
        }
        <li class={classes!("nav-item")}><a href="https://blog.techietenka.com/" rel="noreferrer" target="_blank">{ "Blog" }</a></li>
        </ul>
        <ul class="nav-social-list">
        <li class={classes!("nav-item")}><a href="https://www.facebook.com/elonaire/" rel="noreferrer" target="_blank"><Icon icon_id={IconId::BootstrapFacebook}/></a></li>
        <li class={classes!("nav-item")}><a href="https://twitter.com/elonaire" rel="noreferrer" target="_blank"><Icon icon_id={IconId::BootstrapTwitter}/></a></li>
        <li class={classes!("nav-item")}><a href="https://www.linkedin.com/in/elon-aseneka-elonaire/" rel="noreferrer" target="_blank"><Icon icon_id={IconId::BootstrapLinkedin}/></a></li>
        <li class={classes!("nav-item")}><a href="https://www.instagram.com/elonaire95/" rel="noreferrer" target="_blank"><Icon icon_id={IconId::BootstrapInstagram}/></a></li>
        </ul>
        </nav>
    }
}
