use crate::{error::AppError, state::AppState, utils::create_client_from_headers};
use async_openai::types::models::{ListModelResponse, Model};
use axum::{
    extract::{Path, State},
    http::HeaderMap,
    Json,
};
use std::sync::Arc;
use tracing::{error, info};

pub async fn list_models(
    State(_state): State<Arc<AppState>>,
    headers: HeaderMap,
) -> Result<Json<ListModelResponse>, AppError> {
    info!("📋 List models request");

    let client = create_client_from_headers(&headers)?;

    let response = client
        .models()
        .list()
        .await
        .map_err(|e| {
            error!("❌ List models error: {}", e);
            AppError(format!("List models error: {}", e))
        })?;

    info!("✅ Models list получен");
    Ok(Json(response))
}

pub async fn get_model(
    State(_state): State<Arc<AppState>>,
    Path(model_id): Path<String>,
    headers: HeaderMap,
) -> Result<Json<Model>, AppError> {
    info!("📋 Get model request: {}", model_id);

    let client = create_client_from_headers(&headers)?;

    let response = client
        .models()
        .retrieve(&model_id)
        .await
        .map_err(|e| {
            error!("❌ Get model error: {}", e);
            AppError(format!("Get model error: {}", e))
        })?;

    info!("✅ Model получена");
    Ok(Json(response))
}
