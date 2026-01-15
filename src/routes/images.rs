//! Обработчики DALL-E (Images API).
//!
//! Проксирует запросы генерации изображений к OpenAI Images API.

use crate::{error::AppError, state::AppState, utils::create_client_from_headers};
use async_openai::types::images::{CreateImageRequest, ImagesResponse};
use axum::{extract::State, http::HeaderMap, Json};
use std::sync::Arc;
use tracing::{error, info};

/// Генерирует изображение по текстовому описанию через OpenAI Images API.
///
/// # Arguments
/// * `_state` - Состояние приложения
/// * `headers` - Authorization заголовок клиента
/// * `request` - `CreateImageRequest` с prompt/параметрами генерации
///
/// # Returns
/// * `Ok(Json<ImagesResponse>)` - Сгенерированные изображения/ссылки/base64
/// * `Err(AppError)` - Ошибка запроса или авторизации
pub async fn create_image(
    State(_state): State<Arc<AppState>>,
    headers: HeaderMap,
    Json(request): Json<CreateImageRequest>,
) -> Result<Json<ImagesResponse>, AppError> {
    info!("🎨 Image generation request");

    let client = create_client_from_headers(&headers, false)?;

    let response = client
        .images()
        .generate(request)
        .await
        .map_err(|e| {
            error!("❌ Image generation error: {}", e);
            AppError(format!("Image generation error: {}", e))
        })?;

    info!("✅ Image успешно сгенерирован");
    Ok(Json(response))
}
