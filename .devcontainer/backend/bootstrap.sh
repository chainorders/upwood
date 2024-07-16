rustup update \
&& rustup toolchain install nightly \
&& rustup component add rustfmt \
&& rustup component add --toolchain nightly rustfmt \
&& cargo install --locked cargo-concordium

wget https://distribution.concordium.software/tools/linux/concordium-client_6.3.0-1 -O /usr/local/bin/concordium-client \
&& chmod +x /usr/local/bin/concordium-client