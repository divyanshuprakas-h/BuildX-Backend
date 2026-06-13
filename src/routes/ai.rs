use axum::Json;

use crate::models::intent::{IntentRequest, IntentResponse};

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
    let prompt = payload.prompt.to_lowercase();

    let needs_auth = prompt.contains("login")
        || prompt.contains("signup")
        || prompt.contains("auth")
        || prompt.contains("user account");

    let needs_database = prompt.contains("database")
        || prompt.contains("save")
        || prompt.contains("store")
        || prompt.contains("dashboard")
        || prompt.contains("crud")
        || prompt.contains("todo")
        || prompt.contains("blog");

    let needs_backend = needs_auth
        || needs_database
        || prompt.contains("api")
        || prompt.contains("backend")
        || prompt.contains("server");

    let project_type = if prompt.contains("todo") {
        "todo_app"
    } else if prompt.contains("blog") {
        "blog_app"
    } else if prompt.contains("dashboard") {
        "dashboard"
    } else if prompt.contains("ecommerce") || prompt.contains("shop") {
        "ecommerce"
    } else if prompt.contains("portfolio") {
        "portfolio"
    } else if prompt.contains("landing page") {
        "landing_page"
    } else {
        "custom_app"
    };

    let complexity = if needs_auth && needs_database && needs_backend {
        "high"
    } else if needs_backend || needs_database {
        "medium"
    } else {
        "low"
    };

    let intent = if prompt.contains("create")
        || prompt.contains("build")
        || prompt.contains("make")
        || prompt.contains("generate")
    {
        "generate_project"
    } else {
        "unknown"
    };

    Json(IntentResponse {
        intent: intent.to_string(),
        project_type: project_type.to_string(),
        complexity: complexity.to_string(),
        needs_backend,
        needs_auth,
        needs_database,
        message: format!(
            "BuildX understood your idea as a {} with {} complexity.",
            project_type, complexity
        ),
    })
}