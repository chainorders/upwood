import * as cdk from "aws-cdk-lib";
import * as route53 from "aws-cdk-lib/aws-route53";
import * as ses from "aws-cdk-lib/aws-ses";
import { Construct } from "constructs";
import { constructName, StackProps } from "./shared";

export interface SESStackProps extends StackProps {
    baseDomain: string;
}

export class SESStack extends cdk.Stack {
    public readonly emailIdentity: ses.EmailIdentity;

    constructor(scope: Construct, id: string, props: SESStackProps) {
        super(scope, id, props);

        // Create the SES email identity (domain)
        const emailIdentity = new ses.EmailIdentity(
            this,
            constructName(props, "email-identity"),
            {
                identity: ses.Identity.domain(props.baseDomain),
            },
        );

        // Output the SES verification status
        new cdk.CfnOutput(this, "SESIdentityStatus", {
            value: `SES Domain Identity for ${props.baseDomain} created. Check AWS Console for verification status.`,
            description: "SES Domain Identity Status",
        });

        // Set tags on resources
        cdk.Tags.of(emailIdentity).add("organization", props.organization);
        cdk.Tags.of(emailIdentity).add("environment", props.organization_env);

        this.emailIdentity = emailIdentity;
    }
}
