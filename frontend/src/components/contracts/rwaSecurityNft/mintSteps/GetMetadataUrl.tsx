import { CIS2 } from "@concordium/web-sdk";
import { Stack, TextField, Button } from "@mui/material";
import { useState, useEffect } from "react";
import { isValidUrl, getTokenMetadataHash } from "../../../../lib/cis2Utils";
import ErrorDisplay from "../../../common/ErrorDisplay";

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
export default GetMetadataUrl;
