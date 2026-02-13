use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct ApiResponse<T> {
    data: T,
    metadata: ApiResponseMetadata,
}

#[derive(Clone, Debug, Serialize, Deserialize, Default, PartialEq, Eq)]
pub struct ApiResponseMetadata {
    #[serde(rename = "requestId", alias = "request_id")]
    request_id: String,
    #[serde(rename = "newAccessToken", alias = "new_access_token")]
    new_access_token: Option<String>,
}

impl<T: Sync + Send + Clone> ApiResponse<T> {
    pub fn get_data(&self) -> T {
        self.data.clone()
    }

    pub fn get_request_id(&self) -> String {
        self.metadata.request_id.clone()
    }

    pub fn get_new_access_token(&self) -> Option<String> {
        self.metadata.new_access_token.as_ref().cloned()
    }
}
