use crate::error::AppError;
use async_openai::{config::OpenAIConfig, Client as OpenAIClient};
use axum::http::HeaderMap;

/// Извлекает токен из Authorization заголовка и создает OpenAI клиента
/// 
/// # Arguments
/// * `headers` - HTTP заголовки запроса
/// * `use_beta` - если true, добавляет заголовок OpenAI-Beta для Assistants API
pub fn create_client_from_headers(headers: &HeaderMap, use_beta: bool) -> Result<OpenAIClient<OpenAIConfig>, AppError> {
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
    let mut config = OpenAIConfig::new().with_api_key(api_key);
    
    // Если нужен Beta API, добавляем соответствующий заголовок
    if use_beta {
        config = config
            .with_header("OpenAI-Beta", "assistants=v2")
            .map_err(|e| AppError(format!("Ошибка создания конфигурации: {}", e)))?;
    }
    
    Ok(OpenAIClient::with_config(config))
}
