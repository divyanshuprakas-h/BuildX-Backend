use std::io;

use crate::models::generated_code::CodePreviewResponse;

pub async fn generate_code_with_ai(_ai_prompt: &str) -> io::Result<Option<CodePreviewResponse>> {
    Ok(None)
}
