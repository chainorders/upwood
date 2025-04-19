import { useEffect, useState } from "react";
import { Table, TableBody, TableCell, TableContainer, TableRow, Paper } from "@mui/material";
import TablePagination from "@mui/material/TablePagination";
import { IndexerService, PagedResponse_InvestorUser } from "../../apiClient";
import { formatDateField, toDisplayAmount } from "../../lib/conversions";
import useCommonStyles from "../../theme/useCommonStyles";
import { useForm, Controller } from "react-hook-form";
import { Box, Grid, TextField, Button, Divider } from "@mui/material";

export default function InvestorsTable({ contract_index }: { contract_index: string }) {
	const classes = useCommonStyles();
	const [investors, setInvestors] = useState<PagedResponse_InvestorUser>();
	const [page, setPage] = useState(0);
	const [pageSize, setPageSize] = useState(25);
	const [filters, setFilters] = useState<{ investor?: string; investment_token_id?: string }>({});

	const { control, handleSubmit, reset } = useForm<{ investor?: string; investment_token_id?: string }>({
		defaultValues: {},
	});

	useEffect(() => {
		IndexerService.getAdminIndexerInvestors(
			page,
			pageSize,
			undefined,
			contract_index,
			filters.investment_token_id || undefined,
			filters.investor || undefined,
		)
			.then(setInvestors)
			.catch(console.error);
	}, [contract_index, page, pageSize, filters]);

	const onSubmit = (data: { investor?: string; investment_token_id?: string }) => {
		setFilters({
			investor: data.investor,
			investment_token_id: data.investment_token_id,
		});
		setPage(0);
	};
	const onClear = () => {
		reset({ investor: "", investment_token_id: "" });
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
								name="investor"
								control={control}
								render={({ field }) => <TextField {...field} label="Investor" fullWidth variant="outlined" size="small" />}
							/>
						</Grid>
						<Grid item xs={12} sm={5}>
							<Controller
								name="investment_token_id"
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
							<TableCell sx={classes.tableHeaderCell}>Investor</TableCell>
							<TableCell sx={classes.tableHeaderCell}>Investment Token</TableCell>
							<TableCell sx={classes.tableHeaderCell}>Currency Amount</TableCell>
							<TableCell sx={classes.tableHeaderCell}>Token Amount</TableCell>
							<TableCell sx={classes.tableHeaderCell}>Email</TableCell>
							<TableCell sx={classes.tableHeaderCell}>Created</TableCell>
						</TableRow>
						{investors?.data.length ? (
							investors.data.map((inv) => (
								<TableRow key={inv.investor + inv.investment_token_id} sx={classes.tableRow}>
									<TableCell>{inv.investor}</TableCell>
									<TableCell>{inv.investment_token_id}</TableCell>
									<TableCell>{toDisplayAmount(inv.currency_amount, 6)}</TableCell>
									<TableCell>{inv.token_amount}</TableCell>
									<TableCell>{inv.email || "-"}</TableCell>
									<TableCell>{formatDateField(inv.create_time)}</TableCell>
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
				count={investors?.page_count ? investors.page_count * pageSize : 0}
				page={page}
				onPageChange={(_, newPage) => setPage(newPage)}
				rowsPerPage={pageSize}
				onRowsPerPageChange={(e) => {
					setPageSize(parseInt(e.target.value, 10));
					setPage(0);
				}}
				rowsPerPageOptions={[5, 10, 25, 50, 100]}
			/>
		</Box>
	);
}
