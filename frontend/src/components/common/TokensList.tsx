import { ContractAddress } from "@concordium/web-sdk";
import { Token as ApiToken } from "../../lib/contracts-api-client";
import { useSearchParams } from "react-router-dom";
import { PlayArrow, Pause, OpenInBrowser, Token } from "@mui/icons-material";
import {
	Paper,
	Typography,
	Stack,
	List,
	ListItem,
	IconButton,
	ListItemButton,
	ListItemIcon,
	ListItemText,
	Box,
	Pagination,
	Link,
	Chip,
} from "@mui/material";
import ErrorDisplay from "./ErrorDisplay";
import CCDCis2TokenLink from "./concordium/CCDCis2TokenLink";
import { toHttpUrl, toTokenIdInt } from "../../lib/cis2Utils";

type TokensListProps = {
	contract: ContractAddress.Type;
	tokens: ApiToken[];
	pageCount: number;
	page: number;
	loading: boolean;
	error: string;
};

export default function TokensList(props: TokensListProps) {
	const [searchParams, setSearchParams] = useSearchParams();
	const { contract, tokens, pageCount, page, loading, error } = props;

	return (
		<Paper variant="outlined">
			<Typography variant="h4">Tokens</Typography>
			<Paper variant="outlined" sx={{ p: 2 }}>
				<Stack spacing={2}>
					<List>
						{tokens.map((token, index) => (
							<ListItem
								key={index}
								secondaryAction={
									<>
										<Chip label={`Supply: ${token.supply}`} />
										<IconButton edge="end" aria-label="playOrPause">
											{token.is_paused ? <PlayArrow /> : <Pause />}
										</IconButton>
										<CCDCis2TokenLink
											tokenId={token.token_id}
											contract={contract}
										>
											<IconButton edge="end" aria-label="open market">
												<OpenInBrowser />
											</IconButton>
										</CCDCis2TokenLink>
									</>
								}
							>
								<ListItemButton>
									<ListItemIcon>
										<Token />
									</ListItemIcon>
									<ListItemText
										primary={`${token.token_id} (${toTokenIdInt(token.token_id)})`}
										secondary={
											<Link
												href={toHttpUrl(token.metadata_url)}
												target="_blank"
											>
												Metadata
											</Link>
										}
									/>
								</ListItemButton>
							</ListItem>
						))}
					</List>
					<Box sx={{ display: "flex", justifyContent: "center" }}>
						<Pagination
							count={pageCount}
							page={page + 1}
							variant="outlined"
							shape="rounded"
							onChange={(_e, v) =>
								setSearchParams({ ...searchParams, page: v.toString() })
							}
						/>
						{error && <ErrorDisplay text={error} />}
						{loading && <Typography>Loading...</Typography>}
					</Box>
				</Stack>
			</Paper>
		</Paper>
	);
}
