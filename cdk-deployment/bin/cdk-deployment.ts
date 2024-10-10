#!/usr/bin/env node
import "source-map-support/register";
import * as cdk from "aws-cdk-lib";
import { CognitoStack } from "../lib/cognito-stack";
import { OrganizationEnv } from "../lib/shared";
import { PostgresEngineVersion } from "aws-cdk-lib/aws-rds";
import { InstanceClass, InstanceSize, Vpc } from "aws-cdk-lib/aws-ec2";
import { RdsStack } from "../lib/rds-stack";
import { NetworkStack } from "../lib/network-stack";

const ACCOUNT = process.env.CDK_DEFAULT_ACCOUNT!;
const REGION = process.env.CDK_DEFAULT_REGION || "eu-west-2";
const ORGANIZATION = "upwood";
const DB_ENGINE_VERSION = PostgresEngineVersion.VER_16_4;
const DB_INSTANCE_CLASS = InstanceClass.T4G;
const DB_INSTANCE_SIZE = InstanceSize.MICRO;
const DB_STORAGE_GB = 20;

const app = new cdk.App({
	autoSynth: true,
	context: {
		account: ACCOUNT,
		region: REGION,
		organization: ORGANIZATION,
		organizationEnv: OrganizationEnv.DEV,
		"@aws-cdk/core:newStyleStackSynthesis": true,
	},
});
new CognitoStack(app, "CognitoStack", {
	env: {
		account: ACCOUNT,
		region: REGION,
	},
	organization: ORGANIZATION,
	organization_env: OrganizationEnv.DEV,
	tags: {
		organization: ORGANIZATION,
		environment: OrganizationEnv.DEV,
	},
});
let networkStack = new NetworkStack(app, "NetworkStack", {
	env: {
		account: ACCOUNT,
		region: REGION,
	},
	organization: ORGANIZATION,
	organization_env: OrganizationEnv.DEV,
	tags: {
		organization: ORGANIZATION,
		environment: OrganizationEnv.DEV,
	},
});
new RdsStack(app, "RdsStack", {
	env: {
		account: ACCOUNT,
		region: REGION,
	},
	organization: ORGANIZATION,
	organization_env: OrganizationEnv.DEV,
	tags: {
		organization: ORGANIZATION,
		environment: OrganizationEnv.DEV,
	},
	dbInstanceSize: (process.env.DB_INSTANCE_SIZE as InstanceSize) || DB_INSTANCE_SIZE,
	dbInstanceClass: (process.env.DB_INSTANCE_CLASS as InstanceClass) || DB_INSTANCE_CLASS,
	dbEngineVersion: DB_ENGINE_VERSION,
	dbUsername: "postgres",
	dbPassword: cdk.SecretValue.unsafePlainText("postgres"),
	dbBackupRetentionDays: 7,
	vpc: networkStack.vpc,
	dbPort: 5432,
	dbName: "postgres",
	dbStorageGiB: process.env.DB_STORAGE_GB ? parseInt(process.env.DB_STORAGE_GB) : DB_STORAGE_GB,
});
