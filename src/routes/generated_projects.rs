use axum::{Json, http::StatusCode};
use serde_json::json;

use crate::models::generated_project::GeneratedProjectsListResponse;
use crate::services::file_writer::list_generated_projects;

#[utoipa::path(
    get,
    path = "/projects/generated",
    responses(
        (
            status = 200,
            description = "List generated projects",
            body = GeneratedProjectsListResponse,
        ),
        (
            status = 500,
            description = "Failed to list generated projects",
        )
    ),
    tag = "Generated Projects"
)]
pub async fn list_generated_projects_handler()
-> Result<Json<GeneratedProjectsListResponse>, (StatusCode, Json<serde_json::Value>)> {
    match list_generated_projects() {
        Ok(response) => Ok(Json(response)),
        Err(error) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({
                "message": "Failed to list generated projects",
                "error": error.to_string(),
            })),
        )),
    }
}
