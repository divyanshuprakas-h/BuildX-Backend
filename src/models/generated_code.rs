use serde::Serialize;
use utoipa::ToSchema;

#[derive(Debug, Serialize, ToSchema)]
pub struct GeneratedFile {
    pub path: String,
    pub language: String,
    pub content: String,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct CodePreviewResponse {
    pub files: Vec<GeneratedFile>,
    pub summary: String,
}
