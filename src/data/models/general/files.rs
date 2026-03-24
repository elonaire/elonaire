use serde::Deserialize;

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct UploadedFileResponse {
    pub field_name: String,
    pub file_id: String,
    pub file_name: String,
}
