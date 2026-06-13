use axum::{
    Router,
    routing::{get, post},
};

use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

use crate::docs::ApiDoc;
use crate::routes::{
    ai::{detect_intent, generate_blueprint, generate_frontend_plan},
    health::health_check,
};

pub fn create_app() -> Router {
    Router::new()
        .route("/", get(health_check))
        .route("/health", get(health_check))
        .route("/ai/detect-intent", post(detect_intent))
        .route("/ai/blueprint", post(generate_blueprint))
        .route("/ai/frontend-plan", post(generate_frontend_plan))
        .merge(SwaggerUi::new("/docs").url("/api-docs/openapi.json", ApiDoc::openapi()))
}

