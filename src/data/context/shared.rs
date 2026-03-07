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
    utils::graphql_client::{perform_mutation_or_query_with_vars, perform_query_without_vars},
};
use reactive_stores::Store;

pub async fn fetch_services(
    current_state: &Store<AppStateContext>,
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

    let fetch_services_response = perform_query_without_vars::<FetchSiteResourcesResponse>(
        headers,
        "http://localhost:8080/api/shared",
        fetch_services_query,
    )
    .await;

    match fetch_services_response.get_data() {
        Some(data) => {
            let owned_data = data
                .fetch_site_resources
                .as_ref()
                .unwrap()
                .get_data()
                .services
                .as_ref()
                .unwrap()
                .to_vec();
            current_state.services().set(owned_data);

            Ok(())
        }
        None => Err(fetch_services_response.get_error().to_vec()),
    }
}

pub async fn fetch_professions(
    current_state: &Store<AppStateContext>,
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

    let fetch_professions_response = perform_query_without_vars::<FetchSiteResourcesResponse>(
        headers,
        "http://localhost:8080/api/shared",
        fetch_professions_query,
    )
    .await;

    match fetch_professions_response.get_data() {
        Some(data) => {
            let owned_data = data
                .fetch_site_resources
                .as_ref()
                .unwrap()
                .get_data()
                .professional_info
                .as_ref()
                .unwrap()
                .to_vec();
            current_state.professions().set(owned_data);

            Ok(())
        }
        None => Err(fetch_professions_response.get_error().to_vec()),
    }
}

pub async fn fetch_resume(
    current_state: &Store<AppStateContext>,
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

    let fetch_resume_response = perform_query_without_vars::<FetchSiteResourcesResponse>(
        headers,
        "http://localhost:8080/api/shared",
        fetch_resume_query,
    )
    .await;

    match fetch_resume_response.get_data() {
        Some(data) => {
            let owned_data = data
                .fetch_site_resources
                .as_ref()
                .unwrap()
                .get_data()
                .resume
                .as_ref()
                .unwrap()
                .to_vec();
            current_state.resume().set(owned_data);

            Ok(())
        }
        None => Err(fetch_resume_response.get_error().to_vec()),
    }
}

pub async fn fetch_skills(
    current_state: &Store<AppStateContext>,
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

    let fetch_skills_response = perform_query_without_vars::<FetchSiteResourcesResponse>(
        headers,
        "http://localhost:8080/api/shared",
        fetch_skills_query,
    )
    .await;

    match fetch_skills_response.get_data() {
        Some(data) => {
            let owned_data = data
                .fetch_site_resources
                .as_ref()
                .unwrap()
                .get_data()
                .skills
                .as_ref()
                .unwrap()
                .to_vec();
            current_state.skills().set(owned_data);

            Ok(())
        }
        None => Err(fetch_skills_response.get_error().to_vec()),
    }
}

pub async fn fetch_portfolio(
    current_state: &Store<AppStateContext>,
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

    let fetch_portfolio_response = perform_query_without_vars::<FetchSiteResourcesResponse>(
        headers,
        "http://localhost:8080/api/shared",
        fetch_portfolio_query,
    )
    .await;

    match fetch_portfolio_response.get_data() {
        Some(data) => {
            let owned_data = data
                .fetch_site_resources
                .as_ref()
                .unwrap()
                .get_data()
                .portfolio
                .as_ref()
                .unwrap()
                .to_vec();
            current_state.portfolio().set(owned_data);

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
    let fetch_blog_posts_response = perform_mutation_or_query_with_vars::<
        FetchBlogPostsResponse,
        FetchBlogPostsVars,
    >(
        headers, "http://localhost:8080/api/shared", query, filters
    )
    .await;

    match fetch_blog_posts_response.get_data() {
        Some(data) => {
            let owned_data = data.fetch_blog_posts.as_ref().unwrap().get_data().to_vec();

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

    let response = perform_mutation_or_query_with_vars::<
        FetchSingleBlogPostResponse,
        FetchSingleBlogPostVars,
    >(headers, "http://localhost:8080/api/shared", query, vars)
    .await;

    match response.get_data() {
        Some(data) => {
            let owned_data = data.fetch_single_blog_post.as_ref().unwrap().get_data();

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
                }
                metadata {
                    requestId
                    newAccessToken
                }
            }
        }
       "#;

    let check_auth_response = perform_query_without_vars::<CheckAuthResponse>(
        headers,
        "http://localhost:8080/api/acl",
        check_auth_query,
    )
    .await;

    match check_auth_response.get_data() {
        Some(data) => {
            let owned_data = data.check_auth.as_ref().unwrap().get_data();

            Ok(owned_data)
        }
        None => Err(check_auth_response.get_error().to_vec()),
    }
}

pub async fn fetch_departments(
    current_state: &Store<AppStateContext>,
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

    let fetch_departments_response = perform_query_without_vars::<FetchDepartmentsResponse>(
        headers,
        "http://localhost:8080/api/acl",
        fetch_departments_query,
    )
    .await;

    match fetch_departments_response.get_data() {
        Some(data) => {
            let owned_data = data.fetch_departments.as_ref().unwrap().get_data().to_vec();
            current_state.departments().set(owned_data);

            Ok(())
        }
        None => Err(fetch_departments_response.get_error().to_vec()),
    }
}

pub async fn fetch_organizations(
    current_state: &Store<AppStateContext>,
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

    let fetch_orgs_response = perform_query_without_vars::<FetchOrganizationsResponse>(
        headers,
        "http://localhost:8080/api/acl",
        fetch_orgs_query,
    )
    .await;

    match fetch_orgs_response.get_data() {
        Some(data) => {
            let owned_data = data
                .fetch_organizations
                .as_ref()
                .unwrap()
                .get_data()
                .to_vec();
            current_state.organizations().set(owned_data);

            Ok(())
        }
        None => Err(fetch_orgs_response.get_error().to_vec()),
    }
}

pub async fn fetch_permissions(
    current_state: &Store<AppStateContext>,
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

    let fetch_permissions_response = perform_query_without_vars::<FetchPermissionsResponse>(
        headers,
        "http://localhost:8080/api/acl",
        fetch_permissions_query,
    )
    .await;

    match fetch_permissions_response.get_data() {
        Some(data) => {
            let owned_data = data
                .fetch_current_role_permissions
                .as_ref()
                .unwrap()
                .get_data()
                .to_vec();
            current_state.permissions().set(owned_data);

            Ok(())
        }
        None => Err(fetch_permissions_response.get_error().to_vec()),
    }
}

pub async fn fetch_resources(
    current_state: &Store<AppStateContext>,
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

    let fetch_resources_response = perform_query_without_vars::<FetchResourcesResponse>(
        headers,
        "http://localhost:8080/api/acl",
        fetch_resources_query,
    )
    .await;

    match fetch_resources_response.get_data() {
        Some(data) => {
            let owned_data = data.fetch_resources.as_ref().unwrap().get_data().to_vec();
            current_state.resources().set(owned_data);

            Ok(())
        }
        None => Err(fetch_resources_response.get_error().to_vec()),
    }
}

pub async fn fetch_roles(
    current_state: &Store<AppStateContext>,
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

    let fetch_roles_response = perform_query_without_vars::<FetchSystemRolesResponse>(
        headers,
        "http://localhost:8080/api/acl",
        fetch_roles_query,
    )
    .await;

    match fetch_roles_response.get_data() {
        Some(data) => {
            let owned_data = data
                .fetch_system_roles
                .as_ref()
                .unwrap()
                .get_data()
                .to_vec();
            current_state.roles().set(owned_data);

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
    let fetch_auth_info_response =
        perform_mutation_or_query_with_vars::<FetchSingleUserResponse, FetchSingleUserVars>(
            headers,
            "http://localhost:8080/api/acl",
            query,
            vars.to_owned(),
        )
        .await;

    match fetch_auth_info_response.get_data() {
        Some(data) => {
            let owned_data = data
                .fetch_single_user
                .as_ref()
                .unwrap()
                .get_data()
                .to_owned();

            Ok(owned_data)
        }
        None => Err(fetch_auth_info_response.get_error().to_vec()),
    }
}

pub async fn fetch_ratecards(
    current_state: &Store<AppStateContext>,
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

    let fetch_ratecards_response = perform_query_without_vars::<FetchRatecardsResponse>(
        headers,
        "http://localhost:8080/api/shared",
        fetch_ratecards_query,
    )
    .await;

    match fetch_ratecards_response.get_data() {
        Some(data) => {
            let owned_data = data.fetch_ratecards.as_ref().unwrap().get_data().to_vec();

            current_state.ratecards().set(owned_data);

            Ok(())
        }
        None => Err(fetch_ratecards_response.get_error().to_vec()),
    }
}

pub async fn fetch_billing_rate(
    vars: FetchBillingRateVars,
    headers: Option<&HashMap<String, String>>,
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

    let fetch_billing_rate_response =
        perform_mutation_or_query_with_vars::<FetchBillingRateResponse, FetchBillingRateVars>(
            headers,
            "http://localhost:8080/api/shared",
            fetch_billing_rate_query,
            vars,
        )
        .await;

    match fetch_billing_rate_response.get_data() {
        Some(data) => {
            let owned_data = data
                .fetch_billing_rate
                .as_ref()
                .unwrap()
                .get_data()
                .to_owned();

            Ok(owned_data)
        }
        None => Err(fetch_billing_rate_response.get_error().to_vec()),
    }
}

pub async fn fetch_service_rates(
    current_state: &Store<AppStateContext>,
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

    let fetch_service_rates_response = perform_query_without_vars::<FetchServiceRatesResponse>(
        headers,
        "http://localhost:8080/api/shared",
        fetch_service_rates_query,
    )
    .await;

    match fetch_service_rates_response.get_data() {
        Some(data) => {
            let owned_data = data
                .fetch_service_rates
                .as_ref()
                .unwrap()
                .get_data()
                .to_vec();

            current_state.service_rates().set(owned_data);

            Ok(())
        }
        None => Err(fetch_service_rates_response.get_error().to_vec()),
    }
}

pub async fn fetch_currencies(
    current_state: &Store<AppStateContext>,
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

    let response = perform_query_without_vars::<FetchCurrenciesResponse>(
        headers,
        "http://localhost:8080/api/payments",
        query,
    )
    .await;

    match response.get_data() {
        Some(data) => {
            let owned_data = data.fetch_currencies.as_ref().unwrap().get_data().to_vec();

            current_state.currencies().set(owned_data);

            Ok(())
        }
        None => Err(response.get_error().to_vec()),
    }
}

pub async fn fetch_service_requests(
    current_state: &Store<AppStateContext>,
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

    let response = perform_query_without_vars::<FetchServiceRequestsResponse>(
        headers,
        "http://localhost:8080/api/shared",
        query,
    )
    .await;

    match response.get_data() {
        Some(data) => {
            let owned_data = data
                .fetch_service_requests
                .as_ref()
                .unwrap()
                .get_data()
                .to_vec();

            current_state.service_requests().set(owned_data);

            Ok(())
        }
        None => Err(response.get_error().to_vec()),
    }
}
