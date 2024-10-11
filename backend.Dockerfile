FROM rust:latest AS builder
WORKDIR /app
COPY . .
WORKDIR /app/backend
RUN cargo build --release

FROM rust:slim
RUN apt update && apt install -y libpq-dev
COPY --from=builder /app/backend/target/release/listener_server /listener_server
COPY --from=builder /app/backend/target/release/upwood_api_server /upwood_api_server