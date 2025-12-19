use reactive_stores::Store;

use crate::data::models::{
    general::acl::UserInfo,
    graphql::{acl::User, shared::UserService},
};

#[derive(Clone, Debug, Default, Store)]
pub struct AppStateContext {
    user: UserInfo,
    site_owner_info: User,
    services: Vec<UserService>,
}
