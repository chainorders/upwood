import { useState, useEffect } from "react";
import { useSearchParams } from "react-router";
import { useForm, Controller } from "react-hook-form";
import {
	Box,
	Typography,
	Paper,
	Table,
	TableBody,
	TableCell,
	TableContainer,
	TableHead,
	TableRow,
	TablePagination,
	TextField,
	Button,
	Grid,
	CircularProgress,
	Chip,
} from "@mui/material";
import { InvestmentRecord } from "../../apiClient/models/InvestmentRecord";
import { IndexerService } from "../../apiClient/services/IndexerService";
import { InvestmentRecordType } from "../../apiClient/models/InvestmentRecordType";

// Interface for filter form inputs
interface FilterFormInputs {
	investmentTokenContract: string;
	investmentTokenId: string;
	investor: string;
}

export default function InvestmentRecordsList() {
	// State for investment records data and pagination
	const [records, setRecords] = useState<InvestmentRecord[]>([]);
	const [totalCount, setTotalCount] = useState(0);
	const [page, setPage] = useState(0);
	const [pageSize, setPageSize] = useState(10);
	const [loading, setLoading] = useState(false);

	// URL search params
	const [searchParams, setSearchParams] = useSearchParams();

	// React Hook Form setup
	const { control, handleSubmit, reset } = useForm<FilterFormInputs>({
		defaultValues: {
			investmentTokenContract: searchParams.get("investment_token_contract") || "",
			investmentTokenId: searchParams.get("investment_token_id") || "",
			investor: searchParams.get("investor") || "",
		},
	});

	// Fetch data function
	const fetchInvestmentRecords = async (
		page: number,
		pageSize: number,
		investmentTokenContract?: string,
		investmentTokenId?: string,
		investor?: string,
	) => {
		setLoading(true);
		try {
			const response = await IndexerService.getAdminIndexerInvestmentRecords(
				page,
				pageSize,
				investmentTokenContract || undefined,
				investmentTokenId || undefined,
				investor || undefined,
			);
			setRecords(response.data || []);
			setTotalCount(response.page_count * pageSize || 0);
		} catch (error) {
			console.error("Failed to fetch investment records:", error);
		} finally {
			setLoading(false);
		}
	};

	// Handle form submission
	const onSubmit = (data: FilterFormInputs) => {
		setPage(0); // Reset to first page on new search

		// Update search params
		const params = new URLSearchParams();
		if (data.investmentTokenContract) params.set("investment_token_contract", data.investmentTokenContract);
		if (data.investmentTokenId) params.set("investment_token_id", data.investmentTokenId);
		if (data.investor) params.set("investor", data.investor);
		params.set("page", "0");
		params.set("page_size", pageSize.toString());

		setSearchParams(params);
	};

	// Clear filters
	const handleClearFilters = () => {
		reset({
			investmentTokenContract: "",
			investmentTokenId: "",
			investor: "",
		});
		setSearchParams({});
		setPage(0);
	};

	// Handle pagination changes
	const handleChangePage = (_event: unknown, newPage: number) => {
		setPage(newPage);
		const params = new URLSearchParams(searchParams);
		params.set("page", newPage.toString());
		setSearchParams(params);
	};

	const handleChangeRowsPerPage = (event: React.ChangeEvent<HTMLInputElement>) => {
		const newPageSize = parseInt(event.target.value, 10);
		setPageSize(newPageSize);
		setPage(0);
		const params = new URLSearchParams(searchParams);
		params.set("page_size", newPageSize.toString());
		params.set("page", "0");
		setSearchParams(params);
	};

	// Effect to fetch data when params change
	useEffect(() => {
		const currentPage = parseInt(searchParams.get("page") || "0", 10);
		const currentPageSize = parseInt(searchParams.get("page_size") || "10", 10);

		setPage(currentPage);
		setPageSize(currentPageSize);

		fetchInvestmentRecords(
			currentPage,
			currentPageSize,
			searchParams.get("investment_token_contract") || undefined,
			searchParams.get("investment_token_id") || undefined,
			searchParams.get("investor") || undefined,
		);
	}, [searchParams]);

	// Helper function for record type styling
	const getChipColorForRecordType = (type: InvestmentRecordType) => {
		switch (type) {
			case InvestmentRecordType.INVESTED:
				return "success";
			case InvestmentRecordType.CANCELLED:
				return "error";
			case InvestmentRecordType.CLAIMED:
				return "info";
			default:
				return "default";
		}
	};

	return (
		<Box>
			<Typography variant="h4" gutterBottom>
				Investment Records
			</Typography>

			{/* Filter form */}
			<Paper sx={{ p: 2, mb: 2 }}>
				<form onSubmit={handleSubmit(onSubmit)}>
					<Grid container spacing={2} alignItems="flex-end">
						<Grid item xs={12} sm={4}>
							<Controller
								name="investmentTokenContract"
								control={control}
								render={({ field }) => <TextField {...field} label="Investment Token Contract" variant="outlined" fullWidth />}
							/>
						</Grid>
						<Grid item xs={12} sm={4}>
							<Controller
								name="investmentTokenId"
								control={control}
								render={({ field }) => <TextField {...field} label="Investment Token ID" variant="outlined" fullWidth />}
							/>
						</Grid>
						<Grid item xs={12} sm={4}>
							<Controller
								name="investor"
								control={control}
								render={({ field }) => <TextField {...field} label="Investor" variant="outlined" fullWidth />}
							/>
						</Grid>
						<Grid item xs={12}>
							<Box display="flex" justifyContent="flex-end" gap={1}>
								<Button variant="outlined" onClick={handleClearFilters}>
									Clear
								</Button>
								<Button type="submit" variant="contained" color="primary">
									Search
								</Button>
							</Box>
						</Grid>
					</Grid>
				</form>
			</Paper>

			{/* Records table */}
			<Paper>
				{loading ? (
					<Box display="flex" justifyContent="center" p={3}>
						<CircularProgress />
					</Box>
				) : (
					<>
						<TableContainer>
							<Table>
								<TableHead>
									<TableRow>
										<TableCell>ID</TableCell>
										<TableCell>Investor</TableCell>
										<TableCell>Token Contract</TableCell>
										<TableCell>Token ID</TableCell>
										<TableCell>Type</TableCell>
										<TableCell>Currency Amount</TableCell>
										<TableCell>Token Amount</TableCell>
										<TableCell>Date</TableCell>
									</TableRow>
								</TableHead>
								<TableBody>
									{records.length === 0 ? (
										<TableRow>
											<TableCell colSpan={8} align="center">
												No records found
											</TableCell>
										</TableRow>
									) : (
										records.map((record) => (
											<TableRow key={record.id}>
												<TableCell>{record.id}</TableCell>
												<TableCell>{record.investor}</TableCell>
												<TableCell sx={{ maxWidth: 150, overflow: "hidden", textOverflow: "ellipsis" }}>
													{record.investment_token_contract_address}
												</TableCell>
												<TableCell>{record.investment_token_id}</TableCell>
												<TableCell>
													<Chip
														label={record.investment_record_type}
														color={getChipColorForRecordType(record.investment_record_type)}
														size="small"
													/>
												</TableCell>
												<TableCell>{record.currency_amount}</TableCell>
												<TableCell>{record.token_amount}</TableCell>
												<TableCell>{new Date(record.create_time).toLocaleString()}</TableCell>
											</TableRow>
										))
									)}
								</TableBody>
							</Table>
						</TableContainer>
						<TablePagination
							rowsPerPageOptions={[5, 10, 25, 50]}
							component="div"
							count={totalCount}
							rowsPerPage={pageSize}
							page={page}
							onPageChange={handleChangePage}
							onRowsPerPageChange={handleChangeRowsPerPage}
						/>
					</>
				)}
			</Paper>
		</Box>
	);
}
