//! Модуль обработки ошибок.
//!
//! Содержит типы ошибок и их преобразование в HTTP ответы.

use axum::{http::StatusCode, response::IntoResponse};

/// Основной тип ошибки приложения.
///
/// Оборачивает строковое описание ошибки и автоматически преобразуется
/// в HTTP ответ со статусом 500 Internal Server Error.
pub struct AppError(pub String);

impl IntoResponse for AppError {
    /// Преобразует ошибку в HTTP ответ.
    ///
    /// # Returns
    ///
    /// HTTP ответ со статусом 500 и текстовым описанием ошибки.
    fn into_response(self) -> axum::response::Response {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Ошибка: {}", self.0),
        )
            .into_response()
    }
}

impl From<String> for AppError {
    /// Создает AppError из строки.
    ///
    /// # Arguments
    ///
    /// * `s` - Строка с описанием ошибки.
    ///
    /// # Returns
    ///
    /// Новый экземпляр `AppError`.
    fn from(s: String) -> Self {
        AppError(s)
    }
}
