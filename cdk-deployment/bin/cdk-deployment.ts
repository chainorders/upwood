#!/usr/bin/env node
import "source-map-support/register";
import * as cdk from "aws-cdk-lib";
import { CognitoStack } from "../lib/cognito-stack";
import { OrganizationEnv } from "../lib/shared";
import { PostgresEngineVersion } from "aws-cdk-lib/aws-rds";
import { InstanceClass, InstanceSize, Vpc } from "aws-cdk-lib/aws-ec2";
import { BackendStack } from "../lib/backend-stack";
import { InfraStack } from "../lib/infra-stack";

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
const DB_USERNAME = "postgres";
const DB_PORT = 5432;
const DB_NAME = "postgres";
const DB_BACKUP_RETENTION_DAYS = 1;
const DB_PASSWORD = "postgres"; // TODO: use a secure secret

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
new CognitoStack(app, "CognitoStack", {
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
	dbInstanceSize: (process.env.DB_INSTANCE_SIZE as InstanceSize) || DB_INSTANCE_SIZE,
	dbInstanceClass: (process.env.DB_INSTANCE_CLASS as InstanceClass) || DB_INSTANCE_CLASS,
	dbEngineVersion: DB_ENGINE_VERSION,
	dbUsername: process.env.DB_USERNAME || DB_USERNAME,
	dbPassword: cdk.SecretValue.unsafePlainText(process.env.DB_PASSWORD || DB_PASSWORD),
	dbBackupRetentionDays: process.env.DB_BACKUP_RETENTION_DAYS
		? parseInt(process.env.DB_BACKUP_RETENTION_DAYS)
		: DB_BACKUP_RETENTION_DAYS,
	dbPort: process.env.DB_PORT ? parseInt(process.env.DB_PORT) : DB_PORT,
	dbName: process.env.DB_NAME || DB_NAME,
	dbStorageGiB: process.env.DB_STORAGE_GB ? parseInt(process.env.DB_STORAGE_GB) : DB_STORAGE_GB,
	backendInstanceClass: (process.env.BACKEND_INSTANCE_CLASS as InstanceClass) || BACKEND_INSTANCE_CLASS,
	backendInstanceSize: (process.env.BACKEND_INSTANCE_SIZE as InstanceSize) || BACKEND_INSTANCE_SIZE,
	listenerLogsRetentionDays: process.env.LISTENER_LOGS_RETENTION_DAYS
		? parseInt(process.env.LISTENER_LOGS_RETENTION_DAYS)
		: LISTENER_LOGS_RETENTION_DAYS,
});

new BackendStack(app, "BackendStack", {
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
	dbPoolMaxSize: 10,
	listenerMemoryReservationSoftMiB: 50,
	postgresPort: process.env.DB_PORT ? parseInt(process.env.DB_PORT) : DB_PORT,
	postgresHostname: infraStack.dbInstance.dbInstanceEndpointAddress,
	postgresDb: process.env.DB_NAME || DB_NAME,
	postgresPasswordParameter: infraStack.dbPasswordParam,
	postgresUserParameter: infraStack.dbUsernameParam,
	cluster: infraStack.cluster,
	logGroupListener: infraStack.logGroupListener,
	vpc: infraStack.vpc,
});

app.synth();
