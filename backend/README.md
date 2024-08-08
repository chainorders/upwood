# Concordium RWA Backend

Backend Consists of multiple components bundled together an a executor inside the [`main.rs`](./src/main.rs) file.
The Components are

- [Listener](./src/txn_listener/mod.rs): With following contract event processors.
  - [Identity Registry Processor](./src/txn_processor/rwa_identity_registry/processor.rs)
  - [Market Processor](./src/txn_processor/rwa_market/processor.rs)
  - [Security CIS2 Processor](./src/txn_processor/rwa_security_cis2/processor.rs)
- Contract API's
  - [Market](./src/txn_processor/rwa_security_cis2/api.rs)
  - [Security CIS2](./src/txn_processor/rwa_security_cis2/api.rs)
- [Sponsor Backend](./src/sponsor/mod.rs) : Uses an input wallet and exposes an API which can be accessed with a [Sponsor Contract](../contracts/sponsor/src/lib.rs)
  - Wallet file can be provided by using the default location file [`./sponsor_wallet.export`](./sponsor_wallet.export)
- [Verifier Backend](./src/verifier/mod.rs): Exposes an API with a Identity Registry Agent Account Wallet to add an input Identity (Account Address / Contract Address) to the [Identity Registry Contract](../contracts/identity-registry/src/lib.rs)
  - Wallet file can be provided by copy pasting to the default location [`./agent_wallet.export`](./agent_wallet.export)

## Notable Available Yarn Scripts

All the yarn scripts can be listed using the command `yarn run`

- `yarn build`
- `yarn generate:client`: Used to generate frontend API clients.
- `yarn debug:listener`: Runs the Concordium Listener and starts to process the events for the available contracts.
- `yarn debug:contracts-api`: Runs the API's for the available contracts
- `yarn debug:verifier-api`: Runs the verifier API
- `yarn debug:debug:sponsor-api`: Runs the Sponsor API

All Available commands exposed by the executable can be listed using `cargo run --help`. This will also show the default values being used.

## API's

### Listener

- `debug:listener` : Runs the listener with debug logging enabled.
- `watch:listener` : Watches for changes in the listener code and reruns it with debug logging enabled whenever a change is detected.

### Contracts API

- `debug:contracts-api`: Runs the contracts API with debug logging enabled.
- `watch:contracts-api`: Watches for changes in the contracts API code and reruns it with debug logging enabled whenever a change is detected.
- `generate:contracts-api-specs`: Generates an API specification for the contracts API and saves it to `contracts-api-specs.json`.
- `generate:contracts-api-client`: Generates client code for the contracts API using the API specification in `contracts-api-specs.json`.

### Verifier API

- `debug:verifier-api`: Runs the verifier API with debug logging enabled.
- `watch:verifier-api`: Watches for changes in the verifier API code and reruns it with debug logging enabled whenever a change is detected.
- `generate:verifier-api-specs`: Generates an API specification for the verifier API and saves it to `verifier-api-specs.json`.
- `generate:verifier-api-client`: Generates client code for the verifier API using the API specification in `verifier-api-specs.json`.

### Sponsor API

- `debug:sponsor-api`: Runs the sponsor API with debug logging enabled.
- `watch:sponsor-api`: Watches for changes in the sponsor API code and reruns it with debug logging enabled whenever a change is detected.
- `generate:sponsor-api-specs`: Generates an API specification for the sponsor API and saves it to `sponsor-api-specs.json`.
- `generate:sponsor-api-client`: Generates client code for the sponsor API using the API specification in `sponsor-api-specs.json`.

### General

- `format`: Runs the Rust formatter on the codebase using a specific nightly version of Rust.
- `build`: Builds the Rust project.
- `generate:spec`: Runs all the `generate:*-api-specs` scripts.
- `generate:client`: Runs all the `generate:*-api-client` scripts.

## Environment Variables

### General Variables

| Variable             | Description                                        |
| -------------------- | -------------------------------------------------- |
| CONCORDIUM_NODE_URI  | The URI of the Concordium node.                    |
| MONGODB_URI          | The URI of the MongoDB database.                   |
| WEB_SERVER_ADDR      | The address and port the web server is running on. |
| DEFAULT_BLOCK_HEIGHT | The starting block height for the blockchain.      |
| NETWORK              | The network the application is running on.         |

### Module Refs

| Variable                         | Description                                         |
| -------------------------------- | --------------------------------------------------- |
| RWA_COMPLIANCE_MODULE_REF        | The reference for the RWA Compliance module.        |
| RWA_IDENTITY_REGISTRY_MODULE_REF | The reference for the RWA Identity Registry module. |
| RWA_MARKET_MODULE_REF            | The reference for the RWA Market module.            |
| RWA_SECURITY_NFT_MODULE_REF      | The reference for the RWA Security NFT module.      |
| RWA_SECURITY_SFT_MODULE_REF      | The reference for the RWA Security SFT module.      |
| RWA_SPONSOR_MODULE_REF           | The reference for the RWA Sponsor module.           |

### Contract Names

| Variable                            | Description                                     |
| ----------------------------------- | ----------------------------------------------- |
| RWA_IDENTITY_REGISTRY_CONTRACT_NAME | The name of the RWA Identity Registry contract. |
| RWA_COMPLIANCE_CONTRACT_NAME        | The name of the RWA Compliance contract.        |
| RWA_MARKET_CONTRACT_NAME            | The name of the RWA Market contract.            |
| RWA_SECURITY_NFT_CONTRACT_NAME      | The name of the RWA Security NFT contract.      |
| RWA_SPONSOR_CONTRACT_NAME           | The name of the RWA Sponsor contract.           |

### Verifier

| Variable                 | Description                                                 |
| ------------------------ | ----------------------------------------------------------- |
| IDENTITY_REGISTRY        | The identity registry.                                      |
| AGENT_WALLET_PATH        | The path to the agent's wallet.                             |
| VERIFIER_WEB_SERVER_ADDR | The address and port the verifier web server is running on. |

### Sponsor

| Variable                | Description                                                |
| ----------------------- | ---------------------------------------------------------- |
| SPONSOR_WALLET_PATH     | The path to the sponsor's wallet.                          |
| SPONSOR_WEB_SERVER_ADDR | The address and port the sponsor web server is running on. |
| SPONSOR_CONTRACT        | The sponsor contract.                                      |
