use crate::models::generated_code::CodePreviewResponse;
use crate::services::{
    ai_client::generate_code_with_ai, planner::build_code_preview_response,
    prompt_builder::build_code_generation_prompt,
};

pub async fn generate_code_from_prompt(prompt: &str) -> CodePreviewResponse {
    let ai_prompt = build_code_generation_prompt(prompt);

    match generate_code_with_ai(&ai_prompt).await {
        Ok(Some(ai_response)) => ai_response,

        Ok(None) => build_code_preview_response(prompt),

        Err(_) => build_code_preview_response(prompt),
    }
}
