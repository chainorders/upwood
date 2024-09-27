#!/usr/bin/env node
import "source-map-support/register";
import * as cdk from "aws-cdk-lib";
import { CognitoStack } from "../lib/cognito-stack";

const app = new cdk.App({
	autoSynth: true,
	context: {
		"@aws-cdk/core:newStyleStackSynthesis": true,
	},
});
let cognitoStack = new CognitoStack(app, "CognitoStack", {
	organization: "upwood",
	organization_env: "dev",
	env: { region: "eu-west-2" },
	tags: {
		organization: "upwood",
		environment: "dev",
	},
});