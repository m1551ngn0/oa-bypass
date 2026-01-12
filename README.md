# OpenAI API Bypass Server

Прокси-сервер на Rust для OpenAI API, работающий в режиме passthrough. Принимает токен от клиента в Authorization заголовке и перенаправляет запросы к OpenAI API.

## 🚀 Особенности

- ⚡ **Быстрый и надежный** - написан на Rust с использованием Axum и Tokio
- 🔐 **Безопасный** - токен не хранится на сервере, передается от клиента
- 🐳 **Docker ready** - готовые Dockerfile и docker-compose.yml
- 🌐 **CORS enabled** - поддержка кросс-доменных запросов
- 📡 **Полная совместимость с OpenAI API** - completions, assistants, threads, files, responses

## 📦 Установка и запуск

### Локальный запуск

```bash
# Сборка
cargo build --release

# Запуск
cargo run --release
```

Сервер запустится на `http://0.0.0.0:8080`

### Docker

```bash
# Сборка образа
docker build -t oa-bypass .

# Запуск
docker run -p 8080:8080 oa-bypass
```

### Docker Compose

```bash
docker-compose up -d
```

## 🔧 Использование

### С Python OpenAI SDK

```python
import openai

# Укажите адрес прокси-сервера
openai.api_base = "http://localhost:8080/v1"
# Ваш настоящий OpenAI API ключ
openai.api_key = "sk-your-real-openai-key"

# Используйте как обычно
response = openai.ChatCompletion.create(
    model="gpt-4",
    messages=[{"role": "user", "content": "Hello!"}]
)
```

### С переменными окружения

```bash
export OPENAI_API_BASE=http://localhost:8080/v1
export OPENAI_API_KEY=sk-your-real-openai-key
```

### Прямые HTTP запросы

```bash
curl -X POST http://localhost:8080/v1/chat/completions \
  -H "Content-Type: application/json" \
  -H "Authorization: Bearer sk-your-openai-key" \
  -d '{
    "model": "gpt-4",
    "messages": [{"role": "user", "content": "Hello!"}]
  }'
```

## 📚 Поддерживаемые API эндпоинты

### Completions
- `POST /v1/chat/completions` - Chat completions (GPT-4, GPT-3.5, etc.)
- `POST /v1/completions` - Text completions (legacy)

### Embeddings
- `POST /v1/embeddings` - Создание embeddings

### Models
- `GET /v1/models` - Список всех доступных моделей
- `GET /v1/models/:model_id` - Информация о конкретной модели

### Images (DALL-E)
- `POST /v1/images/generations` - Генерация изображений

### Assistants API
- `POST /v1/assistants` - Создать assistant
- `GET /v1/assistants` - Список assistants
- `GET /v1/assistants/:assistant_id` - Получить assistant
- `POST /v1/assistants/:assistant_id` - Изменить assistant
- `DELETE /v1/assistants/:assistant_id` - Удалить assistant

### Threads API
- `POST /v1/threads` - Создать thread
- `GET /v1/threads/:thread_id` - Получить thread
- `POST /v1/threads/:thread_id` - Изменить thread
- `DELETE /v1/threads/:thread_id` - Удалить thread

### Messages API
- `POST /v1/threads/:thread_id/messages` - Создать сообщение
- `GET /v1/threads/:thread_id/messages` - Список сообщений
- `GET /v1/threads/:thread_id/messages/:message_id` - Получить сообщение
- `POST /v1/threads/:thread_id/messages/:message_id` - Изменить сообщение

### Runs API
- `POST /v1/threads/:thread_id/runs` - Создать run
- `GET /v1/threads/:thread_id/runs` - Список runs
- `GET /v1/threads/:thread_id/runs/:run_id` - Получить run
- `POST /v1/threads/:thread_id/runs/:run_id` - Изменить run
- `POST /v1/threads/:thread_id/runs/:run_id/cancel` - Отменить run
- `POST /v1/threads/:thread_id/runs/:run_id/submit_tool_outputs` - Отправить tool outputs
- `POST /v1/threads/runs` - Создать thread и run одновременно

### Files API
- `POST /v1/files` - Загрузить файл (multipart/form-data)
- `GET /v1/files` - Список файлов
- `GET /v1/files/:file_id` - Информация о файле
- `DELETE /v1/files/:file_id` - Удалить файл
- `GET /v1/files/:file_id/content` - Скачать содержимое файла

### Responses API
- `POST /v1/responses` - Создать response
- `GET /v1/responses/:response_id` - Получить response
- `DELETE /v1/responses/:response_id` - Удалить response
- `POST /v1/responses/:response_id/cancel` - Отменить response

## 🏗️ Архитектура

```
Клиент (Python/JS/etc)
    ↓ (с OpenAI API key в Authorization header)
Rust Proxy Server (oa-bypass)
    ↓ (прокси запрос с тем же токеном)
OpenAI API
```

**Как это работает:**

1. Клиент отправляет запрос на прокси-сервер с токеном в заголовке `Authorization: Bearer sk-...`
2. Сервер извлекает токен из заголовка
3. Сервер создает OpenAI клиента с этим токеном
4. Сервер делает запрос к официальному OpenAI API
5. Ответ возвращается обратно клиенту

**Преимущества:**

- Токен не хранится на сервере
- Каждый клиент использует свой собственный токен
- Возможность централизованного логирования и мониторинга
- Возможность добавления дополнительной логики обработки запросов


## 🛠️ Технологический стек

- **Rust** 1.83+
- **Axum** 0.8 - современный веб-фреймворк
- **Tokio** 1.49 - асинхронный runtime
- **async-openai** 0.32 - OpenAI API клиент (full feature set)
- **Tower HTTP** - CORS middleware

## 📊 Структура проекта

```
oa-bypass/
├── src/
│   ├── main.rs           # Точка входа приложения
│   ├── state.rs          # Состояние приложения
│   ├── error.rs          # Обработка ошибок
│   ├── utils.rs          # Утилиты
│   └── routes/
│       ├── mod.rs        # Роутер приложения
│       ├── completions.rs # Chat и text completions
│       ├── embeddings.rs  # Embeddings API
│       ├── models.rs      # Models API
│       ├── images.rs      # Image generation
│       ├── assistants.rs  # Assistants API
│       ├── threads.rs     # Threads API
│       ├── messages.rs    # Messages API
│       ├── runs.rs        # Runs API
│       ├── responses.rs   # Responses API
│       └── files.rs       # Files API
├── Cargo.toml            # Зависимости Rust
├── Dockerfile            # Multi-stage Docker build
└── docker-compose.yml    # Docker Compose конфигурация
```

## 🧪 Проверка работы

### Health Check

```bash
curl http://localhost:8080/health
# Ответ: "OpenAI API Server is running"
```

### Проверка списка моделей

```bash
curl http://localhost:8080/v1/models \
  -H "Authorization: Bearer sk-your-openai-key"
```

### Тест Chat Completion

```bash
curl -X POST http://localhost:8080/v1/chat/completions \
  -H "Content-Type: application/json" \
  -H "Authorization: Bearer sk-your-openai-key" \
  -d '{
    "model": "gpt-4",
    "messages": [{"role": "user", "content": "Привет!"}]
  }'
```

## 🔍 Логирование

Сервер использует `tracing` для логирования. Уровень логирования можно настроить через переменную окружения:

```bash
# Локально
RUST_LOG=debug cargo run

# Docker
docker run -e RUST_LOG=debug -p 8080:8080 oa-bypass
```

Уровни: `error`, `warn`, `info`, `debug`, `trace`

## ⚙️ Конфигурация

| Параметр | По умолчанию | Описание |
|----------|--------------|----------|
| Порт | 8080 | Порт, на котором работает сервер |
| RUST_LOG | info | Уровень логирования |

## 📝 Лицензия

MIT

## 🤝 Вклад в проект

Приветствуются pull requests. Для значительных изменений сначала откройте issue для обсуждения.

## ⚠️ Важные замечания

- Сервер не хранит и не логирует токены OpenAI
- Все запросы проксируются напрямую к OpenAI API
- Убедитесь, что ваш токен OpenAI имеет необходимые разрешения
- Для production использования рекомендуется настроить HTTPS
- Рассмотрите возможность добавления rate limiting для защиты от злоупотреблений
