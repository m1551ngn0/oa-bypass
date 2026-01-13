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
