import * as cdk from "aws-cdk-lib";
import { Construct } from "constructs";
import * as cognito from "aws-cdk-lib/aws-cognito";

export interface StackProps extends cdk.StackProps {
	organization: string;
	organization_env: string;
}

export class CognitoStack extends cdk.Stack {
	userPool: cdk.aws_cognito.UserPool;
	client: cdk.aws_cognito.UserPoolClient;
	adminGroup: cdk.aws_cognito.CfnUserPoolGroup;
	constructor(scope: Construct, id: string, props: StackProps) {
		super(scope, id, props);
		this.userPool = new cognito.UserPool(this, `${props.organization}-${props.organization_env}-user-pool`, {
			standardAttributes: {
				email: { required: true, mutable: false },
				givenName: { required: true, mutable: true },
				familyName: { required: true, mutable: true },
			},
			autoVerify: { email: true },
			accountRecovery: cognito.AccountRecovery.EMAIL_ONLY,
			selfSignUpEnabled: true,
			signInAliases: { email: true },
			email: cognito.UserPoolEmail.withCognito(), // TODO: Implement email configuration with SES
			// userInvitation: {}, // TODO: Implement user invitation
			// userVerification: {}, //TODO: Implement user verification
		});
		cdk.Tags.of(this.userPool).add("organization", props.organization);
		cdk.Tags.of(this.userPool).add("environment", props.organization_env);
		this.adminGroup = new cognito.CfnUserPoolGroup(
			this,
			`${props.organization}-${props.organization_env}-group-admin`,
			{
				userPoolId: this.userPool.userPoolId,
				description: "Admin group",
				groupName: "admin",
			}
		);
		cdk.Tags.of(this.adminGroup).add("organization", props.organization);
		cdk.Tags.of(this.adminGroup).add("environment", props.organization_env);
		this.client = new cognito.UserPoolClient(this, `${props.organization}-${props.organization_env}-user-pool-client`, {
			userPool: this.userPool,
			userPoolClientName: `${props.organization}-${props.organization_env}-user-pool-client`,
			authFlows: {
				userPassword: true,
				userSrp: true,
			},
		});
		cdk.Tags.of(this.client).add("organization", props.organization);
		cdk.Tags.of(this.client).add("environment", props.organization_env);
	}
}
