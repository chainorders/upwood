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
} from "@mui/material";
import { ForestProjectService, ForestProjectTokenHolder } from "../../apiClient";

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

const TokenHoldersList = () => {
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

	return (
		<Box>
			<Typography variant="h4" component="h1" gutterBottom>
				Token Holders
			</Typography>

			<Paper sx={{ p: 2, mb: 3 }}>
				<form onSubmit={handleSubmit(onSubmit)}>
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
						<Grid item xs={12} display="flex" justifyContent="flex-end" gap={2}>
							<Button variant="outlined" color="secondary" onClick={handleReset}>
								Clear
							</Button>
							<Button type="submit" variant="contained" color="primary">
								Search
							</Button>
						</Grid>
					</Grid>
				</form>
			</Paper>

			<TableContainer component={Paper}>
				{loading ? (
					<Box sx={{ display: "flex", justifyContent: "center", p: 3 }}>
						<CircularProgress />
					</Box>
				) : (
					<>
						<Table sx={{ minWidth: 650 }} aria-label="token holders table">
							<TableHead>
								<TableRow>
									<TableCell>Holder Address</TableCell>
									<TableCell>Email</TableCell>
									<TableCell>Contract</TableCell>
									<TableCell>Type</TableCell>
									<TableCell>Token ID</TableCell>
									<TableCell>Project</TableCell>
									<TableCell>Locked Balance</TableCell>
									<TableCell>Unlocked Balance</TableCell>
									<TableCell>Actions</TableCell>
								</TableRow>
							</TableHead>
							<TableBody>
								{tokenHolders.length > 0 ? (
									tokenHolders.map((holder, index) => (
										<TableRow key={index}>
											<TableCell>{holder.holder_address}</TableCell>
											<TableCell>{holder.email}</TableCell>
											<TableCell>
												{holder.forest_project_id && (
													<Tooltip title="View Contract Details">
														<Link
															to={`/admin/projects/${holder.forest_project_id}/contract/${holder.cis2_address}/details`}
															style={{ textDecoration: "underline", color: "blue" }}
														>
															{holder.cis2_address}
														</Link>
													</Tooltip>
												)}
												{!holder.forest_project_id && holder.cis2_address}
											</TableCell>
											<TableCell>{holder.contract_type}</TableCell>
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
											<TableCell>{holder.frozen_balance}</TableCell>
											<TableCell>{holder.un_frozen_balance}</TableCell>
											<TableCell>
												<MuiLink
													component={Link}
													to={`/admin/projects/token-holders/balance-updates/${holder.cis2_address}/${holder.token_id}/${holder.holder_address}`}
												>
													Balance Updates
												</MuiLink>
											</TableCell>
										</TableRow>
									))
								) : (
									<TableRow>
										<TableCell colSpan={9} align="center">
											No token holders found
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

export default TokenHoldersList;
