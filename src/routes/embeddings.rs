//! Обработчики Embeddings API.
//!
//! Проксирует запросы к OpenAI Embeddings (например, text-embedding-3-large).

use crate::{error::AppError, state::AppState, utils::create_client_from_headers};
use async_openai::types::embeddings::{CreateEmbeddingRequest, CreateEmbeddingResponse};
use axum::{extract::State, http::HeaderMap, Json};
use std::sync::Arc;
use tracing::{error, info};

/// Создает embedding для текста или батча текстов.
///
/// Берет токен из Authorization заголовка, форвардит запрос в OpenAI и возвращает
/// результат без хранения данных на сервере.
///
/// # Arguments
/// * `_state` - Состояние приложения
/// * `headers` - Authorization заголовок клиента
/// * `request` - `CreateEmbeddingRequest` с моделью и входными данными
///
/// # Returns
/// * `Ok(Json<CreateEmbeddingResponse>)` - Векторы эмбеддингов
/// * `Err(AppError)` - Ошибка запроса или авторизации
pub async fn embeddings(
    State(_state): State<Arc<AppState>>,
    headers: HeaderMap,
    Json(request): Json<CreateEmbeddingRequest>,
) -> Result<Json<CreateEmbeddingResponse>, AppError> {
    info!("🔢 Embedding request: model={}", request.model);

    let client = create_client_from_headers(&headers, false)?;

    let response = client
        .embeddings()
        .create(request)
        .await
        .map_err(|e| {
            error!("❌ Embedding error: {}", e);
            AppError(format!("Embedding error: {}", e))
        })?;

    info!("✅ Embedding успешно выполнен");
    Ok(Json(response))
}
