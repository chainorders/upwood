import { Stack, Tags } from "aws-cdk-lib";
import { Construct } from "constructs";
import { constructName, StackProps as SP } from "./shared";
import * as ecs from "aws-cdk-lib/aws-ecs";
import * as ssm from "aws-cdk-lib/aws-ssm";
import * as logs from "aws-cdk-lib/aws-logs";
import { DockerImageAsset } from "aws-cdk-lib/aws-ecr-assets";
import { IVpc, Port } from "aws-cdk-lib/aws-ec2";

interface StackProps extends SP {
	cluster: ecs.ICluster;
	vpc: IVpc;
	logGroupListener: logs.ILogGroup;
	listenerMemoryReservationSoftMiB: number | undefined;
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

export class BackendStack extends Stack {
	listenerService: ecs.Ec2Service;

	constructor(scope: Construct, id: string, props: StackProps) {
		super(scope, id, props);
		let containerImage = new DockerImageAsset(this, constructName(props, "backend-listener-image"), {
			directory: "../",
			file: "backend.Dockerfile",
			assetName: "backend-listener",
		});
		const listenerTaskDefinition = new ecs.Ec2TaskDefinition(
			this,
			constructName(props, "backend-listener-task-definition"),
			{
				family: constructName(props, "backend-listener-task-definition"),
			}
		);
		listenerTaskDefinition.addContainer("backend-listener", {
			image: ecs.ContainerImage.fromDockerImageAsset(containerImage),
			secrets: {
				POSTGRES_PASSWORD: ecs.Secret.fromSsmParameter(props.postgresPasswordParameter),
				POSTGRES_USER: ecs.Secret.fromSsmParameter(props.postgresUserParameter),
			},
			environment: {
				POSTGRES_DB: props.postgresDb,
				POSTGRES_HOST: props.postgresHostname,
				POSTGRES_PORT: String(props.postgresPort),
				DB_POOL_MAX_SIZE: String(props.dbPoolMaxSize),
				RUST_LOG: "info",
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
			memoryReservationMiB: props.listenerMemoryReservationSoftMiB,
			logging: new ecs.AwsLogDriver({
				streamPrefix: "backend-listener",
				logGroup: props.logGroupListener,
				mode: ecs.AwsLogDriverMode.NON_BLOCKING,
				multilinePattern: "^d{4}-d{2}-d{2}Td{2}:d{2}:d{2}.d{6}Z",
			}),
			containerName: "backend-listener",
			dockerLabels: {
				organization: props.organization,
				environment: props.organization_env,
				service: "backend",
			},
			entryPoint: ["/listener_server"],
		});
		Tags.of(listenerTaskDefinition).add("organization", props.organization);
		Tags.of(listenerTaskDefinition).add("environment", props.organization_env);
		Tags.of(listenerTaskDefinition).add("service", "backend");

		this.listenerService = new ecs.Ec2Service(this, constructName(props, "backend-listener-service"), {
			taskDefinition: listenerTaskDefinition,
			cluster: props.cluster,
			serviceName: constructName(props, "backend-listener-service"),
			desiredCount: 1,
			deploymentController: {
				type: ecs.DeploymentControllerType.ECS,
			}
		});
		Tags.of(this.listenerService).add("organization", props.organization);
		Tags.of(this.listenerService).add("environment", props.organization_env);
		Tags.of(this.listenerService).add("service", "backend");
	}
}
