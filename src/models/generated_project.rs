use serde::Serialize;
use utoipa::ToSchema;

#[derive(Debug, Serialize, ToSchema)]
pub struct GeneratedProjectResponse {
    pub project_name: String,
    pub project_path: String,
    pub zip_path: String,
    pub download_url: String,
    pub files_written: Vec<String>,
    pub run_commands: Vec<String>,
    pub preview_entry: String,
    pub summary: String,
}
