import { aws_apigatewayv2, RemovalPolicy, Stack, Tags } from "aws-cdk-lib";
import {
	CorsHttpMethod,
	DomainName,
	HttpApi,
	IVpcLink,
} from "aws-cdk-lib/aws-apigatewayv2";
import { HttpServiceDiscoveryIntegration } from "aws-cdk-lib/aws-apigatewayv2-integrations";
import { Certificate } from "aws-cdk-lib/aws-certificatemanager";
import { IVpc } from "aws-cdk-lib/aws-ec2";
import { DockerImageAsset } from "aws-cdk-lib/aws-ecr-assets";
import * as ecs from "aws-cdk-lib/aws-ecs";
import { PolicyDocument, PolicyStatement } from "aws-cdk-lib/aws-iam";
import { ILogGroup } from "aws-cdk-lib/aws-logs";
import { IBucket } from "aws-cdk-lib/aws-s3";
import { INamespace } from "aws-cdk-lib/aws-servicediscovery";
import { Construct } from "constructs";
import * as ses from "aws-cdk-lib/aws-ses";
import * as cognito from "aws-cdk-lib/aws-cognito";

import { constructName, OrganizationEnv, StackProps as SP } from "./shared";

interface StackProps extends SP {
	certificateArn: string;
	filesBucket: IBucket;
	domainName: string;
	vpcLink: IVpcLink;
	discoveryNamespace: INamespace;
	memoryReservationSoftMiB: number | undefined;
	// Api Container / Execution env params
	containerCount: number | undefined;
	vpc: IVpc;
	cluster: ecs.ICluster;
	logGroup: ILogGroup;
	// Api Listener Params
	apiSocketPort: number;
	environment: Record<string, string>;
	secrets: Record<string, ecs.Secret>;
	emailIdentity: ses.IEmailIdentity;
	cognito: cognito.IUserPool;
}

export class BackendApiStack extends Stack {
	service: ecs.Ec2Service;
	apiGateway: aws_apigatewayv2.HttpApi;

	constructor(scope: Construct, id: string, props: StackProps) {
		super(scope, id, props);
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
		props.filesBucket.grantWrite(taskDef.taskRole);
		props.filesBucket.grantRead(taskDef.taskRole);
		props.filesBucket.grantDelete(taskDef.taskRole);
		props.emailIdentity.grantSendEmail(taskDef.taskRole);
		props.cognito.grant(
			taskDef.taskRole,
			"cognito-idp:AdminCreateUser",
			"cognito-idp:AdminDeleteUser",
			"cognito-idp:AdminDisableUser",
			"cognito-idp:AdminEnableUser",
			"cognito-idp:AdminGetUser",
			"cognito-idp:AdminSetUserPassword",
			"cognito-idp:AdminUpdateUserAttributes",
			"cognito-idp:AdminResetUserPassword",
			"cognito-idp:ListUsers",
		);
		let apiContainer = taskDef.addContainer("backend-api", {
			image: ecs.ContainerImage.fromDockerImageAsset(containerImage),
			secrets: props.secrets,
			environmentFiles: [],
			environment: props.environment,
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
				desiredCount: props.containerCount,
				deploymentController: {
					type: ecs.DeploymentControllerType.ECS,
				},
				cloudMapOptions: {
					cloudMapNamespace: props.discoveryNamespace,
					container: apiContainer,
					containerPort: props.apiSocketPort,
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
