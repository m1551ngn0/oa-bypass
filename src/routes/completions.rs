use crate::{error::AppError, state::AppState, utils::create_client_from_headers};
use async_openai::types::chat::{CreateChatCompletionRequest, CreateChatCompletionResponse};
use async_openai::types::completions::{CreateCompletionRequest, CreateCompletionResponse};
use axum::{extract::State, http::HeaderMap, Json};
use std::sync::Arc;
use tracing::{error, info};

pub async fn chat_completions(
    State(_state): State<Arc<AppState>>,
    headers: HeaderMap,
    Json(request): Json<CreateChatCompletionRequest>,
) -> Result<Json<CreateChatCompletionResponse>, AppError> {
    info!("💬 Chat completion request: model={}", request.model);

    // Создаем клиента из Authorization заголовка
    let client = create_client_from_headers(&headers, false)?;

    let response = client
        .chat()
        .create(request)
        .await
        .map_err(|e| {
            error!("❌ Chat completion error: {}", e);
            AppError(format!("Chat completion error: {}", e))
        })?;

    info!("✅ Chat completion успешно выполнен");
    Ok(Json(response))
}

pub async fn completions(
    State(_state): State<Arc<AppState>>,
    headers: HeaderMap,
    Json(request): Json<CreateCompletionRequest>,
) -> Result<Json<CreateCompletionResponse>, AppError> {
    info!("📝 Text completion request: model={}", request.model);

    // Создаем клиента из Authorization заголовка
    let client = create_client_from_headers(&headers, false)?;

    let response = client
        .completions()
        .create(request)
        .await
        .map_err(|e| {
            error!("❌ Text completion error: {}", e);
            AppError(format!("Text completion error: {}", e))
        })?;

    info!("✅ Text completion успешно выполнен");
    Ok(Json(response))
}
