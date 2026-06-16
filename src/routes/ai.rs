use axum::{Json, http::StatusCode};
use serde_json::json;

use crate::models::backend_plan::BackendPlanResponse;
use crate::models::blueprint::BlueprintResponse;
use crate::models::file_plan::FrontendPlanResponse;
use crate::models::generated_code::CodePreviewResponse;
use crate::models::generated_project::GeneratedProjectResponse;
use crate::models::intent::{IntentRequest, IntentResponse};
use crate::models::project_plan::ProjectPlanResponse;
use crate::services::planner::{
    build_backend_plan_response, build_blueprint_response, build_code_preview_response,
    build_frontend_plan_response, build_intent_response, build_project_plan_response,
};
use crate::services::project_generator::generate_project_from_prompt;

#[utoipa::path(
    post,
    path = "/ai/detect-intent",
    request_body = IntentRequest,
    responses(
        (
            status = 200,
            description = "Detect project intent from user prompt",
            body = IntentResponse
        )
    ),
    tag = "BuildX AI"
)]

pub async fn detect_intent(Json(payload): Json<IntentRequest>) -> Json<IntentResponse> {
    let response = build_intent_response(&payload.prompt);
    Json(response)
}

#[utoipa::path(
    post,
    path = "/ai/blueprint",
    request_body = IntentRequest,
    responses(
        (
            status = 200,
            description = "Generate project blueprint from user prompt",
            body = BlueprintResponse
        )
    ),
    tag = "BuildX AI"
)]

pub async fn generate_blueprint(Json(payload): Json<IntentRequest>) -> Json<BlueprintResponse> {
    let response = build_blueprint_response(&payload.prompt);
    Json(response)
}

#[utoipa::path(
    post,
    path = "/ai/frontend-plan",
    request_body = IntentRequest,
    responses(
        (
            status = 200,
            description = "Generate frontend file plan from user prompt",
            body = FrontendPlanResponse
        )
    ),
    tag = "BuildX AI"
)]

pub async fn generate_frontend_plan(
    Json(payload): Json<IntentRequest>,
) -> Json<FrontendPlanResponse> {
    let response = build_frontend_plan_response(&payload.prompt);
    Json(response)
}

#[utoipa::path(
    post,
    path = "/ai/backend-plan",
    request_body = IntentRequest,
    responses(
        (
            status = 200,
            description = "Generate backend file plan from user prompt",
            body = BackendPlanResponse
        )
    ),
    tag = "BuildX AI"

)]

pub async fn generate_backend_plan(
    Json(payload): Json<IntentRequest>,
) -> Json<BackendPlanResponse> {
    let response = build_backend_plan_response(&payload.prompt);
    Json(response)
}

#[utoipa::path(
    post,
    path = "/ai/project-plan",
    request_body = IntentRequest,
    responses(
        (
            status = 200,
            description = "Generate full project plan from user prompt",
            body = ProjectPlanResponse
        )
    ),
    tag = "BuildX AI"
)]

pub async fn generate_project_plan(
    Json(payload): Json<IntentRequest>,
) -> Json<ProjectPlanResponse> {
    let response = build_project_plan_response(&payload.prompt);
    Json(response)
}

#[utoipa::path(
    post,
    path = "/ai/code-preview",
    request_body = IntentRequest,
    responses(
        (
            status = 200,
            description = "Generate starter code preview from user prompt",
            body = CodePreviewResponse
        )
    ),
    tag = "BuildX AI"
)]

pub async fn generate_code_preview(
    Json(payload): Json<IntentRequest>,
) -> Json<CodePreviewResponse> {
    let response = build_code_preview_response(&payload.prompt);
    Json(response)
}

#[utoipa::path(
    post,
    path = "/ai/generate-project",
    request_body = IntentRequest,
    responses(
        (
            status = 200,
            description = "Generate project files from user prompt",
            body = GeneratedProjectResponse
        ),
        (
            status = 500,
            description = "Failed to generate project files"
        )
    ),
    tag = "BuildX AI"
)]

pub async fn generate_project(
    Json(payload): Json<IntentRequest>,
) -> Result<Json<GeneratedProjectResponse>, (StatusCode, Json<serde_json::Value>)> {
    match generate_project_from_prompt(&payload.prompt).await {
        Ok(response) => Ok(Json(response)),
        Err(error) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({
                "message": "Failed to generate project files",
                "error": error.to_string()
            })),
        )),
    }
}
