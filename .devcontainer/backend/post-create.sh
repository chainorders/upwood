echo "Executing Post Create Commands"

echo "Setting Yarn Version" \
&& corepack enable \
&& corepack install --global yarn@4.3.1 \
&& yarn set version stable \
# && echo "\n Setting up Database" \
# && cargo sqlx database setup \
&& rustc --version
