import * as cdk from "aws-cdk-lib";
import { Construct } from "constructs";
import { constructName, OrganizationEnv, StackProps } from "./shared";
import {
	AccountRecovery,
	CfnUserPoolGroup,
	ClientAttributes,
	IUserPool,
	IUserPoolClient,
	Mfa,
	StringAttribute,
	UserPool,
	UserPoolClient,
	UserPoolEmail,
} from "aws-cdk-lib/aws-cognito";

export interface CognitoStackProps extends StackProps {
	appDomain: string;
}

export class CognitoStack extends cdk.Stack {
	userPool: IUserPool;
	userPoolClient: IUserPoolClient;

	constructor(scope: Construct, id: string, props: CognitoStackProps) {
		super(scope, id, props);
		const userPool = new UserPool(this, constructName(props, "user-pool"), {
			userPoolName: constructName(props, "user-pool"),
			mfa: Mfa.OPTIONAL,
			standardAttributes: {
				email: { required: true, mutable: true },
				familyName: { required: false, mutable: true },
				givenName: { required: false, mutable: true },
				profilePicture: { required: false, mutable: true },
			},
			customAttributes: {
				affiliate_con_accnt: new StringAttribute({
					mutable: true,
					minLen: 50,
					maxLen: 50,
				}),
				con_accnt: new StringAttribute({
					mutable: true,
					minLen: 50,
					maxLen: 50,
				}),
				nationality: new StringAttribute({
					mutable: true,
				}),
			},
			autoVerify: { email: true },
			accountRecovery: AccountRecovery.EMAIL_ONLY,
			selfSignUpEnabled: true,
			signInAliases: { email: true },
			email: UserPoolEmail.withCognito(), // TODO: Implement email configuration with SES
			userInvitation: {
				emailSubject: "Your Upwood account is ready",
				emailBody: `
					<p>Hi, <span style="font-weight: bold; color: #2a9d8f;">{username}</span>, your account is ready to use.</p>
					<p>Please click / visit this link <a style="color: #264653; text-decoration: none; font-weight: bold;" href="https://${props.appDomain}/login">${props.appDomain}/login</a> and use <span style="font-weight: bold; color: #e76f51;">{####}</span> as the first time password to register your account.</p>
				`,
			},
			// userVerification: {},
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
						familyName: true,
						givenName: true,
					})
					.withCustomAttributes("con_accnt")
					.withCustomAttributes("nationality"),
			},
		);
		cdk.Tags.of(client).add("organization", props.organization);
		cdk.Tags.of(client).add("environment", props.organization_env);

		this.userPool = userPool;
		this.userPoolClient = client;
	}
}
