use axum::Json;

#[utoipa::path(
    get,
    path = "/health",
    responses(
        (
            status = 200,
            description = "Backend health check",
        )
    ),
    tag = "Health"
)]

pub async fn health_check() -> Json<serde_json::Value> {
    Json(serde_json::json!({
        "status": "ok",
        "service": "BuildX Backend"
    }))
}