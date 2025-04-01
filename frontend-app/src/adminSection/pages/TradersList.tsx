import { useEffect, useState } from "react";
import { ForestProject, ForestProjectService, PagedResponse_ForestProjectMarketTrader } from "../../apiClient";
import { useSearchParams } from "react-router";
import { useForm, Controller } from "react-hook-form";
import {
	Paper,
	Table,
	TableBody,
	TableCell,
	TableContainer,
	TableHead,
	TableRow,
	Select,
	MenuItem,
	TextField,
	Button,
	Box,
	Typography,
	Grid,
	FormControl,
	InputLabel,
	TablePagination,
	Link as MuiLink,
	Chip,
	Tooltip,
	Divider,
	CircularProgress,
} from "@mui/material";
import { toDisplayAmount } from "../../lib/conversions";
import { Link } from "react-router";
import SearchIcon from "@mui/icons-material/Search";
import RestartAltIcon from "@mui/icons-material/RestartAlt";
import VisibilityIcon from "@mui/icons-material/Visibility";
import StorefrontIcon from "@mui/icons-material/Storefront";
import PersonIcon from "@mui/icons-material/Person";
import ArrowUpwardIcon from "@mui/icons-material/ArrowUpward";
import ArrowDownwardIcon from "@mui/icons-material/ArrowDownward";
import TokenIcon from "@mui/icons-material/Token";
import useCommonStyles from "../../theme/useCommonStyles";

type FilterFormValues = {
	projectId: string;
	tokenId: string;
	tokenContract: string;
};

export default function TradersList() {
	const classes = useCommonStyles();
	const [projects, setProjects] = useState<ForestProject[]>([]);
	const [filters, setFilters] = useSearchParams();
	const [pageSize, setPageSize] = useState<number>(20);
	const [page, setPage] = useState<number>(0);
	const [loading, setLoading] = useState<boolean>(false);
	const [traders, setTraders] = useState<PagedResponse_ForestProjectMarketTrader>({
		data: [],
		page,
		page_count: 1,
	});

	const { control, handleSubmit, reset } = useForm<FilterFormValues>({
		defaultValues: {
			projectId: filters.get("projectId") || "",
			tokenId: filters.get("tokenId") || "",
			tokenContract: filters.get("tokenContract") || "",
		},
	});

	useEffect(() => {
		ForestProjectService.getAdminForestProjectsList(0, undefined, 1000).then((res) => setProjects(res.data));
	}, []);

	useEffect(() => {
		setLoading(true);
		ForestProjectService.getAdminForestProjectsMarketTraderList(
			page,
			filters.get("projectId") || undefined,
			filters.get("tokenId") || undefined,
			filters.get("tokenContract") || undefined,
			pageSize,
		)
			.then((data) => {
				setTraders(data);
				setLoading(false);
			})
			.catch((error) => {
				console.error("Failed to fetch traders:", error);
				setLoading(false);
			});
	}, [filters, page, pageSize]);

	const onSubmitFilters = (data: FilterFormValues) => {
		const newFilters = new URLSearchParams();
		if (data.projectId) newFilters.set("projectId", data.projectId);
		if (data.tokenId) newFilters.set("tokenId", data.tokenId);
		if (data.tokenContract) newFilters.set("tokenContract", data.tokenContract);
		setFilters(newFilters);
		setPage(0);
	};

	const handlePageChange = (_event: unknown, newPage: number) => {
		setPage(newPage);
	};

	const handleRowsPerPageChange = (event: React.ChangeEvent<HTMLInputElement>) => {
		setPageSize(parseInt(event.target.value, 10));
		setPage(0);
	};

	const handleClearFilters = () => {
		setFilters(new URLSearchParams());
		reset();
	};

	return (
		<Box>
			<Typography variant="h4" component="h1" gutterBottom sx={classes.detailsTitle}>
				<StorefrontIcon sx={{ mr: 1, verticalAlign: "middle" }} />
				Market Traders
			</Typography>

			<Paper sx={classes.filterFormSection}>
				<form onSubmit={handleSubmit(onSubmitFilters)}>
					<Typography variant="h6" mb={2}>
						Filter Traders
					</Typography>
					<Divider sx={{ mb: 3 }} />

					<Grid container spacing={2} alignItems="center">
						<Grid item xs={12} sm={4}>
							<FormControl fullWidth>
								<InputLabel id="project-select-label">Project</InputLabel>
								<Controller
									name="projectId"
									control={control}
									render={({ field }) => (
										<Select labelId="project-select-label" label="Project" {...field}>
											<MenuItem value="">
												<em>All Projects</em>
											</MenuItem>
											{projects.map((project) => (
												<MenuItem key={project.id} value={project.id}>
													{project.name}
												</MenuItem>
											))}
										</Select>
									)}
								/>
							</FormControl>
						</Grid>
						<Grid item xs={12} sm={4}>
							<Controller
								name="tokenId"
								control={control}
								render={({ field }) => <TextField {...field} label="Token ID" fullWidth />}
							/>
						</Grid>
						<Grid item xs={12} sm={4}>
							<Controller
								name="tokenContract"
								control={control}
								render={({ field }) => <TextField {...field} label="Token Contract" fullWidth />}
							/>
						</Grid>
						<Grid item xs={12}>
							<Divider sx={{ my: 1 }} />
						</Grid>
						<Grid item xs={12} display="flex" justifyContent="flex-end" gap={2}>
							<Button variant="outlined" color="secondary" onClick={handleClearFilters} startIcon={<RestartAltIcon />}>
								Clear Filters
							</Button>
							<Button type="submit" variant="contained" color="primary" startIcon={<SearchIcon />}>
								Search
							</Button>
						</Grid>
					</Grid>
				</form>
			</Paper>

			<TableContainer component={Paper} sx={classes.tableContainer}>
				{loading ? (
					<Box sx={{ display: "flex", justifyContent: "center", p: 3 }}>
						<CircularProgress />
					</Box>
				) : (
					<>
						<Table aria-label="traders table">
							<TableHead>
								<TableRow>
									<TableCell sx={classes.tableHeaderCell}>Account</TableCell>
									<TableCell sx={classes.tableHeaderCell}>Email</TableCell>
									<TableCell sx={classes.tableHeaderCell}>Project</TableCell>
									<TableCell sx={classes.tableHeaderCell}>Token</TableCell>
									<TableCell sx={classes.tableHeaderCell}>Contract</TableCell>
									<TableCell sx={classes.tableHeaderCell}>Token In</TableCell>
									<TableCell sx={classes.tableHeaderCell}>Token Out</TableCell>
									<TableCell sx={classes.tableHeaderCell}>Currency In</TableCell>
									<TableCell sx={classes.tableHeaderCell}>Currency Out</TableCell>
									<TableCell sx={classes.tableHeaderCell}>Actions</TableCell>
								</TableRow>
							</TableHead>
							<TableBody>
								{traders.data.length > 0 ? (
									traders.data.map((trader, index) => (
										<TableRow key={index} sx={classes.tableRow}>
											<TableCell>
												<Tooltip title={trader.trader.trader}>
													<Typography noWrap sx={{ maxWidth: 150, display: "flex", alignItems: "center" }}>
														<PersonIcon fontSize="small" sx={{ mr: 0.5, opacity: 0.7 }} />
														{trader.trader.trader.substring(0, 10)}...
													</Typography>
												</Tooltip>
											</TableCell>
											<TableCell>
												<Tooltip title={trader.cognito_user_id || ""}>
													<span>{trader.email}</span>
												</Tooltip>
											</TableCell>
											<TableCell>
												<MuiLink component={Link} to={`/admin/projects/${trader.forest_project_id}/details`}>
													{trader.forest_project_name}
												</MuiLink>
											</TableCell>
											<TableCell>
												{trader.trader.token_id ? (
													<Chip
														icon={<TokenIcon fontSize="small" />}
														label={trader.trader.token_id}
														size="small"
														color="primary"
														variant="outlined"
														component={Link}
														to={`/admin/projects/${trader.forest_project_id}/contract/${trader.trader.token_contract_address}/token/${trader.trader.token_id}/details`}
														clickable
													/>
												) : (
													"-"
												)}
											</TableCell>
											<TableCell>
												<MuiLink
													component={Link}
													to={`/admin/projects/${trader.forest_project_id}/contract/${trader.trader.token_contract_address}/details`}
												>
													{trader.trader.token_contract_address.substring(0, 8)}...
												</MuiLink>
											</TableCell>
											<TableCell>
												<Chip
													label={trader.trader.token_in_amount}
													size="small"
													icon={<ArrowDownwardIcon fontSize="small" />}
													color={BigInt(trader.trader.token_in_amount) > 0 ? "success" : "default"}
												/>
											</TableCell>
											<TableCell>
												<Chip
													label={trader.trader.token_out_amount}
													size="small"
													icon={<ArrowUpwardIcon fontSize="small" />}
													color={BigInt(trader.trader.token_out_amount) > 0 ? "warning" : "default"}
												/>
											</TableCell>
											<TableCell>
												<Chip
													label={toDisplayAmount(trader.trader.currency_in_amount, 6, 2)}
													size="small"
													icon={<ArrowDownwardIcon fontSize="small" />}
													color={BigInt(trader.trader.currency_in_amount) > 0 ? "success" : "default"}
												/>
											</TableCell>
											<TableCell>
												<Chip
													label={toDisplayAmount(trader.trader.currency_out_amount, 6, 2)}
													size="small"
													icon={<ArrowUpwardIcon fontSize="small" />}
													color={BigInt(trader.trader.currency_out_amount) > 0 ? "warning" : "default"}
												/>
											</TableCell>
											<TableCell>
												<Button
													component={Link}
													to={`/admin/market/orders?token_contract=${trader.trader.token_contract_address}&token_id=${trader.trader.token_id || ""}&trader=${trader.trader.trader}`}
													size="small"
													variant="outlined"
													color="primary"
													startIcon={<VisibilityIcon fontSize="small" />}
												>
													Orders
												</Button>
											</TableCell>
										</TableRow>
									))
								) : (
									<TableRow>
										<TableCell colSpan={10} align="center">
											<Typography variant="body1" sx={{ py: 4 }}>
												No traders found
											</Typography>
										</TableCell>
									</TableRow>
								)}
							</TableBody>
						</Table>
						<TablePagination
							component="div"
							count={traders.page_count * pageSize}
							page={page}
							onPageChange={handlePageChange}
							rowsPerPage={pageSize}
							onRowsPerPageChange={handleRowsPerPageChange}
							rowsPerPageOptions={[10, 20, 50, 100]}
						/>
					</>
				)}
			</TableContainer>
		</Box>
	);
}
