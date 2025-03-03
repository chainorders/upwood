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
import { FilesS3Stack } from "../lib/files-s3-stack";
import { FrontendAppWebsiteStack } from "../lib/frontend-app-website-stack";
import { InfraStack } from "../lib/infra-stack";
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
let cognitoStack = new CognitoStack(app, "CognitoStack", {
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
});

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
	backendInstanceClass: process.env.BACKEND_INSTANCE_CLASS! as InstanceClass,
	backendInstanceSize: process.env.BACKEND_INSTANCE_SIZE! as InstanceSize,
	logsRetentionDays: parseInt(process.env.LISTENER_LOGS_RETENTION_DAYS!),
});

new BackendListenerStack(app, "BackendListenerStack", {
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
	postgresHostname: infraStack.dbInstance.dbInstanceEndpointAddress,
	postgresDb: process.env.DB_NAME!,
	postgresPasswordParameter: infraStack.dbPasswordParam,
	postgresUserParameter: infraStack.dbUsernameParam,
	cluster: infraStack.cluster,
	logGroup: infraStack.logGroup,
	vpc: infraStack.vpc,
});

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

new BackendApiStack(app, "BackendApiStack", {
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
	userPoolArn: cognitoStack.userPool.userPoolArn,
	filesBucket: filesStack.filesBucket,
	containerCount: 1,
	memoryReservationSoftMiB: parseInt(process.env.MEMORY_RESERVATION_SOFT_MIB!),
	domainName: process.env.API_DOMAIN_NAME!,
	certificateArn: process.env.API_DOMAIN_CERTIFICATE_ARN!,
	secrets: {
		POSTGRES_PASSWORD: ecs.Secret.fromSsmParameter(infraStack.dbPasswordParam),
		POSTGRES_USER: ecs.Secret.fromSsmParameter(infraStack.dbUsernameParam),
	},
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
		POSTGRES_HOST: infraStack.dbInstance.dbInstanceEndpointAddress,
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
	},
});

new FrontendAppWebsiteStack(app, "FrontendAppWebsiteStack", {
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

app.synth();
