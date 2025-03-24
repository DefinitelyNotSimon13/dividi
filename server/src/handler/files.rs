use std::sync::Arc;

use axum::{
    Json,
    extract::{self, Multipart, State},
    http::{HeaderValue, StatusCode, header},
    response::{IntoResponse, Response},
};
use bytes::Bytes;
use tokio::fs;
use tracing::{error, info, warn};

use crate::{state::AppState, uploaded_file::UploadedFile};

pub async fn get_files(State(state): State<Arc<AppState>>) -> impl IntoResponse {
    let files = state.files().await;
    Json(files)
}

pub async fn download_file(
    extract::Path(file_name): extract::Path<String>,
    State(state): State<Arc<AppState>>,
) -> impl IntoResponse {
    let files = state.files().await;
    let file = match files.iter().find(|el| el.name == file_name) {
        Some(file) => file,
        None => {
            return (StatusCode::NOT_FOUND, format!("file not found")).into_response();
        }
    };

    match fs::read(&file.path).await {
        Ok(file_bytes) => {
            let mut response = Response::new(file_bytes.into());
            response.headers_mut().insert(
                header::CONTENT_DISPOSITION,
                format!("attachment; filename=\"{}\"", file_name)
                    .parse()
                    .unwrap(),
            );
            response.headers_mut().insert(
                header::CONTENT_TYPE,
                "application/octet-stream".parse().unwrap(),
            );
            response
        }
        Err(err) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("unable to load file: {}", err),
        )
            .into_response(),
    }
}

pub async fn delete_file(
    extract::Path(file_name): extract::Path<String>,
    State(state): State<Arc<AppState>>,
) -> impl IntoResponse {
    let files = state.files().await;
    let file = match files.iter().find(|el| el.name == file_name) {
        Some(file) => file,
        None => {
            return (StatusCode::NOT_FOUND, format!("file not found")).into_response();
        }
    };

    let mut response = match fs::remove_file(&file.path).await {
        Ok(_) => {
            state.remove_file(&file_name).await;
            (StatusCode::OK, format!("deleted file: {}", file_name))
        }

        Err(err) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("unable to delete file: {}", err),
        ),
    }
    .into_response();

    response
        .headers_mut()
        .insert("HX-Trigger", HeaderValue::from_static("refreshFiles"));
    response
}

pub async fn upload_file(
    State(state): State<Arc<AppState>>,
    mut multipart: Multipart,
) -> impl IntoResponse {
    info!("received file upload request");

    let mut custom_file_name: Option<String> = None;
    let mut file_bytes: Option<Bytes> = None;
    let mut orig_file_name: Option<String> = None;

    // process multipart form fields
    while let Some(field) = multipart.next_field().await.unwrap_or_else(|e| {
        error!("error reading field: {}", e);
        None
    }) {
        let field_name = field.name().unwrap_or("").to_string();
        match field_name.as_str() {
            "title" => {
                let text = field.text().await.unwrap_or_default();
                if !text.trim().is_empty() {
                    custom_file_name = Some(text);
                }
            }
            "file" => {
                orig_file_name = field.file_name().map(String::from);
                file_bytes = Some(field.bytes().await.unwrap());
            }
            _ => {
                warn!("received unknown field name")
            }
        }
    }

    let file_data = match file_bytes {
        Some(data) => data,
        None => return (StatusCode::BAD_REQUEST, "no file was uploaded".to_string()),
    };

    let uploaded_file = match UploadedFile::save(custom_file_name, orig_file_name, file_data).await
    {
        Ok(file) => file,
        Err(err) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("failed to save file: {}", err),
            );
        }
    };

    state.upload_file(uploaded_file).await;

    (StatusCode::OK, "successfully uploaded file".to_string())
}
