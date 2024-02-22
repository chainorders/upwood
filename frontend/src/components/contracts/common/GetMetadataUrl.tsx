import { CIS2 } from "@concordium/web-sdk";
import { TextField, CircularProgress, Stack } from "@mui/material";
import { useState } from "react";
import { getTokenMetadataHash, isValidUrl } from "../../../lib/cis2Utils";
import ErrorDisplay from "../../common/ErrorDisplay";

const GetMetadataUrl = (props: {
	value?: CIS2.MetadataUrl;
	onChange: (metadata: CIS2.MetadataUrl) => void;
}) => {
	const { value, onChange } = props;
	const [error, setError] = useState<string | undefined>();
	const [loadingMetadata, setLoadingMetadata] = useState(false);

	const onUrlChange = (url: string) => {
		props.onChange({ url });
		setError(undefined);
		if (!url) {
			setLoadingMetadata(false);
			return;
		}

		if (!isValidUrl(url)) {
			setError("Invalid URL");
			return;
		}

		setLoadingMetadata(true);
		setError(undefined);
		getTokenMetadataHash(url)
			.then((hash) =>
				onChange({
					url: url,
					hash,
				}),
			)
			.catch((error) => setError(error.message))
			.finally(() => setLoadingMetadata(false));
	};

	return (
		<Stack spacing={1}>
			<TextField
				label="Metadata URL"
				onChange={(e) => onUrlChange(e.target.value)}
				disabled={loadingMetadata}
				helperText={value?.hash && `hash: ${value?.hash}`}
				value={value?.url || ""}
				color={error ? "error" : undefined}
			/>
			{loadingMetadata && <CircularProgress size={20} />}
			{error && <ErrorDisplay text={error} />}
		</Stack>
	);
};
export default GetMetadataUrl;
