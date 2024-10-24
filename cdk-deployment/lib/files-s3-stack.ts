import { App, RemovalPolicy, Stack, Tags } from "aws-cdk-lib";
import { constructName, OrganizationEnv, StackProps as SP } from "./shared";
import { Bucket, IBucket } from "aws-cdk-lib/aws-s3";

export interface StackProps extends SP {}
export class FilesS3Stack extends Stack {
	filesBucket: IBucket;

	constructor(scope: App, id: string, props: StackProps) {
		super(scope, id, props);
		const filesBucket = new Bucket(this, constructName(props, "files-bucket"), {
			removalPolicy:
				props.organization_env === OrganizationEnv.PROD
					? RemovalPolicy.RETAIN
					: RemovalPolicy.DESTROY,
			publicReadAccess: true,
            bucketName: constructName(props, "files-bucket"),
            blockPublicAccess: {
                blockPublicAcls: false,
                ignorePublicAcls: true,
                blockPublicPolicy: false,
                restrictPublicBuckets: false,
            }
		});
		Tags.of(filesBucket).add("organization", props.organization);
		Tags.of(filesBucket).add("environment", props.organization_env);

		this.filesBucket = filesBucket;
	}
}
