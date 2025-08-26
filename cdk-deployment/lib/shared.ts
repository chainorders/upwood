import * as cdk from "aws-cdk-lib";

export enum OrganizationEnv {
	DEV = "dev",
	PROD = "prod",
}
export interface StackProps extends cdk.StackProps {
	organization: string;
	organization_env: OrganizationEnv;
}

/**
 * Constructs a name for a resource based on the organization name, organization environment, and a suffix.
 * @param props StackProps
 * @param suffix Suffix to append to the name
 * @returns {string} Name of the resource
 */
export function constructName(props: StackProps, suffix: String): string {
	return `${props.organization}-${props.organization_env}-${suffix}`;
}
