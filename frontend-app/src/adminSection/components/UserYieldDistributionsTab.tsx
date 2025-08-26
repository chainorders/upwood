import { useEffect, useState } from "react";
import { Table, TableBody, TableCell, TableContainer, TableRow, Paper } from "@mui/material";
import TablePagination from "@mui/material/TablePagination";
import { IndexerService, PagedResponse_UserYieldDistribution } from "../../apiClient";
import useCommonStyles from "../../theme/useCommonStyles";
import { useForm, Controller } from "react-hook-form";
import { Box, Grid, TextField, Button, Divider } from "@mui/material";

export default function UserYieldDistributionsTab({ contract_index }: { contract_index: string }) {
	const classes = useCommonStyles();
	const [distributions, setDistributions] = useState<PagedResponse_UserYieldDistribution>();
	const [page, setPage] = useState(0);
	const [pageSize, setPageSize] = useState(10);
	const [filters, setFilters] = useState<{ token_id?: string; to_address?: string }>({});

	const { control, handleSubmit, reset } = useForm<{ token_id?: string; to_address?: string }>({
		defaultValues: {},
	});

	useEffect(() => {
		if (contract_index) {
			IndexerService.getAdminIndexerYieldDistributions(
				page,
				pageSize,
				undefined,
				contract_index,
				filters.to_address || undefined,
				undefined,
				filters.token_id || undefined,
			)
				.then(setDistributions)
				.catch(console.error);
		}
	}, [contract_index, page, pageSize, filters]);

	const onSubmit = (data: { token_id?: string; to_address?: string }) => {
		setFilters({ token_id: data.token_id, to_address: data.to_address });
		setPage(0);
	};
	const onClear = () => {
		reset({ token_id: "", to_address: "" });
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
								name="token_id"
								control={control}
								render={({ field }) => <TextField {...field} label="Token ID" fullWidth variant="outlined" size="small" />}
							/>
						</Grid>
						<Grid item xs={12} sm={5}>
							<Controller
								name="to_address"
								control={control}
								render={({ field }) => <TextField {...field} label="To Address" fullWidth variant="outlined" size="small" />}
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
							<TableCell sx={classes.tableHeaderCell}>ID</TableCell>
							<TableCell sx={classes.tableHeaderCell}>To Address</TableCell>
							<TableCell sx={classes.tableHeaderCell}>Email</TableCell>
							<TableCell sx={classes.tableHeaderCell}>From Token Ver</TableCell>
							<TableCell sx={classes.tableHeaderCell}>To Token Ver</TableCell>
							<TableCell sx={classes.tableHeaderCell}>Yield Contract</TableCell>
							<TableCell sx={classes.tableHeaderCell}>Yield Token ID</TableCell>
							<TableCell sx={classes.tableHeaderCell}>Yield Amount</TableCell>
							<TableCell sx={classes.tableHeaderCell}>Created</TableCell>
						</TableRow>
						{distributions?.data?.length ? (
							distributions.data.map((item) => (
								<TableRow key={item.id}>
									<TableCell>{item.id}</TableCell>
									<TableCell>{item.to_address}</TableCell>
									<TableCell>{item.email}</TableCell>
									<TableCell>{item.from_token_version}</TableCell>
									<TableCell>{item.to_token_version}</TableCell>
									<TableCell>{item.yield_contract_address}</TableCell>
									<TableCell>{item.yield_token_id}</TableCell>
									<TableCell>{item.yield_amount}</TableCell>
									<TableCell>{item.create_time}</TableCell>
								</TableRow>
							))
						) : (
							<TableRow>
								<TableCell colSpan={8} align="center">
									No data
								</TableCell>
							</TableRow>
						)}
					</TableBody>
				</Table>
			</TableContainer>
			<TablePagination
				component="div"
				count={distributions?.page_count ? distributions.page_count * pageSize : 0}
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
