# Concordium RWA Frontend

## Frontend

- setup
  - Make sure the frontend clients are generated using the `generate:client` command available in the [backend project](../backend/README.md).
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
