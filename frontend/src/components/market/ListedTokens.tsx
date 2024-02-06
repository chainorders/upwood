import { AccountAddress, ContractAddress } from "@concordium/web-sdk";
import { AppBar, Box, Grid, Pagination, Paper, Stack, Toolbar, Typography } from "@mui/material";
import { useEffect, useState } from "react";
import { useSearchParams } from "react-router-dom";
import { MarketToken } from "../../lib/contracts-api-client";
import ErrorDisplay from "../common/ErrorDisplay";
import TokenDisplay from "./TokenDisplay";
import { useContractsApi } from "../ContractsApiProvider";
import { useNodeClient } from "../NodeClientProvider";

type Props = {
	contract: ContractAddress.Type;
	currentAccount: AccountAddress.Type;
	onDeList: (token: MarketToken) => void;
	onList: (token: MarketToken) => void;
	onExchange: (token: MarketToken) => void;
};

export default function ListedTokens(props: Props) {
	const { contract, currentAccount, onDeList, onList, onExchange } = props;
	const [searchParams, setSearchParams] = useSearchParams();
	const [pageCount, setPageCount] = useState(0);
	const [page, setPage] = useState(Number(searchParams.get("page") || "0"));
	const [loading, setLoading] = useState(false);
	const [error, setError] = useState("");
	const [data, setData] = useState<MarketToken[]>([]);
	const { provider: backendApi } = useContractsApi();
	const { provider: grpcClient } = useNodeClient();

	useEffect(() => {
		setLoading(true);
		backendApi.default
			.getRwaMarketListed({
				page,
				index: Number(contract.index),
				subindex: Number(contract.subindex),
			})
			.then((response) => {
				setData(response.data);
				setPageCount(response.page_count);
				setPage(response.page);
			})
			.catch((error) => setError(error.message))
			.finally(() => setLoading(false));
	}, [page, contract, backendApi]);

	return (
		<>
			<AppBar position="static" color="transparent">
				<Toolbar>
					<Typography variant="h3" component="div" sx={{ flexGrow: 1 }}>
						Marketplace
					</Typography>
				</Toolbar>
			</AppBar>
			<Paper variant="outlined" sx={{ p: 2 }}>
				<Stack spacing={2}>
					<Grid container spacing={1}>
						{data.map((token, index) => (
							<Grid item xs={12} md={2} key={index}>
								<TokenDisplay
									grpcClient={grpcClient}
									token={token}
									onDeList={token.owner === currentAccount.address ? onDeList : undefined}
									onList={token.owner === currentAccount.address ? onList : undefined}
									onExchange={token.owner !== currentAccount.address ? onExchange : undefined}
								/>
							</Grid>
						))}
					</Grid>
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
		</>
	);
}
