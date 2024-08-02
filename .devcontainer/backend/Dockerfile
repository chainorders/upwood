# [Choice] Debian OS version (use bullseye on local arm64/Apple Silicon): buster, bullseye
ARG VARIANT="bookworm"
ARG CARGO_HOME="/usr/local/cargo"
ARG RUSTUP_HOME="/usr/local/rustup"

FROM mcr.microsoft.com/vscode/devcontainers/rust:1-${VARIANT}
ENV CARGO_HOME=${CARGO_HOME}
ENV RUSTUP_HOME=${RUSTUP_HOME}

# Include lld linker to improve build times either by using environment variable
# RUSTFLAGS="-C link-arg=-fuse-ld=lld" or with Cargo's configuration file (i.e see .cargo/config.toml).
RUN apt-get update \
   && export DEBIAN_FRONTEND=noninteractive \
   && apt-get -y install clang lld \
   && apt-get autoremove -y && apt-get clean -y
RUN apt install build-essential libpq-dev
RUN rustup toolchain install nightly --allow-downgrade -c rustfmt -c clippy \
   && rustup toolchain install stable --allow-downgrade -c rust-analyzer \
   && rustup update
RUN cargo install --locked cargo-concordium diesel_cli

ADD "https://distribution.concordium.software/tools/linux/concordium-client_6.3.0-1" /usr/local/bin/concordium-client
RUN chmod +x /usr/local/bin/concordium-client
COPY --chmod=0755 bootstrap.sh /bootstrap.sh