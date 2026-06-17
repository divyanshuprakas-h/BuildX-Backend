use std::io;

use crate::config::{ai_api_key, ai_model, ai_provider};
use crate::models::generated_code::CodePreviewResponse;

pub async fn generate_code_with_ai(ai_prompt: &str) -> io::Result<Option<CodePreviewResponse>> {
    let provider = ai_provider();

    if provider == "mock" {
        println!("AI provider is mock. Using fallback generator.");
        return Ok(None);
    }

    let Some(api_key) = ai_api_key() else {
        println!("AI_API_KEY is missing. Using fallback generator.");
        return Ok(None);
    };

    let model = ai_model();

    println!("AI provider: {}", provider);
    println!("AI model: {}", model);
    println!("AI prompt length: {}", ai_prompt.len());
    println!("AI API key loaded: {}", !api_key.is_empty());

    Ok(None)
}
