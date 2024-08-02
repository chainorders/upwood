echo "Executing Post Create Commands"
echo "Setting Yarn Version" \
&& corepack enable \
&& corepack install --global yarn@4.3.1 \
&& yarn set version stable \
&& concordium-client config account import /etc/concordium/default_account.export --name default -s \
&& rustc --version
