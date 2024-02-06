# Real World Asset (RWA) for Concordium

## Repository Setup

```bash
git clone git@github.com:chainorders/concordium-rwa.git
git submodule update --init --recursive
yarn
```

## Contracts

Please find descriptions for all the available contracts [here](./contracts/README.md).

* build

  ```bash
  yarn run build:contracts
  ```

* deploy
  Edit the [env file](./.env.yarn) with the correct values. These values are used directly with `concordium-client` params to deploy

    ```bash
    yarn run deploy:contracts
    ```

* test

    ```bash
    yarn run test:contracts
    ```

* format :

    ```bash
    rustup install nightly-2022-06-09
    yarn run format:contracts
    ```

## Backend

Backend consists of 2 separate API's one for Contracts and one for Verifier API's.

* setup : Backend needs Module References of all the deployed contracts and can be updated in the [env file](./backend/.env)
* build

    ```bash
    yarn build:backend
    ```

* Run Contracts API (Debug Mode) : Running API requires a running Concordium Node and a running Mongo DB Instance. The connection details for both are available in the [env file](./backend/.env). An instance of MongoDb can be started using the [Docker Compose File](./docker-compose.yml)

    ```bash
    docker compose up -d
    yarn run:backend:contracts-api
    ```

* Run Verifier API (Debug Mode) : Running API requires a running Concordium Node and a running Mongo DB Instance. The connection details for both are available in the [env file](./backend/.env). An instance of MongoDb can be started using the [Docker Compose File](./docker-compose.yml)

    ```bash
    docker compose up -d
    yarn run:backend:verifier-api
    ```

* Generate API Clients : The Clients will be generated at the locations [Contracts API](./frontend/src/lib/contracts-api-client/ContractsApi.ts) and [Verifier API](./frontend/src/lib/verifier-api-client/VerifierApi.ts)

    ```bash
    yarn generate:backend:client
    ```

## Frontend

* setup
  * Make sure the frontend clients are generated using the above step
  * Frontend needs running instances of both the API's and the connection details for both are available in the [env file](./frontend/.env)
  * The Frontend also needs a running Concordium Node and the connection details are available in the [env file](./frontend/.env)
  * The Frontend also needs Module References of the deployed contracts and can be updated in the Contract files available at
    * [Identity Registry Contract](./frontend/src/lib/rwaIdentityRegistry.ts)
    * [Compliance Contract](./frontend/src/lib/rwaCompliance.ts)
    * [Compliance Module Allowed Nationalities](./frontend/src/lib/rwaComplianceModuleAllowedNationalities.ts)
    * [Security Nft Contract](./frontend/src/lib/rwaSecurityNft.ts)
    * [Market Contract](./frontend/src/lib/rwaMarket.ts)
    * [Sponsor Contract](./frontend/src/lib/rwaSponsor.ts)
* run

    ```bash
    yarn run:frontend
    ```
