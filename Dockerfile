# Этап сборки
FROM rust:1.83-alpine AS builder

# Установка зависимостей для сборки
RUN apk add --no-cache musl-dev openssl-dev openssl-libs-static pkgconfig

WORKDIR /app

# Копируем манифесты
COPY Cargo.toml ./

# Создаем фиктивный main.rs для кеширования зависимостей
RUN mkdir src && \
    echo "fn main() {}" > src/main.rs && \
    cargo build --release && \
    rm -rf src

# Копируем исходный код
COPY src ./src

# Пересобираем с реальным кодом
RUN touch src/main.rs && \
    cargo build --release

# Финальный образ
FROM alpine:3.19

# Установка runtime зависимостей
RUN apk add --no-cache ca-certificates libgcc

WORKDIR /app

# Копируем бинарник из builder
COPY --from=builder /app/target/release/oa-bypass /app/oa-bypass

# Открываем порт
EXPOSE 8080

# Запуск приложения
CMD ["/app/oa-bypass"]
