import { ContractAddress } from "@concordium/web-sdk";
import { useSearchParams } from "react-router-dom";
import { useEffect, useState } from "react";
import { NftToken } from "../../../lib/contracts-api-client";
import { useContractsApi } from "../../ContractsApiProvider";
import {
	Box,
	IconButton,
	List,
	ListItem,
	ListItemButton,
	ListItemIcon,
	ListItemText,
	Pagination,
	Paper,
	Stack,
	Typography,
} from "@mui/material";
import ErrorDisplay from "../../common/ErrorDisplay";
import { OpenInBrowser, Pause, PlayArrow, Token } from "@mui/icons-material";
import CCDCis2TokenLink from "../../common/concordium/CCDCis2TokenLink";

type Props = {
	contract: ContractAddress.Type;
};
export default function TokenList(props: Props) {
	const { contract } = props;
	const [searchParams, setSearchParams] = useSearchParams();
	const [pageCount, setPageCount] = useState(0);
	const [page, setPage] = useState(Number(searchParams.get("page") || "0"));
	const [loading, setLoading] = useState(false);
	const [error, setError] = useState("");
	const [tokens, setTokens] = useState<NftToken[]>([]);
	const { provider: backendApi } = useContractsApi();

	useEffect(() => {
		setLoading(true);
		backendApi.default
			.getRwaSecurityNftTokens({
				index: Number(contract.index),
				subindex: Number(contract.subindex),
				page,
			})
			.then((response) => {
				setTokens(response.data);
				setPageCount(response.page_count);
				setPage(response.page);
			})
			.catch((error) => setError(error.message))
			.finally(() => setLoading(false));
	}, [contract, backendApi, page]);

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
										<IconButton edge="end" aria-label="playOrPause">
											{token.is_paused ? <PlayArrow /> : <Pause />}
										</IconButton>
										<CCDCis2TokenLink tokenId={token.token_id} contract={contract}>
											<IconButton edge="end" aria-label="open market">
												<OpenInBrowser />
											</IconButton>
										</CCDCis2TokenLink>
									</>
								}>
								<ListItemButton>
									<ListItemIcon>
										<Token />
									</ListItemIcon>
									<ListItemText primary={token.token_id} secondary={token.metadata_url} />
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
							onChange={(_e, v) => setSearchParams({ ...searchParams, page: v.toString() })}
						/>
						{error && <ErrorDisplay text={error} />}
						{loading && <Typography>Loading...</Typography>}
					</Box>
				</Stack>
			</Paper>
		</Paper>
	);
}
