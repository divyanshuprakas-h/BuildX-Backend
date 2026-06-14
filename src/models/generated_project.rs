use serde::Serialize;
use utoipa::ToSchema;

#[derive(Debug, Serialize, ToSchema)]
pub struct GeneratedProjectResponse {
    pub project_name: String,
    pub project_path: String,
    pub files_written: Vec<String>,
    pub summary: String,
}
