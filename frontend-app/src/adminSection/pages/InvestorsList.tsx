import { useEffect, useState } from "react";
import {
	ForestProject,
	ForestProjectService,
	PagedResponse_ForestProjectFundInvestor,
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
} from "@mui/material";
import { toDisplayAmount } from "../../lib/conversions";
import { Link } from "react-router";

type FilterFormValues = {
	projectId: string;
	investmentTokenId: string;
	investmentTokenContract: string;
};

export default function InvestorsList() {
	const [projects, setProjects] = useState<ForestProject[]>([]);
	const [filters, setFilters] = useSearchParams();
	const [pageSize, setPageSize] = useState<number>(20);
	const [page, setPage] = useState<number>(0);
	const [investors, setInvestors] = useState<PagedResponse_ForestProjectFundInvestor>({
		data: [],
		page,
		page_count: 1,
	});

	const { control, handleSubmit } = useForm<FilterFormValues>({
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
		ForestProjectService.getAdminForestProjectsFundInvestorList(
			page,
			filters.get("projectId") || undefined,
			filters.get("investmentTokenId") || undefined,
			filters.get("investmentTokenContract") || undefined,
			pageSize,
		).then(setInvestors);
	}, [filters, page, pageSize]);

	const onSubmitFilters = (data: FilterFormValues) => {
		const newFilters = new URLSearchParams();
		if (data.projectId) newFilters.set("projectId", data.projectId);
		if (data.investmentTokenId) newFilters.set("investmentTokenId", data.investmentTokenId);
		if (data.investmentTokenContract) newFilters.set("investmentTokenContract", data.investmentTokenContract);
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
	};

	return (
		<Box sx={{ p: 3 }}>
			<Typography variant="h4" component="h1" gutterBottom>
				Fund Investors
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
								name="investmentTokenId"
								control={control}
								render={({ field }) => <TextField {...field} label="Investment Token ID" type="number" fullWidth />}
							/>
						</Grid>
						<Grid item xs={12} sm={3}>
							<Controller
								name="investmentTokenContract"
								control={control}
								render={({ field }) => <TextField {...field} label="Investment Token Contract" type="number" fullWidth />}
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
				<Table aria-label="investors table">
					<TableHead>
						<TableRow>
							<TableCell>Account</TableCell>
							<TableCell>Email</TableCell>
							<TableCell>Project</TableCell>
							<TableCell>Contract</TableCell>
							<TableCell>ID</TableCell>
							<TableCell>Token Amount</TableCell>
							<TableCell>Token Amount Total</TableCell>
							<TableCell>Currency Amount</TableCell>
							<TableCell>Currency Amount Total</TableCell>
						</TableRow>
					</TableHead>
					<TableBody>
						{investors.data.length > 0 ? (
							investors.data.map((investor, index) => (
								<TableRow key={index}>
									<TableCell>{investor.investor.investor}</TableCell>
									<TableCell title={investor.cognito_user_id}>{investor.email}</TableCell>
									<TableCell title={investor.forest_project_id}>
										<MuiLink component={Link} to={`/admin/projects/${investor.forest_project_id}/details`}>
											{investor.forest_project_name}
										</MuiLink>
									</TableCell>
									<TableCell>
										<MuiLink
											component={Link}
											to={`/admin/projects/${investor.forest_project_id}/contract/${investor.investor.investment_token_contract_address}/details`}
										>
											{investor.investor.investment_token_contract_address.substring(0, 8)}...
										</MuiLink>{" "}
										({investor.fund_type})
									</TableCell>
									<TableCell>
										{investor.investor.investment_token_id && (
											<MuiLink
												component={Link}
												to={`/admin/projects/${investor.forest_project_id}/contract/${investor.investor.investment_token_contract_address}/token/${investor.investor.investment_token_id}/details`}
											>
												{investor.investor.investment_token_id}
											</MuiLink>
										)}
									</TableCell>
									<TableCell>{investor.investor.token_amount}</TableCell>
									<TableCell>{investor.investor.token_amount_total}</TableCell>
									<TableCell>{toDisplayAmount(investor.investor.currency_amount, 6, 2)}</TableCell>
									<TableCell>{toDisplayAmount(investor.investor.currency_amount_total, 6, 2)}</TableCell>
								</TableRow>
							))
						) : (
							<TableRow>
								<TableCell colSpan={9} align="center">
									No investors found
								</TableCell>
							</TableRow>
						)}
					</TableBody>
				</Table>
				<TablePagination
					component="div"
					count={investors.page_count * pageSize} // Approximate total count
					page={page} // TablePagination is zero-indexed, but our API is 1-indexed
					onPageChange={handlePageChange}
					rowsPerPage={pageSize}
					onRowsPerPageChange={handleRowsPerPageChange}
					rowsPerPageOptions={[10, 20, 50, 100]}
				/>
			</TableContainer>
		</Box>
	);
}
