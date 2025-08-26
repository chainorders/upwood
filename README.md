# Real World Asset (RWA) for Concordium

## Repository Setup

```bash
git clone git@github.com:chainorders/concordium-rwa.git
git submodule update --init --recursive
```

## Development Environment

- Download & Install the [VS code Development env extension](https://marketplace.visualstudio.com/items?itemName=ms-vscode-remote.remote-containers). Please follow through the prerequisites like docker needed by the extension.

### Environments

- [Contracts](./contracts/README.md) : For debugging contracts to the testnet
  - Deploying contract requires access to a testnet wallet. Export your wallet and copy it to the location [`.devcontainer/contracts/default_account.export`](.devcontainer/contracts/default_account.export).
    Or change the path in [contracts container file](.devcontainer/contracts/devcontainer.json)
- [Backend](./backend/README.md) : For debugging backend services
- [Frontend Dapp](./frontend-dapp/README.md) : For debugging frontend dapp
- [CDK deployment](./cdk-deployment/README.md) : For deploying the infrastructure

  - To be able to deploy the application to aws infra access to aws api's via aws cli would be needed. To enable this proceed through the following steps

    - [Create a cli user in the aws account](https://docs.aws.amazon.com/IAM/latest/UserGuide/id_users_create.html#id_users_create_console)
    - [Download the users csv file and add `default` username colum](https://docs.aws.amazon.com/cli/latest/userguide/cli-authentication-user.html#cli-authentication-user-configure-csv). After updating the file it should look like the following. In case you want to use a different username update the `default` in the file to the desired username and update the `AWS_PROFILE` env variable in the [devcontainer.json](./.devcontainer/cdk-deployment/devcontainer.json) file

    ```csv
    User Name,Access key ID,Secret access key
    default,********,************

    ```

    - Copy the csv credentials file to the location [./.devcontainer/cdk-deployment/aws_accessKeys.csv](./.devcontainer/cdk-deployment/aws_accessKeys.csv)

After installing the dev container extension. Execute the command `Dev Containers: Reopen in a Container`.
Which will present a list of mentioned development environments.

**Upon start of the container available yarn scripts will be shown on the terminal.**

### Developing on Apple Silicon

- Update the Container Image Variant `VARIANT` in the following files to `bullseye`
  - [.devcontainer/contracts/devcontainer.json](.devcontainer/contracts/devcontainer.json)
  - [.devcontainer/backend/docker-compose.yml](.devcontainer/backend/docker-compose.yml)
  - [.devcontainer/frontend-dapp/devcontainer.json](.devcontainer/frontend-dapp/devcontainer.json)
