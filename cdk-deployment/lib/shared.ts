import * as cdk from "aws-cdk-lib";

export enum OrganizationEnv {
	DEV = "dev",
	PROD = "prod",
}
export interface StackProps extends cdk.StackProps {
	organization: string;
	organization_env: OrganizationEnv;
}
export function constructName(props: StackProps, suffix: String): string {
	return `${props.organization}-${props.organization_env}-#${suffix}`;
}
