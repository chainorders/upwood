FROM rust:latest AS builder
WORKDIR /app
COPY . .
WORKDIR /app/backend
RUN cargo build --release

FROM debian:12-slim
RUN apt-get update && apt-get install -y --no-install-recommends libpq-dev \
    && apt-get clean \
    && rm -rf /var/lib/apt/lists/*
COPY --from=builder /app/backend/target/release/listener_server /listener_server
COPY --from=builder /app/backend/target/release/upwood_api_server /upwood_api_server