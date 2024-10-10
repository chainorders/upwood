import { Construct } from "constructs";
import { constructName, OrganizationEnv, StackProps as SP } from "./shared";
import { DatabaseInstance, DatabaseInstanceEngine, PostgresEngineVersion } from "aws-cdk-lib/aws-rds";
import { InstanceClass, InstanceSize, InstanceType, IVpc, Peer, SecurityGroup, SubnetType } from "aws-cdk-lib/aws-ec2";
import { aws_ec2, Duration, RemovalPolicy, SecretValue, Stack, Tags } from "aws-cdk-lib";
import { ParameterDataType, StringParameter } from "aws-cdk-lib/aws-ssm";

export interface StackProps extends SP {
	dbStorageGiB: number;
	dbInstanceSize: InstanceSize;
	dbInstanceClass: InstanceClass;
	dbEngineVersion: PostgresEngineVersion;
	dbUsername: string;
	dbPassword: SecretValue;
	dbBackupRetentionDays: number;
	vpc: IVpc;
	dbPort: number;
	dbName: string;
}

export class RdsStack extends Stack {
	db: DatabaseInstance;
	dbEndpointParam: StringParameter;
	dbPortParam: StringParameter;
	dbUsernameParam: StringParameter;
	dbPasswordParam: StringParameter;

	constructor(scope: Construct, id: string, props: StackProps) {
		super(scope, id, props);
		const engine = DatabaseInstanceEngine.postgres({ version: props.dbEngineVersion });
		const instanceType = InstanceType.of(props.dbInstanceClass, props.dbInstanceSize);
		const dbSg = new SecurityGroup(this, constructName(props, "rds-sg"), {
			securityGroupName: constructName(props, "rds-sg"),
			vpc: props.vpc,
			description: "This security group allows access to the RDS instance from within the VPC",
		});
		dbSg.addIngressRule(
			Peer.ipv4(props.vpc.vpcCidrBlock),
			aws_ec2.Port.tcp(props.dbPort),
			`Allow port  ${props.dbPort} from VPC ${props.vpc.vpcId}`
		);
		Tags.of(dbSg).add("organization", props.organization);
		Tags.of(dbSg).add("environment", props.organization_env);

		const dbSgPublic = new SecurityGroup(this, constructName(props, "rds-sg-public"), {
			securityGroupName: constructName(props, "rds-sg-public"),
			vpc: props.vpc,
			description: "This security group allows access to the RDS instance from the internet",
		});
		dbSgPublic.addIngressRule(
			Peer.ipv4("0.0.0.0/0"),
			aws_ec2.Port.tcp(props.dbPort),
			`Allow port  ${props.dbPort} from the internet`
		);
		Tags.of(dbSgPublic).add("organization", props.organization);
		Tags.of(dbSgPublic).add("environment", props.organization_env);

		// in dev env, we allow access to the RDS instance from the internet
		let securityGroups = [dbSg];
		if (props.organization_env === OrganizationEnv.DEV) {
			securityGroups.push(dbSgPublic);
		}

		this.db = new DatabaseInstance(this, constructName(props, "rds"), {
			allocatedStorage: props.dbStorageGiB,
			databaseName: props.dbName,
			port: props.dbPort,
			copyTagsToSnapshot: true,
			storageEncrypted: false,
			credentials: {
				username: props.dbUsername,
				password: props.dbPassword,
			},
			engine: engine,
			vpc: props.vpc,
			// The database is kept in public subnet to allow tools like pgAdmin to connect to it
			vpcSubnets: { subnetType: SubnetType.PUBLIC },
			instanceType: instanceType,
			securityGroups,
			backupRetention:
				props.organization_env === OrganizationEnv.PROD ? Duration.days(props.dbBackupRetentionDays) : Duration.days(0),
			deleteAutomatedBackups: !(props.organization_env === OrganizationEnv.PROD),
			// If the stack is removed then retain the database if the env is `PROD` or destroy it if the env is `DEV`
			removalPolicy: props.organization_env === OrganizationEnv.PROD ? RemovalPolicy.RETAIN : RemovalPolicy.DESTROY,
		});
		Tags.of(this.db).add("organization", props.organization);
		Tags.of(this.db).add("environment", props.organization_env);

		this.dbEndpointParam = new StringParameter(this, constructName(props, "rds-endpoint"), {
			parameterName: constructName(props, "rds-endpoint"),
			stringValue: this.db.dbInstanceEndpointAddress,
			description: `The endpoint of the RDS instance`,
		});
		Tags.of(this.dbEndpointParam).add("organization", props.organization);
		Tags.of(this.dbEndpointParam).add("environment", props.organization_env);
		Tags.of(this.dbEndpointParam).add("service", "rds");

		this.dbPortParam = new StringParameter(this, constructName(props, "rds-port"), {
			parameterName: constructName(props, "rds-port"),
			stringValue: this.db.dbInstanceEndpointPort,
			description: `The port of the RDS instance`,
		});
		Tags.of(this.dbPortParam).add("organization", props.organization);
		Tags.of(this.dbPortParam).add("environment", props.organization_env);
		Tags.of(this.dbPortParam).add("service", "rds");

		this.dbUsernameParam = new StringParameter(this, constructName(props, "rds-username"), {
			parameterName: constructName(props, "rds-username"),
			stringValue: props.dbUsername,
			description: `The username of the RDS instance`,
		});
		Tags.of(this.dbUsernameParam).add("organization", props.organization);
		Tags.of(this.dbUsernameParam).add("environment", props.organization_env);
		Tags.of(this.dbUsernameParam).add("service", "rds");

		this.dbPasswordParam = new StringParameter(this, constructName(props, "rds-password"), {
			parameterName: constructName(props, "rds-password"),
			stringValue: props.dbPassword.unsafeUnwrap(),
			description: `The password of the RDS instance`,
		});
		Tags.of(this.dbPasswordParam).add("organization", props.organization);
		Tags.of(this.dbPasswordParam).add("environment", props.organization_env);
		Tags.of(this.dbPasswordParam).add("service", "rds");
	}
}
