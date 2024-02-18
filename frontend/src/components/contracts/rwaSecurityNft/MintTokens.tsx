import {
	Stack,
	List,
	ListItem,
	ListItemText,
	Typography,
	IconButton,
	Icon,
	Link,
} from "@mui/material";
import { CIS2 } from "@concordium/web-sdk";
import { MintRequestUi } from "../../../lib/rwaSecurityNftUi";
import { WidgetProps } from "@rjsf/utils";
import { Flatten } from "../../market/types";
import { Delete } from "@mui/icons-material";
import GetMetadataUrl from "./mintSteps/GetMetadataUrl";
import PrepareAndUploadMetadata from "./mintSteps/PrepareAndUploadMetadata";

type TokenUi = Flatten<MintRequestUi["tokens"]>;

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
