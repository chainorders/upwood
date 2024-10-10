import { Stack } from "aws-cdk-lib";
import { Construct } from "constructs";
import { constructName, OrganizationEnv, StackProps as SP } from "./shared";
import { IVpc, Vpc } from "aws-cdk-lib/aws-ec2";

interface StackProps extends SP {}

export class NetworkStack extends Stack {
	vpc: IVpc;
	constructor(scope: Construct, id: string, props: StackProps) {
		super(scope, id, props);
		this.vpc = Vpc.fromLookup(this, "Vpc", {
			region: props.env!.region,
			isDefault: true,
		});
	}
}
