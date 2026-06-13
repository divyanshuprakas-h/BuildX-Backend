use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Deserialize, ToSchema)]
pub struct IntentRequest {
    pub prompt: String,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct IntentResponse {
    pub intent: String,
    pub project_type: String,
    pub complexity: String,
    pub needs_backend: bool,
    pub needs_auth: bool,
    pub needs_database: bool,
    pub message: String,
}
