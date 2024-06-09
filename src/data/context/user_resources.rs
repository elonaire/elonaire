use std::fmt::Error;

use serde::{Deserialize, Serialize};
use yew::UseReducerHandle;

use crate::{
    app::{AppState, StateAction},
    data::{
        graphql::api_call::perform_mutation_or_query_with_vars,
        models::resource::GetUserResourcesResponse,
    },
};

#[derive(Debug, Serialize, Deserialize)]
pub struct GetUserResourcesVar {
    #[serde(rename = "userId")]
    user_id: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetResumeAchievementsVar {
    #[serde(rename = "resumeId")]
    resume_id: String,
}

pub async fn get_user_resources(
    user_id: String,
    state_clone: UseReducerHandle<AppState>,
) -> Result<(), Error> {
    let endpoint = "http://localhost:3002";
    let query = r#"
            query Query($userId: String!) {
                getUserResources(userId: $userId) {
                    professionalInfo {
                        id
                        occupation
                        active
                        description
                    }
                    portfolio {
                        id
                        title
                        description
                        link
                        image
                        startDate
                        endDate
                        category
                    }
                    services {
                        id
                        title
                        description
                        image
                    }
                    skills {
                        id
                        name
                        type
                        image
                        level
                    }
                    resume {
                        id
                        title
                        moreInfo
                        startDate
                        endDate
                        link
                        section
                    }
                    achievements
                }
            }
        "#;

    let variables = GetUserResourcesVar { user_id };

    let user_resources_data = perform_mutation_or_query_with_vars::<
        GetUserResourcesResponse,
        GetUserResourcesVar,
    >(endpoint, query, variables)
    .await;

    // log::info!("user: {:?}", user_resources_data);

    state_clone.dispatch(StateAction::UpdateUserResources(
        match user_resources_data.get_data() {
            Some(data) => {
                // update active_professional_info in the state by filtering the record which has active set to true
                state_clone.dispatch(StateAction::UpdateActiveProfessionalInfo(
                    data.get_user_resources.clone().professional_info.unwrap().iter().find(|info| info.active.unwrap()).unwrap().clone(),
                ));
                data.get_user_resources.clone()
            },
            None => Default::default(),
        },
    ));

    Ok(())
}
