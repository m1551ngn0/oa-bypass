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

pub async fn create_thread(
    State(_state): State<Arc<AppState>>,
    headers: HeaderMap,
    Json(request): Json<CreateThreadRequest>,
) -> Result<Json<ThreadObject>, AppError> {
    info!("💬 Create thread request");

    let client = create_client_from_headers(&headers)?;

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

pub async fn get_thread(
    State(_state): State<Arc<AppState>>,
    Path(thread_id): Path<String>,
    headers: HeaderMap,
) -> Result<Json<ThreadObject>, AppError> {
    info!("💬 Get thread request: {}", thread_id);

    let client = create_client_from_headers(&headers)?;

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

pub async fn modify_thread(
    State(_state): State<Arc<AppState>>,
    Path(thread_id): Path<String>,
    headers: HeaderMap,
    Json(request): Json<ModifyThreadRequest>,
) -> Result<Json<ThreadObject>, AppError> {
    info!("💬 Modify thread request: {}", thread_id);

    let client = create_client_from_headers(&headers)?;

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

pub async fn delete_thread(
    State(_state): State<Arc<AppState>>,
    Path(thread_id): Path<String>,
    headers: HeaderMap,
) -> Result<Json<DeleteThreadResponse>, AppError> {
    info!("💬 Delete thread request: {}", thread_id);

    let client = create_client_from_headers(&headers)?;

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
