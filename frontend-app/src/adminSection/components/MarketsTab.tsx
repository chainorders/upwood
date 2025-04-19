import { useEffect, useState } from "react";
import { Table, TableBody, TableCell, TableContainer, TableRow, Paper } from "@mui/material";
import TablePagination from "@mui/material/TablePagination";
import { IndexerService, PagedResponse_Market } from "../../apiClient";
import useCommonStyles from "../../theme/useCommonStyles";
import { toDisplayAmount } from "../../lib/conversions";

export default function MarketsTab({ contract_index }: { contract_index: string }) {
	const classes = useCommonStyles();
	const [markets, setMarkets] = useState<PagedResponse_Market>();
	const [page, setPage] = useState(0);
	const [pageSize, setPageSize] = useState(10);

	useEffect(() => {
		if (contract_index) {
			IndexerService.getAdminIndexerMarkets(page, pageSize, contract_index).then(setMarkets).catch(console.error);
		}
	}, [contract_index, page, pageSize]);

	return (
		<Paper sx={{ p: 2 }}>
			<TableContainer>
				<Table>
					<TableBody>
						<TableRow>
							<TableCell sx={classes.tableHeaderCell}>Market Address</TableCell>
							<TableCell sx={classes.tableHeaderCell}>Type</TableCell>
							<TableCell sx={classes.tableHeaderCell}>Token ID</TableCell>
							<TableCell sx={classes.tableHeaderCell}>Liquidity Provider</TableCell>
							<TableCell sx={classes.tableHeaderCell}>Buy Rate</TableCell>
							<TableCell sx={classes.tableHeaderCell}>Sell Rate</TableCell>
							<TableCell sx={classes.tableHeaderCell}>Token Limit</TableCell>
							<TableCell sx={classes.tableHeaderCell}>Currency Limit</TableCell>
							<TableCell sx={classes.tableHeaderCell}>Currency In Amount</TableCell>
							<TableCell sx={classes.tableHeaderCell}>Token In Amount</TableCell>
							<TableCell sx={classes.tableHeaderCell}>Currency Out Amount</TableCell>
							<TableCell sx={classes.tableHeaderCell}>Currency In Amount</TableCell>
							<TableCell sx={classes.tableHeaderCell}>Created</TableCell>
							<TableCell sx={classes.tableHeaderCell}>Updated</TableCell>
						</TableRow>
						{markets?.data?.length ? (
							markets.data.map((market) => (
								<TableRow key={market.token_contract_address}>
									<TableCell>{market.contract_address}</TableCell>
									<TableCell>{market.market_type}</TableCell>
									<TableCell>{market.token_id}</TableCell>
									<TableCell>{market.liquidity_provider}</TableCell>
									<TableCell>
										{market.buy_rate_numerator && market.buy_rate_denominator
											? toDisplayAmount((Number(market.buy_rate_numerator) / Number(market.buy_rate_denominator)).toString(), 6)
											: "-"}
									</TableCell>
									<TableCell>
										{toDisplayAmount((Number(market.sell_rate_numerator) / Number(market.sell_rate_denominator)).toString(), 6)}
									</TableCell>
									<TableCell>{market.max_token_amount}</TableCell>
									<TableCell>{market.max_currency_amount ? toDisplayAmount(market.max_currency_amount, 6) : "-"}</TableCell>
									<TableCell>{toDisplayAmount(market.currency_in_amount, 6)}</TableCell>
									<TableCell>{toDisplayAmount(market.token_in_amount, 0)}</TableCell>
									<TableCell>{toDisplayAmount(market.currency_out_amount, 6)}</TableCell>
									<TableCell>{toDisplayAmount(market.token_out_amount, 0)}</TableCell>
									<TableCell>{market.create_time}</TableCell>
									<TableCell>{market.update_time}</TableCell>
								</TableRow>
							))
						) : (
							<TableRow>
								<TableCell colSpan={14} align="center">
									No data
								</TableCell>
							</TableRow>
						)}
					</TableBody>
				</Table>
			</TableContainer>
			<TablePagination
				component="div"
				count={markets?.page_count ? markets.page_count * pageSize : 0}
				page={page}
				onPageChange={(_, newPage) => setPage(newPage)}
				rowsPerPage={pageSize}
				onRowsPerPageChange={(e) => {
					setPageSize(parseInt(e.target.value, 10));
					setPage(0);
				}}
				rowsPerPageOptions={[5, 10, 25, 50]}
			/>
		</Paper>
	);
}
