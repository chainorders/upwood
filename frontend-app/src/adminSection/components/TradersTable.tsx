import { useEffect, useState } from "react";
import { Table, TableBody, TableCell, TableContainer, TableRow, Paper } from "@mui/material";
import TablePagination from "@mui/material/TablePagination";
import { IndexerService, PagedResponse_TraderUser } from "../../apiClient";
import useCommonStyles from "../../theme/useCommonStyles";
import { useForm, Controller } from "react-hook-form";
import { Box, Grid, TextField, Button, Divider } from "@mui/material";
import { formatDateField, toDisplayAmount } from "../../lib/conversions";

export default function TradersTable({ contract_index }: { contract_index: string }) {
	const classes = useCommonStyles();
	const [traders, setTraders] = useState<PagedResponse_TraderUser>();
	const [page, setPage] = useState(0);
	const [pageSize, setPageSize] = useState(10);
	const [filters, setFilters] = useState<{ trader?: string; token_id?: string }>({});

	const { control, handleSubmit, reset } = useForm<{ trader?: string; token_id?: string }>({
		defaultValues: {},
	});

	useEffect(() => {
		if (contract_index) {
			IndexerService.getAdminIndexerTraders(
				page,
				pageSize,
				undefined,
				contract_index,
				filters.token_id || undefined,
				filters.trader || undefined,
			)
				.then(setTraders)
				.catch(console.error);
		}
	}, [contract_index, page, pageSize, filters]);

	const onSubmit = (data: { trader?: string; token_id?: string }) => {
		setFilters({ trader: data.trader, token_id: data.token_id });
		setPage(0);
	};
	const onClear = () => {
		reset({ trader: "", token_id: "" });
		setFilters({});
		setPage(0);
	};

	return (
		<Box>
			<Box sx={{ mb: 2, ...classes.filterFormSection }}>
				<form onSubmit={handleSubmit(onSubmit)}>
					<Grid container spacing={2} alignItems="center">
						<Grid item xs={12} sm={5}>
							<Controller
								name="trader"
								control={control}
								render={({ field }) => (
									<TextField {...field} label="Trader Address" fullWidth variant="outlined" size="small" />
								)}
							/>
						</Grid>
						<Grid item xs={12} sm={5}>
							<Controller
								name="token_id"
								control={control}
								render={({ field }) => <TextField {...field} label="Token ID" fullWidth variant="outlined" size="small" />}
							/>
						</Grid>
						<Grid item xs={12} sm={2} display="flex" justifyContent="flex-end" gap={2}>
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
							<TableCell sx={classes.tableHeaderCell}>Trader</TableCell>
							<TableCell sx={classes.tableHeaderCell}>Email</TableCell>
							<TableCell sx={classes.tableHeaderCell}>Token ID</TableCell>
							<TableCell sx={classes.tableHeaderCell}>Token In</TableCell>
							<TableCell sx={classes.tableHeaderCell}>Token Out</TableCell>
							<TableCell sx={classes.tableHeaderCell}>Currency In</TableCell>
							<TableCell sx={classes.tableHeaderCell}>Currency Out</TableCell>
							<TableCell sx={classes.tableHeaderCell}>Created</TableCell>
							<TableCell sx={classes.tableHeaderCell}>Updated</TableCell>
						</TableRow>
						{traders?.data?.length ? (
							traders.data.map((trader) => (
								<TableRow key={trader.trader + trader.token_id}>
									<TableCell>{trader.trader}</TableCell>
									<TableCell>{trader.email || "-"}</TableCell>
									<TableCell>{trader.token_id}</TableCell>
									<TableCell>{toDisplayAmount(trader.token_in_amount, 0)}</TableCell>
									<TableCell>{toDisplayAmount(trader.token_out_amount, 0)}</TableCell>
									<TableCell>{toDisplayAmount(trader.currency_in_amount, 6)}</TableCell>
									<TableCell>{toDisplayAmount(trader.currency_out_amount, 6)}</TableCell>
									<TableCell>{formatDateField(trader.create_time)}</TableCell>
									<TableCell>{formatDateField(trader.update_time)}</TableCell>
								</TableRow>
							))
						) : (
							<TableRow>
								<TableCell colSpan={10} align="center">
									No data
								</TableCell>
							</TableRow>
						)}
					</TableBody>
				</Table>
			</TableContainer>
			<TablePagination
				component="div"
				count={traders?.page_count ? traders.page_count * pageSize : 0}
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
