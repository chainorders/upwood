import * as cdk from "aws-cdk-lib";
import { Construct } from "constructs";
import { constructName, OrganizationEnv, StackProps } from "./shared";
import * as cognito from "aws-cdk-lib/aws-cognito";
import * as iam from "aws-cdk-lib/aws-iam";

export class CognitoStack extends cdk.Stack {
	userPool: cdk.aws_cognito.UserPool;
	client: cdk.aws_cognito.UserPoolClient;
	adminGroup: cdk.aws_cognito.CfnUserPoolGroup;
	adminRole: cdk.aws_iam.Role;

	constructor(scope: Construct, id: string, props: StackProps) {
		super(scope, id, props);
		this.userPool = new cognito.UserPool(
			this,
			constructName(props, "user-pool"),
			{
				userPoolName: constructName(props, "user-pool"),
				standardAttributes: {
					email: { required: true, mutable: false },
					givenName: { required: true, mutable: true },
					familyName: { required: true, mutable: true },
					// This is the Concordium Account Address attribute
					preferredUsername: { required: false, mutable: true },
				},
				customAttributes: {
					// This is affiliate concordium account address alias
					affiliateAddress: new cognito.StringAttribute({ mutable: true }),
					kycVerified: new cognito.BooleanAttribute({ mutable: true }),
				},
				autoVerify: { email: true },
				accountRecovery: cognito.AccountRecovery.EMAIL_ONLY,
				selfSignUpEnabled: true,
				signInAliases: { email: true },
				email: cognito.UserPoolEmail.withCognito(), // TODO: Implement email configuration with SES
				// userInvitation: {}, // TODO: Implement user invitation
				// userVerification: {}, //TODO: Implement user verification,
				removalPolicy:
					props.organization_env === OrganizationEnv.PROD
						? cdk.RemovalPolicy.RETAIN
						: cdk.RemovalPolicy.DESTROY,
			},
		);
		cdk.Tags.of(this.userPool).add("organization", props.organization);
		cdk.Tags.of(this.userPool).add("environment", props.organization_env);
		this.adminRole = new iam.Role(
			this,
			`${props.organization}-${props.organization_env}-role-admin`,
			{
				roleName: `${props.organization}-${props.organization_env}-role-admin`,
				description:
					"Role which will be assumed by users in cognito admin group",
				assumedBy: new iam.FederatedPrincipal(
					"cognito-identity.amazonaws.com",
					{
						StringEquals: {
							"cognito-identity.amazonaws.com:aud": this.userPool.userPoolId,
						},
						"ForAnyValue:StringLike": {
							"cognito-identity.amazonaws.com:amr": "authenticated",
						},
					},
				),
				inlinePolicies: {
					ClientSideAccess: new iam.PolicyDocument({
						statements: [
							new iam.PolicyStatement({
								actions: [
									"cognito-idp:AdminGetUser",
									"cognito-idp:ListUsers",
									"cognito-idp:ListUsersInGroup",
									"cognito-idp:DeleteUser",
									"cognito-idp:AdminEnableUser",
									"cognito-idp:AdminDisableUser",
								],
								resources: [this.userPool.userPoolArn],
							}),
						],
					}),
				},
			},
		);
		cdk.Tags.of(this.adminRole).add("organization", props.organization);
		cdk.Tags.of(this.adminRole).add("environment", props.organization_env);
		this.adminGroup = new cognito.CfnUserPoolGroup(
			this,
			`${props.organization}-${props.organization_env}-group-admin`,
			{
				userPoolId: this.userPool.userPoolId,
				description: "Admin group",
				groupName: "admin",
			},
		);
		cdk.Tags.of(this.adminGroup).add("organization", props.organization);
		cdk.Tags.of(this.adminGroup).add("environment", props.organization_env);
		this.client = new cognito.UserPoolClient(
			this,
			`${props.organization}-${props.organization_env}-user-pool-client`,
			{
				userPool: this.userPool,
				userPoolClientName: `${props.organization}-${props.organization_env}-user-pool-client`,
				authFlows: {
					userPassword: true,
					userSrp: true,
				},
				readAttributes: new cognito.ClientAttributes().withStandardAttributes({
					email: true,
					givenName: true,
					familyName: true,
					preferredUsername: true,
					emailVerified: true,
				}),
				writeAttributes: new cognito.ClientAttributes().withStandardAttributes({
					email: true,
					givenName: true,
					familyName: true,
				}),
			},
		);
		cdk.Tags.of(this.client).add("organization", props.organization);
		cdk.Tags.of(this.client).add("environment", props.organization_env);
	}
}
