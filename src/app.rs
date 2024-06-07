use std::rc::Rc;

use yew::prelude::*;
use yew_router::prelude::*;

use crate::{components::tabs::TabProps, data::models::{blog::BlogPost, resource::{UserProfessionalInfo, UserResources}, user::User}, views::{about::About, blog::Blog, blog_post::BlogPostDetails, home::Home, portfolio::Portfolio, resume::Resume}};

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/about")]
    About,
    #[at("/resume")]
    Resume,
    #[at("/blog")]
    BlogRoot,
    #[at("/blog/*")]
    BlogMain,
    #[at("/portfolio")]
    PortfolioRoot,
    #[at("/portfolio/*")]
    PortfolioMain,
    #[not_found]
    #[at("/404")]
    NotFound,
}

#[derive(Clone, Routable, PartialEq, Debug, Eq)]
pub enum BlogRoute {
    #[at("/blog")]
    Blog,
    #[at("/blog/read/:id")]
    BlogPostDetails { id: String },
    #[not_found]
    #[at("/blog/404")]
    NotFound,
}

#[derive(Clone, Routable, PartialEq, Debug, Eq)]
pub enum PortfolioRoute {
    #[at("/portfolio")]
    Portfolio,
    #[at("/portfolio/details/:id")]
    Projects { id: String },
    #[not_found]
    #[at("/portfolio/404")]
    NotFound,
}

pub enum StateAction {
    UpdateUserInfo(User),
    UpdatePortfolioTabs(Vec<TabProps>),
    UpdateBlogPosts(Vec<BlogPost>),
    UpdateUserResources(UserResources),
    UpdateActiveProfessionalInfo(UserProfessionalInfo),
}

#[derive(Clone, Debug, PartialEq, Eq, Default)]
pub struct AppState {
    pub user_details: User,
    pub portfolio_tabs: Vec<TabProps>,
    pub blog_posts: Vec<BlogPost>,
    pub user_resources: UserResources,
    pub active_professional_info: UserProfessionalInfo,
}

impl Reducible for AppState {
    type Action = StateAction;

    fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
        let updated_state = match action {
            StateAction::UpdateUserInfo(user) => {
                AppState {
                    user_details: user,
                    ..self.as_ref().clone()
                }
            }

            StateAction::UpdatePortfolioTabs(tabs) => {
                AppState {
                    portfolio_tabs: tabs,
                    ..self.as_ref().clone()
                }
            }

            StateAction::UpdateBlogPosts(posts) => {
                AppState {
                    blog_posts: posts,
                    ..self.as_ref().clone()
                }
            }

            StateAction::UpdateUserResources(resources) => {
                AppState {
                    user_resources: resources,
                    ..self.as_ref().clone()
                }
            }

            StateAction::UpdateActiveProfessionalInfo(info) => {
                AppState {
                    active_professional_info: info,
                    ..self.as_ref().clone()
                }
            }
        };
        
        Self { ..updated_state }.into()
    }
}

pub type AppStateContext = UseReducerHandle<AppState>;

pub fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => html! { <Home /> },
        Route::About => html! { <About /> },
        Route::Resume => html! { <Resume /> },
        Route::BlogRoot | Route::BlogMain => html! { <Switch<BlogRoute> render={blog_switch} /> },
        Route::PortfolioRoot | Route::PortfolioMain => html! { <Switch<PortfolioRoute> render={portfolio_switch} />  },
        Route::NotFound => html! { <h1>{ "404" }</h1> },
    }
}

pub fn blog_switch(routes: BlogRoute) -> Html {
    match routes {
        BlogRoute::Blog => html! { <Blog /> },
        BlogRoute::BlogPostDetails { id } => html! { <BlogPostDetails {id} /> },
        BlogRoute::NotFound => html! {<Redirect<Route> to={Route::NotFound} />}
    }
}

pub fn portfolio_switch(routes: PortfolioRoute) -> Html {
    match routes {
        PortfolioRoute::Portfolio => html! { <Redirect<PortfolioRoute> to={PortfolioRoute::Projects { id: "javascript".to_owned() }} /> },
        PortfolioRoute::Projects { id: _ } => html! { <Portfolio /> },
        PortfolioRoute::NotFound => html! {<Redirect<Route> to={Route::NotFound} />}
    }
}

#[function_component(App)]
pub fn app() -> Html {
    let state = use_reducer(|| {
        AppState {
            user_details: User::default(),
            portfolio_tabs: vec![
                TabProps {
                    title: "JavaScript/TypeScript".to_owned(),
                    active: true,
                    url: "javascript".to_owned(),
                },
                TabProps {
                    title: "Rust".to_owned(),
                    active: false,
                    url: "rust".to_owned(),
                },
                TabProps {
                    title: "Databases".to_owned(),
                    active: false,
                    url: "databases".to_owned(),
                },
                TabProps {
                    title: "Cloud".to_owned(),
                    active: false,
                    url: "cloud".to_owned(),
                },
                TabProps {
                    title: "DevOps".to_owned(),
                    active: false,
                    url: "devops".to_owned(),
                },
                TabProps {
                    title: "Mobile".to_owned(),
                    active: false,
                    url: "mobile".to_owned(),
                },
            ],
            blog_posts: vec![],
            user_resources: UserResources::default(),
            active_professional_info: UserProfessionalInfo::default(),
        }
    });

    html! {
        <ContextProvider<AppStateContext> context={state}>
            <BrowserRouter>
                <Switch<Route> render={switch} />
            </BrowserRouter>
        </ContextProvider<AppStateContext>>
    }
}
