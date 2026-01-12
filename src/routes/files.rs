use crate::{error::AppError, state::AppState, utils::create_client_from_headers};
use async_openai::types::files::{CreateFileRequest, DeleteFileResponse, FilePurpose, ListFilesResponse, OpenAIFile, FileInput};
use async_openai::types::InputSource;
use axum::{
    extract::{Multipart, Path, State},
    http::HeaderMap,
    Json,
};
use std::sync::Arc;
use tracing::{error, info};

pub async fn upload_file(
    State(_state): State<Arc<AppState>>,
    headers: HeaderMap,
    mut multipart: Multipart,
) -> Result<Json<OpenAIFile>, AppError> {
    info!("📁 Upload file request");

    let mut file_bytes: Option<Vec<u8>> = None;
    let mut filename: Option<String> = None;
    let mut purpose: Option<String> = None;

    while let Some(field) = multipart
        .next_field()
        .await
        .map_err(|e| AppError(format!("Multipart error: {}", e)))?
    {
        let name = field.name().unwrap_or("").to_string();

        match name.as_str() {
            "file" => {
                filename = field.file_name().map(|s| s.to_string());
                file_bytes = Some(
                    field
                        .bytes()
                        .await
                        .map_err(|e| AppError(format!("File read error: {}", e)))?
                        .to_vec(),
                );
            }
            "purpose" => {
                purpose = Some(
                    field
                        .text()
                        .await
                        .map_err(|e| AppError(format!("Purpose read error: {}", e)))?,
                );
            }
            _ => {}
        }
    }

    let file_bytes = file_bytes.ok_or_else(|| AppError("File not provided".to_string()))?;
    let filename = filename.ok_or_else(|| AppError("Filename not provided".to_string()))?;
    let purpose = purpose.ok_or_else(|| AppError("Purpose not provided".to_string()))?;

    let client = create_client_from_headers(&headers)?;

    let file_purpose = match purpose.as_str() {
        "assistants" => FilePurpose::Assistants,
        "fine-tune" => FilePurpose::FineTune,
        _ => FilePurpose::Assistants,
    };

    let file_request = CreateFileRequest {
        file: FileInput { 
            source: InputSource::Bytes { 
                filename, 
                bytes: file_bytes.into() 
            } 
        },
        purpose: file_purpose,
        expires_after: None,
    };

    let response = client
        .files()
        .create(file_request)
        .await
        .map_err(|e| {
            error!("❌ Upload file error: {}", e);
            AppError(format!("Upload file error: {}", e))
        })?;

    info!("✅ File загружен: {}", response.id);
    Ok(Json(response))
}

pub async fn list_files(
    State(_state): State<Arc<AppState>>,
    headers: HeaderMap,
) -> Result<Json<ListFilesResponse>, AppError> {
    info!("📁 List files request");

    let client = create_client_from_headers(&headers)?;

    let response = client.files().list().await.map_err(|e| {
        error!("❌ List files error: {}", e);
        AppError(format!("List files error: {}", e))
    })?;

    info!("✅ Files list получен");
    Ok(Json(response))
}

pub async fn get_file(
    State(_state): State<Arc<AppState>>,
    Path(file_id): Path<String>,
    headers: HeaderMap,
) -> Result<Json<OpenAIFile>, AppError> {
    info!("📁 Get file request: {}", file_id);

    let client = create_client_from_headers(&headers)?;

    let response = client
        .files()
        .retrieve(&file_id)
        .await
        .map_err(|e| {
            error!("❌ Get file error: {}", e);
            AppError(format!("Get file error: {}", e))
        })?;

    info!("✅ File получен");
    Ok(Json(response))
}

pub async fn delete_file(
    State(_state): State<Arc<AppState>>,
    headers: HeaderMap,
    Path(file_id): Path<String>,
) -> Result<Json<DeleteFileResponse>, AppError> {
    info!("📁 Delete file request: {}", file_id);

    let client = create_client_from_headers(&headers)?;

    let response = client
        .files()
        .delete(&file_id)
        .await
        .map_err(|e| {
            error!("❌ Delete file error: {}", e);
            AppError(format!("Delete file error: {}", e))
        })?;

    info!("✅ File удален");
    Ok(Json(response))
}

pub async fn get_file_content(
    State(_state): State<Arc<AppState>>,
    Path(file_id): Path<String>,
    headers: HeaderMap,
) -> Result<Vec<u8>, AppError> {
    info!("📁 Get file content request: {}", file_id);

    let client = create_client_from_headers(&headers)?;

    // В async-openai 0.24 нет retrieve_content, используем retrieve и возвращаем пустой вектор
    // В реальном использовании нужно обновить async-openai до новой версии
    let _response = client
        .files()
        .retrieve(&file_id)
        .await
        .map_err(|e| {
            error!("❌ Get file content error: {}", e);
            AppError(format!("Get file content error: {}", e))
        })?;

    info!("⚠️  File content endpoint не поддерживается в async-openai 0.24");
    Err(AppError(
        "File content endpoint not supported in this version".to_string(),
    ))
}
