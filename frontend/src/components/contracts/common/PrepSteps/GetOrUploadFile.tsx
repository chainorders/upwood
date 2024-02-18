import { Stack, TextField, Typography, Button } from "@mui/material";
import { useState, useCallback, useEffect } from "react";
import FileUpload from "react-material-file-upload";
import { PinataClient } from "../../../../lib/PinataClient";
import { toHttpUrl } from "../../../../lib/cis2Utils";
import ErrorDisplay from "../../../common/ErrorDisplay";
import InfoDisplay from "../../../common/InfoDisplay";

const PINATA_JWT = import.meta.env.VITE_PINATA_JWT!;

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

export default GetOrUploadFile;
