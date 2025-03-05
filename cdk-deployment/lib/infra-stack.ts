import { RemovalPolicy, Stack, Tags } from "aws-cdk-lib";
import { IVpcLink, VpcLink } from "aws-cdk-lib/aws-apigatewayv2";
import {
	InstanceClass,
	InstanceSize,
	InstanceType,
	IVpc,
	Peer,
	Port,
	SubnetType,
	Vpc,
} from "aws-cdk-lib/aws-ec2";
import { Cluster, Ec2Service } from "aws-cdk-lib/aws-ecs";
import { LogGroup, RetentionDays } from "aws-cdk-lib/aws-logs";
import {
	IPrivateDnsNamespace,
	PrivateDnsNamespace,
} from "aws-cdk-lib/aws-servicediscovery";
import { Construct } from "constructs";

import { constructName, OrganizationEnv, StackProps as SP } from "./shared";

export interface StackProps extends SP {
	logsRetentionDays: RetentionDays;
	backendInstanceSize: InstanceSize;
	backendInstanceClass: InstanceClass;
}

export class InfraStack extends Stack {
	cluster: Cluster;
	listenerService: Ec2Service;
	logGroup: LogGroup;
	vpc: IVpc;
	vpcLink: IVpcLink;
	discoveryNamespace: IPrivateDnsNamespace;

	constructor(scope: Construct, id: string, props: StackProps) {
		super(scope, id, props);
		const vpc = Vpc.fromLookup(this, "Vpc", {
			region: props.env!.region,
			isDefault: true,
		});

		// ECS Cluster setup
		const cluster = new Cluster(this, constructName(props, "backend-cluster"), {
			clusterName: constructName(props, "backend-cluster"),
			vpc: vpc,
			capacity: {
				instanceType: InstanceType.of(
					props.backendInstanceClass,
					props.backendInstanceSize,
				),
				minCapacity: 1,
				maxCapacity: 1,
				// this is needed for the containers to access
				// * Cognito JWKS
				// * Blockchain Nodes
				allowAllOutbound: true,
				// autoScalingGroupName: constructName(props, "backend-asg"),
			},
		});
		cluster.applyRemovalPolicy(RemovalPolicy.DESTROY);
		cluster.connections.allowFrom(
			Peer.ipv4(vpc.vpcCidrBlock),
			Port.allTcp(),
			"Allow inbound traffic from the VPC",
		);
		// cluster.connections.allowTo(
		// 	Peer.ipv4(vpc.vpcCidrBlock),
		// 	Port.allTcp(),
		// 	"Allow outbound traffic to the VPC",
		// );
		Tags.of(cluster).add("organization", props.organization);
		Tags.of(cluster).add("environment", props.organization_env);
		Tags.of(cluster).add("infra", "backend");

		const logGroup = new LogGroup(
			this,
			constructName(props, "listener-log-group"),
			{
				logGroupName: constructName(props, "listener-log-group"),
				retention: props.logsRetentionDays,
				removalPolicy:
					props.organization_env === OrganizationEnv.PROD
						? RemovalPolicy.RETAIN
						: RemovalPolicy.DESTROY,
			},
		);
		Tags.of(logGroup).add("organization", props.organization);
		Tags.of(logGroup).add("environment", props.organization_env);
		Tags.of(logGroup).add("infra", "backend");

		const vpcLink = new VpcLink(this, constructName(props, "vpc-link"), {
			vpc: vpc,
			vpcLinkName: constructName(props, "vpc-link"),
			subnets: { subnetType: SubnetType.PUBLIC },
		});
		vpcLink.applyRemovalPolicy(
			props.organization_env === OrganizationEnv.PROD
				? RemovalPolicy.RETAIN
				: RemovalPolicy.DESTROY,
		);
		Tags.of(vpcLink).add("organization", props.organization);
		Tags.of(vpcLink).add("environment", props.organization_env);

		const namespace = new PrivateDnsNamespace(
			this,
			constructName(props, "discovery-namespace"),
			{
				name: constructName(props, "discovery-namespace"),
				description: "namespace for backend-api in service discovery",
				vpc: vpc,
			},
		);
		namespace.applyRemovalPolicy(
			props.organization_env === OrganizationEnv.PROD
				? RemovalPolicy.RETAIN
				: RemovalPolicy.DESTROY,
		);
		Tags.of(namespace).add("organization", props.organization);
		Tags.of(namespace).add("environment", props.organization_env);

		this.cluster = cluster;
		this.logGroup = logGroup;
		this.vpc = vpc;
		this.vpcLink = vpcLink;
		this.discoveryNamespace = namespace;
	}
}
