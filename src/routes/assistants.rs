//! Обработчики Assistants API v2.
//!
//! Создание, получение, изменение и удаление ассистентов OpenAI.

use crate::{error::AppError, state::AppState, utils::create_client_from_headers};
use async_openai::types::assistants::{
    AssistantObject, CreateAssistantRequest, DeleteAssistantResponse, ListAssistantsResponse,
    ModifyAssistantRequest,
};
use axum::{
    extract::{Path, State},
    http::HeaderMap,
    Json,
};
use std::sync::Arc;
use tracing::{error, info};

/// Создает ассистента (Assistants API v2) с параметрами из тела запроса.
///
/// # Arguments
/// * `_state` - Состояние приложения (не используется, токен приходит с клиента)
/// * `headers` - HTTP заголовки запроса, содержащие Authorization токен
/// * `request` - Тело запроса `CreateAssistantRequest` с настройками ассистента
///
/// # Returns
/// * `Ok(Json<AssistantObject>)` - Созданный ассистент с его `id`
/// * `Err(AppError)` - Ошибка при обращении к OpenAI API или валидации токена
///
/// # Пример
/// ```bash
/// curl -X POST http://localhost:8080/v1/assistants \
///   -H "Content-Type: application/json" \
///   -H "Authorization: Bearer sk-..." \
///   -d '{"name":"My Assistant","model":"gpt-4o-mini"}'
/// ```
pub async fn create_assistant(
    State(_state): State<Arc<AppState>>,
    headers: HeaderMap,
    Json(request): Json<CreateAssistantRequest>,
) -> Result<Json<AssistantObject>, AppError> {
    info!("🤖 Create assistant request");

    let client = create_client_from_headers(&headers, true)?;

    let response = client
        .assistants()
        .create(request)
        .await
        .map_err(|e| {
            error!("❌ Create assistant error: {}", e);
            AppError(format!("Create assistant error: {}", e))
        })?;

    info!("✅ Assistant создан: {}", response.id);
    Ok(Json(response))
}

/// Возвращает список ассистентов текущего пользователя.
///
/// # Arguments
/// * `_state` - Состояние приложения
/// * `headers` - Authorization заголовок клиента
///
/// # Returns
/// * `Ok(Json<ListAssistantsResponse>)` - Страница ассистентов (с пагинацией)
/// * `Err(AppError)` - Ошибка запроса или авторизации
pub async fn list_assistants(
    State(_state): State<Arc<AppState>>,
    headers: HeaderMap,
) -> Result<Json<ListAssistantsResponse>, AppError> {
    info!("📋 List assistants request");

    let client = create_client_from_headers(&headers, true)?;

    let response = client
        .assistants()
        .list()
        .await
        .map_err(|e| {
            error!("❌ List assistants error: {}", e);
            AppError(format!("List assistants error: {}", e))
        })?;

    info!("✅ Assistants list получен");
    Ok(Json(response))
}

/// Возвращает ассистента по `assistant_id`.
///
/// # Arguments
/// * `_state` - Состояние приложения
/// * `assistant_id` - Идентификатор ассистента
/// * `headers` - Authorization заголовок клиента
///
/// # Returns
/// * `Ok(Json<AssistantObject>)` - Найденный ассистент
/// * `Err(AppError)` - Ошибка запроса или ассистент не найден
pub async fn get_assistant(
    State(_state): State<Arc<AppState>>,
    Path(assistant_id): Path<String>,
    headers: HeaderMap,
) -> Result<Json<AssistantObject>, AppError> {
    info!("🤖 Get assistant request: {}", assistant_id);

    let client = create_client_from_headers(&headers, true)?;

    let response = client
        .assistants()
        .retrieve(&assistant_id)
        .await
        .map_err(|e| {
            error!("❌ Get assistant error: {}", e);
            AppError(format!("Get assistant error: {}", e))
        })?;

    info!("✅ Assistant получен");
    Ok(Json(response))
}

/// Обновляет ассистента по `assistant_id`.
///
/// # Arguments
/// * `_state` - Состояние приложения
/// * `assistant_id` - Идентификатор ассистента
/// * `headers` - Authorization заголовок клиента
/// * `request` - `ModifyAssistantRequest` с изменяемыми полями
///
/// # Returns
/// * `Ok(Json<AssistantObject>)` - Обновленный ассистент
/// * `Err(AppError)` - Ошибка запроса или авторизации
pub async fn modify_assistant(
    State(_state): State<Arc<AppState>>,
    Path(assistant_id): Path<String>,
    headers: HeaderMap,
    Json(request): Json<ModifyAssistantRequest>,
) -> Result<Json<AssistantObject>, AppError> {
    info!("🤖 Modify assistant request: {}", assistant_id);

    let client = create_client_from_headers(&headers, true)?;

    let response = client
        .assistants()
        .update(&assistant_id, request)
        .await
        .map_err(|e| {
            error!("❌ Modify assistant error: {}", e);
            AppError(format!("Modify assistant error: {}", e))
        })?;

    info!("✅ Assistant обновлен");
    Ok(Json(response))
}

/// Удаляет ассистента по `assistant_id`.
///
/// # Arguments
/// * `_state` - Состояние приложения
/// * `headers` - Authorization заголовок клиента
/// * `assistant_id` - Идентификатор ассистента для удаления
///
/// # Returns
/// * `Ok(Json<DeleteAssistantResponse>)` - Подтверждение удаления
/// * `Err(AppError)` - Ошибка запроса или авторизации
pub async fn delete_assistant(
    State(_state): State<Arc<AppState>>,
    headers: HeaderMap,
    Path(assistant_id): Path<String>,
) -> Result<Json<DeleteAssistantResponse>, AppError> {
    info!("🤖 Delete assistant request: {}", assistant_id);

    let client = create_client_from_headers(&headers, true)?;

    let response = client
        .assistants()
        .delete(&assistant_id)
        .await
        .map_err(|e| {
            error!("❌ Delete assistant error: {}", e);
            AppError(format!("Delete assistant error: {}", e))
        })?;

    info!("✅ Assistant удален");
    Ok(Json(response))
}
