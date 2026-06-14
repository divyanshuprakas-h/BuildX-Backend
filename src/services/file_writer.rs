use std::{
    fs, io,
    path::{Component, Path, PathBuf},
    time::{SystemTime, UNIX_EPOCH},
};

use crate::models::generated_project::GeneratedProjectResponse;
use crate::services::planner::build_code_preview_response;

pub fn generate_project_from_prompt(prompt_input: &str) -> io::Result<GeneratedProjectResponse> {
    let code_preview = build_code_preview_response(prompt_input);

    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("System time is before UNIX epoch")
        .as_secs();

    let project_name = format!("buildx_app_{}", timestamp);
    let project_root = PathBuf::from("generated_apps").join(&project_name);

    fs::create_dir_all(&project_root)?;

    let mut files_written = Vec::new();

    for file in code_preview.files {
        let output_path = safe_join(&project_root, &file.path)?;

        if let Some(parent_dir) = output_path.parent() {
            fs::create_dir_all(parent_dir)?;
        }

        fs::write(&output_path, file.content)?;
        files_written.push(file.path);
    }

    Ok(GeneratedProjectResponse {
        project_name,
        project_path: project_root.to_string_lossy().to_string(),
        files_written,
        summary: "Project files generated successfully.".to_string(),
    })
}

fn safe_join(root: &Path, relative_path: &str) -> io::Result<PathBuf> {
    let path = Path::new(relative_path);

    for component in path.components() {
        match component {
            Component::Normal(_) | Component::CurDir => {}
            _ => {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidInput,
                    "Invalid generated file path",
                ));
            }
        }
    }

    Ok(root.join(path))
}
