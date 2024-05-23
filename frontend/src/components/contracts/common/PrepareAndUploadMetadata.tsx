import { CIS2 } from "@concordium/web-sdk";
import { ArrowCircleLeft } from "@mui/icons-material";
import {
	Grid,
	Stepper,
	Step,
	StepLabel,
	Stack,
	Paper,
	IconButton,
	Icon,
} from "@mui/material";
import { useState } from "react";
import { TokenMetadata } from "../../../lib/cis2Utils";
import GetOrUploadFile from "./PrepSteps/GetOrUploadFile";
import SetAttributes, { Attributes } from "./PrepSteps/SetAttributes";
import UploadMetadata from "./PrepSteps/UploadMetadata";

enum PrepMetadataStep {
	GetOrUploadImage,
	GetOrUploadArtifact,
	SetAttributes,
	UploadMetadata,
}
const PrepareAndUploadMetadata = (props: {
	onDone: (metadataUrl: CIS2.MetadataUrl) => void;
}) => {
	const [step, setStep] = useState(PrepMetadataStep.GetOrUploadImage);
	const [imageUrl, setImageUrl] = useState<string>("");
	const [artifactUrl, setArtifactUrl] = useState<string>("");
	const [attrs, setAttrs] = useState<Attributes>();
	const [metadata, setMetadata] = useState<TokenMetadata>();

	const onImageDone = (imageUrl: string) => {
		setImageUrl(imageUrl);
		setStep(PrepMetadataStep.GetOrUploadArtifact);
	};

	const onArtifactDone = (artifactUrl: string) => {
		setArtifactUrl(artifactUrl);
		setStep(PrepMetadataStep.SetAttributes);
	};

	const onArtifactSkip = () => {
		setArtifactUrl("");
		setStep(PrepMetadataStep.SetAttributes);
	};

	const onAttributesDone = (attributes: Attributes) => {
		setAttrs(attributes);
		setMetadata({
			name: attributes.name,
			description: attributes.description,
			symbol: attributes.symbol,
			thumbnail: { url: imageUrl },
			artifact: { url: artifactUrl },
			unique: attributes.unique,
			attributes: [
				{ name: "latitude", value: attributes.latitude, type: "string" },
				{ name: "longitude", value: attributes.longitude, type: "string" },
				{
					name: "constructionDate",
					value: attributes.constructionDate,
					type: "string",
				},
			],
		});
		setStep(PrepMetadataStep.UploadMetadata);
	};

	const onUploadDone = (metadataUrl: CIS2.MetadataUrl) => {
		setStep(PrepMetadataStep.GetOrUploadImage);
		setImageUrl("");
		setArtifactUrl("");
		setAttrs({});
		setMetadata(undefined);
		props.onDone(metadataUrl);
	};

	const goBack = () => {
		setStep(Math.max(step - 1, 0));
	};

	return (
		<Grid container spacing={1}>
			<Grid item md={2}>
				<Stepper activeStep={step} orientation="vertical">
					<Step completed={step > PrepMetadataStep.GetOrUploadImage}>
						<StepLabel>Get or Upload Image</StepLabel>
					</Step>
					<Step
						completed={
							step > PrepMetadataStep.GetOrUploadArtifact && !!artifactUrl
						}
					>
						<StepLabel>Get or Upload Artifact</StepLabel>
					</Step>
					<Step completed={step > PrepMetadataStep.SetAttributes}>
						<StepLabel>Metadata Information</StepLabel>
					</Step>
					<Step completed={step > PrepMetadataStep.UploadMetadata}>
						<StepLabel>Upload Metadata</StepLabel>
					</Step>
				</Stepper>
			</Grid>
			<Grid item md={10}>
				<Stack spacing={1}>
					<Paper variant="outlined">
						<IconButton onClick={goBack}>
							<Icon>
								<ArrowCircleLeft />
							</Icon>
						</IconButton>
					</Paper>
					<Paper variant="outlined">
						{
							{
								[PrepMetadataStep.GetOrUploadImage]: GetOrUploadFile({
									onDone: onImageDone,
									url: imageUrl,
									doneButtonText: "Image Uploaded : Next",
								}),
								[PrepMetadataStep.GetOrUploadArtifact]: GetOrUploadFile({
									onDone: onArtifactDone,
									onSkip: onArtifactSkip,
									url: artifactUrl,
									doneButtonText: "Artifact Uploaded: Next",
								}),
								[PrepMetadataStep.SetAttributes]: SetAttributes({
									value: attrs,
									onDone: onAttributesDone,
									doneButtonText: "Next",
								}),
								[PrepMetadataStep.UploadMetadata]: UploadMetadata({
									metadata: metadata!,
									onDone: onUploadDone,
									doneButtonText: "Done",
								}),
							}[step]
						}
					</Paper>
				</Stack>
			</Grid>
		</Grid>
	);
};

export default PrepareAndUploadMetadata;
