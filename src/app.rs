use axum::{
    routing::{get, post},
    Router,
};

use crate::routes::{
    ai::detect_intent,
    health::health_check,
};

pub fn create_app() -> Router {
    Router::new()
        .route("/health", get(health_check))
        .route("/ai/detect-intent", post(detect_intent))

}