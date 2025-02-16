import {
	aws_apigatewayv2,
	RemovalPolicy,
	SecretValue,
	Stack,
	Tags,
} from "aws-cdk-lib";
import { Construct } from "constructs";
import { constructName, OrganizationEnv, StackProps as SP } from "./shared";
import * as ecs from "aws-cdk-lib/aws-ecs";
import * as ssm from "aws-cdk-lib/aws-ssm";
import { DockerImageAsset } from "aws-cdk-lib/aws-ecr-assets";
import { IVpc } from "aws-cdk-lib/aws-ec2";
import {
	CorsHttpMethod,
	DomainName,
	HttpApi,
	IVpcLink,
} from "aws-cdk-lib/aws-apigatewayv2";
import { HttpServiceDiscoveryIntegration } from "aws-cdk-lib/aws-apigatewayv2-integrations";
import { INamespace } from "aws-cdk-lib/aws-servicediscovery";
import { ILogGroup } from "aws-cdk-lib/aws-logs";
import { IParameter } from "aws-cdk-lib/aws-ssm";
import { PolicyDocument, PolicyStatement } from "aws-cdk-lib/aws-iam";
import { IBucket } from "aws-cdk-lib/aws-s3";
import { Certificate } from "aws-cdk-lib/aws-certificatemanager";

interface StackProps extends SP {
	certificateArn: string;
	userPoolArn: string;
	filesBucket: IBucket;
	domainName: string;
	vpcLink: IVpcLink;
	discoveryNamespace: INamespace;
	userPoolRegion: string;
	memoryReservationSoftMiB: number | undefined;
	// Api Container / Execution env params
	containerCount: number | undefined;
	vpc: IVpc;
	cluster: ecs.ICluster;
	logGroup: ILogGroup;
	// Api Listener Params
	apiSocketAddress: string;
	apiSocketPort: number;
	// Database Params
	postgresUserParameter: IParameter;
	postgresPasswordParameter: IParameter;
	postgresPort: number;
	postgresHostname: string;
	postgresDb: string;
	dbPoolMaxSize: number;
	// User Authentication Params
	userPoolId: string;
	userPoolClientId: string;
	/// Tree NFT Agent Wallet Json String
	treeNftAgentWalletJsonStr: SecretValue;
	// Concordium chain Params
	concordiumNodeUri: string;
	concordiumNetwork: string;
	filebaseAccessKeyId: SecretValue;
	filebaseSecretAccessKey: SecretValue;
}

export class BackendApiStack extends Stack {
	service: ecs.Ec2Service;
	apiGateway: aws_apigatewayv2.HttpApi;

	constructor(scope: Construct, id: string, props: StackProps) {
		super(scope, id, props);
		let treeNftAgentWalletJsonStrParameter = new ssm.StringParameter(
			this,
			constructName(props, "tree-nft-agent-wallet-json-str-parameter"),
			{
				parameterName: `/${props.organization}/${props.organization_env}/wallet/tree-nft-agent-wallet-json`,
				stringValue: props.treeNftAgentWalletJsonStr.unsafeUnwrap(),
				description: `Tree NFT Agent Wallet Json String`,
			},
		);
		let offchainRewardsAgentWalletJsonStrParameter = new ssm.StringParameter(
			this,
			constructName(props, "offchain-rewards-agent-wallet-json-str-parameter"),
			{
				parameterName: `/${props.organization}/${props.organization_env}/wallet/offchain-rewards-agent-wallet-json`,
				stringValue: props.treeNftAgentWalletJsonStr.unsafeUnwrap(),
				description: `Offchain Rewards Agent Wallet Json String`,
			},
		);
		let filebaseAccessKeyIdParameter = new ssm.StringParameter(
			this,
			constructName(props, "filebase-access-key-id-parameter"),
			{
				parameterName: `/${props.organization}/${props.organization_env}/filebase/access-key-id`,
				stringValue: props.filebaseAccessKeyId.unsafeUnwrap(),
				description: `Filebase Access Key ID`,
			},
		);
		let filebaseSecretAccessKeyParameter = new ssm.StringParameter(
			this,
			constructName(props, "filebase-secret-access-key-parameter"),
			{
				parameterName: `/${props.organization}/${props.organization_env}/filebase/secret-access-key`,
				stringValue: props.filebaseSecretAccessKey.unsafeUnwrap(),
				description: `Filebase Secret Access Key`,
			},
		);

		let containerImage = new DockerImageAsset(
			this,
			constructName(props, "backend-api-image"),
			{
				directory: "../",
				file: "backend.Dockerfile",
				assetName: "backend-api",
			},
		);
		const fileBucketAccessPolicy: PolicyDocument = new PolicyDocument();
		fileBucketAccessPolicy.addStatements(
			new PolicyStatement({
				actions: ["s3:*"],
				resources: [props.filesBucket.bucketArn + "/*"],
			}),
		);
		const taskDef = new ecs.Ec2TaskDefinition(
			this,
			constructName(props, "backend-api-task-definition"),
			{
				family: constructName(props, "backend-api-task-definition"),
			},
		);
		taskDef.applyRemovalPolicy(
			props.organization_env === OrganizationEnv.PROD
				? RemovalPolicy.RETAIN
				: RemovalPolicy.DESTROY,
		);
		taskDef.addToTaskRolePolicy(
			new PolicyStatement({
				actions: ["s3:*"],
				resources: [props.filesBucket.bucketArn + "/*"],
			}),
		);
		taskDef.addToTaskRolePolicy(
			new PolicyStatement({
				actions: [
					"cognito-idp:AdminCreateUser",
					"cognito-idp:AdminDeleteUser",
					"cognito-idp:AdminDisableUser",
					"cognito-idp:AdminEnableUser",
					"cognito-idp:AdminGetUser",
					"cognito-idp:AdminSetUserPassword",
					"cognito-idp:AdminUpdateUserAttributes",
					"cognito-idp:AdminResetUserPassword",
					"cognito-idp:ListUsers",
				],
				resources: [props.userPoolArn],
			}),
		);
		let apiContainer = taskDef.addContainer("backend-api", {
			image: ecs.ContainerImage.fromDockerImageAsset(containerImage),
			secrets: {
				POSTGRES_PASSWORD: ecs.Secret.fromSsmParameter(
					props.postgresPasswordParameter,
				),
				POSTGRES_USER: ecs.Secret.fromSsmParameter(props.postgresUserParameter),
				TREE_NFT_AGENT_WALLET_JSON_STR: ecs.Secret.fromSsmParameter(
					treeNftAgentWalletJsonStrParameter,
				),
				OFFCHAIN_REWARDS_AGENT_WALLET_JSON_STR: ecs.Secret.fromSsmParameter(
					offchainRewardsAgentWalletJsonStrParameter,
				),
				FILEBASE_ACCESS_KEY_ID: ecs.Secret.fromSsmParameter(
					filebaseAccessKeyIdParameter,
				),
				FILEBASE_SECRET_ACCESS_KEY: ecs.Secret.fromSsmParameter(
					filebaseSecretAccessKeyParameter,
				),
			},
			environmentFiles: [],
			environment: {
				RUST_LOG: "info",
				AWS_REGION: String(props.env!.region!),
				API_SOCKET_PORT: String(props.apiSocketPort),
				API_SOCKET_ADDRESS: String(props.apiSocketAddress),
				POSTGRES_DB: props.postgresDb,
				POSTGRES_HOST: props.postgresHostname,
				POSTGRES_PORT: String(props.postgresPort),
				DB_POOL_MAX_SIZE: String(props.dbPoolMaxSize),
				AWS_USER_POOL_ID: props.userPoolId,
				AWS_USER_POOL_CLIENT_ID: props.userPoolClientId,
				AWS_USER_POOL_REGION: props.userPoolRegion,
				CONCORDIUM_NETWORK: props.concordiumNetwork,
				CONCORDIUM_NODE_URI: props.concordiumNodeUri,
				EURO_E_CONTRACT_INDEX: "10589",
				IDENTITY_REGISTRY_CONTRACT_INDEX: "10590",
				COMPLIANCE_CONTRACT_INDEX: "10592",
				CARBON_CREDIT_CONTRACT_INDEX: "10596",
				TREE_FT_CONTRACT_INDEX: "10597",
				TREE_NFT_CONTRACT_INDEX: "10598",
				MINT_FUNDS_CONTRACT_INDEX: "10593",
				TRADING_CONTRACT_INDEX: "10594",
				YIELDER_CONTRACT_INDEX: "10595",
				OFFCHAIN_REWARDS_CONTRACT_INDEX: "10599",
				AFFILIATE_COMMISSION: "5",
				FILES_BUCKET_NAME: props.filesBucket.bucketName,
				FILES_PRESIGNED_URL_EXPIRY_SECS: "20000",
				FILEBASE_BUCKET_NAME: "upwood-dev-files",
				FILEBASE_S3_ENDPOINT_URL: "https://s3.filebase.com",
				ID_STATEMENT: "[]",
			},
			logging: new ecs.AwsLogDriver({
				streamPrefix: "api",
				logGroup: props.logGroup,
				mode: ecs.AwsLogDriverMode.NON_BLOCKING,
			}),
			containerName: "backend-api",
			dockerLabels: {
				organization: props.organization,
				environment: props.organization_env,
				service: "backend/api",
			},
			entryPoint: ["/upwood_api_server"],
			portMappings: [
				{
					containerPort: props.apiSocketPort,
				},
			],
			memoryReservationMiB: props.memoryReservationSoftMiB,
		});
		Tags.of(taskDef).add("organization", props.organization);
		Tags.of(taskDef).add("environment", props.organization_env);
		Tags.of(taskDef).add("service", "backend/api");

		const service = new ecs.Ec2Service(
			this,
			constructName(props, "backend-api-service"),
			{
				taskDefinition: taskDef,
				cluster: props.cluster,
				serviceName: constructName(props, "backend-api-service"),
				desiredCount: props.containerCount,
				deploymentController: {
					type: ecs.DeploymentControllerType.ECS,
				},
				cloudMapOptions: {
					cloudMapNamespace: props.discoveryNamespace,
					container: apiContainer,
					containerPort: props.apiSocketPort,
					name: constructName(props, "backend-api-discovery-service"),
				},
			},
		);
		service.applyRemovalPolicy(
			props.organization_env === OrganizationEnv.PROD
				? RemovalPolicy.RETAIN
				: RemovalPolicy.DESTROY,
		);
		Tags.of(service).add("organization", props.organization);
		Tags.of(service).add("environment", props.organization_env);
		Tags.of(service).add("service", "backend/api");

		const domainName = new DomainName(
			this,
			constructName(props, "backend-api-domain-name"),
			{
				domainName: props.domainName,
				certificate: Certificate.fromCertificateArn(
					this,
					constructName(props, "backend-api-certificate"),
					props.certificateArn,
				),
			},
		);
		Tags.of(domainName).add("organization", props.organization);
		Tags.of(domainName).add("environment", props.organization_env);
		Tags.of(domainName).add("service", "backend/api");

		const apiGateway = new HttpApi(
			this,
			constructName(props, "backend-api-gateway"),
			{
				apiName: constructName(props, "backend-api-gateway"),
				description: "Backend API proxy",
				corsPreflight: {
					allowOrigins: ["*"],
					allowMethods: [CorsHttpMethod.ANY],
					allowHeaders: ["*"],
				},
				defaultIntegration: new HttpServiceDiscoveryIntegration(
					"backend-api-gateway-integration",
					service.cloudMapService!,
					{
						vpcLink: props.vpcLink,
					},
				),
				defaultDomainMapping: {
					domainName,
				},
			},
		);
		apiGateway.applyRemovalPolicy(
			props.organization_env === OrganizationEnv.PROD
				? RemovalPolicy.RETAIN
				: RemovalPolicy.DESTROY,
		);
		Tags.of(apiGateway).add("organization", props.organization);
		Tags.of(apiGateway).add("environment", props.organization_env);
		Tags.of(apiGateway).add("service", "backend/api");

		this.apiGateway = apiGateway;
		this.service = service;
	}
}
