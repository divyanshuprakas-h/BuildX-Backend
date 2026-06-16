use std::{
    fs::{self, File},
    io::{self, Read, Write},
    path::{Component, Path, PathBuf},
    time::{SystemTime, UNIX_EPOCH},
};

use zip::{CompressionMethod, ZipWriter, write::FileOptions};

use crate::models::generated_project::{
    GeneratedProjectListItem, GeneratedProjectResponse, GeneratedProjectsListResponse,
};
use crate::services::planner::build_code_preview_response;

pub fn generate_project_from_prompt(prompt_input: &str) -> io::Result<GeneratedProjectResponse> {
    let code_preview = build_code_preview_response(prompt_input);

    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("System time is before UNIX epoch")
        .as_secs();

    let project_name = format!("buildx_app_{}", timestamp);
    let zip_file_name = format!("{}.zip", project_name);

    let generated_apps_dir = PathBuf::from("generated_apps");
    let project_root = generated_apps_dir.join(&project_name);
    let zip_path = generated_apps_dir.join(&zip_file_name);
    let download_url = format!("/download/{}", zip_file_name);

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

    create_zip_from_folder(&project_root, &zip_path)?;

    Ok(GeneratedProjectResponse {
        project_name,
        project_path: project_root.to_string_lossy().to_string(),
        zip_path: zip_path.to_string_lossy().to_string(),
        download_url,
        files_written,
        run_commands: vec!["npm install".to_string(), "npm run dev".to_string()],
        preview_entry: "src/App.jsx".to_string(),
        summary: "Project files and ZIP generated successfully.".to_string(),
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

fn create_zip_from_folder(folder_path: &Path, zip_path: &Path) -> io::Result<()> {
    let zip_file = File::create(zip_path)?;
    let mut zip = ZipWriter::new(zip_file);

    let options = FileOptions::default().compression_method(CompressionMethod::Stored);

    add_folder_to_zip(folder_path, folder_path, &mut zip, options)?;

    zip.finish()?;

    Ok(())
}

fn add_folder_to_zip(
    base_path: &Path,
    current_path: &Path,
    zip: &mut ZipWriter<File>,
    options: FileOptions,
) -> io::Result<()> {
    for entry in fs::read_dir(current_path)? {
        let entry = entry?;
        let path = entry.path();

        if path.is_dir() {
            add_folder_to_zip(base_path, &path, zip, options)?;
        } else {
            let relative_path = path
                .strip_prefix(base_path)
                .map_err(|error| io::Error::new(io::ErrorKind::Other, error.to_string()))?;

            let zip_file_name = relative_path.to_string_lossy().replace('\\', "/");

            zip.start_file(zip_file_name, options)?;

            let mut file = File::open(&path)?;
            let mut buffer = Vec::new();

            file.read_to_end(&mut buffer)?;
            zip.write_all(&buffer)?;
        }
    }

    Ok(())
}

pub fn list_generated_projects() -> io::Result<GeneratedProjectsListResponse> {
    let generated_apps_dir = PathBuf::from("generated_apps");

    if !generated_apps_dir.exists() {
        return Ok(GeneratedProjectsListResponse {
            total: 0,
            projects: Vec::new(),
            summary: "No generated projects found..".to_string(),
        });
    }

    let mut projects = Vec::new();

    for entry in fs::read_dir(&generated_apps_dir)? {
        let entry = entry?;
        let path = entry.path();

        if !path.is_file() {
            continue;
        }

        let Some(file_name) = path.file_name() else {
            continue;
        };

        let zip_name = file_name.to_string_lossy().to_string();

        if !zip_name.ends_with(".zip") {
            continue;
        }

        let project_name = zip_name.trim_end_matches(".zip").to_string();
        projects.push(GeneratedProjectListItem {
            project_name,
            zip_name: zip_name.clone(),
            zip_path: path.to_string_lossy().to_string(),
            download_url: format!("/download/{}", zip_name),
        });
    }

    projects.sort_by(|a, b| b.project_name.cmp(&a.project_name));

    Ok(GeneratedProjectsListResponse {
        total: projects.len(),
        projects,
        summary: "Generated projects fetched successfully.".to_string(),
    })
}
