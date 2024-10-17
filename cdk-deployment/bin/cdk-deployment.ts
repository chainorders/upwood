#!/usr/bin/env node
import "source-map-support/register";
import * as cdk from "aws-cdk-lib";
import { CognitoStack } from "../lib/cognito-stack";
import { OrganizationEnv } from "../lib/shared";
import { PostgresEngineVersion } from "aws-cdk-lib/aws-rds";
import { InstanceClass, InstanceSize, Vpc } from "aws-cdk-lib/aws-ec2";
import { BackendListenerStack } from "../lib/backend-listener-stack";
import { InfraStack } from "../lib/infra-stack";
import { BackendApiStack } from "../lib/backend-api-stack";
import * as path from "path";
import * as fs from "fs";

const ACCOUNT = process.env.CDK_DEFAULT_ACCOUNT!;
const REGION = process.env.CDK_DEFAULT_REGION || "eu-west-2";
const ORGANIZATION = "upwood";
const ORGANIZATION_ENV = process.env.ORGANIZATION_ENV
	? (process.env.ORGANIZATION_ENV as OrganizationEnv)
	: OrganizationEnv.DEV;
const DB_ENGINE_VERSION = PostgresEngineVersion.VER_16_4;
const DB_INSTANCE_CLASS = InstanceClass.T4G;
const DB_INSTANCE_SIZE = InstanceSize.MICRO;
const DB_STORAGE_GB = 20;
const BACKEND_INSTANCE_SIZE = InstanceSize.NANO;
const BACKEND_INSTANCE_CLASS = InstanceClass.T2;
const LISTENER_LOGS_RETENTION_DAYS = 1;
const DB_PORT = 5432;
const DB_NAME = "postgres";
const DB_BACKUP_RETENTION_DAYS = 1;
const LISTENER_DB_POOL_MAX_SIZE = 10;
const BACKEND_DB_POOL_MAX_SIZE = 10;

// TODO: use a secure secret
const DB_PASSWORD = "postgres";
const DB_USERNAME = "postgres";
const TREE_NFT_AGENT_WALLET_JSON_STR = fs.readFileSync(
	path.join(__dirname, "../tree-nft-agent-wallet.json"),
	"utf8",
);

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
	dbInstanceSize:
		(process.env.DB_INSTANCE_SIZE as InstanceSize) || DB_INSTANCE_SIZE,
	dbInstanceClass:
		(process.env.DB_INSTANCE_CLASS as InstanceClass) || DB_INSTANCE_CLASS,
	dbEngineVersion: DB_ENGINE_VERSION,
	dbUsername: process.env.DB_USERNAME || DB_USERNAME,
	dbPassword: cdk.SecretValue.unsafePlainText(
		process.env.DB_PASSWORD || DB_PASSWORD,
	),
	dbBackupRetentionDays: process.env.DB_BACKUP_RETENTION_DAYS
		? parseInt(process.env.DB_BACKUP_RETENTION_DAYS)
		: DB_BACKUP_RETENTION_DAYS,
	dbPort: process.env.DB_PORT ? parseInt(process.env.DB_PORT) : DB_PORT,
	dbName: process.env.DB_NAME || DB_NAME,
	dbStorageGiB: process.env.DB_STORAGE_GB
		? parseInt(process.env.DB_STORAGE_GB)
		: DB_STORAGE_GB,
	backendInstanceClass:
		(process.env.BACKEND_INSTANCE_CLASS as InstanceClass) ||
		BACKEND_INSTANCE_CLASS,
	backendInstanceSize:
		(process.env.BACKEND_INSTANCE_SIZE as InstanceSize) ||
		BACKEND_INSTANCE_SIZE,
	logsRetentionDays: process.env.LISTENER_LOGS_RETENTION_DAYS
		? parseInt(process.env.LISTENER_LOGS_RETENTION_DAYS)
		: LISTENER_LOGS_RETENTION_DAYS,
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
	concordiumNodeUri: "http://node.testnet.concordium.com:20000",
	listenerAccountAddress: "4fWTMJSAymJoFeTbohJzwejT6Wzh1dAa2BtnbDicgjQrc94TgW",
	listenerDefaultBlockHeight: 19010377,
	dbPoolMaxSize: process.env.LISTENER_DB_POOL_MAX_SIZE
		? parseInt(process.env.LISTENER_DB_POOL_MAX_SIZE)
		: LISTENER_DB_POOL_MAX_SIZE,
	memoryReservationSoftMiB: 50,
	postgresPort: process.env.DB_PORT ? parseInt(process.env.DB_PORT) : DB_PORT,
	postgresHostname: infraStack.dbInstance.dbInstanceEndpointAddress,
	postgresDb: process.env.DB_NAME || DB_NAME,
	postgresPasswordParameter: infraStack.dbPasswordParam,
	postgresUserParameter: infraStack.dbUsernameParam,
	cluster: infraStack.cluster,
	logGroup: infraStack.logGroup,
	vpc: infraStack.vpc,
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
	apiSocketAddress: "0.0.0.0",
	apiSocketPort: 3000,
	awsUserPoolId: cognitoStack.userPool.userPoolId,
	awsUserPoolClientId: cognitoStack.userPoolClient.userPoolClientId,
	awsUserPoolRegion: cognitoStack.userPool.env.region,
	concordiumNodeUri: "http://node.testnet.concordium.com:20000",
	concordiumNetwork: "testnet",
	postgresPort: process.env.DB_PORT ? parseInt(process.env.DB_PORT) : DB_PORT,
	postgresHostname: infraStack.dbInstance.dbInstanceEndpointAddress,
	postgresDb: process.env.DB_NAME || DB_NAME,
	postgresPasswordParameter: infraStack.dbPasswordParam,
	postgresUserParameter: infraStack.dbUsernameParam,
	dbPoolMaxSize: process.env.API_DB_POOL_MAX_SIZE
		? parseInt(process.env.API_DB_POOL_MAX_SIZE)
		: BACKEND_DB_POOL_MAX_SIZE,
	containerCount: 1,
	userChallengeExpiryDurationMins: 1,
	treeNftAgentWalletJsonStr: cdk.SecretValue.unsafePlainText(
		process.env.TREE_NFT_AGENT_WALLET_JSON_STR ||
			TREE_NFT_AGENT_WALLET_JSON_STR,
	),
	memoryReservationSoftMiB: 50,
});

app.synth();
