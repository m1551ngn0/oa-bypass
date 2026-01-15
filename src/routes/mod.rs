//! Модуль маршрутизации.
//!
//! Содержит все обработчики эндпоинтов и конфигурацию роутера приложения.

pub mod assistants;
pub mod completions;
pub mod embeddings;
pub mod files;
pub mod images;
pub mod messages;
pub mod models;
pub mod responses;
pub mod runs;
pub mod threads;

use crate::state::AppState;
use axum::{routing::{delete, get, post}, Router};
use std::sync::Arc;

/// Создает и конфигурирует главный роутер приложения.
///
/// Регистрирует все эндпоинты для различных сервисов OpenAI API:
/// - Health checks
/// - Completions (chat и legacy)
/// - Embeddings
/// - Models
/// - Images (DALL-E)
/// - Assistants API (assistants, threads, messages, runs)
/// - Files API
/// - Responses API
///
/// # Arguments
///
/// * `state` - Общее состояние приложения, передаваемое во все обработчики
///
/// # Returns
///
/// Сконфигурированный `Router` с зарегистрированными маршрутами
pub fn create_router(state: Arc<AppState>) -> Router {
    Router::new()
        // Health check
        .route("/", get(health_check))
        .route("/health", get(health_check))
        
        // ===== Completions API =====
        .route("/v1/chat/completions", post(completions::chat_completions))
        .route("/v1/completions", post(completions::completions))
        
        // ===== Embeddings =====
        .route("/v1/embeddings", post(embeddings::embeddings))
        
        // ===== Models =====
        .route("/v1/models", get(models::list_models))
        .route("/v1/models/{model_id}", get(models::get_model))
        
        // ===== Images =====
        .route("/v1/images/generations", post(images::create_image))
        
        // ===== Assistants API =====
        .route("/v1/assistants", post(assistants::create_assistant))
        .route("/v1/assistants", get(assistants::list_assistants))
        .route("/v1/assistants/{assistant_id}", get(assistants::get_assistant))
        .route("/v1/assistants/{assistant_id}", post(assistants::modify_assistant))
        .route("/v1/assistants/{assistant_id}", delete(assistants::delete_assistant))
        
        // ===== Threads API =====
        .route("/v1/threads", post(threads::create_thread))
        .route("/v1/threads/{thread_id}", get(threads::get_thread))
        .route("/v1/threads/{thread_id}", post(threads::modify_thread))
        .route("/v1/threads/{thread_id}", delete(threads::delete_thread))
        
        // ===== Messages API =====
        .route("/v1/threads/{thread_id}/messages", post(messages::create_message))
        .route("/v1/threads/{thread_id}/messages", get(messages::list_messages))
        .route("/v1/threads/{thread_id}/messages/{message_id}", get(messages::get_message))
        .route("/v1/threads/{thread_id}/messages/{message_id}", post(messages::modify_message))
        
        // ===== Runs API =====
        .route("/v1/threads/{thread_id}/runs", post(runs::create_run))
        .route("/v1/threads/{thread_id}/runs", get(runs::list_runs))
        .route("/v1/threads/{thread_id}/runs/{run_id}", get(runs::get_run))
        .route("/v1/threads/{thread_id}/runs/{run_id}", post(runs::modify_run))
        .route("/v1/threads/{thread_id}/runs/{run_id}/cancel", post(runs::cancel_run))
        .route("/v1/threads/{thread_id}/runs/{run_id}/submit_tool_outputs", post(runs::submit_tool_outputs))
        .route("/v1/threads/runs", post(runs::create_thread_and_run))
        
        // ===== Files API =====
        .route("/v1/files", post(files::upload_file))
        .route("/v1/files", get(files::list_files))
        .route("/v1/files/{file_id}", get(files::get_file))
        .route("/v1/files/{file_id}", delete(files::delete_file))
        .route("/v1/files/{file_id}/content", get(files::get_file_content))
        
        // ===== Responses API =====
        .route("/v1/responses", post(responses::create_response))
        .route("/v1/responses/{response_id}", get(responses::get_response))
        .route("/v1/responses/{response_id}", delete(responses::delete_response))
        .route("/v1/responses/{response_id}/cancel", post(responses::cancel_response))
        
        .with_state(state)
}

/// Обработчик health check эндпоинта.
///
/// Возвращает простую строку, подтверждающую работоспособность сервера.
/// Используется для мониторинга и проверки доступности сервиса.
///
/// # Returns
///
/// Статическая строка "OpenAI API Server is running"
async fn health_check() -> &'static str {
    "OpenAI API Server is running"
}
