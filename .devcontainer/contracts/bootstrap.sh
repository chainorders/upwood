rustup update \
&& rustup component add rustfmt \
&& rustup target add wasm32-unknown-unknown \
&& rustup toolchain install nightly-2023-04-01 \
&& rustup component add --toolchain nightly-2023-04-01-x86_64-unknown-linux-gnu rustfmt \
&& cargo install --locked cargo-concordium

wget https://distribution.concordium.software/tools/linux/concordium-client_6.3.0-1 -O /usr/local/bin/concordium-client \
&& chmod +x /usr/local/bin/concordium-client