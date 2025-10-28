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
    data::models::general::acl::AppStateContext,
    views::{
        dashboard::{
            blog::{Blog, BlogList, CreateBlog},
            home::DashboardHome,
            layout::DashboardLayout,
            portfolio::{CreatePortfolio, Portfolio, PortfolioList},
            professional_details::{
                CreateProfessionalDetail, ProfessionalDetails, ProfessionalDetailsList,
            },
            resume::{CreateResumeItem, Resume, ResumeItemsList},
            roles::{CreateRole, Roles, RolesList},
            skills::{CreateSkill, Skills, SkillsList},
            user_services::{CreateUserService, UserService, UserServicesList},
            users::{CreateUser, Users, UsersList},
        },
        home::Home,
        login::SignIn,
    },
};

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
                    <ParentRoute path=path!("/dashboard") view=|| view! { <ProtectedRoute><DashboardLayout /></ProtectedRoute> }>
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
                        <ParentRoute path=path!("/blog") view=Blog>
                            <Route path=path!("") view=BlogList />
                            <Route path=path!("create") view=CreateBlog />
                        </ParentRoute>
                        <ParentRoute path=path!("/users") view=Users>
                            <Route path=path!("") view=UsersList />
                            <Route path=path!("create") view=CreateUser />
                        </ParentRoute>
                        <ParentRoute path=path!("/roles") view=Roles>
                            <Route path=path!("") view=RolesList />
                            <Route path=path!("create") view=CreateRole />
                        </ParentRoute>
                        <Route path=path!("") view=DashboardHome />
                    </ParentRoute>
                    <Route path=StaticSegment("/sign-in") view=SignIn/>
                </Routes>
            </Router>
        </ErrorBoundary>
    }
}
