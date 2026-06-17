use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Deserialize, Serialize, ToSchema)]
pub struct GeneratedFile {
    pub path: String,
    pub language: String,
    pub content: String,
}

#[derive(Debug, Deserialize, Serialize, ToSchema)]
pub struct CodePreviewResponse {
    pub files: Vec<GeneratedFile>,
    pub summary: String,
}
