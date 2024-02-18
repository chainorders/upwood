import { CIS2, sha256, toBuffer } from "@concordium/web-sdk";
import { Stack, Button, Link } from "@mui/material";
import { useState, useEffect } from "react";
import { PinataClient } from "../../../../lib/PinataClient";
import { TokenMetadata, toHttpUrl } from "../../../../lib/cis2Utils";
import ErrorDisplay from "../../../common/ErrorDisplay";
import InfoDisplay from "../../../common/InfoDisplay";

const PINATA_JWT = import.meta.env.VITE_PINATA_JWT!;

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

export default UploadMetadata;
