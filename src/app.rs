use leptos::prelude::*;
use leptos_meta::*;
use leptos_router::{
    StaticSegment,
    components::{ParentRoute, Route, Router, Routes},
    path,
};
use reactive_stores::Store;

use crate::{
    components::general::hocs::protected_route::ProtectedRoute,
    views::{
        dashboard::Dashboard,
        dashboard_home::DashboardHome,
        home::Home,
        portfolio::{CreatePortfolio, Portfolio, PortfolioList},
        professional_details::{
            CreateProfessionalDetail, ProfessionalDetails, ProfessionalDetailsList,
        },
        resume::{CreateResumeItem, Resume, ResumeItemsList},
        skills::{CreateSkill, Skills, SkillsList},
        user_services::{CreateUserService, UserService, UserServicesList},
    },
};
use crate::{schemas::general::acl::AppStateContext, views::login::SignIn};

#[component]
pub fn App() -> impl IntoView {
    provide_context(Store::new(AppStateContext::default()));
    provide_meta_context();

    view! {
        // <Link rel="shortcut icon" type_="image/ico" href="/favicon.ico"/>
        <div id="modal-root"></div>
        <ErrorBoundary
                        // the fallback receives a signal containing current errors
                        fallback=|errors| view! {
                            <div class="error">
                                <p>"Something went wrong: "</p>
                                // we can render a list of errors
                                // as strings, if we'd like
                                <ul>
                                    {move || errors.get()
                                        .into_iter()
                                        .map(|(_, e)| view! { <li>{e.to_string()}</li>})
                                        .collect::<Vec<_>>()
                                    }
                                </ul>
                            </div>
                        }
                    >
            <Router>
                <Routes fallback=|| "Page not found.">
                    <Route path=StaticSegment("") view=Home />
                    // <Route path=StaticSegment("/dashboard") view=|| view! { <ProtectedRoute><Dashboard /></ProtectedRoute> } />
                    <ParentRoute path=path!("/dashboard") view=|| view! { <ProtectedRoute><Dashboard /></ProtectedRoute> }>
                        <ParentRoute path=path!("/portfolio") view=Portfolio>
                            <Route path=path!("") view=PortfolioList />
                            <Route path=path!("create") view=CreatePortfolio />
                        </ParentRoute>
                        <ParentRoute path=path!("/professional-details") view=ProfessionalDetails>
                            <Route path=path!("") view=ProfessionalDetailsList />
                            <Route path=path!("create") view=CreateProfessionalDetail />
                        </ParentRoute>
                        <ParentRoute path=path!("/services") view=UserService>
                            <Route path=path!("") view=UserServicesList />
                            <Route path=path!("create") view=CreateUserService />
                        </ParentRoute>
                        <ParentRoute path=path!("/resume") view=Resume>
                            <Route path=path!("") view=ResumeItemsList />
                            <Route path=path!("create") view=CreateResumeItem />
                        </ParentRoute>
                        <ParentRoute path=path!("/skills") view=Skills>
                            <Route path=path!("") view=SkillsList />
                            <Route path=path!("create") view=CreateSkill />
                        </ParentRoute>
                        <Route path=path!("") view=DashboardHome />
                    </ParentRoute>
                    <Route path=StaticSegment("/sign-in") view=SignIn/>
                </Routes>
            </Router>
        </ErrorBoundary>
    }
}
