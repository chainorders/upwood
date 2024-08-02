# Real World Asset (RWA) for Concordium

## Repository Setup

```bash
git clone git@github.com:chainorders/concordium-rwa.git
git submodule update --init --recursive
yarn
```

## Development Environment

- Download & Install the [VS code Development env extension](https://marketplace.visualstudio.com/items?itemName=ms-vscode-remote.remote-containers). Please follow through the prerequisites like docker needed by the extension.
- Since deploying contract requires access to a testnet wallet. Export your wallet and copy it to the location `~/etc/concordium/default_account.export`. Or change the path in [contracts container file](.devcontainer/contracts/devcontainer.json)

### Environments

- Contracts : For debugging and deploying contracts to the testnet
- Backend : For debugging and deploying backend services
- Frontend : FOr debugging and deploying frontend

## [Contracts](./contracts/README.md)

- build : `yarn run build:contracts`
- deploy : `yarn run deploy:contracts`
- test : `yarn run test:contracts`
- format : `yarn run format:contracts`
- Check other available scripts `yarn run`

## [Backend](./backend/README.md)

Backend consists of 2 separate API's one for Contracts and one for Verifier API's.

- setup : Backend needs Module References of all the deployed contracts and can be updated in the [env file](./backend/.env)
- build

  ```bash
  yarn build:backend
  ```

- Run Contracts API (Debug Mode) : Running API requires a running Concordium Node and a running Mongo DB Instance. The connection details for both are available in the [env file](./backend/.env). An instance of MongoDb can be started using the [Docker Compose File](./docker-compose.yml)

  ```bash
  docker compose up -d
  yarn run:backend:contracts-api
  ```

- Run Verifier API (Debug Mode) : Running API requires a running Concordium Node and a running Mongo DB Instance. The connection details for both are available in the [env file](./backend/.env). An instance of MongoDb can be started using the [Docker Compose File](./docker-compose.yml)

  ```bash
  docker compose up -d
  yarn run:backend:verifier-api
  ```

- Generate API Clients : The Clients will be generated at the locations [Contracts API](./frontend/src/lib/contracts-api-client/ContractsApi.ts) and [Verifier API](./frontend/src/lib/verifier-api-client/VerifierApi.ts)

  ```bash
  yarn generate:backend:client
  ```

## Frontend

- setup
  - Make sure the frontend clients are generated using the above step
  - Frontend needs running instances of both the API's and the connection details for both are available in the [env file](./frontend/.env)
  - The Frontend also needs a running Concordium Node and the connection details are available in the [env file](./frontend/.env)
  - The Frontend also needs Module References of the deployed contracts and can be updated in the Contract files available at
    - [Identity Registry Contract](./frontend/src/lib/rwaIdentityRegistry.ts)
    - [Compliance Contract](./frontend/src/lib/rwaCompliance.ts)
    - [Compliance Module Allowed Nationalities](./frontend/src/lib/rwaComplianceModuleAllowedNationalities.ts)
    - [Security Nft Contract](./frontend/src/lib/rwaSecurityNft.ts)
    - [Market Contract](./frontend/src/lib/rwaMarket.ts)
    - [Sponsor Contract](./frontend/src/lib/rwaSponsor.ts)
- run

  ```bash
  yarn run:frontend
  ```
