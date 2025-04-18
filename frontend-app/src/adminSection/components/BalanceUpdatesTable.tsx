import { useEffect, useState } from "react";
import {
	TableContainer,
	Table,
	TableHead,
	TableRow,
	TableCell,
	TableBody,
	Paper,
	TablePagination,
	Box,
	CircularProgress,
	TextField,
	Button,
	Grid,
	Divider,
	MenuItem,
	Select,
	InputLabel,
	FormControl,
} from "@mui/material";
import { useForm, Controller } from "react-hook-form";
import { format } from "date-fns";
import { IndexerService, TokenHolderBalanceUpdateType } from "../../apiClient";
import useCommonStyles from "../../theme/useCommonStyles";
import { PagedResponse_TokenHolderUserBalanceUpdate } from "../../apiClient/models/PagedResponse_TokenHolderUserBalanceUpdate";
import { TokenHolderUserBalanceUpdate } from "../../apiClient/models/TokenHolderUserBalanceUpdate";

interface BalanceUpdatesTableProps {
	contract_index: string;
}
interface FilterFormValues {
	token_id?: string;
	holder_address?: string;
	update_type?: TokenHolderBalanceUpdateType;
}

const BalanceUpdatesTable: React.FC<BalanceUpdatesTableProps> = ({ contract_index }) => {
	const classes = useCommonStyles();
	const [updates, setUpdates] = useState<PagedResponse_TokenHolderUserBalanceUpdate>();
	const [filters, setFilters] = useState<FilterFormValues & { page: number; pageSize: number }>({
		page: 0,
		pageSize: 10,
	});
	const [loading, setLoading] = useState(false);

	const { control, handleSubmit, reset, getValues } = useForm<{
		token_id?: string;
		holder_address?: string;
		update_type?: TokenHolderBalanceUpdateType;
	}>({ defaultValues: {} });

	useEffect(() => {
		setLoading(true);
		IndexerService.getAdminIndexerCis2BalanceUpdatesList(
			filters.page,
			contract_index,
			filters.token_id || undefined,
			filters.holder_address || undefined,
			filters.update_type || undefined,
			filters.pageSize,
		)
			.then(setUpdates)
			.finally(() => setLoading(false));
	}, [contract_index, getValues, filters]);

	const onSubmit = (data: FilterFormValues) => {
		setFilters((prev) => ({
			...prev,
			token_id: data.token_id,
			holder_address: data.holder_address,
			update_type: data.update_type,
			page: 0,
		}));
	};
	const onClear = () => {
		reset();
		setFilters({ page: 0, pageSize: filters.pageSize });
	};
	const setPage = (newPage: number) => {
		setFilters((prev) => ({ ...prev, page: newPage }));
	};
	const setPageSize = (newPageSize: number) => {
		setFilters((prev) => ({ ...prev, pageSize: newPageSize, page: 0 }));
	};

	if (loading) return <CircularProgress />;

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
								name="holder_address"
								control={control}
								render={({ field }) => (
									<TextField {...field} label="Holder Address" fullWidth variant="outlined" size="small" />
								)}
							/>
						</Grid>
						<Grid item xs={12} sm={4}>
							<Controller
								name="update_type"
								control={control}
								render={({ field }) => (
									<FormControl fullWidth size="small">
										<InputLabel>Update Type</InputLabel>
										<Select {...field} label="Update Type">
											<MenuItem value="">All</MenuItem>
											<MenuItem value={TokenHolderBalanceUpdateType.MINT}>Mint</MenuItem>
											<MenuItem value={TokenHolderBalanceUpdateType.BURN}>Burn</MenuItem>
											<MenuItem value={TokenHolderBalanceUpdateType.TRANSFER_IN}>Transfer In</MenuItem>
											<MenuItem value={TokenHolderBalanceUpdateType.TRANSFER_OUT}>Transfer Out</MenuItem>
											<MenuItem value={TokenHolderBalanceUpdateType.FREEZE}>Freeze</MenuItem>
											<MenuItem value={TokenHolderBalanceUpdateType.UN_FREEZE}>Unfreeze</MenuItem>
										</Select>
									</FormControl>
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
					<TableHead>
						<TableRow>
							<TableCell sx={classes.tableHeaderCell}>Block Height</TableCell>
							<TableCell sx={classes.tableHeaderCell}>Txn Index</TableCell>
							<TableCell sx={classes.tableHeaderCell}>Token ID</TableCell>
							<TableCell sx={classes.tableHeaderCell}>Holder Address</TableCell>
							<TableCell sx={classes.tableHeaderCell}>Amount</TableCell>
							<TableCell sx={classes.tableHeaderCell}>Frozen Balance</TableCell>
							<TableCell sx={classes.tableHeaderCell}>Unfrozen Balance</TableCell>
							<TableCell sx={classes.tableHeaderCell}>Txn Sender</TableCell>
							<TableCell sx={classes.tableHeaderCell}>Txn Instigator</TableCell>
							<TableCell sx={classes.tableHeaderCell}>Update Type</TableCell>
							<TableCell sx={classes.tableHeaderCell}>Created</TableCell>
							<TableCell sx={classes.tableHeaderCell}>Email</TableCell>
						</TableRow>
					</TableHead>
					<TableBody>
						{updates?.data?.length ? (
							updates.data.map((update: TokenHolderUserBalanceUpdate) => (
								<TableRow key={update.id}>
									<TableCell>{update.block_height}</TableCell>
									<TableCell>{update.txn_index}</TableCell>
									<TableCell>{update.token_id}</TableCell>
									<TableCell>{update.holder_address}</TableCell>
									<TableCell>{update.amount}</TableCell>
									<TableCell>{update.frozen_balance}</TableCell>
									<TableCell>{update.un_frozen_balance}</TableCell>
									<TableCell>{update.txn_sender}</TableCell>
									<TableCell>{update.txn_instigator}</TableCell>
									<TableCell>{update.update_type}</TableCell>
									<TableCell>{format(new Date(update.create_time), "yyyy-MM-dd HH:mm:ss")}</TableCell>
									<TableCell>{update.email || "-"}</TableCell>
								</TableRow>
							))
						) : (
							<TableRow>
								<TableCell colSpan={12} align="center">
									No balance updates found.
								</TableCell>
							</TableRow>
						)}
					</TableBody>
				</Table>
			</TableContainer>
			<TablePagination
				component="div"
				count={updates?.page_count || 0 * filters.pageSize}
				page={filters.page}
				onPageChange={(_, newPage) => setPage(newPage)}
				rowsPerPage={filters.pageSize}
				onRowsPerPageChange={(e) => {
					setPageSize(parseInt(e.target.value, 10));
					setPage(0);
				}}
				rowsPerPageOptions={[10, 20, 50]}
			/>
		</Box>
	);
};

export default BalanceUpdatesTable;
