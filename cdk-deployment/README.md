# Deployment

This project contains AWS CDK scripts to deploy all the AWS components.
The deployment of Blockchain components is a prerequisite for the deployment of the backend.

## Useful commands

- `npm run build` compile typescript to js
- `npm run watch` watch for changes and compile
- `npm run test` perform the jest unit tests
- `npx cdk deploy` deploy this stack to your default AWS account/region
- `npx cdk diff` compare deployed stack with current state
- `npx cdk synth` emits the synthesized CloudFormation template

## Deployment Topology

### Components

- [ ] Concordium Blockchain Node: [Amazon ECS](./lib/concordium-node-ecs-stack.ts)
- [x] User Authentication & Management: [Amazon Cognito](./lib/cognito-stack.ts)
- [x] Database Server: [Amazon RDS](./lib/rds-stack.ts)
- [x] Concordium Listener Server: [Amazon ECS](./lib/backend-listener-stack.ts)
- [ ] Upwood APIs/Web Server: [Amazon ECS](./lib/backend-api-stack.ts)
- [x] API Gateway using Amazon API Gateway: [Amazon API Gateway](./lib/backend-api-stack.ts)
- [ ] Frontend Deployment & Distribution: [Amazon S3 & Cloudfront](./lib/frontend-stack.ts)
- [ ] Frontend Deployment & Distribution Dapp: [Amazon S3 & Cloudfront](./lib/frontend-stack.ts)

### Prerequisites

- AWS Account
- Db Username SSM Parameter
- Db Password SSM Parameter
- Tree NFT Agent Wallet SSM Parameter
