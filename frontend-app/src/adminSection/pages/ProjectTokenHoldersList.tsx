import { useState, useEffect, useCallback } from "react";
import { useForm, Controller } from "react-hook-form";
import { Link } from "react-router";
import {
	Box,
	Button,
	TextField,
	Grid,
	Paper,
	Typography,
	FormControl,
	InputLabel,
	MenuItem,
	Select,
	Table,
	TableBody,
	TableCell,
	TableContainer,
	TableHead,
	TableRow,
	TablePagination,
	CircularProgress,
	Tooltip,
	Link as MuiLink,
	Chip,
	Stack,
	Divider,
} from "@mui/material";
import { ForestProjectService, ForestProjectTokenHolder } from "../../apiClient";
import SearchIcon from "@mui/icons-material/Search";
import RestartAltIcon from "@mui/icons-material/RestartAlt";
import AcUnitIcon from "@mui/icons-material/AcUnit";
import LocalFireDepartmentIcon from "@mui/icons-material/LocalFireDepartment";
import useCommonStyles from "../../theme/useCommonStyles";

// Add SecurityTokenContractType enum
enum SecurityTokenContractType {
	PROPERTY = "Property",
	BOND = "Bond",
	PROPERTY_PRE_SALE = "PropertyPreSale",
	BOND_PRE_SALE = "BondPreSale",
}

interface ForestProject {
	id: string;
	name: string;
}

interface FilterForm {
	cis2_address?: string;
	token_id?: string;
	holder_address?: string;
	project_id?: string;
}

const ProjectTokenHoldersList = () => {
	const classes = useCommonStyles();
	const [tokenHolders, setTokenHolders] = useState<ForestProjectTokenHolder[]>([]);
	const [projects, setProjects] = useState<ForestProject[]>([]);
	const [loading, setLoading] = useState<boolean>(false);
	const [totalRows, setTotalRows] = useState<number>(0);
	const [pageSize, setPageSize] = useState<number>(25);
	const [page, setPage] = useState<number>(0);

	const { control, handleSubmit, reset } = useForm<FilterForm>({
		defaultValues: {
			cis2_address: "",
			token_id: "",
			holder_address: "",
			project_id: "",
		},
	});

	useEffect(() => {
		const loadProjects = async () => {
			try {
				const response = await ForestProjectService.getAdminForestProjectsList(0, undefined, 1000);
				setProjects(response.data);
			} catch (error) {
				console.error("Failed to load projects:", error);
			}
		};

		loadProjects();
	}, []);

	const fetchTokenHolders = useCallback(async (filters: FilterForm, page: number, pageSize: number) => {
		setLoading(true);
		try {
			const response = await ForestProjectService.getAdminForestProjectsTokenHoldersList(
				page,
				filters.cis2_address || undefined,
				filters.token_id || undefined,
				filters.holder_address || undefined,
				filters.project_id || undefined,
				pageSize,
			);

			// Map the response data to include project IDs
			// Note: This assumes the API response includes forest_project_id
			// If it doesn't, we would need to modify the API response or fetch project IDs separately
			setTokenHolders(response.data || []);
			setTotalRows(response.page_count * pageSize || 0);
		} catch (error) {
			console.error("Failed to fetch token holders:", error);
		} finally {
			setLoading(false);
		}
	}, []);

	useEffect(() => {
		fetchTokenHolders({}, page, pageSize);
	}, [fetchTokenHolders, page, pageSize]);

	const onSubmit = (data: FilterForm) => {
		setPage(0);
		fetchTokenHolders(data, 0, pageSize);
	};

	const handleReset = () => {
		reset();
		setPage(0);
		fetchTokenHolders({}, 0, pageSize);
	};

	const handleChangePage = (_event: unknown, newPage: number) => {
		setPage(newPage);
		fetchTokenHolders({}, newPage, pageSize);
	};

	const handleChangeRowsPerPage = (event: React.ChangeEvent<HTMLInputElement>) => {
		setPageSize(parseInt(event.target.value, 10));
		setPage(0);
		fetchTokenHolders({}, 0, parseInt(event.target.value, 10));
	};

	// Helper function to determine contract type chip color
	const getContractTypeChip = (contractType: string) => {
		let color: "default" | "primary" | "secondary" | "error" | "info" | "success" | "warning" = "default";

		switch (contractType) {
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
			case "Carbon Credit": // Keep support for Carbon Credit if it exists
				color = "success";
				break;
			default:
				color = "default";
		}

		return <Chip label={contractType} size="small" color={color} />;
	};

	return (
		<Box>
			<Typography variant="h4" component="h1" gutterBottom sx={classes.detailsTitle}>
				Token Holders Management
			</Typography>

			<Paper sx={classes.filterFormSection}>
				<form onSubmit={handleSubmit(onSubmit)}>
					<Typography variant="h6" mb={2}>
						Filter Token Holders
					</Typography>
					<Divider sx={{ mb: 3 }} />

					<Grid container spacing={2}>
						<Grid item xs={12} sm={6} md={3}>
							<Controller
								name="cis2_address"
								control={control}
								render={({ field }) => <TextField {...field} fullWidth label="CIS2 Address" variant="outlined" />}
							/>
						</Grid>
						<Grid item xs={12} sm={6} md={3}>
							<Controller
								name="token_id"
								control={control}
								render={({ field }) => <TextField {...field} fullWidth label="Token ID" variant="outlined" />}
							/>
						</Grid>
						<Grid item xs={12} sm={6} md={3}>
							<Controller
								name="holder_address"
								control={control}
								render={({ field }) => <TextField {...field} fullWidth label="Holder Address" variant="outlined" />}
							/>
						</Grid>
						<Grid item xs={12} sm={6} md={3}>
							<Controller
								name="project_id"
								control={control}
								render={({ field }) => (
									<FormControl fullWidth>
										<InputLabel id="project-select-label">Project</InputLabel>
										<Select {...field} labelId="project-select-label" label="Project">
											<MenuItem value="">
												<em>All</em>
											</MenuItem>
											{projects.map((project) => (
												<MenuItem key={project.id} value={project.id}>
													{project.name}
												</MenuItem>
											))}
										</Select>
									</FormControl>
								)}
							/>
						</Grid>
						<Grid item xs={12}>
							<Divider sx={{ my: 1 }} />
						</Grid>
						<Grid item xs={12} display="flex" justifyContent="flex-end" gap={2}>
							<Button variant="outlined" color="secondary" onClick={handleReset} startIcon={<RestartAltIcon />}>
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
						<Table sx={{ minWidth: 650 }} aria-label="token holders table">
							<TableHead>
								<TableRow>
									<TableCell sx={classes.tableHeaderCell}>Holder Address</TableCell>
									<TableCell sx={classes.tableHeaderCell}>Email</TableCell>
									<TableCell sx={classes.tableHeaderCell}>Contract</TableCell>
									<TableCell sx={classes.tableHeaderCell}>Type</TableCell>
									<TableCell sx={classes.tableHeaderCell}>Token ID</TableCell>
									<TableCell sx={classes.tableHeaderCell}>Project</TableCell>
									<TableCell sx={classes.tableHeaderCell}>Locked Balance</TableCell>
									<TableCell sx={classes.tableHeaderCell}>Unlocked Balance</TableCell>
									<TableCell sx={classes.tableHeaderCell}>Actions</TableCell>
								</TableRow>
							</TableHead>
							<TableBody>
								{tokenHolders.length > 0 ? (
									tokenHolders.map((holder, index) => {
										return (
											<TableRow key={index} sx={classes.tableRow}>
												<TableCell>
													<Tooltip title={holder.holder_address}>
														<Typography noWrap sx={{ maxWidth: 180 }}>
															{holder.holder_address}
														</Typography>
													</Tooltip>
												</TableCell>
												<TableCell>{holder.email}</TableCell>
												<TableCell>
													{holder.forest_project_id && (
														<Tooltip title="View Contract Details">
															<Link
																to={`/admin/projects/${holder.forest_project_id}/contract/${holder.cis2_address}/details`}
																style={{ textDecoration: "underline", color: "blue" }}
															>
																{holder.cis2_address.substring(0, 10)}...
															</Link>
														</Tooltip>
													)}
													{!holder.forest_project_id && holder.cis2_address.substring(0, 10) + "..."}
												</TableCell>
												<TableCell>{getContractTypeChip(holder.contract_type)}</TableCell>
												<TableCell>
													{holder.forest_project_id && (
														<Tooltip title="View Token Details">
															<Link
																to={`/admin/projects/${holder.forest_project_id}/contract/${holder.cis2_address}/token/${holder.token_id}/details`}
																style={{ textDecoration: "underline", color: "blue" }}
															>
																{holder.token_id}
															</Link>
														</Tooltip>
													)}
													{!holder.forest_project_id && holder.token_id}
												</TableCell>
												<TableCell>{holder.forest_project_name}</TableCell>
												<TableCell>
													<Chip
														label={holder.frozen_balance}
														icon={<AcUnitIcon fontSize="small" />}
														color={BigInt(holder.frozen_balance) > 0 ? "primary" : "default"}
													/>
												</TableCell>
												<TableCell>
													<Chip
														label={holder.un_frozen_balance}
														icon={<LocalFireDepartmentIcon fontSize="small" />}
														color={BigInt(holder.un_frozen_balance) > 0 ? "success" : "default"}
													/>
												</TableCell>
												<TableCell>
													<Stack direction="row" spacing={1}>
														<MuiLink
															component={Link}
															to={`/admin/projects/token-holders/balance-updates/${holder.cis2_address}/${holder.token_id}/${holder.holder_address}`}
														>
															Balance Updates
														</MuiLink>
													</Stack>
												</TableCell>
											</TableRow>
										);
									})
								) : (
									<TableRow>
										<TableCell colSpan={9} align="center">
											<Typography variant="body1" sx={{ py: 4 }}>
												No token holders found
											</Typography>
										</TableCell>
									</TableRow>
								)}
							</TableBody>
						</Table>
						<TablePagination
							rowsPerPageOptions={[5, 10, 25, 50, 100]}
							component="div"
							count={totalRows}
							rowsPerPage={pageSize}
							page={page}
							onPageChange={handleChangePage}
							onRowsPerPageChange={handleChangeRowsPerPage}
						/>
					</>
				)}
			</TableContainer>
		</Box>
	);
};

export default ProjectTokenHoldersList;
