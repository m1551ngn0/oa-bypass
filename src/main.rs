//! OpenAI API Bypass Server
//!
//! Высокопроизводительный прокси-сервер для OpenAI API на Rust.
//! Принимает токен от клиента в Authorization заголовке и перенаправляет
//! запросы к официальному OpenAI API без хранения конфиденциальных данных.

mod error;
mod routes;
mod state;
mod utils;

use state::AppState;
use std::sync::Arc;
use tower_http::cors::CorsLayer;
use tracing::info;

/// Точка входа приложения.
///
/// Инициализирует логирование, создает состояние приложения, настраивает роутер
/// с CORS middleware и запускает HTTP сервер на порту 8080.
#[tokio::main]
async fn main() {
    // Инициализация логирования
    tracing_subscriber::fmt::init();

    // Создаем состояние приложения (токен будет приходить от клиента)
    let state = Arc::new(AppState::new());

    // Создаем роутер
    let app = routes::create_router(state).layer(CorsLayer::permissive());

    let addr = "0.0.0.0:8080";
    info!("🚀 OpenAI API сервер запущен на http://{}", addr);
    info!("📡 Сервер работает в режиме passthrough");
    info!("📡 Токен OpenAI должен передаваться в Authorization заголовке от клиента");
    info!("📡 Доступные эндпоинты:");
    info!("   Completions: POST /v1/chat/completions, /v1/completions");
    info!("   Models: GET /v1/models");
    info!("   Assistants: POST/GET/DELETE /v1/assistants");
    info!("   Threads: POST/GET/DELETE /v1/threads");
    info!("   Messages: POST/GET /v1/threads/:id/messages");
    info!("   Runs: POST/GET /v1/threads/:id/runs");
    info!("   Files: POST/GET/DELETE /v1/files");
    info!("   Responses: POST/GET/DELETE /v1/responses");

    let listener = tokio::net::TcpListener::bind(addr)
        .await
        .expect("Не удалось привязаться к адресу");

    axum::serve(listener, app)
        .await
        .expect("Ошибка запуска сервера");
}
