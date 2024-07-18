# [Choice] Debian OS version (use bullseye on local arm64/Apple Silicon): buster, bullseye
ARG VARIANT="bullseye"
FROM mcr.microsoft.com/vscode/devcontainers/rust:1-${VARIANT}

# Include lld linker to improve build times either by using environment variable
# RUSTFLAGS="-C link-arg=-fuse-ld=lld" or with Cargo's configuration file (i.e see .cargo/config.toml).
RUN apt-get update && export DEBIAN_FRONTEND=noninteractive \
   && apt-get -y install clang lld \
   && apt-get autoremove -y && apt-get clean -y
RUN apt install build-essential
RUN rustup toolchain install nightly --allow-downgrade -c rustfmt -c clippy \
   && rustup toolchain install stable --allow-downgrade -c rust-analyzer \
   && rustup update
RUN cargo install --locked cargo-concordium
ADD https://distribution.concordium.software/tools/linux/concordium-client_6.3.0-1 /usr/local/bin/concordium-client
RUN chmod +x /usr/local/bin/concordium-client
RUN cargo install sqlx-cli --no-default-features --features native-tls,postgres

COPY --chmod=0755 post-create.sh /post-create.sh