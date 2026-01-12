use crate::{error::AppError, state::AppState, utils::create_client_from_headers};
use async_openai::types::images::{CreateImageRequest, ImagesResponse};
use axum::{extract::State, http::HeaderMap, Json};
use std::sync::Arc;
use tracing::{error, info};

pub async fn create_image(
    State(_state): State<Arc<AppState>>,
    headers: HeaderMap,
    Json(request): Json<CreateImageRequest>,
) -> Result<Json<ImagesResponse>, AppError> {
    info!("🎨 Image generation request");

    let client = create_client_from_headers(&headers)?;

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
