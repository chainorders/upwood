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
	HttpApi,
	IVpcLink,
} from "aws-cdk-lib/aws-apigatewayv2";
import { HttpServiceDiscoveryIntegration } from "aws-cdk-lib/aws-apigatewayv2-integrations";
import { INamespace } from "aws-cdk-lib/aws-servicediscovery";
import { ILogGroup } from "aws-cdk-lib/aws-logs";
import { IParameter } from "aws-cdk-lib/aws-ssm";
import { Role, ServicePrincipal } from "aws-cdk-lib/aws-iam";

interface StackProps extends SP {
	vpcLink: IVpcLink;
	discoveryNamespace: INamespace;
	awsUserPoolRegion: string;
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
	awsUserPoolId: string;
	awsUserPoolClientId: string;
	userChallengeExpiryDurationMins: number;
	/// Tree NFT Agent Wallet Json String
	treeNftAgentWalletJsonStr: SecretValue;
	// Concordium chain Params
	concordiumNodeUri: string;
	concordiumNetwork: string;
}

export class BackendApiStack extends Stack {
	service: ecs.Ec2Service;
	apiGateway: aws_apigatewayv2.HttpApi;

	constructor(scope: Construct, id: string, props: StackProps) {
		super(scope, id, props);
		let treeNftAgentWalletJsonStrParameter = new ssm.StringParameter(
			this,
			constructName(props, "rds-password"),
			{
				parameterName: `/${props.organization}/${props.organization_env}/wallet/treent-agent-wallet-json`,
				stringValue: props.treeNftAgentWalletJsonStr.unsafeUnwrap(),
				description: `Tree NFT Agent Wallet Json String`,
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
		const taskDef = new ecs.Ec2TaskDefinition(
			this,
			constructName(props, "backend-api-task-definition"),
			{
				family: constructName(props, "backend-api-task-definition"),
				taskRole: new Role(
					this,
					constructName(props, "backend-api-task-role"),
					{
						assumedBy: new ServicePrincipal("ecs-tasks.amazonaws.com"),
						roleName: constructName(props, "backend-api-task-role"),
						description: "Task Role for backend-api",
					},
				),
			},
		);
		taskDef.applyRemovalPolicy(
			props.organization_env === OrganizationEnv.PROD
				? RemovalPolicy.RETAIN
				: RemovalPolicy.DESTROY,
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
			},
			environment: {
				RUST_LOG: "info",
				AWS_REGION: String(props.env!.region!),
				API_SOCKET_PORT: String(props.apiSocketPort),
				API_SOCKET_ADDRESS: String(props.apiSocketAddress),
				POSTGRES_DB: props.postgresDb,
				POSTGRES_HOST: props.postgresHostname,
				POSTGRES_PORT: String(props.postgresPort),
				DB_POOL_MAX_SIZE: String(props.dbPoolMaxSize),
				AWS_USER_POOL_ID: props.awsUserPoolId,
				AWS_USER_POOL_CLIENT_ID: props.awsUserPoolClientId,
				AWS_USER_POOL_REGION: props.awsUserPoolRegion,
				USER_CHALLENGE_EXPIRY_DURATION_MINS: String(
					props.userChallengeExpiryDurationMins,
				),
				CONCORDIUM_NETWORK: props.concordiumNetwork,
				CONCORDIUM_NODE_URI: props.concordiumNodeUri,
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
