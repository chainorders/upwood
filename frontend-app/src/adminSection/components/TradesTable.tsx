import { useEffect, useState } from "react";
import { Table, TableBody, TableCell, TableContainer, TableRow, Paper } from "@mui/material";
import TablePagination from "@mui/material/TablePagination";
import { IndexerService, PagedResponse_ExchangeRecord } from "../../apiClient";
import useCommonStyles from "../../theme/useCommonStyles";
import { useForm, Controller } from "react-hook-form";
import { Box, Grid, TextField, Button, Divider } from "@mui/material";
import { toDisplayAmount } from "../../lib/conversions";

export default function TradesTable({ contract_index }: { contract_index: string }) {
	const classes = useCommonStyles();
	const [trades, setTrades] = useState<PagedResponse_ExchangeRecord>();
	const [page, setPage] = useState(0);
	const [pageSize, setPageSize] = useState(10);
	const [filters, setFilters] = useState<{ token_id?: string; buyer?: string; seller?: string }>({});

	const { control, handleSubmit, reset } = useForm<{ token_id?: string; buyer?: string; seller?: string }>({
		defaultValues: {},
	});

	useEffect(() => {
		if (contract_index) {
			IndexerService.getAdminIndexerExchangeRecords(
				page,
				pageSize,
				contract_index,
				filters.token_id || undefined,
				filters.buyer || undefined,
				filters.seller || undefined,
			)
				.then(setTrades)
				.catch(console.error);
		}
	}, [contract_index, page, pageSize, filters]);

	const onSubmit = (data: { token_id?: string; buyer?: string; seller?: string }) => {
		setFilters({
			token_id: data.token_id,
			buyer: data.buyer,
			seller: data.seller,
		});
		setPage(0);
	};
	const onClear = () => {
		reset({ token_id: "", buyer: "", seller: "" });
		setFilters({});
		setPage(0);
	};

	return (
		<Box>
			<Box sx={{ mb: 2, ...classes.filterFormSection }}>
				<form onSubmit={handleSubmit(onSubmit)}>
					<Grid container spacing={2} alignItems="center">
						<Grid item xs={12} sm={4}>
							<Controller
								name="token_id"
								control={control}
								render={({ field }) => <TextField {...field} label="Token ID" fullWidth variant="outlined" size="small" />}
							/>
						</Grid>
						<Grid item xs={12} sm={4}>
							<Controller
								name="buyer"
								control={control}
								render={({ field }) => <TextField {...field} label="Buyer Address" fullWidth variant="outlined" size="small" />}
							/>
						</Grid>
						<Grid item xs={12} sm={4}>
							<Controller
								name="seller"
								control={control}
								render={({ field }) => (
									<TextField {...field} label="Seller Address" fullWidth variant="outlined" size="small" />
								)}
							/>
						</Grid>
						<Grid item xs={12} display="flex" justifyContent="flex-end" gap={2}>
							<Button type="submit" variant="contained" color="primary" sx={{ minWidth: 100 }}>
								Search
							</Button>
							<Button onClick={onClear} variant="outlined" color="secondary" sx={{ minWidth: 100 }}>
								Clear
							</Button>
						</Grid>
					</Grid>
				</form>
			</Box>
			<Divider sx={{ mb: 2 }} />
			<TableContainer component={Paper} sx={classes.tableContainer}>
				<Table>
					<TableBody>
						<TableRow>
							<TableCell sx={classes.tableHeaderCell}>ID</TableCell>
							<TableCell sx={classes.tableHeaderCell}>Block Height</TableCell>
							<TableCell sx={classes.tableHeaderCell}>Txn Index</TableCell>
							<TableCell sx={classes.tableHeaderCell}>Token Contract</TableCell>
							<TableCell sx={classes.tableHeaderCell}>Token ID</TableCell>
							<TableCell sx={classes.tableHeaderCell}>Seller</TableCell>
							<TableCell sx={classes.tableHeaderCell}>Buyer</TableCell>
							<TableCell sx={classes.tableHeaderCell}>Token Amount</TableCell>
							<TableCell sx={classes.tableHeaderCell}>Currency Amount</TableCell>
							<TableCell sx={classes.tableHeaderCell}>Rate</TableCell>
							<TableCell sx={classes.tableHeaderCell}>Type</TableCell>
							<TableCell sx={classes.tableHeaderCell}>Created</TableCell>
						</TableRow>
						{trades?.data?.length ? (
							trades.data.map((trade) => (
								<TableRow key={trade.id}>
									<TableCell>{trade.id}</TableCell>
									<TableCell>{trade.block_height}</TableCell>
									<TableCell>{trade.txn_index}</TableCell>
									<TableCell>{trade.token_contract_address}</TableCell>
									<TableCell>{trade.token_id}</TableCell>
									<TableCell>{trade.seller}</TableCell>
									<TableCell>{trade.buyer}</TableCell>
									<TableCell>{toDisplayAmount(trade.token_amount, 0)}</TableCell>
									<TableCell>{toDisplayAmount(trade.currency_amount, 6)}</TableCell>
									<TableCell>{toDisplayAmount(trade.rate, 6)}</TableCell>
									<TableCell>{trade.exchange_record_type}</TableCell>
									<TableCell>{trade.create_time}</TableCell>
								</TableRow>
							))
						) : (
							<TableRow>
								<TableCell colSpan={13} align="center">
									No data
								</TableCell>
							</TableRow>
						)}
					</TableBody>
				</Table>
			</TableContainer>
			<TablePagination
				component="div"
				count={trades?.page_count ? trades.page_count * pageSize : 0}
				page={page}
				onPageChange={(_, newPage) => setPage(newPage)}
				rowsPerPage={pageSize}
				onRowsPerPageChange={(e) => {
					setPageSize(parseInt(e.target.value, 10));
					setPage(0);
				}}
				rowsPerPageOptions={[5, 10, 25, 50]}
			/>
		</Box>
	);
}
