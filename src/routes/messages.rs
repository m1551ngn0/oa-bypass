//! Обработчики Messages API (Assistants v2).
//!
//! Создание, получение, изменение и листинг сообщений внутри thread.

use crate::{error::AppError, state::AppState, utils::create_client_from_headers};
use async_openai::types::assistants::{
    CreateMessageRequest, ListMessagesResponse, MessageObject, ModifyMessageRequest,
};
use axum::{
    extract::{Path, State},
    http::HeaderMap,
    Json,
};
use std::sync::Arc;
use tracing::{error, info};

/// Создает сообщение внутри thread.
///
/// # Arguments
/// * `_state` - Состояние приложения
/// * `thread_id` - Идентификатор thread, в котором создается сообщение
/// * `headers` - Authorization заголовок клиента
/// * `request` - `CreateMessageRequest` с содержимым сообщения
///
/// # Returns
/// * `Ok(Json<MessageObject>)` - Созданное сообщение
/// * `Err(AppError)` - Ошибка запроса или авторизации
pub async fn create_message(
    State(_state): State<Arc<AppState>>,
    Path(thread_id): Path<String>,
    headers: HeaderMap,
    Json(request): Json<CreateMessageRequest>,
) -> Result<Json<MessageObject>, AppError> {
    info!("💭 Create message request in thread: {}", thread_id);

    let client = create_client_from_headers(&headers, true)?;

    let response = client
        .threads()
        .messages(&thread_id)
        .create(request)
        .await
        .map_err(|e| {
            error!("❌ Create message error: {}", e);
            AppError(format!("Create message error: {}", e))
        })?;

    info!("✅ Message создано: {}", response.id);
    Ok(Json(response))
}

/// Возвращает список сообщений в thread.
///
/// # Arguments
/// * `_state` - Состояние приложения
/// * `thread_id` - Идентификатор thread
/// * `headers` - Authorization заголовок клиента
///
/// # Returns
/// * `Ok(Json<ListMessagesResponse>)` - Список сообщений (с пагинацией)
/// * `Err(AppError)` - Ошибка запроса или авторизации
pub async fn list_messages(
    State(_state): State<Arc<AppState>>,
    Path(thread_id): Path<String>,
    headers: HeaderMap,
) -> Result<Json<ListMessagesResponse>, AppError> {
    info!("💭 List messages request in thread: {}", thread_id);

    let client = create_client_from_headers(&headers, true)?;

    let response = client
        .threads()
        .messages(&thread_id)
        .list()
        .await
        .map_err(|e| {
            error!("❌ List messages error: {}", e);
            AppError(format!("List messages error: {}", e))
        })?;

    info!("✅ Messages list получен");
    Ok(Json(response))
}

/// Возвращает конкретное сообщение по `message_id` в рамках thread.
///
/// # Arguments
/// * `_state` - Состояние приложения
/// * `thread_id` - Идентификатор thread
/// * `message_id` - Идентификатор сообщения
/// * `headers` - Authorization заголовок клиента
///
/// # Returns
/// * `Ok(Json<MessageObject>)` - Найденное сообщение
/// * `Err(AppError)` - Ошибка запроса или сообщение не найдено
pub async fn get_message(
    State(_state): State<Arc<AppState>>,
    Path((thread_id, message_id)): Path<(String, String)>,
    headers: HeaderMap,
) -> Result<Json<MessageObject>, AppError> {
    info!(
        "💭 Get message request: {} in thread: {}",
        message_id, thread_id
    );

    let client = create_client_from_headers(&headers, true)?;

    let response = client
        .threads()
        .messages(&thread_id)
        .retrieve(&message_id)
        .await
        .map_err(|e| {
            error!("❌ Get message error: {}", e);
            AppError(format!("Get message error: {}", e))
        })?;

    info!("✅ Message получено");
    Ok(Json(response))
}

/// Обновляет сообщение по `message_id` в рамках thread.
///
/// # Arguments
/// * `_state` - Состояние приложения
/// * `thread_id` - Идентификатор thread
/// * `message_id` - Идентификатор сообщения
/// * `headers` - Authorization заголовок клиента
/// * `request` - `ModifyMessageRequest` с изменениями
///
/// # Returns
/// * `Ok(Json<MessageObject>)` - Обновленное сообщение
/// * `Err(AppError)` - Ошибка запроса или авторизации
pub async fn modify_message(
    State(_state): State<Arc<AppState>>,
    Path((thread_id, message_id)): Path<(String, String)>,
    headers: HeaderMap,
    Json(request): Json<ModifyMessageRequest>,
) -> Result<Json<MessageObject>, AppError> {
    info!(
        "💭 Modify message request: {} in thread: {}",
        message_id, thread_id
    );

    let client = create_client_from_headers(&headers, true)?;

    let response = client
        .threads()
        .messages(&thread_id)
        .update(&message_id, request)
        .await
        .map_err(|e| {
            error!("❌ Modify message error: {}", e);
            AppError(format!("Modify message error: {}", e))
        })?;

    info!("✅ Message обновлено");
    Ok(Json(response))
}
