import * as cdk from "aws-cdk-lib";
import { Construct } from "constructs";
import { constructName, OrganizationEnv, StackProps as SP } from "./shared";
import * as s3 from "aws-cdk-lib/aws-s3";
import { aws_s3_deployment as s3deploy } from "aws-cdk-lib";
import { aws_certificatemanager as acm } from "aws-cdk-lib";
import { aws_cloudfront as cloudfront } from "aws-cdk-lib";
import { aws_cloudfront_origins as origins } from "aws-cdk-lib";

export interface StackProps extends SP {
	domainName: string;
	certificateArn: string;
}

export class FrontendAppWebsiteStack extends cdk.Stack {
	constructor(scope: Construct, id: string, props: StackProps) {
		super(scope, id, props);

		const bucket = new s3.Bucket(this, constructName(props, "AppWebsite"), {
			bucketName: props.domainName,
			removalPolicy:
				props.organization_env === OrganizationEnv.PROD
					? cdk.RemovalPolicy.RETAIN
					: cdk.RemovalPolicy.DESTROY,
			autoDeleteObjects: true,
			publicReadAccess: true,
			blockPublicAccess: new s3.BlockPublicAccess({
				blockPublicAcls: false,
				blockPublicPolicy: false,
				ignorePublicAcls: false,
				restrictPublicBuckets: false,
			}),
			versioned: true,
			websiteIndexDocument: "index.html",
			websiteErrorDocument: "error.html",
		});
		new s3deploy.BucketDeployment(
			this,
			constructName(props, "AppWebsiteDeployment"),
			{
				sources: [s3deploy.Source.asset("../frontend-app/dist")],
				destinationBucket: bucket,
			},
		);
		const certificate = acm.Certificate.fromCertificateArn(
			this,
			constructName(props, "AppWebsiteCertificate"),
			props.certificateArn,
		);
		const distribution = new cloudfront.Distribution(
			this,
			constructName(props, "AppWebsiteDistribution"),
			{
				domainNames: [props.domainName],
				defaultBehavior: {
					origin: new origins.S3StaticWebsiteOrigin(bucket, {
						protocolPolicy: cloudfront.OriginProtocolPolicy.HTTP_ONLY,
						httpPort: 80,
						httpsPort: 443,
					}),
					allowedMethods: cloudfront.AllowedMethods.ALLOW_GET_HEAD_OPTIONS,
					viewerProtocolPolicy:
						cloudfront.ViewerProtocolPolicy.REDIRECT_TO_HTTPS,
				},
				errorResponses: [
					{
						httpStatus: 404,
						responseHttpStatus: 200,
						responsePagePath: "/index.html",
					},
				],
				certificate: certificate,
			},
		);
		new cdk.CfnOutput(this, constructName(props, "AppWebsiteUrl"), {
			value: distribution.distributionDomainName,
		});
	}
}
