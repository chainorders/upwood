import * as cdk from "aws-cdk-lib";
import { Construct } from "constructs";
import { constructName, OrganizationEnv, StackProps } from "./shared";
import {
	AccountRecovery,
	BooleanAttribute,
	CfnUserPoolGroup,
	ClientAttributes,
	IUserPool,
	IUserPoolClient,
	StringAttribute,
	UserPool,
	UserPoolClient,
	UserPoolEmail,
} from "aws-cdk-lib/aws-cognito";

export class CognitoStack extends cdk.Stack {
	userPool: IUserPool;
	userPoolClient: IUserPoolClient;

	constructor(scope: Construct, id: string, props: StackProps) {
		super(scope, id, props);
		const userPool = new UserPool(this, constructName(props, "user-pool"), {
			userPoolName: constructName(props, "user-pool"),
			standardAttributes: {
				email: { required: true, mutable: false },
			},
			customAttributes: {
				con_accnt: new StringAttribute({
					mutable: true,
					minLen: 50,
					maxLen: 50,
				}),
			},
			autoVerify: { email: true },
			accountRecovery: AccountRecovery.EMAIL_ONLY,
			selfSignUpEnabled: true,
			signInAliases: { email: true },
			email: UserPoolEmail.withCognito(), // TODO: Implement email configuration with SES
			// userInvitation: {}, // TODO: Implement user invitation
			// userVerification: {}, //TODO: Implement user verification,
			removalPolicy:
				props.organization_env === OrganizationEnv.PROD
					? cdk.RemovalPolicy.RETAIN
					: cdk.RemovalPolicy.DESTROY,
		});
		cdk.Tags.of(userPool).add("organization", props.organization);
		cdk.Tags.of(userPool).add("environment", props.organization_env);

		const adminGroup = new CfnUserPoolGroup(
			this,
			constructName(props, "admin-group"),
			{
				userPoolId: userPool.userPoolId,
				description: "Admin group",
				groupName: "admin",
			},
		);
		cdk.Tags.of(adminGroup).add("organization", props.organization);
		cdk.Tags.of(adminGroup).add("environment", props.organization_env);

		const client = new UserPoolClient(
			this,
			constructName(props, "user-pool-client"),
			{
				userPool: userPool,
				userPoolClientName: constructName(props, "user-pool-client"),
				authFlows: {
					userPassword: true,
					userSrp: true,
				},
				readAttributes: new ClientAttributes()
					.withStandardAttributes({
						email: true,
						emailVerified: true,
					})
					.withCustomAttributes("con_accnt"),
			},
		);
		cdk.Tags.of(client).add("organization", props.organization);
		cdk.Tags.of(client).add("environment", props.organization_env);

		this.userPool = userPool;
		this.userPoolClient = client;
	}
}
