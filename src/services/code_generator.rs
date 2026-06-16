use crate::models::generated_code::CodePreviewResponse;
use crate::services::planner::build_code_preview_response;

pub async fn generate_code_from_prompt(prompt: &str) -> CodePreviewResponse {
    build_code_preview_response(prompt)
}
