# Welcome to your CDK TypeScript project

This is a blank project for CDK development with TypeScript.

The `cdk.json` file tells the CDK Toolkit how to execute your app.

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
- [ ] Database Server: [Amazon RDS](./lib/rds-stack.ts)
- [ ] Concordium Listener Server: [Amazon ECS](./lib/ecs-stack.ts)
- [ ] Upwood APIs/Web Server: [Amazon ECS](./lib/ecs-stack.ts)
- [ ] API Gateway using [Caddy](https://caddyserver.com/): [Amazon ECS](./lib/ecs-stack.ts)
- [ ] Frontend Deployment & Distribution: [Amazon S3 & Cloudfront](./lib/frontend-stack.ts)
