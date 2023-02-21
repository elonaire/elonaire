use std::rc::Rc;

use yew::prelude::*;
use yew_router::prelude::*;

use crate::views::{about::About, home::Home, portfolio::Portfolio, resume::Resume};

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/about")]
    About,
    #[at("/resume")]
    Resume,
    #[at("/portfolio")]
    Portfolio,
    #[not_found]
    #[at("/404")]
    NotFound,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AppState {
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
    pub phone: u32
}

impl Reducible for AppState {
    type Action = AppState;

    fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
        AppState {
            title: action.title,
            description: action.description,
            auto_bio: action.auto_bio,
            first_name: action.first_name,
            middle_name: action.middle_name,
            last_name: action.last_name,
            date_of_birth: action.date_of_birth,
            residence: action.residence,
            address: action.address,
            email: action.email,
            phone: action.phone,
        }
        .into()
    }
}

pub type AppStateContext = UseReducerHandle<AppState>;

pub fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => html! { <Home /> },
        Route::About => html! { <About /> },
        Route::Resume => html! { <Resume /> },
        Route::Portfolio => html! { <Portfolio /> },
        Route::NotFound => html! { <h1>{ "404" }</h1> },
    }
}

#[function_component(App)]
pub fn app() -> Html {
    let state = use_reducer(|| {
        AppState {
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
            phone: 0704730039
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
