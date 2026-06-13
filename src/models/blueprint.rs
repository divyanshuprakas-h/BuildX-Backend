use serde::Serialize;
use utoipa::ToSchema;

#[derive(Debug, Serialize, ToSchema)]
pub struct PageBlueprint {
    pub name: String,
    pub path: String,
    pub purpose: String,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct ApiRouteBlueprint {
    pub method: String,
    pub path: String,
    pub purpose: String,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct BlueprintResponse {
    pub project_name: String,
    pub project_type: String,
    pub complexity: String,
    pub pages: Vec<PageBlueprint>,
    pub components: Vec<String>,
    pub features: Vec<String>,
    pub api_routes: Vec<ApiRouteBlueprint>,
    pub database_tables: Vec<String>,
    pub summary: String,
}
