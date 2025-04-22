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
	Tooltip,
	IconButton,
} from "@mui/material";
import { PagedResponse_TokenHolderUser, IndexerService, TokenHolderUser } from "../../apiClient";
import useCommonStyles from "../../theme/useCommonStyles";
import { format } from "date-fns";
import { toDisplayAmount } from "../../lib/conversions";
import { useForm, Controller } from "react-hook-form";
import LockOpenIcon from "@mui/icons-material/LockOpen";
import LockIcon from "@mui/icons-material/Lock";
import SwapHorizIcon from "@mui/icons-material/SwapHoriz";
import DeleteForeverIcon from "@mui/icons-material/DeleteForever";

interface TokenHoldersTableProps {
	contract_index: string;
	onFreezeHolder?: (holder: TokenHolderUser) => void;
	onUnfreezeHolder?: (holder: TokenHolderUser) => void;
	onTransferHolder?: (holder: TokenHolderUser) => void;
	onBurnHolder?: (holder: TokenHolderUser) => void;
	refreshCounter?: number;
}

const TokenHoldersTable: React.FC<TokenHoldersTableProps> = ({
	contract_index,
	onFreezeHolder,
	onUnfreezeHolder,
	onTransferHolder,
	onBurnHolder,
	refreshCounter,
}) => {
	const classes = useCommonStyles();
	const [holders, setHolders] = useState<PagedResponse_TokenHolderUser>();
	const [page, setPage] = useState(0);
	const [pageSize, setPageSize] = useState(10);
	const [loading, setLoading] = useState(false);

	// React Hook Form setup
	const { control, handleSubmit, reset, getValues } = useForm({
		defaultValues: {
			token_id: "",
			holder_address: "",
		},
	});

	const fetchHolders = (page: number, pageSize: number, filters?: { token_id?: string; holder_address?: string }) => {
		setLoading(true);
		IndexerService.getAdminIndexerHolders(
			page,
			pageSize,
			undefined,
			contract_index,
			filters?.token_id || undefined,
			filters?.holder_address || undefined,
		)
			.then(setHolders)
			.finally(() => setLoading(false));
	};

	// Fetch holders on mount and when filters/page/pageSize change
	useEffect(() => {
		const values = getValues();
		fetchHolders(page, pageSize, values);
		// eslint-disable-next-line react-hooks/exhaustive-deps
	}, [contract_index, page, pageSize]);

	const onSubmit = (data: { token_id: string; holder_address: string }) => {
		setPage(0);
		fetchHolders(0, pageSize, data);
	};

	const onClear = () => {
		reset();
		setPage(0);
		fetchHolders(0, pageSize, {});
	};

	if (loading) return <CircularProgress />;

	return (
		<Box>
			<Box sx={{ mb: 2, ...classes.filterFormSection }}>
				<form onSubmit={handleSubmit(onSubmit)}>
					<Grid container spacing={2} alignItems="center">
						<Grid item xs={12} sm={5} md={4}>
							<Controller
								name="token_id"
								control={control}
								render={({ field }) => <TextField {...field} label="Token ID" fullWidth variant="outlined" size="small" />}
							/>
						</Grid>
						<Grid item xs={12} sm={5} md={4}>
							<Controller
								name="holder_address"
								control={control}
								render={({ field }) => (
									<TextField {...field} label="Holder Address" fullWidth variant="outlined" size="small" />
								)}
							/>
						</Grid>
						<Grid item xs={12} sm={2} md={4} display="flex" gap={1} justifyContent={{ xs: "flex-start", sm: "flex-end" }}>
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
							<TableCell sx={classes.tableHeaderCell}>Token ID</TableCell>
							<TableCell sx={classes.tableHeaderCell}>Holder Address</TableCell>
							<TableCell sx={classes.tableHeaderCell}>Email</TableCell>
							<TableCell sx={classes.tableHeaderCell}>Frozen Balance</TableCell>
							<TableCell sx={classes.tableHeaderCell}>Unfrozen Balance</TableCell>
							<TableCell sx={classes.tableHeaderCell}>Created</TableCell>
							<TableCell sx={classes.tableHeaderCell}>Updated</TableCell>
							<TableCell sx={classes.tableHeaderCell}>Actions</TableCell>
						</TableRow>
					</TableHead>
					<TableBody>
						{holders?.data?.length ? (
							holders.data.map((holder) => (
								<TableRow key={holder.token_id + holder.holder_address}>
									<TableCell>{holder.token_id}</TableCell>
									<TableCell>{holder.holder_address}</TableCell>
									<TableCell>
										<Tooltip title={holder.cognito_user_id || "No Cognito ID"} placement="top" arrow>
											<span>{holder.email || "-"}</span>
										</Tooltip>
									</TableCell>
									<TableCell>{toDisplayAmount(holder.frozen_balance, 0)}</TableCell>
									<TableCell>{toDisplayAmount(holder.un_frozen_balance, 0)}</TableCell>
									<TableCell>{format(new Date(holder.create_time), "yyyy-MM-dd HH:mm:ss")}</TableCell>
									<TableCell>{format(new Date(holder.update_time), "yyyy-MM-dd HH:mm:ss")}</TableCell>
									<TableCell>
										<Box display="flex" alignItems="center">
											{onUnfreezeHolder && (
												<Tooltip title="Unfreeze holder" arrow>
													<IconButton onClick={() => onUnfreezeHolder(holder)} color="success">
														<LockOpenIcon />
													</IconButton>
												</Tooltip>
											)}
											{onFreezeHolder && (
												<Tooltip title="Freeze holder" arrow>
													<IconButton onClick={() => onFreezeHolder(holder)} color="warning" sx={{ ml: 1 }}>
														<LockIcon />
													</IconButton>
												</Tooltip>
											)}
											{onTransferHolder && (
												<Tooltip title="Transfer holder" arrow>
													<IconButton onClick={() => onTransferHolder(holder)} color="primary" sx={{ ml: 1 }}>
														<SwapHorizIcon />
													</IconButton>
												</Tooltip>
											)}
											{onBurnHolder && (
												<Tooltip title="Burn holder" arrow>
													<IconButton onClick={() => onBurnHolder(holder)} color="error" sx={{ ml: 1 }}>
														<DeleteForeverIcon />
													</IconButton>
												</Tooltip>
											)}
										</Box>
									</TableCell>
								</TableRow>
							))
						) : (
							<TableRow>
								<TableCell colSpan={8} align="center">
									No holders found.
								</TableCell>
							</TableRow>
						)}
					</TableBody>
				</Table>
			</TableContainer>
			<TablePagination
				component="div"
				count={holders?.page_count || 0 * pageSize}
				page={page}
				onPageChange={(_, newPage) => setPage(newPage)}
				rowsPerPage={pageSize}
				onRowsPerPageChange={(e) => {
					setPageSize(parseInt(e.target.value, 10));
					setPage(0);
				}}
				rowsPerPageOptions={[10, 20, 50]}
			/>
		</Box>
	);
};

export default TokenHoldersTable;
