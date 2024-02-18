import {
	Stack,
	Grid,
	Paper,
	List,
	ListItem,
	ListItemText,
	TextField,
	Button,
	Typography,
	Stepper,
	Step,
	StepLabel,
	IconButton,
	Icon,
	Link,
} from "@mui/material";
import { CIS2, sha256, toBuffer } from "@concordium/web-sdk";
import { MintRequestUi } from "../../../lib/rwaSecurityNftUi";
import { WidgetProps } from "@rjsf/utils";
import { Flatten } from "../../market/types";
import { useCallback, useEffect, useState } from "react";
import {
	TokenMetadata,
	getTokenMetadataHash,
	isValidUrl,
	toHttpUrl,
} from "../../../lib/cis2Utils";
import ErrorDisplay from "../../common/ErrorDisplay";
import FileUpload from "react-material-file-upload";
import { PinataClient } from "../../../lib/PinataClient";
import { ArrowCircleLeft, Delete } from "@mui/icons-material";
import InfoDisplay from "../../common/InfoDisplay";

type TokenUi = Flatten<MintRequestUi["tokens"]>;
const PINATA_JWT = import.meta.env.VITE_PINATA_JWT!;

const GetMetadataUrl = (props: {
	onDone: (metadata: CIS2.MetadataUrl) => void;
}) => {
	const [url, setUrl] = useState("");
	const [error, setError] = useState<string | undefined>();
	const [loadingMetadata, setLoadingMetadata] = useState(false);
	const [metadataUrl, setMetadataUrl] = useState<CIS2.MetadataUrl>();

	useEffect(() => {
		if (!url) {
			setError(undefined);
			setMetadataUrl(undefined);
			setLoadingMetadata(false);
			return;
		}

		if (!isValidUrl(url)) {
			setError("Invalid URL");
			return;
		}

		setMetadataUrl(undefined);
		setLoadingMetadata(true);
		setError(undefined);
		getTokenMetadataHash(url)
			.then((hash) =>
				setMetadataUrl({
					url: url,
					hash,
				}),
			)
			.catch((error) => setError(error.message))
			.finally(() => setLoadingMetadata(false));
	}, [url]);

	const onDone = (metadataUrl: CIS2.MetadataUrl) => {
		setUrl("");
		setMetadataUrl(undefined);
		setError(undefined);
		setLoadingMetadata(false);
		props.onDone(metadataUrl);
	};

	return (
		<Stack>
			<TextField
				label="Metadata URL"
				value={url}
				onChange={(e) => setUrl(e.target.value)}
				disabled={loadingMetadata}
				fullWidth
			/>
			{error && <ErrorDisplay text={error} />}
			<Button onClick={() => onDone(metadataUrl!)} disabled={!metadataUrl}>
				Done
			</Button>
		</Stack>
	);
};

const GetOrUploadFile = (props: {
	doneButtonText: string;
	url?: string;
	onDone: (url: string) => void;
	onSkip?: () => void;
}) => {
	const [isUploading, setIsUploading] = useState(false);
	const [url, setUrl] = useState<string>(props.url || "");
	const [files, setFiles] = useState<File[]>([]);
	const [error, setError] = useState<string>();

	const onDone = useCallback(
		(url: string) => {
			setUrl("");
			setFiles([]);
			setError(undefined);
			setIsUploading(false);
			props.onDone(url);
		},
		[props],
	);

	useEffect(() => {
		if (!files || files.length === 0) {
			setIsUploading(false);
			setError(undefined);
			setUrl("");
			return;
		}

		const file = files[0];
		setIsUploading(true);
		const pinata = new PinataClient(PINATA_JWT);
		pinata
			.isJwtValid()
			.then((isValid) => {
				if (!isValid) {
					throw new Error("Invalid JWT");
				}
			})
			.then(() => pinata.uploadFile(files[0], file.name))
			.then((result) => {
				onDone(toHttpUrl(result));
			})
			.catch((error) => setError(error.message))
			.finally(() => setIsUploading(false));
	}, [files, onDone]);

	return (
		<Stack spacing={2}>
			<TextField
				label="URL"
				value={url}
				onChange={(e) => setUrl(e.target.value)}
				disabled={isUploading}
			/>
			<Typography>Or</Typography>
			<FileUpload multiple={false} value={files} onChange={setFiles} />
			<Button variant="contained" onClick={() => onDone(url!)} disabled={!url}>
				{props.doneButtonText}
			</Button>
			{props.onSkip && <Button onClick={() => props.onSkip!()}>Skip</Button>}
			{error && <ErrorDisplay text={error} />}
			{isUploading && <InfoDisplay text="Uploading..." />}
		</Stack>
	);
};

type Attributes = {
	name?: string;
	description?: string;
	symbol?: string;
};
const SetAttributes = (props: {
	onDone: (attributes: Attributes) => void;
	value?: Attributes;
	doneButtonText: string;
}) => {
	const [attrs, setAttrs] = useState<Attributes>(props.value || {});
	const isValid = attrs.name && attrs.description;

	return (
		<Stack>
			<TextField
				label="Name"
				value={attrs.name || ""}
				onChange={(e) => setAttrs({ ...attrs, name: e.target.value })}
			/>
			<TextField
				label="Description"
				value={attrs.description || ""}
				onChange={(e) => setAttrs({ ...attrs, description: e.target.value })}
			/>
			<TextField
				label="Symbol"
				value={attrs.symbol || ""}
				onChange={(e) => setAttrs({ ...attrs, symbol: e.target.value })}
			/>
			<Button onClick={() => props.onDone(attrs)} disabled={!isValid}>
				{props.doneButtonText}
			</Button>
		</Stack>
	);
};

const UploadMetadata = (props: {
	metadata: TokenMetadata;
	onDone: (metadataUrl: CIS2.MetadataUrl) => void;
	doneButtonText: string;
}) => {
	const [isUploading, setIsUploading] = useState(false);
	const [url, setUrl] = useState<CIS2.MetadataUrl>();
	const [error, setError] = useState<string | undefined>();

	useEffect(() => {
		if (!props.metadata) {
			setIsUploading(false);
			setError(undefined);
			setUrl(undefined);
			return;
		}

		setIsUploading(true);
		const pinata = new PinataClient(PINATA_JWT);
		pinata
			.isJwtValid()
			.then((isValid) => {
				if (!isValid) {
					throw new Error("Invalid JWT");
				}
			})
			.then(() => pinata.uploadJson(props.metadata, "metadata.json"))
			.then((result) => {
				setUrl({
					url: toHttpUrl(result),
					hash: sha256([toBuffer(JSON.stringify(props.metadata))]).toString(
						"hex",
					),
				});
			})
			.catch((error) => setError(error.message))
			.finally(() => setIsUploading(false));
	}, [props.metadata]);

	return (
		<Stack>
			{isUploading && <InfoDisplay text="Uploading..." />}
			{error && <ErrorDisplay text={error} />}
			{url && (
				<Link href={url.url} target="_blank" p={1}>
					{url.url}
				</Link>
			)}
			<Button
				variant="contained"
				onClick={() => props.onDone(url!)}
				disabled={!url}
			>
				{props.doneButtonText}
			</Button>
		</Stack>
	);
};

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

const toUiToken = (token: CIS2.MetadataUrl): TokenUi =>
	({
		metadata_url: {
			url: token.url,
			hash: token.hash ? { tag: "Some", Some: [token.hash] } : { tag: "None" },
		},
	}) as TokenUi;

const fromUiToken = (token: TokenUi): CIS2.MetadataUrl => ({
	url: token.metadata_url.url,
	hash:
		token.metadata_url.hash.tag === "Some"
			? token.metadata_url.hash.Some[0]
			: "",
});

const MintTokens = (props: WidgetProps) => {
	const {
		value,
		onChange,
	}: { value: TokenUi[]; onChange: (value: TokenUi[]) => void } = props;
	const tokens: CIS2.MetadataUrl[] = value.map(fromUiToken);

	const addMetadata = (metadataUrl: CIS2.MetadataUrl) => {
		const existingMetadata = tokens.findIndex(
			(t) => t.hash === metadataUrl.hash,
		);

		if (existingMetadata === -1) {
			onChange([...tokens, metadataUrl].map(toUiToken));
		}
	};

	const removeMetadata = (metadataUrl: CIS2.MetadataUrl) => {
		const existingMetadata = tokens.findIndex(
			(t) => t.hash === metadataUrl.hash,
		);

		if (existingMetadata !== -1) {
			tokens.splice(existingMetadata, 1);
			onChange([...tokens].map(toUiToken));
		}
	};

	return (
		<Stack>
			<List>
				{tokens.map((t, index) => (
					<ListItem
						key={index}
						secondaryAction={
							<IconButton onClick={() => removeMetadata(t)}>
								<Icon>
									<Delete />
								</Icon>
							</IconButton>
						}
					>
						<ListItemText
							primary={
								<Link href={t.url} target="_blank">
									{t.url}
								</Link>
							}
							secondary={t.hash}
						/>
					</ListItem>
				))}
			</List>
			<Stack spacing={2}>
				<GetMetadataUrl onDone={addMetadata} />
				<Typography>Or Prepare</Typography>
				<PrepareAndUploadMetadata onDone={addMetadata} />
			</Stack>
		</Stack>
	);
};

export default MintTokens;
