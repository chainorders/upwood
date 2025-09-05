FROM rust:1.85 AS builder
WORKDIR /app
COPY . .
# Install build dependencies required by Diesel/Postgres and general builds
RUN apt-get update \
    && apt-get install -y --no-install-recommends libpq-dev pkg-config build-essential ca-certificates \
    && rm -rf /var/lib/apt/lists/*
WORKDIR /app/backend
RUN cargo build --release

FROM debian:12-slim
RUN apt-get update && apt-get install -y --no-install-recommends libpq-dev ca-certificates \
    && apt-get clean \
    && rm -rf /var/lib/apt/lists/*
COPY --from=builder /app/backend/target/release/listener_server /listener_server
COPY --from=builder /app/backend/target/release/upwood_api_server /upwood_api_server
