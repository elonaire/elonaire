use std::rc::Rc;

use yew::prelude::*;
use yew_router::prelude::*;

use crate::{components::tabs::TabProps, data::models::{blog::BlogPost, user::User}, views::{about::About, blog::Blog, home::Home, portfolio::Portfolio, resume::Resume, blog_post::BlogPostDetails}};

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
}

#[derive(Clone, Debug, PartialEq, Eq, Default)]
pub struct AppState {
    pub user_details: User,
    pub title: String,
    pub description: String,
    pub auto_bio: String,
    pub first_name: String,
    pub middle_name: String,
    pub last_name: String,
    pub date_of_birth: String,
    pub residence: String,
    pub address: String,
    pub email: String,
    pub phone: u32,
    pub portfolio_tabs: Vec<TabProps>,
    pub blog_posts: Vec<BlogPost>,
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
            title: "Software Engineer".to_owned(),
            description: "I am a talented full-stack software engineer, with 6+ years of experience in full-stack development. I have an interest in Game Development and the Internet of Things technology.".to_owned(),
            first_name: "Elon".to_owned(),
            middle_name: "Aseneka".to_owned(),
            last_name: "Idiong'o".to_owned(),
            auto_bio: "I am a talented full-stack software engineer and I have 6+ years of experience in building robust small and enterprise applications. I have built various web applications using Node.js, NestJS, Angular, and React. I have built desktop applications using ElectronJS and mobile applications using React Native. Besides that, I use Figma for application designing and prototyping. I have an interest in Game Development and the Internet of Things.".to_owned(),
            date_of_birth: "".to_owned(),
            residence: "Kenya".to_owned(),
            address: "Unity West, Tatu City".to_owned(),
            email: "elon@techietenka.com".to_owned(),
            phone: 0704730039,
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
