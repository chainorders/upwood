import * as cdk from "aws-cdk-lib";
import { Construct } from "constructs";
import { constructName, OrganizationEnv, StackProps as SP } from "./shared";
import { DatabaseInstance, DatabaseInstanceEngine, PostgresEngineVersion } from "aws-cdk-lib/aws-rds";
import { InstanceClass, InstanceSize, InstanceType, SecurityGroup, SubnetType } from "aws-cdk-lib/aws-ec2";
import { Duration } from "aws-cdk-lib";

export interface StackProps extends SP {
	dbBackupRetentionDays: number;
	vpc: cdk.aws_ec2.Vpc;
	dbPort: number;
	dbName: string;
}

export class RdsStack extends cdk.Stack {
	db: DatabaseInstance;

	constructor(scope: Construct, id: string, props: StackProps) {
		super(scope, id, props);
		const engine = DatabaseInstanceEngine.postgres({ version: PostgresEngineVersion.VER_16_4 });
		const instanceType = InstanceType.of(InstanceClass.T4G, InstanceSize.MICRO);
		const dbSg = new SecurityGroup(this, constructName(props, "rds-sg"), {
			securityGroupName: constructName(props, "rds-sg"),
			vpc: props.vpc,
		});
		this.db = new DatabaseInstance(this, constructName(props, "rds"), {
            databaseName: props.dbName,
            port: props.dbPort,
            copyTagsToSnapshot: true,
            storageEncrypted: props.organization_env === OrganizationEnv.PROD,
			engine: engine,
			vpc: props.vpc,
			vpcSubnets: { subnetType: SubnetType.PRIVATE_ISOLATED },
			instanceType: instanceType,
			securityGroups: [dbSg],
			backupRetention:
				props.organization_env === OrganizationEnv.PROD ? Duration.days(props.dbBackupRetentionDays) : Duration.days(0), // disable automatic DB snapshot retention
			deleteAutomatedBackups: props.organization_env === OrganizationEnv.PROD,
			removalPolicy:
				props.organization_env === OrganizationEnv.PROD ? cdk.RemovalPolicy.RETAIN : cdk.RemovalPolicy.DESTROY,
        });
	}
}
