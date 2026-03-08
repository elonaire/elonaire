use std::fmt::{self, Display};

use reactive_stores::Store;
use serde::{Deserialize, Serialize};

use crate::{
    data::models::general::{
        acl::{AuthDetails, OauthClientName},
        shared::ApiResponse,
    },
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

#[derive(Debug, Deserialize, Serialize, Clone, Default)]
pub struct SignInResponse {
    #[serde(rename = "signIn")]
    pub sign_in: Option<ApiResponse<AuthDetails>>, // this is the return type expected from the API on success
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SignInVars {
    #[serde(rename = "rawUserDetails")]
    pub raw_user_details: UserLoginsInput,
}

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct AuthStatus {
    #[serde(rename = "isAuth")]
    pub is_auth: bool,
    pub sub: String,
    #[serde(rename = "currentRole")]
    pub current_role: String,
    #[serde(rename = "newAccessToken")]
    pub new_access_token: Option<String>,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize, Store, PartialEq, Eq)]
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

#[derive(Debug, Deserialize, Serialize, Clone, Default)]
pub struct FetchUsersResponse {
    #[serde(rename = "fetchUsers")]
    pub fetch_users: Option<ApiResponse<Vec<User>>>, // this is the return type expected from the API on success
}

#[derive(Debug, Deserialize, Serialize, Clone, Default)]
pub struct FetchSiteOwnerResponse {
    #[serde(rename = "fetchSiteOwnerInfo")]
    pub fetch_site_owner_info: Option<ApiResponse<User>>, // this is the return type expected from the API on success
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct FetchSingleUserVars {
    #[serde(rename = "userId")]
    pub user_id: String, // this is the return type expected from the API on success
}

#[derive(Debug, Deserialize, Serialize, Clone, Default)]
pub struct FetchSingleUserResponse {
    #[serde(rename = "fetchSingleUser")]
    pub fetch_single_user: Option<ApiResponse<User>>, // this is the return type expected from the API on success
}

#[derive(Debug, Deserialize, Serialize, Clone, Default)]
pub struct SignUpResponse {
    #[serde(rename = "signUp")]
    pub sign_up: Option<ApiResponse<User>>, // this is the return type expected from the API on success
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

#[derive(Clone, Debug, Default, Serialize, Deserialize, Store, PartialEq, Eq)]
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

#[derive(Debug, Deserialize, Serialize, Clone, Default)]
pub struct FetchSystemRolesResponse {
    #[serde(rename = "fetchSystemRoles")]
    pub fetch_system_roles: Option<ApiResponse<Vec<SystemRole>>>, // this is the return type expected from the API on success
}

#[derive(Debug, Deserialize, Serialize, Clone, Default)]
pub struct CreateSystemRoleResponse {
    #[serde(rename = "createSystemRole")]
    pub create_system_role: Option<ApiResponse<SystemRole>>, // this is the return type expected from the API on success
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
    #[serde(rename = "adminPrivilege", alias = "admin_privilege")]
    pub admin_privilege: AdminPrivilege,
    #[serde(rename = "organizationId", alias = "organization_id")]
    pub organization_id: Option<String>,
    #[serde(rename = "departmentId", alias = "department_id")]
    pub department_id: Option<String>,
    #[serde(rename = "permissionIds", alias = "permission_ids")]
    pub permission_ids: Option<Vec<String>>,
}

#[derive(Clone, Debug, Serialize, Deserialize, Copy, Eq, PartialEq, Default)]
pub enum AdminPrivilege {
    Admin,
    SuperAdmin,
    #[default]
    None,
}

impl EnumerableEnum for AdminPrivilege {
    fn variants_slice() -> Vec<Self> {
        vec![Self::Admin, Self::SuperAdmin, Self::None]
    }
}

impl Display for AdminPrivilege {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SuperAdmin => write!(f, "Super Admin"),
            any_other => write!(f, "{any_other:?}"),
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct OrganizationInput {
    #[serde(rename = "orgName", alias = "org_name")]
    pub org_name: String,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
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

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateOrganizationVars {
    #[serde(rename = "organizationInput", alias = "organization_input")]
    pub organization_input: OrganizationInput,
}

#[derive(Debug, Deserialize, Serialize, Clone, Default)]
pub struct CreateOrganizationResponse {
    #[serde(rename = "createOrganization")]
    pub create_organization: Option<ApiResponse<Organization>>, // this is the return type expected from the API on success
}

#[derive(Debug, Deserialize, Serialize, Clone, Default)]
pub struct FetchOrganizationsResponse {
    #[serde(rename = "fetchOrganizations")]
    pub fetch_organizations: Option<ApiResponse<Vec<Organization>>>, // this is the return type expected from the API on success
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DepartmentInput {
    #[serde(rename = "depName", alias = "dep_name")]
    pub dep_name: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DepartmentMetadata {
    #[serde(rename = "organizationId", alias = "organization_id")]
    pub organization_id: Option<String>,
    #[serde(rename = "departmentId", alias = "department_id")]
    pub department_id: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
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

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateDepartmentVars {
    #[serde(rename = "departmentInput", alias = "department_input")]
    pub department_input: DepartmentInput,
    #[serde(rename = "departmentMetadata", alias = "department_metadata")]
    pub department_metadata: DepartmentMetadata,
}

#[derive(Debug, Deserialize, Serialize, Clone, Default)]
pub struct CreateDepartmentResponse {
    #[serde(rename = "createDepartment")]
    pub create_department: Option<ApiResponse<Department>>, // this is the return type expected from the API on success
}

#[derive(Debug, Deserialize, Serialize, Clone, Default)]
pub struct FetchDepartmentsResponse {
    #[serde(rename = "fetchDepartments")]
    pub fetch_departments: Option<ApiResponse<Vec<Department>>>, // this is the return type expected from the API on success
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ResourceInput {
    pub name: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ResourceMetadata {
    #[serde(rename = "organizationId", alias = "organization_id")]
    pub organization_id: Option<String>,
    #[serde(rename = "departmentId", alias = "department_id")]
    pub department_id: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateResourceVars {
    #[serde(rename = "resourceInput")]
    pub resource_input: ResourceInput,
    #[serde(rename = "resourceMetadata")]
    pub resource_metadata: ResourceMetadata,
}

#[derive(Debug, Deserialize, Serialize, Clone, Default)]
pub struct CreateResourceResponse {
    #[serde(rename = "createResource")]
    pub create_resource: Option<ApiResponse<Resource>>, // this is the return type expected from the API on success
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq, Default)]
pub struct Resource {
    pub id: Option<String>,
    pub name: Option<String>,
    #[serde(rename = "createdBy", alias = "created_by")]
    pub created_by: Option<String>,
    #[serde(rename = "createdAt")]
    pub created_at: Option<String>,
    #[serde(rename = "updatedAt")]
    pub updated_at: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PermissionInput {
    pub name: String,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct PermissionMetadata {
    #[serde(rename = "adminPrivilege", alias = "admin_privilege")]
    pub admin_privilege: AdminPrivilege,
    #[serde(rename = "resourceId", alias = "resource_id")]
    pub resource_id: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreatePermissionVars {
    #[serde(rename = "permissionInput")]
    pub permission_input: PermissionInput,
    #[serde(rename = "permissionMetadata")]
    pub permission_metadata: PermissionMetadata,
}

#[derive(Debug, Deserialize, Serialize, Clone, Default)]
pub struct CreatePermissionResponse {
    #[serde(rename = "createPermission")]
    pub create_permission: Option<ApiResponse<Permission>>, // this is the return type expected from the API on success
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct Permission {
    pub id: Option<String>,
    pub name: Option<String>,
    pub resource: Option<Resource>,
    #[serde(rename = "isAdmin")]
    pub is_admin: Option<bool>,
    #[serde(rename = "isSuperAdmin")]
    pub is_super_admin: Option<bool>,
    #[serde(rename = "createdBy")]
    pub created_by: Option<String>,
    #[serde(rename = "createdAt")]
    pub created_at: Option<String>,
    #[serde(rename = "updatedAt")]
    pub updated_at: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, Clone, Default)]
pub struct FetchPermissionsResponse {
    #[serde(rename = "fetchCurrentRolePermissions")]
    pub fetch_current_role_permissions: Option<ApiResponse<Vec<Permission>>>, // this is the return type expected from the API on success
}

#[derive(Debug, Deserialize, Serialize, Clone, Default)]
pub struct FetchResourcesResponse {
    #[serde(rename = "fetchResources")]
    pub fetch_resources: Option<ApiResponse<Vec<Resource>>>, // this is the return type expected from the API on success
}

#[derive(Debug, Deserialize, Serialize, Clone, Default)]
pub struct CheckAuthResponse {
    #[serde(rename = "checkAuth")]
    pub check_auth: Option<ApiResponse<AuthStatus>>, // this is the return type expected from the API on success
}

#[derive(Debug, Deserialize, Serialize, Clone, Default)]
pub struct SignOutResponse {
    #[serde(rename = "signOut")]
    pub sign_out: Option<ApiResponse<bool>>, // this is the return type expected from the API on success
}
