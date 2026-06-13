use axum::Json;

use crate::models::blueprint::BlueprintResponse;
use crate::models::intent::{IntentRequest, IntentResponse};
use crate::services::planner::{build_blueprint_response, build_intent_response};

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
