echo "Executing Post Create Commands"
echo "Setting Yarn Version" \
&& corepack enable \
&& corepack install --global yarn@4.3.1 \
&& yarn set version stable \
&& rustc --version
echo "Setting AWS cli" \
&& aws configure import --csv file:///etc/aws/aws_accessKeys.export \
&& aws sts get-caller-identity --query Account --output text