import { AccountAddress, ContractAddress } from "@concordium/web-sdk";
import { Box, Grid, Pagination, Paper, Stack, Typography } from "@mui/material";
import { useEffect, useState } from "react";
import { useSearchParams } from "react-router-dom";
import { MarketToken } from "../../lib/contracts-api-client";
import ErrorDisplay from "../common/ErrorDisplay";
import TokenDisplay from "./TokenDisplay";
import { useContractsApi } from "../ContractsApiProvider";
import { useNodeClient } from "../NodeClientProvider";

type Props = {
	contract: ContractAddress.Type;
	owner: AccountAddress.Type;
	onWithdraw: (token: MarketToken) => void;
	onList: (token: MarketToken) => void;
};

export default function UnListedTokens(props: Props) {
	const { contract, owner, onWithdraw, onList } = props;
	const [searchParams, setSearchParams] = useSearchParams();
	const [pageCount, setPageCount] = useState(0);
	const [page, setPage] = useState(Number(searchParams.get("page") || "0"));
	const [data, setData] = useState<MarketToken[]>([]);
	const [loading, setLoading] = useState(false);
	const [error, setError] = useState("");
	const { provider: backendApi } = useContractsApi();
	const { provider: grpcClient } = useNodeClient();

	useEffect(() => {
		setLoading(true);
		backendApi.default
			.getRwaMarketUnlisted({
				page,
				index: Number(contract.index),
				subindex: Number(contract.subindex),
				owner: owner.address,
			})
			.then((response) => {
				setData(response.data);
				setPageCount(response.page_count);
				setPage(response.page);
			})
			.catch((error) => setError(error.message))
			.finally(() => setLoading(false));
	}, [page, contract, owner, backendApi]);

	return (
		<Paper variant="outlined">
			<Typography variant="h4">
				UnListed Tokens ({props.contract.index.toString()}/{props.contract.subindex.toString()})
			</Typography>
			<Paper variant="outlined" sx={{ p: 2 }}>
				<Stack spacing={2}>
					<Grid container spacing={1}>
						{data.map((token, index) => (
							<Grid item xs={12} md={2} key={index}>
								<TokenDisplay token={token} onWithdraw={onWithdraw} onList={onList} grpcClient={grpcClient} />
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
		</Paper>
	);
}
