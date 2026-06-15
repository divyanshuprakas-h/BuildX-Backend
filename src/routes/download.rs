use axum::{
    body::Body,
    extract::Path,
    http::{
        StatusCode,
        header::{CONTENT_DISPOSITION, CONTENT_TYPE},
    },
    response::Response,
};

use std::path::PathBuf;

#[utoipa::path(
    get,
    path = "/download/{zip_name}",
    params(
        ("zip_name" = String, Path, description = "Generated ZIP file name")
    ),
    responses(
        (
            status = 200,
            description = "Download generated project ZIP file",
            content_type = "application/zip"
        ),
        (
            status = 400,
            description = "Invalid ZIP file name"
        ),
        (
            status = 404,
            description = "ZIP file not found"
        )
    ),
    tag = "Download"
)]
pub async fn download_zip(Path(zip_name): Path<String>) -> Result<Response<Body>, StatusCode> {
    if !is_valid_zip_name(&zip_name) {
        return Err(StatusCode::BAD_REQUEST);
    }

    let zip_path = PathBuf::from("generated_apps").join(&zip_name);

    let file_bytes = tokio::fs::read(zip_path)
        .await
        .map_err(|_| StatusCode::NOT_FOUND)?;

    let response = Response::builder()
        .status(StatusCode::OK)
        .header(CONTENT_TYPE, "application/zip")
        .header(
            CONTENT_DISPOSITION,
            format!("attachment; filename=\"{}\"", zip_name),
        )
        .body(Body::from(file_bytes))
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(response)
}

fn is_valid_zip_name(zip_name: &str) -> bool {
    zip_name.ends_with(".zip")
        && zip_name
            .chars()
            .all(|ch| ch.is_ascii_alphanumeric() || ch == '_' || ch == '-' || ch == '.')
}
