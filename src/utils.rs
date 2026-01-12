use crate::error::AppError;
use async_openai::{config::OpenAIConfig, Client as OpenAIClient};
use axum::http::HeaderMap;

/// Извлекает токен из Authorization заголовка и создает OpenAI клиента
pub fn create_client_from_headers(headers: &HeaderMap) -> Result<OpenAIClient<OpenAIConfig>, AppError> {
    // Получаем Authorization заголовок
    let auth_header = headers
        .get("authorization")
        .or_else(|| headers.get("Authorization"))
        .ok_or_else(|| AppError("Authorization заголовок не найден".to_string()))?;

    // Извлекаем токен
    let auth_str = auth_header
        .to_str()
        .map_err(|_| AppError("Неверный формат Authorization заголовка".to_string()))?;

    // Убираем "Bearer " если есть
    let api_key = auth_str
        .strip_prefix("Bearer ")
        .or_else(|| auth_str.strip_prefix("bearer "))
        .unwrap_or(auth_str)
        .to_string();

    if api_key.is_empty() {
        return Err(AppError("API ключ пустой".to_string()));
    }

    // Создаем OpenAI клиента с полученным токеном
    let config = OpenAIConfig::new().with_api_key(api_key);
    Ok(OpenAIClient::with_config(config))
}
