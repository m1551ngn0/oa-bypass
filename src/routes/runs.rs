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

pub async fn create_run(
    State(_state): State<Arc<AppState>>,
    Path(thread_id): Path<String>,
    headers: HeaderMap,
    Json(request): Json<CreateRunRequest>,
) -> Result<Json<RunObject>, AppError> {
    info!("🏃 Create run request in thread: {}", thread_id);

    let client = create_client_from_headers(&headers)?;

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

pub async fn list_runs(
    State(_state): State<Arc<AppState>>,
    Path(thread_id): Path<String>,
    headers: HeaderMap,
) -> Result<Json<ListRunsResponse>, AppError> {
    info!("🏃 List runs request in thread: {}", thread_id);

    let client = create_client_from_headers(&headers)?;

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

pub async fn get_run(
    State(_state): State<Arc<AppState>>,
    Path((thread_id, run_id)): Path<(String, String)>,
    headers: HeaderMap,
) -> Result<Json<RunObject>, AppError> {
    info!("🏃 Get run request: {} in thread: {}", run_id, thread_id);

    let client = create_client_from_headers(&headers)?;

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

pub async fn modify_run(
    State(_state): State<Arc<AppState>>,
    Path((thread_id, run_id)): Path<(String, String)>,
    headers: HeaderMap,
    Json(request): Json<ModifyRunRequest>,
) -> Result<Json<RunObject>, AppError> {
    info!("🏃 Modify run request: {} in thread: {}", run_id, thread_id);

    let client = create_client_from_headers(&headers)?;

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

pub async fn cancel_run(
    State(_state): State<Arc<AppState>>,
    Path((thread_id, run_id)): Path<(String, String)>,
    headers: HeaderMap,
) -> Result<Json<RunObject>, AppError> {
    info!("🏃 Cancel run request: {} in thread: {}", run_id, thread_id);

    let client = create_client_from_headers(&headers)?;

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

    let client = create_client_from_headers(&headers)?;

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

pub async fn create_thread_and_run(
    State(_state): State<Arc<AppState>>,
    headers: HeaderMap,
    Json(request): Json<CreateThreadAndRunRequest>,
) -> Result<Json<RunObject>, AppError> {
    info!("🏃 Create thread and run request");

    let client = create_client_from_headers(&headers)?;

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
