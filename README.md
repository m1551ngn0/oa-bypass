# OpenAI API Bypass Server

Высокопроизводительный прокси-сервер на Rust для OpenAI API, работающий в режиме passthrough. Принимает токен от клиента в Authorization заголовке и перенаправляет запросы к OpenAI API без хранения конфиденциальных данных.

## 🚀 Особенности

- ⚡ **Быстрый и надежный** - написан на Rust с использованием Axum и Tokio
- 🔐 **Безопасный** - токен не хранится на сервере, передается от клиента
- 🐳 **Docker ready** - готовые Dockerfile (multi-stage Alpine) и docker-compose.yml
- 🌐 **CORS enabled** - поддержка кросс-доменных запросов из любого источника
- 📡 **Полная совместимость с OpenAI API** - поддержка всех основных эндпоинтов
- 🏥 **Health checks** - встроенная проверка работоспособности сервиса
- 📊 **Структурированное логирование** - использование tracing для мониторинга

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

# Запуск контейнера
docker run -p 8080:8080 oa-bypass

# С настройкой логирования
docker run -e RUST_LOG=debug -p 8080:8080 oa-bypass
```

### Docker Compose

```bash
# Запуск в фоновом режиме
docker-compose up -d

# Просмотр логов
docker-compose logs -f

# Остановка
docker-compose down
```

**Примечание:** Docker Compose конфигурация использует образ `ghcr.io/m1551ngn0/oa-bypass:latest` и проксирует порт 3000 на хост (вместо 8080).

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

Сервер поддерживает полный спектр эндпоинтов OpenAI API:

### Health Check
- `GET /` - Проверка работоспособности
- `GET /health` - Проверка работоспособности (альтернативный путь)

### Completions
- `POST /v1/chat/completions` - Chat completions (GPT-4, GPT-4 Turbo, GPT-3.5, etc.)
- `POST /v1/completions` - Text completions (legacy модели)

### Embeddings
- `POST /v1/embeddings` - Создание текстовых embeddings

### Models
- `GET /v1/models` - Список всех доступных моделей
- `GET /v1/models/{model_id}` - Информация о конкретной модели

### Images (DALL-E)
- `POST /v1/images/generations` - Генерация изображений с помощью DALL-E

### Assistants API
- `POST /v1/assistants` - Создать assistant
- `GET /v1/assistants` - Список assistants
- `GET /v1/assistants/{assistant_id}` - Получить assistant
- `POST /v1/assistants/{assistant_id}` - Изменить assistant
- `DELETE /v1/assistants/{assistant_id}` - Удалить assistant

### Threads API
- `POST /v1/threads` - Создать thread
- `GET /v1/threads/{thread_id}` - Получить thread
- `POST /v1/threads/{thread_id}` - Изменить thread
- `DELETE /v1/threads/{thread_id}` - Удалить thread

### Messages API
- `POST /v1/threads/{thread_id}/messages` - Создать сообщение в thread
- `GET /v1/threads/{thread_id}/messages` - Список сообщений в thread
- `GET /v1/threads/{thread_id}/messages/{message_id}` - Получить конкретное сообщение
- `POST /v1/threads/{thread_id}/messages/{message_id}` - Изменить сообщение

### Runs API
- `POST /v1/threads/{thread_id}/runs` - Создать run
- `GET /v1/threads/{thread_id}/runs` - Список runs для thread
- `GET /v1/threads/{thread_id}/runs/{run_id}` - Получить run
- `POST /v1/threads/{thread_id}/runs/{run_id}` - Изменить run
- `POST /v1/threads/{thread_id}/runs/{run_id}/cancel` - Отменить run
- `POST /v1/threads/{thread_id}/runs/{run_id}/submit_tool_outputs` - Отправить результаты выполнения инструментов
- `POST /v1/threads/runs` - Создать thread и run одновременно

### Files API
- `POST /v1/files` - Загрузить файл (поддержка multipart/form-data)
- `GET /v1/files` - Список загруженных файлов
- `GET /v1/files/{file_id}` - Информация о файле
- `DELETE /v1/files/{file_id}` - Удалить файл
- `GET /v1/files/{file_id}/content` - Скачать содержимое файла

### Responses API
- `POST /v1/responses` - Создать response
- `GET /v1/responses/{response_id}` - Получить response
- `DELETE /v1/responses/{response_id}` - Удалить response
- `POST /v1/responses/{response_id}/cancel` - Отменить response

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

- **Rust** 1.83+ (2021 edition)
- **Axum** 0.8 - современный веб-фреймворк с поддержкой multipart
- **Tokio** 1.49 - асинхронный runtime (full features)
- **async-openai** 0.32 - официальный OpenAI API клиент для Rust (full feature set)
- **Serde** 1.0 - сериализация/десериализация JSON
- **Tower HTTP** 0.6 - CORS middleware
- **Tracing** 0.1 - структурированное логирование

## 📊 Структура проекта

```
oa-bypass/
├── src/
│   ├── main.rs           # Точка входа, инициализация сервера
│   ├── state.rs          # Состояние приложения (AppState)
│   ├── error.rs          # Обработка ошибок и типы ошибок
│   ├── utils.rs          # Вспомогательные функции
│   └── routes/
│       ├── mod.rs        # Главный роутер и регистрация маршрутов
│       ├── completions.rs # Chat и text completions
│       ├── embeddings.rs  # Embeddings API
│       ├── models.rs      # Models API
│       ├── images.rs      # Image generation (DALL-E)
│       ├── assistants.rs  # Assistants API
│       ├── threads.rs     # Threads API
│       ├── messages.rs    # Messages API (в рамках threads)
│       ├── runs.rs        # Runs API (выполнение assistants)
│       ├── responses.rs   # Responses API
│       └── files.rs       # Files API (загрузка/скачивание)
├── Cargo.toml            # Зависимости и метаданные проекта
├── Dockerfile            # Multi-stage Docker build (Rust 1.83 Alpine)
├── docker-compose.yml    # Docker Compose конфигурация
└── README.md             # Документация проекта
```

### Детали Docker образа

- **Builder stage**: Rust 1.83 на Alpine Linux с musl и OpenSSL
- **Runtime stage**: Alpine 3.19 (~20MB финальный размер)
- **Оптимизация**: Кеширование зависимостей для быстрой пересборки
- **Security**: Минимальный набор runtime зависимостей (ca-certificates, libgcc)

## 🧪 Проверка работы

### Health Check

```bash
# Базовая проверка
curl http://localhost:8080/health
# Ответ: "OpenAI API Server is running"

# Альтернативный эндпоинт
curl http://localhost:8080/
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

### Тест генерации изображений

```bash
curl -X POST http://localhost:8080/v1/images/generations \
  -H "Content-Type: application/json" \
  -H "Authorization: Bearer sk-your-openai-key" \
  -d '{
    "prompt": "A cute cat",
    "n": 1,
    "size": "1024x1024"
  }'
```

## 🔍 Логирование

Сервер использует `tracing` для структурированного логирования. При запуске выводится информация о доступных эндпоинтах:

```
🚀 OpenAI API сервер запущен на http://0.0.0.0:8080
📡 Сервер работает в режиме passthrough
📡 Токен OpenAI должен передаваться в Authorization заголовке от клиента
📡 Доступные эндпоинты:
   Completions: POST /v1/chat/completions, /v1/completions
   Models: GET /v1/models
   Assistants: POST/GET/DELETE /v1/assistants
   Threads: POST/GET/DELETE /v1/threads
   Messages: POST/GET /v1/threads/:id/messages
   Runs: POST/GET /v1/threads/:id/runs
   Files: POST/GET/DELETE /v1/files
   Responses: POST/GET/DELETE /v1/responses
```

Уровень логирования можно настроить через переменную окружения:

```bash
# Локально
RUST_LOG=debug cargo run

# Docker
docker run -e RUST_LOG=debug -p 8080:8080 oa-bypass

# docker-compose
docker-compose logs -f
```

Уровни логирования: `error`, `warn`, `info` (по умолчанию), `debug`, `trace`

## ⚙️ Конфигурация

### Переменные окружения

| Параметр | По умолчанию | Описание |
|----------|--------------|----------|
| Порт | 8080 | Порт, на котором работает сервер (жестко задан в коде) |
| RUST_LOG | info | Уровень логирования: error, warn, info, debug, trace |

### Настройка через Docker

```bash
# Запуск с debug логированием
docker run -e RUST_LOG=debug -p 8080:8080 oa-bypass
```

### Настройка через docker-compose.yml

```yaml
environment:
  - RUST_LOG=debug  # Изменить уровень логирования
ports:
  - "3000:8080"     # Изменить внешний порт (в примере используется 3000)
```


## ⚠️ Важные замечания

- ✅ Сервер **не хранит и не логирует** токены OpenAI
- ✅ Все запросы проксируются **напрямую** к официальному OpenAI API
- ✅ Каждый клиент использует **свой собственный токен**
- ⚠️ Убедитесь, что ваш токен OpenAI имеет необходимые разрешения
- 🔒 Для production использования рекомендуется настроить **HTTPS** (например, через reverse proxy)
- 🛡️ Рассмотрите возможность добавления **rate limiting** для защиты от злоупотреблений
- 🐳 Docker образ собран на **Alpine Linux** для минимального размера (~20MB)
- 💚 Docker Compose включает **health checks** для автоматической проверки работоспособности

## 🚀 Быстрый старт (Docker)

Самый простой способ запустить сервер:

```bash
# Используя готовый образ из GitHub Container Registry
docker run -d \
  --name oa-bypass \
  -p 3000:8080 \
  -e RUST_LOG=info \
  ghcr.io/m1551ngn0/oa-bypass:latest
```

Или через docker-compose:

```bash
docker-compose up -d
```

Сервер будет доступен на `http://localhost:3000`
