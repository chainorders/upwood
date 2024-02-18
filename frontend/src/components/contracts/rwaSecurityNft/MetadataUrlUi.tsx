import { CIS2 } from "@concordium/web-sdk";
import { Stack, Typography, Button, Collapse } from "@mui/material";
import { FieldProps } from "@rjsf/utils";
import { useState } from "react";
import { Flatten } from "../../market/types";
import GetMetadataUrl from "../common/GetMetadataUrl";
import PrepareAndUploadMetadata from "../common/PrepareAndUploadMetadata";
import { MintRequestUi } from "../../../lib/rwaSecurityNftUi";

type MetadataUrlUi = Flatten<MintRequestUi["tokens"]>["metadata_url"];
const toUi = (token: CIS2.MetadataUrl): MetadataUrlUi =>
	({
		url: token.url,
		hash: token.hash ? { tag: "Some", Some: [token.hash] } : { tag: "None" },
	}) as MetadataUrlUi;

const fromUi = (token: MetadataUrlUi): CIS2.MetadataUrl => ({
	url: token.url,
	hash: token.hash.tag === "Some" ? token.hash.Some[0] : "",
});

const MetadataUrlUi = (props: FieldProps) => {
	const [prepDisplayed, setPrepDisplayed] = useState(false);

	return (
		<Stack spacing={2}>
			<Typography variant="h5">Metadata URL</Typography>
			<GetMetadataUrl
				value={props.formData && fromUi(props.formData)}
				onChange={(metadataUrl) => props.onChange(toUi(metadataUrl))}
			/>
			<Button onClick={() => setPrepDisplayed(!prepDisplayed)}>
				Or Prepare
			</Button>
			<Collapse in={prepDisplayed}>
				<PrepareAndUploadMetadata
					onDone={(metadataUrl) => {
						props.onChange(toUi(metadataUrl));
						setPrepDisplayed(false);
					}}
				/>
			</Collapse>
		</Stack>
	);
};

export default MetadataUrlUi;
