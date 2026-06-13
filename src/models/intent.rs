use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct IntentRequest {
    pub prompt: String,
}

#[derive(Debug, Serialize)]
pub struct IntentResponse {
    pub intent: String,
    pub project_type: String,
    pub complexity: String,
    pub needs_backend: bool,
    pub needs_auth: bool,
    pub needs_database: bool,
    pub message: String,
}