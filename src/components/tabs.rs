use yew::prelude::*;
use yew_router::prelude::*;

use crate::{app::{AppStateContext, PortfolioRoute, StateAction}, data::models::resource::UserPortfolioCategory};

#[derive(Clone, PartialEq, Properties)]
pub struct TabsProps {
    pub tabs: Vec<TabProps>,
}

#[derive(Clone, PartialEq, Properties, Debug, Eq)]
pub struct TabProps {
    pub title: String,
    pub active: bool,
    pub url: String,
    pub category: UserPortfolioCategory,
}

impl TabProps {
    pub fn update_active(&mut self, active: bool) {
        self.active = active;
    }
}

#[function_component(Tabs)]
pub fn tabs(props: &TabsProps) -> Html {

    html! {
        <ul class="tabs">
            {
                props.tabs.clone().iter().map(|tab| {
                    html!{<Tab url={tab.url.clone()} category={tab.category.clone()} title={tab.title.clone()} active={tab.active.clone()} />}
                }).collect::<Html>()
            }
        </ul>
    }
}

#[function_component(Tab)]
pub fn tab(props: &TabProps) -> Html {
    let state_ctx_reducer = use_context::<AppStateContext>().unwrap();
    let cl_props = props.clone();
    let navigator = use_navigator().unwrap();
        
    let onclick = Callback::from(move |_| {
        let mut cloned_tabs = state_ctx_reducer.portfolio_tabs.clone();
        // let state_value = state_ctx_reducer.deref().to_owned();

        for tab in cloned_tabs.iter_mut() {
            if tab.title == cl_props.title.clone() {
                tab.update_active(true);
            } else {
                tab.update_active(false);
            };
        };

        state_ctx_reducer.dispatch(StateAction::UpdatePortfolioTabs(cloned_tabs.clone()));

        // log::info!("Clicked on tab: {:?}", cloned_tabs.deref().to_owned());
        navigator.push(&PortfolioRoute::Projects { id: cl_props.url.clone() })
    });

    html! {
        <li {onclick} class={format!("tab {}", if props.active { "active-tab" } else { "" })}>
            { &props.title }
        </li>
    }
}