# ☁️ AWS Infrastructure Workspace

**AWS Cloud Development Kit (CDK) infrastructure as code for Upwood's Concordium RWA platform, managing production and staging environments.**

## 🌟 Overview

This workspace contains AWS infrastructure definitions using TypeScript and AWS CDK v2. It manages the complete cloud infrastructure including ECS services, databases, CDN, monitoring, and security configurations for the Concordium RWA platform.

## 🏗️ Directory Structure

```
cdk-deployment/
├── bin/                         # CDK application entry points
│   └── cdk-deployment.js       # Main CDK application
├── lib/                         # CDK stack definitions
│   ├── stacks/                 # Individual stack implementations
│   ├── constructs/             # Reusable CDK constructs
│   └── config/                 # Environment configurations
├── cdk.out/                     # CDK generated CloudFormation templates
├── test/                        # Infrastructure tests
├── cdk.json                     # CDK configuration
├── tsconfig.json               # TypeScript configuration
└── package.json                # Dependencies and scripts
```

## 🚀 Development Environment Setup

### Using VS Code Dev Containers

1. **Open VS Code in repository root**

   ```bash
   cd /path/to/concordium-rwa
   code .
   ```

2. **Open in Dev Container**
   - Press `F1` or `Ctrl+Shift+P`
   - Type: `Dev Containers: Reopen in Container`
   - Select: **cdk-deployment**

3. **Container Setup**
   - The container automatically installs Node.js, TypeScript, and AWS CDK
   - AWS CLI is pre-installed for credential management
   - Terminal shows available yarn scripts upon completion

### AWS Credentials Setup

```bash
# Place your AWS credentials CSV file at:
.devcontainer/cdk-deployment/aws_accessKeys.csv

# Format:
# User Name,Access key ID,Secret access key
# default,AKIA...,wJalr...

# The container automatically configures AWS CLI with these credentials
```

## 🛠️ Available Scripts

All scripts are defined in `package.json` and can be run with `yarn <script>`:

### Development

```bash
yarn build      # Compile TypeScript to JavaScript (tsc)
yarn watch      # Watch mode compilation (tsc -w)
yarn test       # Run infrastructure tests (jest)
yarn format     # Format code with Prettier
```

### CDK Commands

```bash
yarn cdk deploy           # Deploy infrastructure to AWS
yarn cdk deploy --all     # Deploy all stacks
yarn cdk diff             # Show differences between local and deployed
yarn cdk synth            # Synthesize CloudFormation templates
yarn cdk bootstrap        # Bootstrap CDK in AWS account
yarn cdk destroy          # Destroy deployed infrastructure
yarn cdk ls               # List all stacks
```

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
