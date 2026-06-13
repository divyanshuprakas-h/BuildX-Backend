use axum::{
    routing::{get, post},
    Router,
};

use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

use crate::docs::ApiDoc;
use crate::routes::{
    ai::{detect_intent, generate_blueprint},
    health::health_check,
};

pub fn create_app() -> Router {
    Router::new()
        .route("/", get(health_check))
        .route("/health", get(health_check))
        .route("/ai/detect-intent", post(detect_intent))
        .route("/ai/blueprint", post(generate_blueprint))
        .merge(
            SwaggerUi::new("/docs")
                .url("/api-docs/openapi.json", ApiDoc::openapi())
        )

}