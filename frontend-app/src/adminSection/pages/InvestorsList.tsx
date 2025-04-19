import { useEffect, useState } from "react";
import {
	ForestProject,
	ForestProjectService,
	IndexerService,
	PagedResponse_InvestorUser,
	SecurityTokenContractType,
} from "../../apiClient";
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
import AccountBalanceIcon from "@mui/icons-material/AccountBalance";
import PersonIcon from "@mui/icons-material/Person";
import useCommonStyles from "../../theme/useCommonStyles";

type FilterFormValues = {
	projectId: string;
	investmentTokenId: string;
	investmentTokenContract: string;
};

export default function InvestorsList() {
	const classes = useCommonStyles();
	const [projects, setProjects] = useState<ForestProject[]>([]);
	const [filters, setFilters] = useSearchParams();
	const [pageSize, setPageSize] = useState<number>(20);
	const [page, setPage] = useState<number>(0);
	const [loading, setLoading] = useState<boolean>(false);
	const [investors, setInvestors] = useState<PagedResponse_InvestorUser>({
		data: [],
		page,
		page_count: 1,
	});

	const { control, handleSubmit, reset } = useForm<FilterFormValues>({
		defaultValues: {
			projectId: filters.get("projectId") || "",
			investmentTokenId: filters.get("investmentTokenId") || "",
			investmentTokenContract: filters.get("investmentTokenContract") || "",
		},
	});

	useEffect(() => {
		ForestProjectService.getAdminForestProjectsList(0, undefined, 1000).then((res) => setProjects(res.data));
	}, []);

	useEffect(() => {
		setLoading(true);
		IndexerService.getAdminIndexerInvestors(
			page,
			pageSize,
			filters.get("projectId") || undefined,
			filters.get("investmentTokenId") || undefined,
			filters.get("investmentTokenContract") || undefined,
		)
			.then((data) => {
				setInvestors(data);
				setLoading(false);
			})
			.catch((error) => {
				console.error("Failed to fetch investors:", error);
				setLoading(false);
			});
	}, [filters, page, pageSize]);

	const onSubmitFilters = (data: FilterFormValues) => {
		const newFilters = new URLSearchParams();
		if (data.projectId) newFilters.set("projectId", data.projectId);
		if (data.investmentTokenId) newFilters.set("investmentTokenId", data.investmentTokenId);
		if (data.investmentTokenContract) newFilters.set("investmentTokenContract", data.investmentTokenContract);
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

	// Helper function to get contract type chip
	const getFundTypeChip = (type: string) => {
		let color: "default" | "primary" | "secondary" | "error" | "info" | "success" | "warning" = "default";

		switch (type) {
			case SecurityTokenContractType.PROPERTY:
				color = "success";
				break;
			case SecurityTokenContractType.BOND:
				color = "info";
				break;
			case SecurityTokenContractType.PROPERTY_PRE_SALE:
				color = "warning";
				break;
			case SecurityTokenContractType.BOND_PRE_SALE:
				color = "secondary";
				break;
			default:
				color = "default";
		}

		return <Chip label={type} size="small" color={color} />;
	};

	return (
		<Box>
			<Typography variant="h4" component="h1" gutterBottom sx={classes.detailsTitle}>
				<AccountBalanceIcon sx={{ mr: 1, verticalAlign: "middle" }} />
				Fund Investors Management
			</Typography>

			<Paper sx={classes.filterFormSection}>
				<form onSubmit={handleSubmit(onSubmitFilters)}>
					<Typography variant="h6" mb={2}>
						Filter Investors
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
								name="investmentTokenId"
								control={control}
								render={({ field }) => <TextField {...field} label="Investment Token ID" fullWidth />}
							/>
						</Grid>
						<Grid item xs={12} sm={4}>
							<Controller
								name="investmentTokenContract"
								control={control}
								render={({ field }) => <TextField {...field} label="Investment Token Contract" fullWidth />}
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
						<Table aria-label="investors table">
							<TableHead>
								<TableRow>
									<TableCell sx={classes.tableHeaderCell}>Account</TableCell>
									<TableCell sx={classes.tableHeaderCell}>Email</TableCell>
									<TableCell sx={classes.tableHeaderCell}>Project</TableCell>
									<TableCell sx={classes.tableHeaderCell}>Contract</TableCell>
									<TableCell sx={classes.tableHeaderCell}>Fund Type</TableCell>
									<TableCell sx={classes.tableHeaderCell}>ID</TableCell>
									<TableCell sx={classes.tableHeaderCell}>Token Amount</TableCell>
									<TableCell sx={classes.tableHeaderCell}>Token Total</TableCell>
									<TableCell sx={classes.tableHeaderCell}>Currency Amount</TableCell>
									<TableCell sx={classes.tableHeaderCell}>Currency Total</TableCell>
									<TableCell sx={classes.tableHeaderCell}>Actions</TableCell>
								</TableRow>
							</TableHead>
							<TableBody>
								{investors.data.length > 0 ? (
									investors.data.map((investor, index) => (
										<TableRow key={index} sx={classes.tableRow}>
											<TableCell>
												<Tooltip title={investor.investor}>
													<Typography noWrap sx={{ maxWidth: 150, display: "flex", alignItems: "center" }}>
														<PersonIcon fontSize="small" sx={{ mr: 0.5, opacity: 0.7 }} />
														{investor.investor}
													</Typography>
												</Tooltip>
											</TableCell>
											<TableCell>
												<Tooltip title={investor.cognito_user_id || ""}>
													<span>{investor.email}</span>
												</Tooltip>
											</TableCell>
											<TableCell>
												<MuiLink component={Link} to={`/admin/projects/${investor.forest_project_id}/details`}>
													{investor.forest_project_name}
												</MuiLink>
											</TableCell>
											<TableCell>
												<MuiLink
													component={Link}
													to={`/admin/projects/${investor.forest_project_id}/contract/${investor.investment_token_contract_address}/details`}
												>
													{investor.investment_token_contract_address}
												</MuiLink>
											</TableCell>
											<TableCell>
												{investor.forest_project_contract_type ? getFundTypeChip(investor.forest_project_contract_type) : null}
											</TableCell>
											<TableCell>
												{investor.investment_token_id && (
													<MuiLink
														component={Link}
														to={`/admin/projects/${investor.forest_project_id}/contract/${investor.investment_token_contract_address}/token/${investor.investment_token_id}/details`}
													>
														{investor.investment_token_id}
													</MuiLink>
												)}
											</TableCell>
											<TableCell>{investor.token_amount}</TableCell>
											<TableCell>{investor.token_amount_total}</TableCell>
											<TableCell>{toDisplayAmount(investor.currency_amount, 6)}</TableCell>
											<TableCell>{toDisplayAmount(investor.currency_amount_total, 6)}</TableCell>
											<TableCell>
												<Button
													component={Link}
													to={`/admin/fund/investment-records?investment_token_contract=${investor.investment_token_contract_address}&investment_token_id=${investor.investment_token_id || ""}&investor=${investor.investor}`}
													size="small"
													variant="outlined"
													color="primary"
													startIcon={<VisibilityIcon fontSize="small" />}
												>
													Records
												</Button>
											</TableCell>
										</TableRow>
									))
								) : (
									<TableRow>
										<TableCell colSpan={10} align="center">
											<Typography variant="body1" sx={{ py: 4 }}>
												No investors found
											</Typography>
										</TableCell>
									</TableRow>
								)}
							</TableBody>
						</Table>
						<TablePagination
							component="div"
							count={investors.page_count * pageSize}
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
