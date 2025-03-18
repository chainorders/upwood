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

### Stacks

- [ ] Concordium Blockchain Node: Amazon ECS
- [x] User Authentication & Management: [Amazon Cognito](./lib/cognito-stack.ts)
  - Creates an AWS Cognito User Pool for user management.
  - Configures user attributes, custom attributes, and email settings.
  - Sets up an admin group and a user pool client.
- [x] Infra Stack: [ECS Cluster & Private DNS](./lib/infra-stack.ts)
  - Sets up an ECS Cluster for running containerized applications.
  - Creates a Private DNS Namespace for service discovery within the VPC.
  - Configures a VPC Link for API Gateway integration with the VPC.
  - Creates a Log Group for application logs.
- [x] Database Server: [Amazon RDS](./lib/db-stack.ts)
  - Deploys an Amazon RDS instance for database storage.
  - Configures security groups for database access.
  - Sets up SSM parameters for database username and password.
- [x] Concordium Listener Server: [Amazon ECS](./lib/backend-listener-stack.ts)
  - Deploys a Concordium Listener Server as an ECS service.
  - Configures a task definition with the necessary environment variables and secrets.
  - Sets up logging for the listener server.
- [x] Upwood APIs/Web Server: [Amazon ECS](./lib/backend-api-stack.ts)
  - Deploys the Upwood APIs as an ECS service.
  - Configures a task definition with the necessary environment variables, secrets, and permissions.
  - Sets up logging for the API server.
- [x] API Gateway using Amazon API Gateway: [Amazon API Gateway](./lib/backend-api-stack.ts)
  - Creates an API Gateway to proxy requests to the backend API.
  - Configures CORS settings and integrates with the ECS service using a VPC Link.
  - Sets up a domain name and certificate for the API Gateway.
- [x] Frontend Deployment & Distribution: [Amazon S3 & Cloudfront](./lib/frontend-app-website-stack.ts)
  - Creates an S3 bucket to host the frontend application.
  - Configures CloudFront to distribute the frontend application.
  - Sets up a domain name and certificate for the CloudFront distribution.
- [x] Files S3 Stack: [Amazon S3 & Cloudfront](./lib/files-s3-stack.ts)
  - Creates an S3 bucket to store files.
  - Configures CloudFront to distribute the files.
  - Sets up a domain name and certificate for the CloudFront distribution.
- [ ] Dapp Frontend Deployment & Distribution: Amazon S3 & Cloudfront

### Prerequisites

- AWS Account

#### SSM Params

- Db Username SSM Parameter
- Db Password SSM Parameter
- Tree NFT Agent Wallet SSM Parameter
- Offchain Rewards Agent Wallet SSM Parameter
- Filebase Access Key ID SSM Parameter
- Filebase Secret Access Key SSM Parameter
- ACM CERTIFICATE ARN SSM Parameter

#### Domain Setup
