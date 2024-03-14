# Concordium RWA Backend

## Generate docs

    ```bash
    cargo doc --no-deps --open
    ```

## Contracts API

- `format`: Runs the Rust formatter on the codebase using a specific nightly version of Rust.
- `build`: Builds the Rust project.
- `debug:contracts-api`: Runs the contracts API with debug logging enabled.
- `watch:contracts-api`: Watches for changes in the contracts API code and reruns it with debug logging enabled whenever a change is detected.
- `generate:contracts-api-specs`: Generates an API specification for the contracts API and saves it to `contracts-api-specs.json`.
- `generate:contracts-api-client`: Generates client code for the contracts API using the API specification in `contracts-api-specs.json`.

## Verifier API

- `debug:verifier-api`: Runs the verifier API with debug logging enabled.
- `watch:verifier-api`: Watches for changes in the verifier API code and reruns it with debug logging enabled whenever a change is detected.
- `generate:verifier-api-specs`: Generates an API specification for the verifier API and saves it to `verifier-api-specs.json`.
- `generate:verifier-api-client`: Generates client code for the verifier API using the API specification in `verifier-api-specs.json`.

## Sponsor API

- `debug:sponsor-api`: Runs the sponsor API with debug logging enabled.
- `watch:sponsor-api`: Watches for changes in the sponsor API code and reruns it with debug logging enabled whenever a change is detected.
- `generate:sponsor-api-specs`: Generates an API specification for the sponsor API and saves it to `sponsor-api-specs.json`.
- `generate:sponsor-api-client`: Generates client code for the sponsor API using the API specification in `sponsor-api-specs.json`.

## General

- `generate:spec`: Runs all the `generate:*-api-specs` scripts.
- `generate:client`: Runs all the `generate:*-api-client` scripts.
