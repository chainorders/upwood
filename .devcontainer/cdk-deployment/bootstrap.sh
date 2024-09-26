echo "Executing Post Create Commands"
echo "Setting Yarn Version" \
&& corepack enable \
&& corepack install --global pnpm yarn@4.3.1 \
&& yarn set version stable
echo "Setting AWS cli" \
&& aws configure import --csv file:///etc/aws/aws_accessKeys.export
&& aws sts get-caller-identity --query Account --output text