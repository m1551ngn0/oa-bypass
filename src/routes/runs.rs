//! Обработчики Runs API (Assistants v2).
//!
//! Управление выполнениями (runs) ассистентов в рамках thread: создание, получение,
//! отмена, обновление и отправка результатов инструментов.

use crate::{error::AppError, state::AppState, utils::create_client_from_headers};
use async_openai::types::assistants::{
    CreateRunRequest, CreateThreadAndRunRequest, ListRunsResponse, ModifyRunRequest, RunObject,
    SubmitToolOutputsRunRequest,
};
use axum::{
    extract::{Path, State},
    http::HeaderMap,
    Json,
};
use std::sync::Arc;
use tracing::{error, info};

/// Создает run в указанном thread.
///
/// # Arguments
/// * `_state` - Состояние приложения
/// * `thread_id` - Идентификатор thread
/// * `headers` - Authorization заголовок клиента
/// * `request` - `CreateRunRequest` с инструкциями/параметрами запуска
///
/// # Returns
/// * `Ok(Json<RunObject>)` - Созданный run
/// * `Err(AppError)` - Ошибка запроса или авторизации
pub async fn create_run(
    State(_state): State<Arc<AppState>>,
    Path(thread_id): Path<String>,
    headers: HeaderMap,
    Json(request): Json<CreateRunRequest>,
) -> Result<Json<RunObject>, AppError> {
    info!("🏃 Create run request in thread: {}", thread_id);

    let client = create_client_from_headers(&headers, true)?;

    let response = client
        .threads()
        .runs(&thread_id)
        .create(request)
        .await
        .map_err(|e| {
            error!("❌ Create run error: {}", e);
            AppError(format!("Create run error: {}", e))
        })?;

    info!("✅ Run создан: {}", response.id);
    Ok(Json(response))
}

/// Возвращает список runs в thread.
///
/// # Arguments
/// * `_state` - Состояние приложения
/// * `thread_id` - Идентификатор thread
/// * `headers` - Authorization заголовок клиента
///
/// # Returns
/// * `Ok(Json<ListRunsResponse>)` - Список runs (с пагинацией)
/// * `Err(AppError)` - Ошибка запроса или авторизации
pub async fn list_runs(
    State(_state): State<Arc<AppState>>,
    Path(thread_id): Path<String>,
    headers: HeaderMap,
) -> Result<Json<ListRunsResponse>, AppError> {
    info!("🏃 List runs request in thread: {}", thread_id);

    let client = create_client_from_headers(&headers, true)?;

    let response = client
        .threads()
        .runs(&thread_id)
        .list()
        .await
        .map_err(|e| {
            error!("❌ List runs error: {}", e);
            AppError(format!("List runs error: {}", e))
        })?;

    info!("✅ Runs list получен");
    Ok(Json(response))
}

/// Возвращает run по идентификатору в рамках thread.
///
/// # Arguments
/// * `_state` - Состояние приложения
/// * `thread_id` - Идентификатор thread
/// * `run_id` - Идентификатор run
/// * `headers` - Authorization заголовок клиента
///
/// # Returns
/// * `Ok(Json<RunObject>)` - Найденный run
/// * `Err(AppError)` - Ошибка запроса или run не найден
pub async fn get_run(
    State(_state): State<Arc<AppState>>,
    Path((thread_id, run_id)): Path<(String, String)>,
    headers: HeaderMap,
) -> Result<Json<RunObject>, AppError> {
    info!("🏃 Get run request: {} in thread: {}", run_id, thread_id);

    let client = create_client_from_headers(&headers, true)?;

    let response = client
        .threads()
        .runs(&thread_id)
        .retrieve(&run_id)
        .await
        .map_err(|e| {
            error!("❌ Get run error: {}", e);
            AppError(format!("Get run error: {}", e))
        })?;

    info!("✅ Run получен");
    Ok(Json(response))
}

/// Обновляет run по идентификатору.
///
/// # Arguments
/// * `_state` - Состояние приложения
/// * `thread_id` - Идентификатор thread
/// * `run_id` - Идентификатор run
/// * `headers` - Authorization заголовок клиента
/// * `request` - `ModifyRunRequest` с изменениями
///
/// # Returns
/// * `Ok(Json<RunObject>)` - Обновленный run
/// * `Err(AppError)` - Ошибка запроса или авторизации
pub async fn modify_run(
    State(_state): State<Arc<AppState>>,
    Path((thread_id, run_id)): Path<(String, String)>,
    headers: HeaderMap,
    Json(request): Json<ModifyRunRequest>,
) -> Result<Json<RunObject>, AppError> {
    info!("🏃 Modify run request: {} in thread: {}", run_id, thread_id);

    let client = create_client_from_headers(&headers, true)?;

    let response = client
        .threads()
        .runs(&thread_id)
        .update(&run_id, request)
        .await
        .map_err(|e| {
            error!("❌ Modify run error: {}", e);
            AppError(format!("Modify run error: {}", e))
        })?;

    info!("✅ Run обновлен");
    Ok(Json(response))
}

/// Отменяет run по идентификатору.
///
/// # Arguments
/// * `_state` - Состояние приложения
/// * `thread_id` - Идентификатор thread
/// * `run_id` - Идентификатор run
/// * `headers` - Authorization заголовок клиента
///
/// # Returns
/// * `Ok(Json<RunObject>)` - Отмененный run
/// * `Err(AppError)` - Ошибка запроса или авторизации
pub async fn cancel_run(
    State(_state): State<Arc<AppState>>,
    Path((thread_id, run_id)): Path<(String, String)>,
    headers: HeaderMap,
) -> Result<Json<RunObject>, AppError> {
    info!("🏃 Cancel run request: {} in thread: {}", run_id, thread_id);

    let client = create_client_from_headers(&headers, true)?;

    let response = client
        .threads()
        .runs(&thread_id)
        .cancel(&run_id)
        .await
        .map_err(|e| {
            error!("❌ Cancel run error: {}", e);
            AppError(format!("Cancel run error: {}", e))
        })?;

    info!("✅ Run отменен");
    Ok(Json(response))
}

/// Отправляет результаты работы инструментов (tool outputs) для run.
///
/// # Arguments
/// * `_state` - Состояние приложения
/// * `thread_id` - Идентификатор thread
/// * `run_id` - Идентификатор run
/// * `headers` - Authorization заголовок клиента
/// * `request` - `SubmitToolOutputsRunRequest` с данными инструментов
///
/// # Returns
/// * `Ok(Json<RunObject>)` - Обновленный run после передачи результатов
/// * `Err(AppError)` - Ошибка запроса или авторизации
pub async fn submit_tool_outputs(
    State(_state): State<Arc<AppState>>,
    Path((thread_id, run_id)): Path<(String, String)>,
    headers: HeaderMap,
    Json(request): Json<SubmitToolOutputsRunRequest>,
) -> Result<Json<RunObject>, AppError> {
    info!(
        "🏃 Submit tool outputs request: {} in thread: {}",
        run_id, thread_id
    );

    let client = create_client_from_headers(&headers, true)?;

    let response = client
        .threads()
        .runs(&thread_id)
        .submit_tool_outputs(&run_id, request)
        .await
        .map_err(|e| {
            error!("❌ Submit tool outputs error: {}", e);
            AppError(format!("Submit tool outputs error: {}", e))
        })?;

    info!("✅ Tool outputs отправлены");
    Ok(Json(response))
}

/// Создает thread и сразу же run (удобно для single-call сценариев).
///
/// # Arguments
/// * `_state` - Состояние приложения
/// * `headers` - Authorization заголовок клиента
/// * `request` - `CreateThreadAndRunRequest` с параметрами thread и run
///
/// # Returns
/// * `Ok(Json<RunObject>)` - Созданный run (и thread) с идентификатором
/// * `Err(AppError)` - Ошибка запроса или авторизации
pub async fn create_thread_and_run(
    State(_state): State<Arc<AppState>>,
    headers: HeaderMap,
    Json(request): Json<CreateThreadAndRunRequest>,
) -> Result<Json<RunObject>, AppError> {
    info!("🏃 Create thread and run request");

    let client = create_client_from_headers(&headers, true)?;

    let response = client
        .threads()
        .create_and_run(request)
        .await
        .map_err(|e| {
            error!("❌ Create thread and run error: {}", e);
            AppError(format!("Create thread and run error: {}", e))
        })?;

    info!("✅ Thread and run созданы: {}", response.id);
    Ok(Json(response))
}
