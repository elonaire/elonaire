use std::collections::HashMap;

use gql_client::GraphQLErrorMessage;
use leptos::prelude::*;

use crate::{
    data::{
        context::store::{AppStateContext, AppStateContextStoreFields},
        models::graphql::{
            acl::{
                FetchDepartmentsResponse, FetchOrganizationsResponse, FetchPermissionsResponse,
                FetchResourcesResponse, FetchSystemRolesResponse,
            },
            shared::{
                FetchBillingRateResponse, FetchBillingRateVars, FetchCurrenciesResponse,
                FetchRatecardsResponse, FetchServiceRatesResponse, FetchSiteResourcesResponse,
                Ratecard, ServiceRate,
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
                    services {
                        title
                        description
                        thumbnail
                        id
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
                .services
                .as_ref()
                .unwrap()
                .to_vec();
            *current_state.services().write() = owned_data;

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
                    professionalInfo {
                        description
                        active
                        occupation
                        startDate
                        id
                        yearsOfExperience
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
                .professional_info
                .as_ref()
                .unwrap()
                .to_vec();
            *current_state.professions().write() = owned_data;

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
                .resume
                .as_ref()
                .unwrap()
                .to_vec();
            *current_state.resume().write() = owned_data;

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
                .skills
                .as_ref()
                .unwrap()
                .to_vec();
            *current_state.skills().write() = owned_data;

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
                .portfolio
                .as_ref()
                .unwrap()
                .to_vec();
            *current_state.portfolio().write() = owned_data;

            Ok(())
        }
        None => Err(fetch_portfolio_response.get_error().to_vec()),
    }
}

pub async fn fetch_departments(
    current_state: &Store<AppStateContext>,
    headers: Option<&HashMap<String, String>>,
) -> Result<(), Vec<GraphQLErrorMessage>> {
    let fetch_departments_query = r#"
           query FetchDepartments {
                fetchDepartments {
                    depName
                    createdAt
                    updatedAt
                    id
                    createdBy
                }
           }
       "#;

    let fetch_departments_response = perform_query_without_vars::<FetchDepartmentsResponse>(
        headers,
        "http://localhost:8080/api/shared",
        fetch_departments_query,
    )
    .await;

    match fetch_departments_response.get_data() {
        Some(data) => {
            let owned_data = data.fetch_departments.as_ref().unwrap().to_vec();
            *current_state.departments().write() = owned_data;

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
                    orgName
                    createdAt
                    updatedAt
                    id
                    createdBy
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
            let owned_data = data.fetch_organizations.as_ref().unwrap().to_vec();
            *current_state.organizations().write() = owned_data;

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
                   name
                   isAdmin
                   isSuperAdmin
                   id
                   createdBy
                   resource {
                       name
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
                .to_vec();
            *current_state.permissions().write() = owned_data;

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
                    name
                    id
                    createdBy
                    createdAt
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
            let owned_data = data.fetch_resources.as_ref().unwrap().to_vec();
            *current_state.resources().write() = owned_data;

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
                    roleName
                    isAdmin
                    isDefault
                    isSuperAdmin
                    id
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
            let owned_data = data.fetch_system_roles.as_ref().unwrap().to_vec();
            *current_state.roles().write() = owned_data;

            Ok(())
        }
        None => Err(fetch_roles_response.get_error().to_vec()),
    }
}

pub async fn fetch_ratecards(
    current_state: &Store<AppStateContext>,
    headers: Option<&HashMap<String, String>>,
) -> Result<(), Vec<GraphQLErrorMessage>> {
    let fetch_ratecards_query = r#"
        query FetchRatecards {
            fetchRatecards {
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
            let owned_data = data.fetch_ratecards.as_ref().unwrap().to_vec();

            *current_state.ratecards().write() = owned_data;

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
            fetchBillingRate(billingInterval: $billingInterval, serviceIds: $serviceIds)
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
            let owned_data = data.fetch_billing_rate.as_ref().unwrap().to_owned();

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
            let owned_data = data.fetch_service_rates.as_ref().unwrap().to_vec();

            *current_state.service_rates().write() = owned_data;

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
                code
                numeric
                name
                symbol
                createdAt
                updatedAt
                id
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
            let owned_data = data.fetch_currencies.as_ref().unwrap().to_vec();

            *current_state.currencies().write() = owned_data;

            Ok(())
        }
        None => Err(response.get_error().to_vec()),
    }
}
