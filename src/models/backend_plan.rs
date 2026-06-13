use serde::Serialize;
use utoipa::ToSchema;

#[derive(Debug, Serialize, ToSchema)]
pub struct BackendPlanItem {
    pub path: String,
    pub file_type: String,
    pub purpose: String,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct BackendPlanResponse {
    pub framework: String,
    pub database: String,
    pub files: Vec<BackendPlanItem>,
    pub dependencies: Vec<String>,
    pub summary: String,
}
