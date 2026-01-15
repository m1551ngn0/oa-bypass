//! Обработчики Responses API.
//!
//! Управление объектами responses: создание, получение, удаление и отмена.

use crate::{error::AppError, state::AppState, utils::create_client_from_headers};
use async_openai::types::responses::{CreateResponse, DeleteResponse, Response};
use axum::{
    extract::{Path, State},
    http::HeaderMap,
    Json,
};
use std::sync::Arc;
use tracing::{error, info};

/// Создает response через OpenAI Responses API.
///
/// # Arguments
/// * `_state` - Состояние приложения
/// * `headers` - Authorization заголовок клиента
/// * `request` - `CreateResponse` с параметрами ответа
///
/// # Returns
/// * `Ok(Json<Response>)` - Созданный response
/// * `Err(AppError)` - Ошибка запроса или авторизации
pub async fn create_response(
    State(_state): State<Arc<AppState>>,
    headers: HeaderMap,
    Json(request): Json<CreateResponse>,
) -> Result<Json<Response>, AppError> {
    info!("💬 Create response request");

    let client = create_client_from_headers(&headers, false)?;

    let response = client
        .responses()
        .create(request)
        .await
        .map_err(|e| {
            error!("❌ Create response error: {}", e);
            AppError(format!("Create response error: {}", e))
        })?;

    info!("✅ Response создан: {}", response.id);
    Ok(Json(response))
}

/// Возвращает response по идентификатору.
///
/// # Arguments
/// * `_state` - Состояние приложения
/// * `response_id` - Идентификатор response
/// * `headers` - Authorization заголовок клиента
///
/// # Returns
/// * `Ok(Json<Response>)` - Найденный response
/// * `Err(AppError)` - Ошибка запроса или объект не найден
pub async fn get_response(
    State(_state): State<Arc<AppState>>,
    Path(response_id): Path<String>,
    headers: HeaderMap,
) -> Result<Json<Response>, AppError> {
    info!("💬 Get response request: {}", response_id);

    let client = create_client_from_headers(&headers, false)?;

    let response = client
        .responses()
        .retrieve(&response_id)
        .await
        .map_err(|e| {
            error!("❌ Get response error: {}", e);
            AppError(format!("Get response error: {}", e))
        })?;

    info!("✅ Response получен: {}", response_id);
    Ok(Json(response))
}

/// Удаляет response по идентификатору.
///
/// # Arguments
/// * `_state` - Состояние приложения
/// * `response_id` - Идентификатор response
/// * `headers` - Authorization заголовок клиента
///
/// # Returns
/// * `Ok(Json<DeleteResponse>)` - Подтверждение удаления
/// * `Err(AppError)` - Ошибка запроса или авторизации
pub async fn delete_response(
    State(_state): State<Arc<AppState>>,
    Path(response_id): Path<String>,
    headers: HeaderMap,
) -> Result<Json<DeleteResponse>, AppError> {
    info!("💬 Delete response request: {}", response_id);

    let client = create_client_from_headers(&headers, false)?;

    let response = client
        .responses()
        .delete(&response_id)
        .await
        .map_err(|e| {
            error!("❌ Delete response error: {}", e);
            AppError(format!("Delete response error: {}", e))
        })?;

    info!("✅ Response удалён: {}", response_id);
    Ok(Json(response))
}

/// Отменяет выполнение response по идентификатору.
///
/// # Arguments
/// * `_state` - Состояние приложения
/// * `response_id` - Идентификатор response
/// * `headers` - Authorization заголовок клиента
///
/// # Returns
/// * `Ok(Json<Response>)` - Отмененный response
/// * `Err(AppError)` - Ошибка запроса или авторизации
pub async fn cancel_response(
    State(_state): State<Arc<AppState>>,
    Path(response_id): Path<String>,
    headers: HeaderMap,
) -> Result<Json<Response>, AppError> {
    info!("💬 Cancel response request: {}", response_id);

    let client = create_client_from_headers(&headers, false)?;

    let response = client
        .responses()
        .cancel(&response_id)
        .await
        .map_err(|e| {
            error!("❌ Cancel response error: {}", e);
            AppError(format!("Cancel response error: {}", e))
        })?;

    info!("✅ Response отменён: {}", response_id);
    Ok(Json(response))
}
