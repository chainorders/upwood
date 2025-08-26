import { useEffect, useState } from "react";
import { Table, TableBody, TableCell, TableContainer, TableRow, Paper } from "@mui/material";
import TablePagination from "@mui/material/TablePagination";
import { IndexerService, PagedResponse_Yield } from "../../apiClient";
import useCommonStyles from "../../theme/useCommonStyles";
import { useForm, Controller } from "react-hook-form";
import { Box, Grid, TextField, Button, Divider } from "@mui/material";
import { toDisplayAmount } from "../../lib/conversions";

export default function YieldsTab({ contract_index }: { contract_index: string }) {
	const classes = useCommonStyles();
	const [yields, setYields] = useState<PagedResponse_Yield>();
	const [page, setPage] = useState(0);
	const [pageSize, setPageSize] = useState(10);
	const [filters, setFilters] = useState<{ token_id?: string }>({});

	const { control, handleSubmit, reset } = useForm<{ token_id?: string }>({
		defaultValues: {},
	});

	useEffect(() => {
		if (contract_index) {
			IndexerService.getAdminIndexerYields(page, pageSize, contract_index, filters.token_id || undefined)
				.then(setYields)
				.catch(console.error);
		}
	}, [contract_index, page, pageSize, filters]);

	const onSubmit = (data: { token_id?: string }) => {
		setFilters({ token_id: data.token_id });
		setPage(0);
	};
	const onClear = () => {
		reset({ token_id: "" });
		setFilters({});
		setPage(0);
	};

	return (
		<Box>
			<Box sx={{ mb: 2, ...classes.filterFormSection }}>
				<form onSubmit={handleSubmit(onSubmit)}>
					<Grid container spacing={2} alignItems="center">
						<Grid item xs={12} sm={10}>
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
							<TableCell sx={classes.tableHeaderCell}>Token ID</TableCell>
							<TableCell sx={classes.tableHeaderCell}>Yield Contract</TableCell>
							<TableCell sx={classes.tableHeaderCell}>Yield Token ID</TableCell>
							<TableCell sx={classes.tableHeaderCell}>Yield Rate</TableCell>
							<TableCell sx={classes.tableHeaderCell}>Yield Type</TableCell>
							<TableCell sx={classes.tableHeaderCell}>Created</TableCell>
						</TableRow>
						{yields?.data?.length ? (
							yields.data.map((item, idx) => (
								<TableRow key={idx}>
									<TableCell>{item.token_id}</TableCell>
									<TableCell>{item.yield_contract_address}</TableCell>
									<TableCell>{item.yield_token_id}</TableCell>
									<TableCell>
										{toDisplayAmount((Number(item.yield_rate_numerator) / Number(item.yield_rate_denominator)).toString(), 6)}
									</TableCell>
									<TableCell>{item.yield_type}</TableCell>
									<TableCell>{item.create_time}</TableCell>
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
				count={yields?.page_count ? yields.page_count * pageSize : 0}
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
