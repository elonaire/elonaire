// Pull in the ACL schema we registered in build.rs
#[cynic::schema("acl")]
mod schema {}

#[derive(cynic::QueryFragment, Debug)]
#[cynic(
    schema = "acl",
    graphql_type = "Mutation",
    variables = "UserLoginsInputFields"
)]
pub struct SignInMutation {
    #[arguments(rawUserDetails: $raw_user_details)]
    pub sign_in: Option<AuthDetails>,
}

#[derive(cynic::QueryFragment, Debug)]
pub struct AuthDetails {
    pub token: Option<String>,
    pub url: Option<String>,
}

#[derive(cynic::QueryVariables, Debug)]
pub struct UserLoginsInputFields {
    pub raw_user_details: UserLoginsInput,
}

#[derive(cynic::InputObject, Debug)]
pub struct UserLoginsInput {
    pub user_name: Option<String>,
    pub password: Option<String>,
    pub oauth_client: Option<OauthClientName>,
}

#[derive(cynic::Enum, Clone, Copy, Debug)]
#[cynic(rename_all = "None")]
pub enum OauthClientName {
    Github,
    Google,
}
