use std::collections::HashMap;

use gql_client::GraphQLErrorMessage;
use leptos::prelude::*;

use crate::{
    data::{
        context::store::{AppStateContext, AppStateContextStoreFields},
        models::graphql::{
            acl::{
                AuthStatus, CheckAuthResponse, FetchDepartmentsResponse,
                FetchOrganizationsResponse, FetchPermissionsResponse, FetchResourcesResponse,
                FetchSingleUserResponse, FetchSingleUserVars, FetchSystemRolesResponse, User,
            },
            shared::{
                BlogPost, FetchBillingRateResponse, FetchBillingRateVars, FetchBlogPostsResponse,
                FetchBlogPostsVars, FetchCurrenciesResponse, FetchRatecardsResponse,
                FetchServiceRatesResponse, FetchServiceRequestsResponse,
                FetchSingleBlogPostResponse, FetchSingleBlogPostVars, FetchSiteResourcesResponse,
            },
        },
    },
    utils::{
        errors::handle_graphql_errors,
        graphql_client::{perform_mutation_or_query_with_vars, perform_query_without_vars},
    },
};
use reactive_stores::Store;

const ACL_SERVICE_API: Option<&str> = option_env!("ACL_SERVICE_API");
const SHARED_SERVICE_API: Option<&str> = option_env!("SHARED_SERVICE_API");
const PAYMENTS_SERVICE_API: Option<&str> = option_env!("PAYMENTS_SERVICE_API");

pub async fn fetch_services(
    store: &Store<AppStateContext>,
    headers: Option<&HashMap<String, String>>,
) -> Result<(), Vec<GraphQLErrorMessage>> {
    let fetch_services_query = r#"
           query FetchSiteResources {
                fetchSiteResources {
                    data {
                        services {
                            title
                            description
                            thumbnail
                            id
                        }
                    }
                    metadata {
                        newAccessToken
                        requestId
                    }
                }
           }
       "#;

    let Some(shared_service_api) = SHARED_SERVICE_API else {
        return Err(vec![]);
    };

    let fetch_services_response = perform_query_without_vars::<FetchSiteResourcesResponse>(
        headers,
        shared_service_api,
        fetch_services_query,
    )
    .await;

    match fetch_services_response.get_data() {
        Some(data) => {
            let owned_data = data
                .fetch_site_resources
                .as_ref()
                .unwrap_or(&Default::default())
                .get_data()
                .services
                .as_ref()
                .unwrap_or(&Default::default())
                .to_vec();
            store.services().set(owned_data);

            Ok(())
        }
        None => Err(fetch_services_response.get_error().to_vec()),
    }
}

pub async fn fetch_professions(
    store: &Store<AppStateContext>,
    headers: Option<&HashMap<String, String>>,
) -> Result<(), Vec<GraphQLErrorMessage>> {
    let fetch_professions_query = r#"
           query FetchSiteResources {
                fetchSiteResources {
                    data {
                        professionalInfo {
                            description
                            active
                            occupation
                            startDate
                            id
                            yearsOfExperience
                        }
                    }
                    metadata {
                        newAccessToken
                        requestId
                    }
                }
           }
       "#;

    let Some(shared_service_api) = SHARED_SERVICE_API else {
        return Err(vec![]);
    };

    let fetch_professions_response = perform_query_without_vars::<FetchSiteResourcesResponse>(
        headers,
        shared_service_api,
        fetch_professions_query,
    )
    .await;

    match fetch_professions_response.get_data() {
        Some(data) => {
            let owned_data = data
                .fetch_site_resources
                .as_ref()
                .unwrap_or(&Default::default())
                .get_data()
                .professional_info
                .as_ref()
                .unwrap_or(&Default::default())
                .to_vec();
            store.professions().set(owned_data);

            Ok(())
        }
        None => Err(fetch_professions_response.get_error().to_vec()),
    }
}

pub async fn fetch_resume(
    store: &Store<AppStateContext>,
    headers: Option<&HashMap<String, String>>,
) -> Result<(), Vec<GraphQLErrorMessage>> {
    let fetch_resume_query = r#"
           query FetchSiteResources {
                fetchSiteResources {
                    data {
                        resume {
                            title
                            moreInfo
                            startDate
                            endDate
                            link
                            section
                            id
                            yearsOfExperience
                            achievements {
                                id
                                description
                            }
                        }
                    }
                    metadata {
                        newAccessToken
                        requestId
                    }
                }
           }
       "#;

    let Some(shared_service_api) = SHARED_SERVICE_API else {
        return Err(vec![]);
    };

    let fetch_resume_response = perform_query_without_vars::<FetchSiteResourcesResponse>(
        headers,
        shared_service_api,
        fetch_resume_query,
    )
    .await;

    match fetch_resume_response.get_data() {
        Some(data) => {
            let owned_data = data
                .fetch_site_resources
                .as_ref()
                .unwrap_or(&Default::default())
                .get_data()
                .resume
                .as_ref()
                .unwrap_or(&Default::default())
                .to_vec();
            store.resume().set(owned_data);

            Ok(())
        }
        None => Err(fetch_resume_response.get_error().to_vec()),
    }
}

pub async fn fetch_skills(
    store: &Store<AppStateContext>,
    headers: Option<&HashMap<String, String>>,
) -> Result<(), Vec<GraphQLErrorMessage>> {
    let fetch_skills_query = r#"
           query FetchSiteResources {
                fetchSiteResources {
                    data {
                        skills {
                            thumbnail
                            name
                            description
                            level
                            type
                            startDate
                            id
                            yearsOfExperience
                        }
                    }
                    metadata {
                        newAccessToken
                        requestId
                    }
                }
           }
       "#;

    let Some(shared_service_api) = SHARED_SERVICE_API else {
        return Err(vec![]);
    };

    let fetch_skills_response = perform_query_without_vars::<FetchSiteResourcesResponse>(
        headers,
        shared_service_api,
        fetch_skills_query,
    )
    .await;

    match fetch_skills_response.get_data() {
        Some(data) => {
            let owned_data = data
                .fetch_site_resources
                .as_ref()
                .unwrap_or(&Default::default())
                .get_data()
                .skills
                .as_ref()
                .unwrap_or(&Default::default())
                .to_vec();
            store.skills().set(owned_data);

            Ok(())
        }
        None => Err(fetch_skills_response.get_error().to_vec()),
    }
}

pub async fn fetch_portfolio(
    store: &Store<AppStateContext>,
    headers: Option<&HashMap<String, String>>,
) -> Result<(), Vec<GraphQLErrorMessage>> {
    let fetch_portfolio_query = r#"
           query FetchSiteResources {
                fetchSiteResources {
                    data {
                        portfolio {
                            title
                            description
                            startDate
                            endDate
                            link
                            category
                            thumbnail
                            id
                            yearsOfExperience
                        }
                    }
                    metadata {
                        newAccessToken
                        requestId
                    }
                }
           }
       "#;

    let Some(shared_service_api) = SHARED_SERVICE_API else {
        return Err(vec![]);
    };

    let fetch_portfolio_response = perform_query_without_vars::<FetchSiteResourcesResponse>(
        headers,
        shared_service_api,
        fetch_portfolio_query,
    )
    .await;

    match fetch_portfolio_response.get_data() {
        Some(data) => {
            let owned_data = data
                .fetch_site_resources
                .as_ref()
                .unwrap_or(&Default::default())
                .get_data()
                .portfolio
                .as_ref()
                .unwrap_or(&Default::default())
                .to_vec();
            store.portfolio().set(owned_data);

            Ok(())
        }
        None => Err(fetch_portfolio_response.get_error().to_vec()),
    }
}

pub async fn fetch_blog_posts(
    headers: Option<&HashMap<String, String>>,
    filters: FetchBlogPostsVars,
    query: &str,
) -> Result<Vec<BlogPost>, Vec<GraphQLErrorMessage>> {
    let Some(shared_service_api) = SHARED_SERVICE_API else {
        return Err(vec![]);
    };

    let fetch_blog_posts_response = perform_mutation_or_query_with_vars::<
        FetchBlogPostsResponse,
        FetchBlogPostsVars,
    >(headers, shared_service_api, query, filters)
    .await;

    match fetch_blog_posts_response.get_data() {
        Some(data) => {
            let owned_data = data
                .fetch_blog_posts
                .as_ref()
                .unwrap_or(&Default::default())
                .get_data()
                .to_vec();

            Ok(owned_data)
        }
        None => Err(fetch_blog_posts_response.get_error().to_vec()),
    }
}

pub async fn fetch_single_blog_post(
    headers: Option<&HashMap<String, String>>,
    vars: FetchSingleBlogPostVars,
) -> Result<BlogPost, Vec<GraphQLErrorMessage>> {
    let query = r#"
        query FetchSingleBlogPost($blogIdOrSlug: String!) {
            fetchSingleBlogPost(blogIdOrSlug: $blogIdOrSlug) {
                data {
                    title
                    shortDescription
                    status
                    thumbnail
                    category
                    link
                    publishedDate
                    isFeatured
                    isPremium
                    createdAt
                    updatedAt
                    id
                    author
                    content
                    readTime
                    comments {
                        content
                        createdAt
                        updatedAt
                        id
                        replyCount
                        author
                        reactionCount
                        currentUserReaction {
                            type
                            id
                        }
                    }
                    reactionCount
                    currentUserReaction {
                        type
                        id
                    }
                    bookmarksCount
                    sharesCount
                    currentUserBookmarked
                }
                metadata {
                    requestId
                    newAccessToken
                }
            }
        }
       "#;

    let Some(shared_service_api) = SHARED_SERVICE_API else {
        return Err(vec![]);
    };

    let response = perform_mutation_or_query_with_vars::<
        FetchSingleBlogPostResponse,
        FetchSingleBlogPostVars,
    >(headers, shared_service_api, query, vars)
    .await;

    match response.get_data() {
        Some(data) => {
            let owned_data = data
                .fetch_single_blog_post
                .as_ref()
                .unwrap_or(&Default::default())
                .get_data();

            Ok(owned_data)
        }
        None => Err(response.get_error().to_vec()),
    }
}

pub async fn check_auth(
    headers: Option<&HashMap<String, String>>,
) -> Result<AuthStatus, Vec<GraphQLErrorMessage>> {
    let check_auth_query = r#"
        query CheckAuth {
            checkAuth {
                data {
                    isAuth
                    sub
                    currentRole
                    newAccessToken
                    currentRolePermissions
                }
                metadata {
                    requestId
                    newAccessToken
                }
            }
        }
       "#;

    let Some(acl_service_api) = ACL_SERVICE_API else {
        return Err(vec![]);
    };

    let check_auth_response =
        perform_query_without_vars::<CheckAuthResponse>(headers, acl_service_api, check_auth_query)
            .await;

    match check_auth_response.get_data() {
        Some(data) => {
            let owned_data = data
                .check_auth
                .as_ref()
                .unwrap_or(&Default::default())
                .get_data();

            Ok(owned_data)
        }
        None => Err(check_auth_response.get_error().to_vec()),
    }
}

pub async fn fetch_departments(
    store: &Store<AppStateContext>,
    headers: Option<&HashMap<String, String>>,
) -> Result<(), Vec<GraphQLErrorMessage>> {
    let fetch_departments_query = r#"
           query FetchDepartments {
                fetchDepartments {
                    data {
                        depName
                        createdAt
                        updatedAt
                        id
                        createdBy
                    }
                    metadata {
                        newAccessToken
                        requestId
                    }
                }
           }
       "#;

    let Some(acl_service_api) = ACL_SERVICE_API else {
        return Err(vec![]);
    };

    let fetch_departments_response = perform_query_without_vars::<FetchDepartmentsResponse>(
        headers,
        acl_service_api,
        fetch_departments_query,
    )
    .await;

    match fetch_departments_response.get_data() {
        Some(data) => {
            let owned_data = data
                .fetch_departments
                .as_ref()
                .unwrap_or(&Default::default())
                .get_data()
                .to_vec();
            store.departments().set(owned_data);

            Ok(())
        }
        None => Err(fetch_departments_response.get_error().to_vec()),
    }
}

pub async fn fetch_organizations(
    store: &Store<AppStateContext>,
    headers: Option<&HashMap<String, String>>,
) -> Result<(), Vec<GraphQLErrorMessage>> {
    let fetch_orgs_query = r#"
           query FetchOrganizations {
                fetchOrganizations {
                    data {
                        orgName
                        createdAt
                        updatedAt
                        id
                        createdBy
                    }
                    metadata {
                        newAccessToken
                        requestId
                    }
                }
           }
       "#;

    let Some(acl_service_api) = ACL_SERVICE_API else {
        return Err(vec![]);
    };

    let fetch_orgs_response = perform_query_without_vars::<FetchOrganizationsResponse>(
        headers,
        acl_service_api,
        fetch_orgs_query,
    )
    .await;

    match fetch_orgs_response.get_data() {
        Some(data) => {
            let owned_data = data
                .fetch_organizations
                .as_ref()
                .unwrap_or(&Default::default())
                .get_data()
                .to_vec();
            store.organizations().set(owned_data);

            Ok(())
        }
        None => Err(fetch_orgs_response.get_error().to_vec()),
    }
}

pub async fn fetch_permissions(
    store: &Store<AppStateContext>,
    headers: Option<&HashMap<String, String>>,
) -> Result<(), Vec<GraphQLErrorMessage>> {
    let fetch_permissions_query = r#"
           query FetchCurrentRolePermissions {
                fetchCurrentRolePermissions {
                    data {
                        name
                        isAdmin
                        isSuperAdmin
                        id
                        createdBy
                        resource {
                            name
                        }
                    }
                    metadata {
                        newAccessToken
                        requestId
                    }
                }
           }
       "#;

    let Some(acl_service_api) = ACL_SERVICE_API else {
        return Err(vec![]);
    };

    let fetch_permissions_response = perform_query_without_vars::<FetchPermissionsResponse>(
        headers,
        acl_service_api,
        fetch_permissions_query,
    )
    .await;

    match fetch_permissions_response.get_data() {
        Some(data) => {
            let owned_data = data
                .fetch_current_role_permissions
                .as_ref()
                .unwrap_or(&Default::default())
                .get_data()
                .to_vec();
            store.permissions().set(owned_data);

            Ok(())
        }
        None => Err(fetch_permissions_response.get_error().to_vec()),
    }
}

pub async fn fetch_resources(
    store: &Store<AppStateContext>,
    headers: Option<&HashMap<String, String>>,
) -> Result<(), Vec<GraphQLErrorMessage>> {
    let fetch_resources_query = r#"
           query FetchResources {
                fetchResources {
                    data {
                        name
                        id
                        createdBy
                        createdAt
                    }
                    metadata {
                        newAccessToken
                        requestId
                    }
                }
           }
       "#;

    let Some(acl_service_api) = ACL_SERVICE_API else {
        return Err(vec![]);
    };

    let fetch_resources_response = perform_query_without_vars::<FetchResourcesResponse>(
        headers,
        acl_service_api,
        fetch_resources_query,
    )
    .await;

    match fetch_resources_response.get_data() {
        Some(data) => {
            let owned_data = data
                .fetch_resources
                .as_ref()
                .unwrap_or(&Default::default())
                .get_data()
                .to_vec();
            store.resources().set(owned_data);

            Ok(())
        }
        None => Err(fetch_resources_response.get_error().to_vec()),
    }
}

pub async fn fetch_roles(
    store: &Store<AppStateContext>,
    headers: Option<&HashMap<String, String>>,
) -> Result<(), Vec<GraphQLErrorMessage>> {
    let fetch_roles_query = r#"
           query FetchSystemRoles {
                fetchSystemRoles {
                    data {
                        roleName
                        isAdmin
                        isDefault
                        isSuperAdmin
                        id
                    }
                    metadata {
                        newAccessToken
                        requestId
                    }
                }
           }
       "#;

    let Some(acl_service_api) = ACL_SERVICE_API else {
        return Err(vec![]);
    };

    let fetch_roles_response = perform_query_without_vars::<FetchSystemRolesResponse>(
        headers,
        acl_service_api,
        fetch_roles_query,
    )
    .await;

    match fetch_roles_response.get_data() {
        Some(data) => {
            let owned_data = data
                .fetch_system_roles
                .as_ref()
                .unwrap_or(&Default::default())
                .get_data()
                .to_vec();
            store.roles().set(owned_data);

            Ok(())
        }
        None => Err(fetch_roles_response.get_error().to_vec()),
    }
}

pub async fn fetch_single_user(
    vars: &FetchSingleUserVars,
    headers: Option<&HashMap<String, String>>,
    query: &str,
) -> Result<User, Vec<GraphQLErrorMessage>> {
    let Some(acl_service_api) = ACL_SERVICE_API else {
        return Err(vec![]);
    };

    let fetch_auth_info_response = perform_mutation_or_query_with_vars::<
        FetchSingleUserResponse,
        FetchSingleUserVars,
    >(headers, acl_service_api, query, vars.to_owned())
    .await;

    match fetch_auth_info_response.get_data() {
        Some(data) => {
            let owned_data = data
                .fetch_single_user
                .as_ref()
                .unwrap_or(&Default::default())
                .get_data()
                .to_owned();

            Ok(owned_data)
        }
        None => Err(fetch_auth_info_response.get_error().to_vec()),
    }
}

pub async fn fetch_ratecards(
    store: &Store<AppStateContext>,
    headers: Option<&HashMap<String, String>>,
) -> Result<(), Vec<GraphQLErrorMessage>> {
    let fetch_ratecards_query = r#"
        query FetchRatecards {
            fetchRatecards {
                data {
                    name
                    createdAt
                    updatedAt
                    id
                    services {
                        title
                        description
                        thumbnail
                        id
                    }
                }
                metadata {
                    newAccessToken
                    requestId
                }
            }
        }
       "#;

    let Some(shared_service_api) = SHARED_SERVICE_API else {
        return Err(vec![]);
    };

    let fetch_ratecards_response = perform_query_without_vars::<FetchRatecardsResponse>(
        headers,
        shared_service_api,
        fetch_ratecards_query,
    )
    .await;

    match fetch_ratecards_response.get_data() {
        Some(data) => {
            let owned_data = data
                .fetch_ratecards
                .as_ref()
                .unwrap_or(&Default::default())
                .get_data()
                .to_vec();

            store.ratecards().set(owned_data);

            Ok(())
        }
        None => Err(fetch_ratecards_response.get_error().to_vec()),
    }
}

pub async fn fetch_billing_rate(
    vars: FetchBillingRateVars,
    headers: Option<&HashMap<String, String>>,
    store: &Store<AppStateContext>,
) -> Result<String, Vec<GraphQLErrorMessage>> {
    let fetch_billing_rate_query = r#"
        query FetchBillingRate($billingInterval: BillingInterval!, $serviceIds: [String!]!) {
            fetchBillingRate(billingInterval: $billingInterval, serviceIds: $serviceIds) {
                data
                metadata {
                    newAccessToken
                    requestId
                }
            }
        }
       "#;

    let Some(shared_service_api) = SHARED_SERVICE_API else {
        return Err(vec![]);
    };

    let fetch_billing_rate_response = perform_mutation_or_query_with_vars::<
        FetchBillingRateResponse,
        FetchBillingRateVars,
    >(
        headers, shared_service_api, fetch_billing_rate_query, vars
    )
    .await;

    match fetch_billing_rate_response.get_data() {
        Some(data) => {
            let owned_data = data
                .fetch_billing_rate
                .as_ref()
                .unwrap_or(&Default::default())
                .get_data()
                .to_owned();

            Ok(owned_data)
        }
        None => {
            let _handle_errors =
                handle_graphql_errors(&fetch_billing_rate_response, &store, None);
            Err(fetch_billing_rate_response.get_error().to_vec())
        }
    }
}

pub async fn fetch_service_rates(
    store: &Store<AppStateContext>,
    headers: Option<&HashMap<String, String>>,
) -> Result<(), Vec<GraphQLErrorMessage>> {
    let fetch_service_rates_query = r#"
        query FetchServiceRates {
            fetchServiceRates {
                data {
                    hourWeek
                    createdAt
                    updatedAt
                    id
                    baseRate
                    service {
                        title
                        description
                        thumbnail
                        id
                    }
                    currencyId {
                        currencyId
                        id
                    }
                }
                metadata {
                    newAccessToken
                    requestId
                }
            }
        }
       "#;

    let Some(shared_service_api) = SHARED_SERVICE_API else {
        return Err(vec![]);
    };

    let fetch_service_rates_response = perform_query_without_vars::<FetchServiceRatesResponse>(
        headers,
        shared_service_api,
        fetch_service_rates_query,
    )
    .await;

    match fetch_service_rates_response.get_data() {
        Some(data) => {
            let owned_data = data
                .fetch_service_rates
                .as_ref()
                .unwrap_or(&Default::default())
                .get_data()
                .to_vec();

            store.service_rates().set(owned_data);

            Ok(())
        }
        None => Err(fetch_service_rates_response.get_error().to_vec()),
    }
}

pub async fn fetch_currencies(
    store: &Store<AppStateContext>,
    headers: Option<&HashMap<String, String>>,
) -> Result<(), Vec<GraphQLErrorMessage>> {
    let query = r#"
        query FetchCurrencies {
            fetchCurrencies {
                data {
                    code
                    numeric
                    name
                    symbol
                    createdAt
                    updatedAt
                    id
                }
                metadata {
                    newAccessToken
                    requestId
                }
            }
        }
       "#;

    let Some(payments_service_api) = PAYMENTS_SERVICE_API else {
        return Err(vec![]);
    };

    let response =
        perform_query_without_vars::<FetchCurrenciesResponse>(headers, payments_service_api, query)
            .await;

    match response.get_data() {
        Some(data) => {
            let owned_data = data
                .fetch_currencies
                .as_ref()
                .unwrap_or(&Default::default())
                .get_data()
                .to_vec();

            store.currencies().set(owned_data);

            Ok(())
        }
        None => Err(response.get_error().to_vec()),
    }
}

pub async fn fetch_service_requests(
    store: &Store<AppStateContext>,
    headers: Option<&HashMap<String, String>>,
) -> Result<(), Vec<GraphQLErrorMessage>> {
    let query = r#"
        query FetchServiceRequests {
            fetchServiceRequests {
                data {
                    description
                    startDate
                    endDate
                    createdAt
                    updatedAt
                    id
                    supportingDocs {
                        fileId
                        id
                    }
                }
                metadata {
                    newAccessToken
                    requestId
                }
            }
        }
       "#;

    let Some(shared_service_api) = SHARED_SERVICE_API else {
        return Err(vec![]);
    };

    let response = perform_query_without_vars::<FetchServiceRequestsResponse>(
        headers,
        shared_service_api,
        query,
    )
    .await;

    match response.get_data() {
        Some(data) => {
            let owned_data = data
                .fetch_service_requests
                .as_ref()
                .unwrap_or(&Default::default())
                .get_data()
                .to_vec();

            store.service_requests().set(owned_data);

            Ok(())
        }
        None => Err(response.get_error().to_vec()),
    }
}
