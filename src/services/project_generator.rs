use std::io;

use crate::models::generated_project::GeneratedProjectResponse;
use crate::services::{
    code_generator::generate_code_from_prompt, file_writer::write_generated_project,
};

pub fn generate_project_from_prompt(prompt: &str) -> io::Result<GeneratedProjectResponse> {
    let code_preview = generate_code_from_prompt(prompt);
    write_generated_project(code_preview)
}
