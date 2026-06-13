use serde::Serialize;
use utoipa::ToSchema;

#[derive(Debug, Serialize, ToSchema)]
pub struct FilePlanItem {
    pub path: String,
    pub file_type: String,
    pub purpose: String,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct FrontendPlanResponse {
    pub framework: String,
    pub styling: String,
    pub files: Vec<FilePlanItem>,
    pub summary: String,
}
