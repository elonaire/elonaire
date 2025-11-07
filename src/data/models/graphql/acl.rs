use reactive_stores::Store;
use serde::{Deserialize, Serialize};

use crate::{
    data::models::general::acl::{AuthDetails, OauthClientName},
    utils::custom_traits::EnumerableEnum,
};

#[derive(Debug, Deserialize, Serialize)]
pub struct UserLoginsInput {
    #[serde(rename = "userName", alias = "user_name")]
    pub user_name: Option<String>,
    pub password: Option<String>,
    #[serde(rename = "oauthClient", alias = "oauth_client")]
    pub oauth_client: Option<OauthClientName>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct SignInResponse {
    #[serde(rename = "signIn")]
    pub sign_in: Option<AuthDetails>, // this is the return type expected from the API on success
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SignInVars {
    #[serde(rename = "rawUserDetails")]
    pub raw_user_details: UserLoginsInput,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AuthStatus {
    #[serde(rename = "isAuth")]
    pub is_auth: bool,
    pub sub: String,
    #[serde(rename = "currentRole")]
    pub current_role: String,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize, Store)]
#[allow(dead_code)]
pub struct User {
    pub id: Option<String>,
    #[serde(rename = "userName", alias = "user_name")]
    pub user_name: Option<String>,
    #[serde(rename = "firstName", alias = "first_name")]
    pub first_name: Option<String>,
    #[serde(rename = "middleName", alias = "middle_name")]
    pub middle_name: Option<String>,
    #[serde(rename = "lastName", alias = "last_name")]
    pub last_name: Option<String>,
    #[serde(rename = "fullName", alias = "full_name")]
    pub full_name: Option<String>,
    pub age: Option<u32>,
    pub gender: Option<Gender>,
    pub dob: Option<String>,
    pub email: String,
    pub country: Option<String>,
    pub phone: Option<String>,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
    pub status: Option<AccountStatus>,
    #[serde(rename = "oauthClient", alias = "oauth_client")]
    pub oauth_client: Option<OauthClientName>,
    #[serde(rename = "oauthUserId", alias = "oauth_user_id")]
    pub oauth_user_id: Option<String>,
    #[serde(rename = "profilePicture", alias = "profile_picture")]
    pub profile_picture: Option<String>,
    pub bio: Option<String>,
    pub website: Option<String>,
    pub address: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize, Copy, Eq, PartialEq)]
pub enum Gender {
    Male,
    Female,
    Other,
}

#[derive(Clone, Debug, Serialize, Deserialize, Copy, Eq, PartialEq, Default)]
pub enum AccountStatus {
    Active,
    #[default]
    Inactive,
    Suspended,
    Deleted,
}

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct UserInput {
    #[serde(rename = "userName", alias = "user_name")]
    pub user_name: Option<String>,
    #[serde(rename = "firstName", alias = "first_name")]
    pub first_name: Option<String>,
    #[serde(rename = "middleName", alias = "middle_name")]
    pub middle_name: Option<String>,
    #[serde(rename = "lastName", alias = "last_name")]
    pub last_name: Option<String>,
    pub gender: Option<Gender>,
    pub dob: Option<String>,
    pub email: String,
    pub country: Option<String>,
    pub phone: Option<String>,
    pub password: String,
    #[serde(rename = "profilePicture", alias = "profile_picture")]
    pub profile_picture: Option<String>,
    pub bio: Option<String>,
    pub website: Option<String>,
    pub address: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct FetchUsersResponse {
    #[serde(rename = "fetchUsers")]
    pub fetch_users: Option<Vec<User>>, // this is the return type expected from the API on success
}

#[derive(Debug, Deserialize, Serialize)]
pub struct SignUpResponse {
    #[serde(rename = "signUp")]
    pub sign_up: Option<User>, // this is the return type expected from the API on success
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SignUpVars {
    pub user: UserInput,
}

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct RoleInput {
    #[serde(rename = "roleName", alias = "role_name")]
    pub role_name: String,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize, Store)]
pub struct SystemRole {
    pub id: Option<String>,
    #[serde(rename = "roleName", alias = "role_name")]
    pub role_name: Option<String>,
    #[serde(rename = "createdBy", alias = "created_by")]
    pub created_by: Option<String>,
    #[serde(rename = "createdAt", alias = "created_at")]
    pub created_at: Option<String>,
    #[serde(rename = "isAdmin", alias = "is_admin")]
    pub is_admin: Option<bool>,
    #[serde(rename = "isDefault", alias = "is_default")]
    pub is_default: Option<bool>,
    #[serde(rename = "isSuperAdmin", alias = "is_super_admin")]
    pub is_super_admin: Option<bool>,
    #[serde(rename = "updatedAt", alias = "updated_at")]
    pub updated_at: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct FetchSystemRolesResponse {
    #[serde(rename = "fetchSystemRoles")]
    pub fetch_system_roles: Option<Vec<SystemRole>>, // this is the return type expected from the API on success
}

#[derive(Debug, Deserialize, Serialize)]
pub struct CreateSystemRoleResponse {
    #[serde(rename = "createSystemRole")]
    pub create_system_role: Option<SystemRole>, // this is the return type expected from the API on success
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateSystemRoleVars {
    #[serde(rename = "roleInput")]
    pub role_input: RoleInput,
    #[serde(rename = "roleMetadata")]
    pub role_metadata: RoleMetadata,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct RoleMetadata {
    #[serde(rename = "roleType", alias = "role_type")]
    pub role_type: RoleType,
    #[serde(rename = "organizationId", alias = "organization_id")]
    pub organization_id: Option<String>,
    #[serde(rename = "departmentId", alias = "department_id")]
    pub department_id: Option<String>,
    #[serde(rename = "permissionIds", alias = "permission_ids")]
    pub permission_ids: Option<Vec<String>>,
}

#[derive(Clone, Debug, Serialize, Deserialize, Copy, Eq, PartialEq, Default)]
pub enum RoleType {
    Admin,
    #[default]
    Other,
}

impl EnumerableEnum for RoleType {
    fn variants_slice() -> Vec<String> {
        vec![
            String::new(),
            format!("{:?}", Self::Admin),
            format!("{:?}", Self::Other),
        ]
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct OrganizationInput {
    #[serde(rename = "orgName", alias = "org_name")]
    pub org_name: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Organization {
    pub id: Option<String>,
    #[serde(rename = "orgName")]
    pub org_name: Option<String>,
    #[serde(rename = "createdBy")]
    pub created_by: Option<String>,
    #[serde(rename = "createdAt")]
    pub created_at: Option<String>,
    #[serde(rename = "updatedAt")]
    pub updated_at: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct FetchOrganizationsResponse {
    #[serde(rename = "fetchOrganizations")]
    pub fetch_organizations: Option<Vec<Organization>>, // this is the return type expected from the API on success
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DepartmentInput {
    #[serde(rename = "depName", alias = "dep_name")]
    pub dep_name: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DepartmentInputMetadata {
    #[serde(rename = "organizationId", alias = "organization_id")]
    pub organization_id: Option<String>,
    #[serde(rename = "departmentId", alias = "department_id")]
    pub department_id: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Department {
    pub id: Option<String>,
    #[serde(rename = "depName")]
    pub dep_name: Option<String>,
    #[serde(rename = "createdBy")]
    pub created_by: Option<String>,
    #[serde(rename = "createdAt")]
    pub created_at: Option<String>,
    #[serde(rename = "updatedAt")]
    pub updated_at: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct FetchDepartmentsResponse {
    #[serde(rename = "fetchDepartments")]
    pub fetch_departments: Option<Vec<Department>>, // this is the return type expected from the API on success
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Permission {
    pub id: Option<String>,
    pub name: Option<String>,
    pub resource: Option<String>,
    pub is_admin: Option<bool>,
    pub is_super_admin: Option<bool>,
    #[serde(rename = "createdBy")]
    pub created_by: Option<String>,
    #[serde(rename = "createdAt")]
    pub created_at: Option<String>,
    #[serde(rename = "updatedAt")]
    pub updated_at: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct FetchPermissionsResponse {
    #[serde(rename = "fetchCurrentRolePermissions")]
    pub fetch_current_role_permissions: Option<Vec<Permission>>, // this is the return type expected from the API on success
}
