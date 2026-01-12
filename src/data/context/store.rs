use reactive_stores::Store;

use crate::data::models::{
    general::acl::UserInfo,
    graphql::{
        acl::{Department, Organization, Permission, Resource, SystemRole, User},
        shared::{
            Currency, Ratecard, ServiceRate, UserPortfolio, UserProfessionalInfo, UserResume,
            UserService, UserSkill,
        },
    },
};

#[derive(Clone, Debug, Default, Store, PartialEq, Eq)]
pub struct AppStateContext {
    user: UserInfo,
    site_owner_info: User,
    services: Vec<UserService>,
    resume: Vec<UserResume>,
    skills: Vec<UserSkill>,
    portfolio: Vec<UserPortfolio>,
    departments: Vec<Department>,
    organizations: Vec<Organization>,
    permissions: Vec<Permission>,
    resources: Vec<Resource>,
    professions: Vec<UserProfessionalInfo>,
    roles: Vec<SystemRole>,
    service_rates: Vec<ServiceRate>,
    currencies: Vec<Currency>,
    ratecards: Vec<Ratecard>,
}
