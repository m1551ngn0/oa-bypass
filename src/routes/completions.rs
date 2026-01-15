//! Модуль обработки запросов на генерацию текста (completions).
//!
//! Поддерживает два типа completions:
//! - Chat Completions (GPT-4, GPT-3.5 Turbo и другие чат-модели)
//! - Legacy Text Completions (старые модели)

use crate::{error::AppError, state::AppState, utils::create_client_from_headers};
use async_openai::types::chat::{CreateChatCompletionRequest, CreateChatCompletionResponse};
use async_openai::types::completions::{CreateCompletionRequest, CreateCompletionResponse};
use axum::{extract::State, http::HeaderMap, Json};
use std::sync::Arc;
use tracing::{error, info};

/// Обработчик для создания chat completion.
///
/// Проксирует запрос к OpenAI API для генерации ответа от чат-модели (GPT-4, GPT-3.5-turbo и т.д.).
/// Использует токен из Authorization заголовка клиента.
///
/// # Arguments
///
/// * `_state` - Состояние приложения (не используется, так как токен передается от клиента)
/// * `headers` - HTTP заголовки запроса, содержащие Authorization токен
/// * `request` - Параметры запроса для создания chat completion
///
/// # Returns
///
/// * `Ok(Json<CreateChatCompletionResponse>)` - Успешный ответ от OpenAI API
/// * `Err(AppError)` - Ошибка при выполнении запроса
///
/// # Examples
///
/// ```bash
/// curl -X POST http://localhost:8080/v1/chat/completions \
///   -H "Content-Type: application/json" \
///   -H "Authorization: Bearer sk-..." \
///   -d '{
///     "model": "gpt-4",
///     "messages": [{"role": "user", "content": "Hello!"}]
///   }'
/// ```
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

/// Обработчик для создания legacy text completion.
///
/// Проксирует запрос к OpenAI API для генерации текста с использованием старых моделей completions.
/// Использует токен из Authorization заголовка клиента.
///
/// # Arguments
///
/// * `_state` - Состояние приложения (не используется, так как токен передается от клиента)
/// * `headers` - HTTP заголовки запроса, содержащие Authorization токен
/// * `request` - Параметры запроса для создания text completion
///
/// # Returns
///
/// * `Ok(Json<CreateCompletionResponse>)` - Успешный ответ от OpenAI API
/// * `Err(AppError)` - Ошибка при выполнении запроса
///
/// # Examples
///
/// ```bash
/// curl -X POST http://localhost:8080/v1/completions \
///   -H "Content-Type: application/json" \
///   -H "Authorization: Bearer sk-..." \
///   -d '{
///     "model": "text-davinci-003",
///     "prompt": "Once upon a time"
///   }'
/// ```
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
