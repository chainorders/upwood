import {
	App,
	aws_certificatemanager as acm,
	aws_cloudfront as cloudfront,
	aws_cloudfront_origins as origins,
	CfnOutput,
	RemovalPolicy,
	Stack,
	Tags,
} from "aws-cdk-lib";
import * as s3 from "aws-cdk-lib/aws-s3";

import {
	constructName,
	OrganizationEnv,
	StackProps as SP,
} from "./shared";

export interface StackProps extends SP {
	domainName: string;
	certificateArn: string;
}

export class FilesS3Stack extends Stack {
	filesBucket: s3.Bucket;

	constructor(scope: App, id: string, props: StackProps) {
		super(scope, id, props);
		const bucket = new s3.Bucket(this, constructName(props, "files-bucket"), {
			bucketName: props.domainName,
			removalPolicy:
				props.organization_env === OrganizationEnv.PROD
					? RemovalPolicy.RETAIN
					: RemovalPolicy.DESTROY,
			publicReadAccess: true,
			blockPublicAccess: new s3.BlockPublicAccess({
				blockPublicAcls: false,
				blockPublicPolicy: false,
				ignorePublicAcls: false,
				restrictPublicBuckets: false,
			}),
			cors: [
				{
					allowedMethods: [s3.HttpMethods.GET, s3.HttpMethods.HEAD, s3.HttpMethods.PUT],
					allowedOrigins: ["*"],
					allowedHeaders: ["Content-Type"],
				}
			],
			websiteIndexDocument: "index.html",
		});
		const certificate = acm.Certificate.fromCertificateArn(
			this,
			constructName(props, "FilesCertificate"),
			props.certificateArn,
		);
		const distribution = new cloudfront.Distribution(
			this,
			constructName(props, "FilesDistribution"),
			{
				domainNames: [props.domainName],
				defaultBehavior: {
					origin: new origins.S3StaticWebsiteOrigin(bucket, {
						protocolPolicy: cloudfront.OriginProtocolPolicy.HTTP_ONLY,
						httpPort: 80,
						httpsPort: 443,
					}),
					originRequestPolicy: cloudfront.OriginRequestPolicy.CORS_S3_ORIGIN,
					allowedMethods: cloudfront.AllowedMethods.ALLOW_GET_HEAD_OPTIONS,
					viewerProtocolPolicy:
						cloudfront.ViewerProtocolPolicy.REDIRECT_TO_HTTPS,
					compress: true,
				},
				certificate: certificate,
			},
		);
		new CfnOutput(this, constructName(props, "FilesUrl"), {
			value: distribution.distributionDomainName,
		});
		Tags.of(bucket).add("organization", props.organization);
		Tags.of(bucket).add("environment", props.organization_env);
		this.filesBucket = bucket;
	}
}
