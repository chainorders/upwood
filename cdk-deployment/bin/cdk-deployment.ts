#!/usr/bin/env node
import "source-map-support/register";

import * as cdk from "aws-cdk-lib";
import { InstanceClass, InstanceSize } from "aws-cdk-lib/aws-ec2";
import * as ecs from "aws-cdk-lib/aws-ecs";
import { PostgresEngineVersion } from "aws-cdk-lib/aws-rds";
import * as dotenv from "dotenv";
import * as path from "path";

import { BackendApiStack } from "../lib/backend-api-stack";
import { BackendListenerStack } from "../lib/backend-listener-stack";
import { CognitoStack } from "../lib/cognito-stack";
import { DbStack } from "../lib/db-stack";
import { FilesS3Stack } from "../lib/files-s3-stack";
import { FrontendAppWebsiteStack } from "../lib/frontend-app-website-stack";
import { InfraStack } from "../lib/infra-stack";
import { SESStack } from "../lib/ses-stack";
import { OrganizationEnv } from "../lib/shared";

dotenv.config({ path: path.resolve(__dirname, "../.env") });
dotenv.config({ path: path.resolve(__dirname, "../.secure.env") });

const ACCOUNT = process.env.CDK_DEFAULT_ACCOUNT!;
const REGION = process.env.CDK_DEFAULT_REGION!;
const ORGANIZATION = process.env.ORGANIZATION!;
const ORGANIZATION_ENV = process.env.ORGANIZATION_ENV as OrganizationEnv;

const app = new cdk.App({
	autoSynth: true,
	context: {
		account: ACCOUNT,
		region: REGION,
		organization: ORGANIZATION,
		organizationEnv: ORGANIZATION_ENV,
		"@aws-cdk/core:newStyleStackSynthesis": true,
	},
});

const sesStack = new SESStack(app, "SESStack", {
	env: {
		account: ACCOUNT,
		region: REGION,
	},
	organization: ORGANIZATION,
	organization_env: ORGANIZATION_ENV,
	tags: {
		organization: ORGANIZATION,
		environment: ORGANIZATION_ENV,
	},
	baseDomain: process.env.SES_DOMAIN_NAME!,
});

const cognitoStack = new CognitoStack(app, "CognitoStack", {
	appDomain: process.env.APP_DOMAIN_NAME!,
	env: {
		account: ACCOUNT,
		region: REGION,
	},
	organization: ORGANIZATION,
	organization_env: ORGANIZATION_ENV,
	tags: {
		organization: ORGANIZATION,
		environment: ORGANIZATION_ENV,
	},
	fromEmail: process.env.COGNITO_FROM_EMAIL!,
	fromName: process.env.COGNITO_FROM_NAME!,
	replyTo: process.env.COGNITO_REPLY_TO!,
	sesDomain: process.env.SES_DOMAIN_NAME!,
});
cognitoStack.addDependency(sesStack, "Cogntito sends emails through SES");

const infraStack = new InfraStack(app, "InfraStack", {
	env: {
		account: ACCOUNT,
		region: REGION,
	},
	organization: ORGANIZATION,
	organization_env: ORGANIZATION_ENV,
	tags: {
		organization: ORGANIZATION,
		environment: ORGANIZATION_ENV,
	},
	backendInstanceClass: process.env.BACKEND_INSTANCE_CLASS! as InstanceClass,
	backendInstanceSize: process.env.BACKEND_INSTANCE_SIZE! as InstanceSize,
	logsRetentionDays: parseInt(process.env.LISTENER_LOGS_RETENTION_DAYS!),
});

const dbStack = new DbStack(app, "DbStack", {
	env: {
		account: ACCOUNT,
		region: REGION,
	},
	organization: ORGANIZATION,
	organization_env: ORGANIZATION_ENV,
	tags: {
		organization: ORGANIZATION,
		environment: ORGANIZATION_ENV,
	},
	dbInstanceSize: process.env.DB_INSTANCE_SIZE! as InstanceSize,
	dbInstanceClass: process.env.DB_INSTANCE_CLASS! as InstanceClass,
	dbEngineVersion: PostgresEngineVersion.of(
		process.env.DB_POSTGRES_FULL_VER!,
		process.env.DB_POSTGRES_MAJOR_VER!,
	),
	dbUsername: process.env.DB_USERNAME!,
	dbPassword: cdk.SecretValue.unsafePlainText(process.env.DB_PASSWORD!),
	dbBackupRetentionDays: parseInt(process.env.DB_BACKUP_RETENTION_DAYS!),
	dbPort: parseInt(process.env.DB_PORT!),
	dbName: process.env.DB_NAME!,
	dbStorageGiB: parseInt(process.env.DB_STORAGE_GB!),
});

const listenerStack = new BackendListenerStack(app, "BackendListenerStack", {
	env: {
		account: ACCOUNT,
		region: REGION,
	},
	organization: ORGANIZATION,
	organization_env: ORGANIZATION_ENV,
	tags: {
		organization: ORGANIZATION,
		environment: ORGANIZATION_ENV,
	},
	concordiumNodeUri: process.env.CONCORDIUM_NODE_URI!,
	listenerAccountAddress: process.env.LISTENER_ACCOUNT_ADDRESS!,
	listenerDefaultBlockHeight: parseInt(
		process.env.LISTENER_DEFAULT_BLOCK_HEIGHT!,
	),
	dbPoolMaxSize: parseInt(process.env.LISTENER_DB_POOL_MAX_SIZE!),
	memoryReservationSoftMiB: parseInt(process.env.MEMORY_RESERVATION_SOFT_MIB!),
	postgresPort: parseInt(process.env.DB_PORT!),
	postgresHostname: dbStack.dbInstance.dbInstanceEndpointAddress,
	postgresDb: process.env.DB_NAME!,
	postgresPasswordParameter: dbStack.dbPasswordParam,
	postgresUserParameter: dbStack.dbUsernameParam,
	cluster: infraStack.cluster,
	logGroup: infraStack.logGroup,
	vpc: infraStack.vpc,
	secrets: {
		POSTGRES_PASSWORD: ecs.Secret.fromSsmParameter(dbStack.dbPasswordParam),
		POSTGRES_USER: ecs.Secret.fromSsmParameter(dbStack.dbUsernameParam),
	},
	environment: {
		RUST_LOG: process.env.RUST_LOG!,
		AWS_REGION: process.env.CDK_DEFAULT_REGION!,
		POSTGRES_DB: process.env.DB_NAME!,
		POSTGRES_HOST: dbStack.dbInstance.dbInstanceEndpointAddress,
		POSTGRES_PORT: process.env.DB_PORT!,
		DB_POOL_MAX_SIZE: process.env.LISTENER_DB_POOL_MAX_SIZE!,
		CONCORDIUM_NODE_URI: process.env.CONCORDIUM_NODE_URI!,
		NODE_RATE_LIMIT: process.env.NODE_RATE_LIMIT!,
		NODE_RATE_LIMIT_DURATION_MILLIS:
			process.env.NODE_RATE_LIMIT_DURATION_MILLIS!,
		NODE_REQUEST_TIMEOUT_MILLIS: process.env.NODE_REQUEST_TIMEOUT_MILLIS!,
		NODE_CONNECT_TIMEOUT_MILLIS: process.env.NODE_CONNECT_TIMEOUT_MILLIS!,
		ACCOUNT: process.env.LISTENER_ACCOUNT_ADDRESS!,
		DEFAULT_BLOCK_HEIGHT: process.env.LISTENER_DEFAULT_BLOCK_HEIGHT!,
		LISTENER_RETRY_TIMES: process.env.LISTENER_RETRY_TIMES!,
		LISTENER_RETRY_MIN_DELAY_MILLIS:
			process.env.LISTENER_RETRY_MIN_DELAY_MILLIS!,
		LISTENER_RETRY_MAX_DELAY_MILLIS:
			process.env.LISTENER_RETRY_MAX_DELAY_MILLIS!,
	},
});
listenerStack.addDependency(dbStack, "Listener uses RDS for data storage");
listenerStack.addDependency(infraStack, "Listener runs on ECS");

const filesStack = new FilesS3Stack(app, "FilesS3Stack", {
	env: {
		account: ACCOUNT,
		region: REGION,
	},
	organization: ORGANIZATION,
	organization_env: ORGANIZATION_ENV,
	tags: {
		organization: ORGANIZATION,
		environment: ORGANIZATION_ENV,
	},
	domainName: process.env.FILES_DOMAIN_NAME!,
	certificateArn: process.env.FILES_FRONT_CERTIFICATE_ARN!,
});

const apiStack = new BackendApiStack(app, "BackendApiStack", {
	env: {
		account: ACCOUNT,
		region: REGION,
	},
	organization: ORGANIZATION,
	organization_env: ORGANIZATION_ENV,
	tags: {
		organization: ORGANIZATION,
		environment: ORGANIZATION_ENV,
	},
	cluster: infraStack.cluster,
	vpc: infraStack.vpc,
	vpcLink: infraStack.vpcLink,
	discoveryNamespace: infraStack.discoveryNamespace,
	logGroup: infraStack.logGroup,
	apiSocketPort: parseInt(process.env.API_SOCKET_PORT!),
	filesBucket: filesStack.filesBucket,
	containerCount: 1,
	memoryReservationSoftMiB: parseInt(process.env.MEMORY_RESERVATION_SOFT_MIB!),
	domainName: process.env.API_DOMAIN_NAME!,
	certificateArn: process.env.API_DOMAIN_CERTIFICATE_ARN!,
	secrets: {
		POSTGRES_PASSWORD: ecs.Secret.fromSsmParameter(dbStack.dbPasswordParam),
		POSTGRES_USER: ecs.Secret.fromSsmParameter(dbStack.dbUsernameParam),
	},
	cognito: cognitoStack.userPool,
	emailIdentity: sesStack.emailIdentity,
	environment: {
		//secrets
		TREE_NFT_AGENT_WALLET_JSON_STR: process.env.TREE_NFT_AGENT_WALLET_JSON_STR!,
		OFFCHAIN_REWARDS_AGENT_WALLET_JSON_STR:
			process.env.OFFCHAIN_REWARDS_AGENT_WALLET_JSON_STR!,
		FILEBASE_ACCESS_KEY_ID: process.env.FILEBASE_ACCESS_KEY_ID!,
		FILEBASE_SECRET_ACCESS_KEY: process.env.FILEBASE_SECRET_ACCESS_KEY!,
		//other
		RUST_LOG: process.env.RUST_LOG!,
		AWS_REGION: process.env.CDK_DEFAULT_REGION!,
		API_SOCKET_PORT: process.env.API_SOCKET_PORT!,
		API_SOCKET_ADDRESS: process.env.API_SOCKET_ADDRESS!,
		POSTGRES_DB: process.env.DB_NAME!,
		POSTGRES_HOST: dbStack.dbInstance.dbInstanceEndpointAddress,
		POSTGRES_PORT: process.env.DB_PORT!,
		DB_POOL_MAX_SIZE:
			process.env.API_DB_POOL_MAX_SIZE || process.env.BACKEND_DB_POOL_MAX_SIZE!,
		AWS_USER_POOL_ID: cognitoStack.userPool.userPoolId,
		AWS_USER_POOL_CLIENT_ID: cognitoStack.userPoolClient.userPoolClientId,
		AWS_USER_POOL_REGION: cognitoStack.userPool.env.region,
		CONCORDIUM_NETWORK: process.env.CONCORDIUM_NETWORK!,
		CONCORDIUM_NODE_URI: process.env.CONCORDIUM_NODE_URI!,
		EURO_E_CONTRACT_INDEX: process.env.EURO_E_CONTRACT_INDEX!,
		IDENTITY_REGISTRY_CONTRACT_INDEX:
			process.env.IDENTITY_REGISTRY_CONTRACT_INDEX!,
		COMPLIANCE_CONTRACT_INDEX: process.env.COMPLIANCE_CONTRACT_INDEX!,
		CARBON_CREDIT_CONTRACT_INDEX: process.env.CARBON_CREDIT_CONTRACT_INDEX!,
		TREE_FT_CONTRACT_INDEX: process.env.TREE_FT_CONTRACT_INDEX!,
		TREE_NFT_CONTRACT_INDEX: process.env.TREE_NFT_CONTRACT_INDEX!,
		MINT_FUNDS_CONTRACT_INDEX: process.env.MINT_FUNDS_CONTRACT_INDEX!,
		TRADING_CONTRACT_INDEX: process.env.TRADING_CONTRACT_INDEX!,
		YIELDER_CONTRACT_INDEX: process.env.YIELDER_CONTRACT_INDEX!,
		OFFCHAIN_REWARDS_CONTRACT_INDEX:
			process.env.OFFCHAIN_REWARDS_CONTRACT_INDEX!,
		AFFILIATE_COMMISSION: process.env.AFFILIATE_COMMISSION!,
		FILES_BUCKET_NAME: filesStack.filesBucket.bucketName,
		FILES_PRESIGNED_URL_EXPIRY_SECS:
			process.env.FILES_PRESIGNED_URL_EXPIRY_SECS!,
		FILEBASE_BUCKET_NAME: process.env.FILEBASE_BUCKET_NAME!,
		FILEBASE_S3_ENDPOINT_URL: process.env.FILEBASE_S3_ENDPOINT_URL!,
		ID_STATEMENT: process.env.ID_STATEMENT!,
		SES_FROM_EMAIL: process.env.SES_FROM_EMAIL!,
		COMPANY_INVITATION_ACCEPT_URL: process.env.COMPANY_INVITATION_ACCEPT_URL!,
	},
});
apiStack.addDependency(sesStack, "Backend API sends emails through SES");
apiStack.addDependency(cognitoStack, "Backend API uses Cognito for user management");
apiStack.addDependency(filesStack, "Backend API uses S3 for file storage");
apiStack.addDependency(dbStack, "Backend API uses RDS for data storage");
apiStack.addDependency(infraStack, "Backend API runs on ECS");
apiStack.addDependency(listenerStack, "Backend API needs Indexer for Concordium");

const websiteStack = new FrontendAppWebsiteStack(app, "FrontendAppWebsiteStack", {
	env: {
		account: ACCOUNT,
		region: REGION,
	},
	organization: ORGANIZATION,
	organization_env: ORGANIZATION_ENV,
	tags: {
		organization: ORGANIZATION,
		environment: ORGANIZATION_ENV,
	},
	domainName: process.env.APP_DOMAIN_NAME!,
	certificateArn: process.env.APP_FRONT_CERTIFICATE_ARN!,
});
websiteStack.addDependency(apiStack, "Frontend app consumes the backend API");
websiteStack.addDependency(filesStack, "Frontend app uses S3 for file storage");
websiteStack.addDependency(cognitoStack, "Frontend app uses Cognito for user management");

app.synth();
