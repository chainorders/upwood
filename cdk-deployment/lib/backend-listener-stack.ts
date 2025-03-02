import { Stack, Tags } from "aws-cdk-lib";
import { IVpc } from "aws-cdk-lib/aws-ec2";
import { DockerImageAsset } from "aws-cdk-lib/aws-ecr-assets";
import * as ecs from "aws-cdk-lib/aws-ecs";
import { Ec2Service } from "aws-cdk-lib/aws-ecs";
import { ILogGroup } from "aws-cdk-lib/aws-logs";
import * as ssm from "aws-cdk-lib/aws-ssm";
import { Construct } from "constructs";

import { constructName, StackProps as SP } from "./shared";

interface StackProps extends SP {
	logGroup: ILogGroup;
	cluster: ecs.ICluster;
	vpc: IVpc;
	memoryReservationSoftMiB: number | undefined;
	listenerDefaultBlockHeight: number;
	listenerAccountAddress: string;
	concordiumNodeUri: string;
	dbPoolMaxSize: number;
	postgresPort: number;
	postgresHostname: string;
	postgresDb: string;
	postgresUserParameter: ssm.IParameter;
	postgresPasswordParameter: ssm.IParameter;
}

export class BackendListenerStack extends Stack {
	service: Ec2Service;

	constructor(scope: Construct, id: string, props: StackProps) {
		super(scope, id, props);

		let containerImage = new DockerImageAsset(
			this,
			constructName(props, "backend-listener-image"),
			{
				directory: "../",
				file: "backend.Dockerfile",
				assetName: "backend-listener",
			},
		);
		const listenerTaskDefinition = new ecs.Ec2TaskDefinition(
			this,
			constructName(props, "backend-listener-task-definition"),
			{
				family: constructName(props, "backend-listener-task-definition"),
			},
		);
		listenerTaskDefinition.addContainer("backend-listener", {
			image: ecs.ContainerImage.fromDockerImageAsset(containerImage),
			secrets: {
				POSTGRES_PASSWORD: ecs.Secret.fromSsmParameter(
					props.postgresPasswordParameter,
				),
				POSTGRES_USER: ecs.Secret.fromSsmParameter(props.postgresUserParameter),
			},
			environment: {
				RUST_LOG: "info",
				AWS_REGION: String(props.env!.region!),
				POSTGRES_DB: props.postgresDb,
				POSTGRES_HOST: props.postgresHostname,
				POSTGRES_PORT: String(props.postgresPort),
				DB_POOL_MAX_SIZE: String(props.dbPoolMaxSize),
				CONCORDIUM_NODE_URI: props.concordiumNodeUri,
				NODE_RATE_LIMIT: String(100),
				NODE_RATE_LIMIT_DURATION_MILLIS: String(1000),
				NODE_REQUEST_TIMEOUT_MILLIS: "1000",
				NODE_CONNECT_TIMEOUT_MILLIS: "2000",
				ACCOUNT: props.listenerAccountAddress,
				DEFAULT_BLOCK_HEIGHT: props.listenerDefaultBlockHeight.toString(),
				LISTENER_RETRY_TIMES: "10",
				LISTENER_RETRY_MIN_DELAY_MILLIS: "500",
				LISTENER_RETRY_MAX_DELAY_MILLIS: "10000",
			},
			memoryReservationMiB: props.memoryReservationSoftMiB,
			logging: new ecs.AwsLogDriver({
				streamPrefix: "listener",
				logGroup: props.logGroup,
				mode: ecs.AwsLogDriverMode.NON_BLOCKING,
			}),
			containerName: "listener",
			dockerLabels: {
				organization: props.organization,
				environment: props.organization_env,
				service: "backend/listener",
			},
			entryPoint: ["/listener_server"],
		});
		Tags.of(listenerTaskDefinition).add("organization", props.organization);
		Tags.of(listenerTaskDefinition).add("environment", props.organization_env);
		Tags.of(listenerTaskDefinition).add("service", "backend/listener");

		const service = new ecs.Ec2Service(
			this,
			constructName(props, "backend-listener-service"),
			{
				taskDefinition: listenerTaskDefinition,
				cluster: props.cluster,
				desiredCount: 1,
				deploymentController: {
					type: ecs.DeploymentControllerType.ECS,
				},
			},
		);
		Tags.of(service).add("organization", props.organization);
		Tags.of(service).add("environment", props.organization_env);
		Tags.of(service).add("service", "backend/listener");
		this.service = service;
	}
}
