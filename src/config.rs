use std::env;

pub fn ai_provider() -> String {
    env::var("AI_PROVIDER").unwrap_or_else(|_| "mock".to_string())
}

pub fn ai_api_key() -> Option<String> {
    env::var("AI_API_KEY").ok()
}

pub fn ai_model() -> String {
    env::var("AI_MODEL").unwrap_or_else(|_| "gemini-3.5-flash".to_string())
}
