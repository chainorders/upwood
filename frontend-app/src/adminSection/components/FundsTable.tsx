import { useEffect, useState } from "react";
import { Table, TableBody, TableCell, TableContainer, TableRow, Paper } from "@mui/material";
import TablePagination from "@mui/material/TablePagination";
import { IndexerService, PagedResponse_SecurityMintFund } from "../../apiClient";
import { formatDateField, toDisplayAmount } from "../../lib/conversions";
import useCommonStyles from "../../theme/useCommonStyles";

export default function FundsTable({ contract_index }: { contract_index: string }) {
	const classes = useCommonStyles();
	const [funds, setFunds] = useState<PagedResponse_SecurityMintFund>();
	const [page, setPage] = useState(0);
	const [pageSize, setPageSize] = useState(10);

	useEffect(() => {
		if (contract_index) {
			IndexerService.getAdminIndexerFunds(contract_index, page, pageSize).then(setFunds).catch(console.error);
		}
	}, [contract_index, page, pageSize]);

	return (
		<>
			<TableContainer component={Paper} sx={classes.tableContainer}>
				<Table>
					<TableBody>
						<TableRow>
							<TableCell sx={classes.tableHeaderCell}>Fund Address</TableCell>
							<TableCell sx={classes.tableHeaderCell}>Investment Token</TableCell>
							<TableCell sx={classes.tableHeaderCell}>Currency Amount</TableCell>
							<TableCell sx={classes.tableHeaderCell}>Token Amount</TableCell>
							<TableCell sx={classes.tableHeaderCell}>Rate</TableCell>
							<TableCell sx={classes.tableHeaderCell}>Fund State</TableCell>
							<TableCell sx={classes.tableHeaderCell}>Receiver Address</TableCell>
							<TableCell sx={classes.tableHeaderCell}>Created</TableCell>
						</TableRow>
						{funds?.data?.length ? (
							funds.data.map((fund) => (
								<TableRow key={fund.contract_address}>
									<TableCell>{fund.contract_address}</TableCell>
									<TableCell>{fund.investment_token_id}</TableCell>
									<TableCell>{toDisplayAmount(fund.currency_amount, 6)}</TableCell>
									<TableCell>{fund.token_amount}</TableCell>
									<TableCell>
										{toDisplayAmount((Number(fund.rate_numerator) / Number(fund.rate_denominator)).toString(), 6)}
									</TableCell>
									<TableCell>{fund.fund_state}</TableCell>
									<TableCell>{fund.receiver_address}</TableCell>
									<TableCell>{formatDateField(fund.create_time)}</TableCell>
								</TableRow>
							))
						) : (
							<TableRow>
								<TableCell colSpan={7} align="center">
									No data
								</TableCell>
							</TableRow>
						)}
					</TableBody>
				</Table>
			</TableContainer>
			<TablePagination
				component="div"
				count={funds?.page_count ? funds.page_count * pageSize : 0}
				page={page}
				onPageChange={(_, newPage) => setPage(newPage)}
				rowsPerPage={pageSize}
				onRowsPerPageChange={(e) => {
					setPageSize(parseInt(e.target.value, 10));
					setPage(0);
				}}
				rowsPerPageOptions={[5, 10, 25, 50]}
			/>
		</>
	);
}
