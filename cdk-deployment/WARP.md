# WARP.md

This file provides guidance to WARP (warp.dev) when working with the AWS infrastructure workspace.

## Workspace Overview

AWS Cloud Development Kit (CDK) infrastructure as code for managing production and staging environments. Built with TypeScript and AWS CDK v2, handling complete cloud infrastructure including ECS services, databases, CDN, monitoring, and security configurations.

## Development Environment

### Prerequisites

- Node.js with TypeScript and AWS CDK on host
- AWS CLI installed and configured on host (`~/.aws`)

### Local Setup

```bash
cd cdk-deployment
corepack enable && yarn install
# Example commands
yarn cdk synth
yarn cdk diff
yarn cdk deploy
```

### AWS Credentials Setup

Place AWS credentials CSV file at:

```
cdk-deployment/aws_accessKeys.csv

Format:
User Name,Access key ID,Secret access key
default,AKIA...,wJalr...
```

## Core Development Commands

### Development & Building

```bash
yarn build                      # Compile TypeScript to JavaScript
yarn watch                      # Watch mode compilation (tsc -w)
yarn test                       # Run infrastructure tests (jest)
yarn format                     # Format code with Prettier
```

### CDK Operations

```bash
yarn cdk deploy                 # Deploy infrastructure to AWS
yarn cdk deploy --all           # Deploy all stacks
yarn cdk diff                   # Show differences between local and deployed
yarn cdk synth                  # Generate CloudFormation templates
yarn cdk bootstrap             # Bootstrap CDK in AWS account (one-time)
yarn cdk destroy               # Destroy deployed infrastructure
yarn cdk ls                     # List all available stacks
```

## Infrastructure Architecture

### Deployed Stacks

#### ✅ User Authentication & Management

- **Service**: Amazon Cognito (`lib/cognito-stack.ts`)
- **Features**: User pools, admin groups, email settings, custom attributes

#### ✅ Infrastructure Foundation

- **Service**: ECS Cluster & Private DNS (`lib/infra-stack.ts`)
- **Features**: ECS cluster, private DNS namespace, VPC link, application logs

#### ✅ Database Server

- **Service**: Amazon RDS (`lib/db-stack.ts`)
- **Features**: PostgreSQL instance, security groups, SSM parameters for credentials

#### ✅ Concordium Listener Server

- **Service**: Amazon ECS (`lib/backend-listener-stack.ts`)
- **Features**: ECS service for blockchain event processing, environment variables, logging

#### ✅ Upwood APIs/Web Server

- **Service**: Amazon ECS (`lib/backend-api-stack.ts`)
- **Features**: ECS service for REST APIs, task definitions, secrets management

#### ✅ API Gateway

- **Service**: Amazon API Gateway (`lib/backend-api-stack.ts`)
- **Features**: API proxy, CORS configuration, VPC link integration, domain setup

#### ✅ Frontend Distribution

- **Service**: S3 & CloudFront (`lib/frontend-app-website-stack.ts`)
- **Features**: S3 hosting, CloudFront CDN, domain configuration, SSL certificates

#### ✅ File Storage

- **Service**: S3 & CloudFront (`lib/files-s3-stack.ts`)
- **Features**: File storage bucket, CDN distribution, domain setup

#### 🚧 Planned: Concordium Blockchain Node

- **Service**: Amazon ECS (planned)
- **Features**: Dedicated Concordium node infrastructure

#### 🚧 Planned: DApp Frontend Distribution

- **Service**: S3 & CloudFront (planned)
- **Features**: Legacy DApp hosting and distribution

## Development Patterns

### CDK Stack Structure

```typescript
// CDK stack pattern
import { Stack, StackProps } from 'aws-cdk-lib';
import { Construct } from 'constructs';

export class ServiceStack extends Stack {
  constructor(scope: Construct, id: string, props?: StackProps) {
    super(scope, id, props);
    
    // Define AWS resources
    // Configure resource dependencies
    // Output important values
  }
}
```

### Environment Configuration

```typescript
// Environment-specific configurations
const config = {
  dev: {
    // development environment settings
  },
  staging: {
    // staging environment settings  
  },
  prod: {
    // production environment settings
  }
};
```

### Resource Naming

```typescript
// Consistent resource naming pattern
const resourceName = `${props.environment}-${serviceName}-${resourceType}`;
```

## Prerequisites & Setup

### Required SSM Parameters

Before deployment, ensure these SSM parameters exist:

- Database Username SSM Parameter
- Database Password SSM Parameter  
- Tree NFT Agent Wallet SSM Parameter
- Offchain Rewards Agent Wallet SSM Parameter
- Filebase Access Key ID SSM Parameter
- Filebase Secret Access Key SSM Parameter
- ACM Certificate ARN SSM Parameter

### Domain Setup

Configure domains for:

- API Gateway endpoints
- CloudFront distributions
- SSL certificate management

## Common Development Tasks

### Adding New Stack

1. Create new stack file in `lib/stacks/`
2. Implement CDK stack class
3. Add stack to main CDK application in `bin/`
4. Configure stack dependencies
5. Update deployment scripts

### Modifying Existing Stack

1. Update stack implementation in `lib/stacks/`
2. Run `yarn cdk diff` to review changes
3. Test changes with `yarn cdk synth`
4. Deploy with `yarn cdk deploy <stack-name>`

### Managing Secrets & Configuration

```bash
# Store secrets in AWS Systems Manager Parameter Store
aws ssm put-parameter --name "/app/secret-name" --value "secret-value" --type "SecureString"

# Reference in CDK
const secret = ssm.StringParameter.valueFromLookup(this, '/app/secret-name');
```

## Testing Strategy

### Infrastructure Testing

```bash
# Run CDK unit tests
yarn test

# Test specific stack
yarn test -- --testPathPattern=stack-name

# Generate test coverage
yarn test -- --coverage
```

### Stack Validation

```bash
# Validate CloudFormation templates
yarn cdk synth

# Check for security issues
cdk-nag analysis (if configured)
```

## Environment Management

### Multiple Environments

```bash
# Deploy to specific environment
yarn cdk deploy --context environment=dev
yarn cdk deploy --context environment=staging  
yarn cdk deploy --context environment=prod
```

### Stack Dependencies

- Cognito stack must be deployed first
- Infrastructure stack provides foundation for other services
- Database stack required before backend services
- Backend stacks must exist before frontend deployment

## Deployment Workflow

### Initial Setup (One-time)

```bash
# Bootstrap CDK in AWS account
yarn cdk bootstrap

# Deploy foundation stacks
yarn cdk deploy InfraStack
yarn cdk deploy CognitoStack
yarn cdk deploy DatabaseStack
```

### Regular Deployment

```bash
# Review changes
yarn cdk diff

# Deploy all changes
yarn cdk deploy --all

# Deploy specific stack
yarn cdk deploy BackendApiStack
```

### Rollback Strategy

```bash
# Destroy specific stack
yarn cdk destroy StackName

# Redeploy previous version
git checkout previous-version
yarn cdk deploy StackName
```

## Monitoring & Debugging

### CloudFormation Console

- Monitor stack deployment progress
- View stack events and errors
- Check resource creation status

### CDK Debugging

```bash
# Verbose CDK output
yarn cdk deploy --verbose

# Debug CDK synthesis
yarn cdk synth --verbose

# Check CDK version
yarn cdk --version
```

### AWS CLI Integration

```bash
# Check stack status
aws cloudformation describe-stacks --stack-name StackName

# View stack events  
aws cloudformation describe-stack-events --stack-name StackName
```

## Configuration Files

### CDK Configuration

- **cdk.json** - CDK app configuration and feature flags
- **tsconfig.json** - TypeScript compilation settings
- **jest.config.js** - Testing framework configuration (if present)

### Environment Variables

```bash
# CDK-specific environment variables
CDK_DEFAULT_ACCOUNT=123456789012
CDK_DEFAULT_REGION=us-east-1
```

## Cost Management

### Resource Optimization

- Use appropriate instance sizes for ECS tasks
- Configure CloudFront caching for cost efficiency
- Set up S3 lifecycle policies for file storage
- Monitor RDS instance sizing and usage

### Cost Monitoring

```bash
# Estimate costs before deployment
yarn cdk synth | grep -i cost

# Use AWS Cost Explorer for deployed resources
```

## Prettier Configuration

```json
{
  "tabWidth": 2,
  "useTabs": true
}
```
