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
