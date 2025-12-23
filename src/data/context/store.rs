use reactive_stores::Store;

use crate::data::models::{
    general::acl::UserInfo,
    graphql::{
        acl::User,
        shared::{UserResume, UserService, UserSkill},
    },
};

#[derive(Clone, Debug, Default, Store, PartialEq, Eq)]
pub struct AppStateContext {
    user: UserInfo,
    site_owner_info: User,
    services: Vec<UserService>,
    resume: Vec<UserResume>,
    skills: Vec<UserSkill>,
}
