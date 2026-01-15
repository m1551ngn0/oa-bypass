//! Обработчики Threads API (Assistants v2).
//!
//! Создание, получение, изменение и удаление потоков (threads) ассистентов.

use crate::{error::AppError, state::AppState, utils::create_client_from_headers};
use async_openai::types::assistants::{
    CreateThreadRequest, DeleteThreadResponse, ModifyThreadRequest, ThreadObject,
};
use axum::{
    extract::{Path, State},
    http::HeaderMap,
    Json,
};
use std::sync::Arc;
use tracing::{error, info};

/// Создает новый thread для Assistants API v2.
///
/// # Arguments
/// * `_state` - Состояние приложения
/// * `headers` - Authorization заголовок клиента
/// * `request` - `CreateThreadRequest` с начальными сообщениями/параметрами
///
/// # Returns
/// * `Ok(Json<ThreadObject>)` - Созданный thread с его `id`
/// * `Err(AppError)` - Ошибка запроса или авторизации
pub async fn create_thread(
    State(_state): State<Arc<AppState>>,
    headers: HeaderMap,
    Json(request): Json<CreateThreadRequest>,
) -> Result<Json<ThreadObject>, AppError> {
    info!("💬 Create thread request");

    let client = create_client_from_headers(&headers, true)?;

    let response = client
        .threads()
        .create(request)
        .await
        .map_err(|e| {
            error!("❌ Create thread error: {}", e);
            AppError(format!("Create thread error: {}", e))
        })?;

    info!("✅ Thread создан: {}", response.id);
    Ok(Json(response))
}

/// Возвращает thread по идентификатору.
///
/// # Arguments
/// * `_state` - Состояние приложения
/// * `thread_id` - Идентификатор thread
/// * `headers` - Authorization заголовок клиента
///
/// # Returns
/// * `Ok(Json<ThreadObject>)` - Найденный thread
/// * `Err(AppError)` - Ошибка запроса или thread не найден
pub async fn get_thread(
    State(_state): State<Arc<AppState>>,
    Path(thread_id): Path<String>,
    headers: HeaderMap,
) -> Result<Json<ThreadObject>, AppError> {
    info!("💬 Get thread request: {}", thread_id);

    let client = create_client_from_headers(&headers, true)?;

    let response = client
        .threads()
        .retrieve(&thread_id)
        .await
        .map_err(|e| {
            error!("❌ Get thread error: {}", e);
            AppError(format!("Get thread error: {}", e))
        })?;

    info!("✅ Thread получен");
    Ok(Json(response))
}

/// Обновляет thread по идентификатору.
///
/// # Arguments
/// * `_state` - Состояние приложения
/// * `thread_id` - Идентификатор thread
/// * `headers` - Authorization заголовок клиента
/// * `request` - `ModifyThreadRequest` с полями для обновления
///
/// # Returns
/// * `Ok(Json<ThreadObject>)` - Обновленный thread
/// * `Err(AppError)` - Ошибка запроса или авторизации
pub async fn modify_thread(
    State(_state): State<Arc<AppState>>,
    Path(thread_id): Path<String>,
    headers: HeaderMap,
    Json(request): Json<ModifyThreadRequest>,
) -> Result<Json<ThreadObject>, AppError> {
    info!("💬 Modify thread request: {}", thread_id);

    let client = create_client_from_headers(&headers, true)?;

    let response = client
        .threads()
        .update(&thread_id, request)
        .await
        .map_err(|e| {
            error!("❌ Modify thread error: {}", e);
            AppError(format!("Modify thread error: {}", e))
        })?;

    info!("✅ Thread обновлен");
    Ok(Json(response))
}

/// Удаляет thread по идентификатору.
///
/// # Arguments
/// * `_state` - Состояние приложения
/// * `thread_id` - Идентификатор thread
/// * `headers` - Authorization заголовок клиента
///
/// # Returns
/// * `Ok(Json<DeleteThreadResponse>)` - Подтверждение удаления
/// * `Err(AppError)` - Ошибка запроса или авторизации
pub async fn delete_thread(
    State(_state): State<Arc<AppState>>,
    Path(thread_id): Path<String>,
    headers: HeaderMap,
) -> Result<Json<DeleteThreadResponse>, AppError> {
    info!("💬 Delete thread request: {}", thread_id);

    let client = create_client_from_headers(&headers, true)?;

    let response = client
        .threads()
        .delete(&thread_id)
        .await
        .map_err(|e| {
            error!("❌ Delete thread error: {}", e);
            AppError(format!("Delete thread error: {}", e))
        })?;

    info!("✅ Thread удален");
    Ok(Json(response))
}
