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
} from "@mui/material";
import { toDisplayAmount } from "../../lib/conversions";
import { Link } from "react-router";

type FilterFormValues = {
	projectId: string;
	tokenId: string;
	tokenContract: string;
};

export default function TradersList() {
	const [projects, setProjects] = useState<ForestProject[]>([]);
	const [filters, setFilters] = useSearchParams();
	const [pageSize, setPageSize] = useState<number>(20);
	const [page, setPage] = useState<number>(0);
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
		ForestProjectService.getAdminForestProjectsMarketTraderList(
			page,
			filters.get("projectId") || undefined,
			filters.get("tokenId") || undefined,
			filters.get("tokenContract") || undefined,
			pageSize,
		).then(setTraders);
	}, [filters, page, pageSize]);

	const onSubmitFilters = (data: FilterFormValues) => {
		const newFilters = new URLSearchParams();
		if (data.projectId) newFilters.set("projectId", data.projectId);
		if (data.tokenId) newFilters.set("tokenId", data.tokenId);
		if (data.tokenContract) newFilters.set("tokenContract", data.tokenContract);
		setFilters(newFilters);
		setPage(0);
	};

	const handlePageChange = (event: unknown, newPage: number) => {
		setPage(newPage); // TablePagination is zero-indexed, but our API is 1-indexed
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
		<Box sx={{ p: 3 }}>
			<Typography variant="h4" component="h1" gutterBottom>
				Market Traders
			</Typography>

			<Paper sx={{ p: 2, mb: 3 }}>
				<form onSubmit={handleSubmit(onSubmitFilters)}>
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
						<Grid item xs={12} sm={3}>
							<Controller
								name="tokenId"
								control={control}
								render={({ field }) => <TextField {...field} label="Token ID" type="number" fullWidth />}
							/>
						</Grid>
						<Grid item xs={12} sm={3}>
							<Controller
								name="tokenContract"
								control={control}
								render={({ field }) => <TextField {...field} label="Token Contract" type="number" fullWidth />}
							/>
						</Grid>
						<Grid item xs={12} sm={2}>
							<Box sx={{ display: "flex", gap: 1 }}>
								<Button variant="contained" type="submit">
									Filter
								</Button>
								<Button variant="outlined" onClick={handleClearFilters}>
									Clear
								</Button>
							</Box>
						</Grid>
					</Grid>
				</form>
			</Paper>

			<TableContainer component={Paper}>
				<Table aria-label="traders table">
					<TableHead>
						<TableRow>
							<TableCell>Account</TableCell>
							<TableCell>Email</TableCell>
							<TableCell>Project Name</TableCell>
							<TableCell>Token ID</TableCell>
							<TableCell>Token Contract</TableCell>
							<TableCell>Token In</TableCell>
							<TableCell>Token Out</TableCell>
							<TableCell>Currency In</TableCell>
							<TableCell>Currency Out</TableCell>
						</TableRow>
					</TableHead>
					<TableBody>
						{traders.data.length > 0 ? (
							traders.data.map((trader, index) => (
								<TableRow key={index}>
									<TableCell>{trader.trader.trader}</TableCell>
									<TableCell title={trader.cognito_user_id}>{trader.email}</TableCell>
									<TableCell>
										<MuiLink component={Link} to={`/admin/projects/${trader.forest_project_id}/details`}>
											{trader.forest_project_name}
										</MuiLink>
									</TableCell>
									<TableCell>
										{trader.trader.token_id && (
											<MuiLink
												component={Link}
												to={`/admin/projects/${trader.forest_project_id}/contract/${trader.trader.token_contract_address}/token/${trader.trader.token_id}/details`}
											>
												{trader.trader.token_id}
											</MuiLink>
										)}
									</TableCell>
									<TableCell>
										<MuiLink
											component={Link}
											to={`/admin/projects/${trader.forest_project_id}/contract/${trader.trader.token_contract_address}/details`}
										>
											{`${trader.trader.token_contract_address.substring(0, 8)}...`}
										</MuiLink>
									</TableCell>
									<TableCell>{trader.trader.token_in_amount}</TableCell>
									<TableCell>{trader.trader.token_out_amount}</TableCell>
									<TableCell>{toDisplayAmount(trader.trader.currency_in_amount, 6, 2)}</TableCell>
									<TableCell>{toDisplayAmount(trader.trader.currency_out_amount, 6, 2)}</TableCell>
								</TableRow>
							))
						) : (
							<TableRow>
								<TableCell colSpan={11} align="center">
									No traders found
								</TableCell>
							</TableRow>
						)}
					</TableBody>
				</Table>
				<TablePagination
					component="div"
					count={traders.page_count * pageSize} // Approximate total count
					page={page}
					onPageChange={handlePageChange}
					rowsPerPage={pageSize}
					onRowsPerPageChange={handleRowsPerPageChange}
					rowsPerPageOptions={[10, 20, 50, 100]}
				/>
			</TableContainer>
		</Box>
	);
}
