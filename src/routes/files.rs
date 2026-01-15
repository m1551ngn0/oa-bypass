//! Обработчики Files API (загрузка, список, удаление, метаданные, контент).
//!
//! Принимает токен из Authorization заголовка и проксирует вызовы к OpenAI Files API
//! без хранения пользовательских данных на сервере.

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

/// Загружает файл в OpenAI Files API через multipart/form-data.
///
/// Ожидает поля `file` (binary) и `purpose` (`assistants` или `fine-tune`). Токен
/// берется из Authorization заголовка клиента. По умолчанию purpose `assistants`.
///
/// # Arguments
/// * `_state` - Состояние приложения
/// * `headers` - Authorization заголовок клиента
/// * `multipart` - Поля multipart/form-data (`file`, `purpose`)
///
/// # Returns
/// * `Ok(Json<OpenAIFile>)` - Метаданные загруженного файла
/// * `Err(AppError)` - Ошибка чтения multipart или запроса к OpenAI
///
/// # Пример
/// ```bash
/// curl -X POST http://localhost:8080/v1/files \
///   -H "Authorization: Bearer sk-..." \
///   -F "purpose=assistants" \
///   -F "file=@./myfile.txt"
/// ```
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

    let client = create_client_from_headers(&headers, false)?;

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

/// Возвращает список файлов в аккаунте пользователя OpenAI.
///
/// # Arguments
/// * `_state` - Состояние приложения
/// * `headers` - Authorization заголовок клиента
///
/// # Returns
/// * `Ok(Json<ListFilesResponse>)` - Список файлов (с пагинацией)
/// * `Err(AppError)` - Ошибка запроса или авторизации
pub async fn list_files(
    State(_state): State<Arc<AppState>>,
    headers: HeaderMap,
) -> Result<Json<ListFilesResponse>, AppError> {
    info!("📁 List files request");

    let client = create_client_from_headers(&headers, false)?;

    let response = client.files().list().await.map_err(|e| {
        error!("❌ List files error: {}", e);
        AppError(format!("List files error: {}", e))
    })?;

    info!("✅ Files list получен");
    Ok(Json(response))
}

/// Возвращает метаданные файла по его `file_id`.
///
/// # Arguments
/// * `_state` - Состояние приложения
/// * `file_id` - Идентификатор файла
/// * `headers` - Authorization заголовок клиента
///
/// # Returns
/// * `Ok(Json<OpenAIFile>)` - Метаданные файла
/// * `Err(AppError)` - Ошибка запроса или файл не найден
pub async fn get_file(
    State(_state): State<Arc<AppState>>,
    Path(file_id): Path<String>,
    headers: HeaderMap,
) -> Result<Json<OpenAIFile>, AppError> {
    info!("📁 Get file request: {}", file_id);

    let client = create_client_from_headers(&headers, false)?;

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

/// Удаляет файл по `file_id`.
///
/// # Arguments
/// * `_state` - Состояние приложения
/// * `headers` - Authorization заголовок клиента
/// * `file_id` - Идентификатор файла для удаления
///
/// # Returns
/// * `Ok(Json<DeleteFileResponse>)` - Подтверждение удаления
/// * `Err(AppError)` - Ошибка запроса или авторизации
pub async fn delete_file(
    State(_state): State<Arc<AppState>>,
    headers: HeaderMap,
    Path(file_id): Path<String>,
) -> Result<Json<DeleteFileResponse>, AppError> {
    info!("📁 Delete file request: {}", file_id);

    let client = create_client_from_headers(&headers, false)?;

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

/// Возвращает содержимое файла байтами.
///
/// # Arguments
/// * `_state` - Состояние приложения
/// * `file_id` - Идентификатор файла
/// * `headers` - Authorization заголовок клиента
///
/// # Returns
/// * `Ok(Vec<u8>)` - Байтовое содержимое файла
/// * `Err(AppError)` - Ошибка запроса или авторизации
pub async fn get_file_content(
    State(_state): State<Arc<AppState>>,
    Path(file_id): Path<String>,
    headers: HeaderMap,
) -> Result<Vec<u8>, AppError> {
    info!("📁 Get file content request: {}", file_id);

    let client = create_client_from_headers(&headers, false)?;

    let bytes = client
        .files()
        .content(&file_id)
        .await
        .map_err(|e| {
            error!("❌ Get file content error: {}", e);
            AppError(format!("Get file content error: {}", e))
        })?;

    info!("✅ File content получен: {} bytes", bytes.len());
    Ok(bytes.to_vec())
}
