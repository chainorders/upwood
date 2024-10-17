import { Construct } from "constructs";
import { constructName, OrganizationEnv, StackProps as SP } from "./shared";
import {
	InstanceClass,
	InstanceSize,
	InstanceType,
	IVpc,
	Peer,
	Port,
	SecurityGroup,
	SubnetType,
	Vpc,
} from "aws-cdk-lib/aws-ec2";
import { Duration, RemovalPolicy, SecretValue, Stack, Tags } from "aws-cdk-lib";
import {
	PostgresEngineVersion,
	DatabaseInstance,
	DatabaseInstanceEngine,
	IDatabaseInstance,
} from "aws-cdk-lib/aws-rds";
import { StringParameter } from "aws-cdk-lib/aws-ssm";
import { Cluster, Ec2Service } from "aws-cdk-lib/aws-ecs";
import { LogGroup, RetentionDays } from "aws-cdk-lib/aws-logs";
import { IVpcLink, VpcLink } from "aws-cdk-lib/aws-apigatewayv2";
import {
	IPrivateDnsNamespace,
	PrivateDnsNamespace,
} from "aws-cdk-lib/aws-servicediscovery";
import { Monitoring } from "aws-cdk-lib/aws-autoscaling";

export interface StackProps extends SP {
	dbStorageGiB: number;
	dbInstanceSize: InstanceSize;
	dbInstanceClass: InstanceClass;
	dbEngineVersion: PostgresEngineVersion;
	dbUsername: string;
	dbPassword: SecretValue;
	dbBackupRetentionDays: number;
	dbPort: number;
	dbName: string;
	logsRetentionDays: RetentionDays;
	backendInstanceSize: InstanceSize;
	backendInstanceClass: InstanceClass;
}

export class InfraStack extends Stack {
	dbUsernameParam: StringParameter;
	dbPasswordParam: StringParameter;
	cluster: Cluster;
	listenerService: Ec2Service;
	logGroup: LogGroup;
	vpc: IVpc;
	vpcLink: IVpcLink;
	discoveryNamespace: IPrivateDnsNamespace;
	dbInstance: IDatabaseInstance;

	constructor(scope: Construct, id: string, props: StackProps) {
		super(scope, id, props);
		const vpc = Vpc.fromLookup(this, "Vpc", {
			region: props.env!.region,
			isDefault: true,
		});
		const dbSg = new SecurityGroup(this, constructName(props, "rds-sg"), {
			securityGroupName: constructName(props, "rds-sg"),
			vpc: vpc,
			description:
				"This security group allows access to the RDS instance from within the VPC",
		});
		dbSg.addIngressRule(
			Peer.ipv4(vpc.vpcCidrBlock),
			Port.tcp(props.dbPort),
			`Allow port  ${props.dbPort} from VPC ${vpc.vpcId}`,
		);
		Tags.of(dbSg).add("organization", props.organization);
		Tags.of(dbSg).add("environment", props.organization_env);

		const dbSgPublic = new SecurityGroup(
			this,
			constructName(props, "rds-sg-public"),
			{
				securityGroupName: constructName(props, "rds-sg-public"),
				vpc: vpc,
				description:
					"This security group allows access to the RDS instance from the internet",
			},
		);
		dbSgPublic.addIngressRule(
			Peer.ipv4("0.0.0.0/0"),
			Port.tcp(props.dbPort),
			`Allow port  ${props.dbPort} from the internet`,
		);
		Tags.of(dbSgPublic).add("organization", props.organization);
		Tags.of(dbSgPublic).add("environment", props.organization_env);

		// in dev env, we allow access to the RDS instance from the internet
		let securityGroups = [dbSg];
		if (props.organization_env === OrganizationEnv.DEV) {
			securityGroups.push(dbSgPublic);
		}

		const dbInstance = new DatabaseInstance(this, constructName(props, "rds"), {
			instanceIdentifier: constructName(props, "rds-instance"),
			allocatedStorage: props.dbStorageGiB,
			databaseName: props.dbName,
			port: props.dbPort,
			copyTagsToSnapshot: true,
			storageEncrypted: false,
			credentials: {
				username: props.dbUsername,
				password: props.dbPassword,
			},
			engine: DatabaseInstanceEngine.postgres({
				version: props.dbEngineVersion,
			}),
			vpc: vpc,
			// The database is kept in public subnet to allow tools like pgAdmin to connect to it
			vpcSubnets: { subnetType: SubnetType.PUBLIC },
			instanceType: InstanceType.of(
				props.dbInstanceClass,
				props.dbInstanceSize,
			),
			securityGroups,
			backupRetention:
				props.organization_env === OrganizationEnv.PROD
					? Duration.days(props.dbBackupRetentionDays)
					: Duration.days(0),
			deleteAutomatedBackups: !(
				props.organization_env === OrganizationEnv.PROD
			),
			// If the stack is removed then retain the database if the env is `PROD` or destroy it if the env is `DEV`
			removalPolicy:
				props.organization_env === OrganizationEnv.PROD
					? RemovalPolicy.RETAIN
					: RemovalPolicy.DESTROY,
		});
		Tags.of(dbInstance).add("organization", props.organization);
		Tags.of(dbInstance).add("environment", props.organization_env);
		Tags.of(dbInstance).add("infra", "rds");

		this.dbUsernameParam = new StringParameter(
			this,
			constructName(props, "rds-username"),
			{
				parameterName: `/${props.organization}/${props.organization_env}/rds/username`,
				stringValue: props.dbUsername,
				description: `The username of the RDS instance`,
			},
		);
		Tags.of(this.dbUsernameParam).add("organization", props.organization);
		Tags.of(this.dbUsernameParam).add("environment", props.organization_env);
		Tags.of(this.dbUsernameParam).add("infra", "rds");

		this.dbPasswordParam = new StringParameter(
			this,
			constructName(props, "rds-password"),
			{
				parameterName: `/${props.organization}/${props.organization_env}/rds/password`,
				stringValue: props.dbPassword.unsafeUnwrap(),
				description: `The password of the RDS instance`,
			},
		);
		Tags.of(this.dbPasswordParam).add("organization", props.organization);
		Tags.of(this.dbPasswordParam).add("environment", props.organization_env);
		Tags.of(this.dbPasswordParam).add("infra", "rds");

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
		this.dbInstance = dbInstance;
	}
}
