import {
	Box,
	Table,
	TableHead,
	TableBody,
	TableRow,
	TableCell,
	Typography,
	Link as MuiLink,
	TableContainer,
	Paper,
	Divider,
} from "@mui/material";
import TablePagination from "@mui/material/TablePagination";
import { Link } from "react-router";
import {
	IndexerService,
	PagedResponse_ForestProjectContract,
	SystemContractsConfigApiModel,
	TokenContract,
	UserService,
} from "../../apiClient";
import { useEffect, useState } from "react";
import MonetizationOnIcon from "@mui/icons-material/MonetizationOn";
import ForestIcon from "@mui/icons-material/Forest";
import NatureIcon from "@mui/icons-material/Nature";
import useCommonStyles from "../../theme/useCommonStyles";
import { format } from "date-fns";

export default function TokenContractsList() {
	const [contracts, setContracts] = useState<SystemContractsConfigApiModel>();
	const [carbonCreditsContracts, setCarbonCreditsContracts] = useState<TokenContract>();
	const [eTreesFtContract, setETreesFtContract] = useState<TokenContract>();
	const [eTreesNftContract, setETreesNftContract] = useState<TokenContract>();
	const [fpTokenContracts, setFpTokenContracts] = useState<PagedResponse_ForestProjectContract>();
	const [fpTokenContractsPage, setFpTokenContractsPage] = useState(0);
	const [fpTokenContractsPageSize, setFpTokenContractsPageSize] = useState(100);
	const classes = useCommonStyles();

	const formatDate = (dateStr?: string) => {
		if (!dateStr) return "-";
		const date = new Date(dateStr);
		return isNaN(date.getTime()) ? "-" : format(date, "yyyy-MM-dd HH:mm:ss");
	};

	useEffect(() => {
		UserService.getSystemConfig().then(setContracts).catch(console.error);
	}, []);
	useEffect(() => {
		if (!contracts) {
			return;
		}

		IndexerService.getAdminIndexerTokenContract(contracts.carbon_credit_contract_index)
			.then(setCarbonCreditsContracts)
			.catch(console.error);
		IndexerService.getAdminIndexerTokenContract(contracts.tree_ft_contract_index)
			.then(setETreesFtContract)
			.catch(console.error);
		IndexerService.getAdminIndexerTokenContract(contracts.tree_nft_contract_index)
			.then(setETreesNftContract)
			.catch(console.error);
	}, [contracts]);

	useEffect(() => {
		IndexerService.getAdminIndexerFpTokenContracts(fpTokenContractsPage, fpTokenContractsPageSize)
			.then(setFpTokenContracts)
			.catch(console.error);
	}, [fpTokenContractsPage, fpTokenContractsPageSize]);

	const contractDisplayData = [
		{
			contract: carbonCreditsContracts,
			name: "Carbon Credits",
			icon: <MonetizationOnIcon sx={{ verticalAlign: "middle", mr: 1 }} />,
		},
		{
			contract: eTreesFtContract,
			name: "Fungible Tree",
			icon: <ForestIcon sx={{ verticalAlign: "middle", mr: 1 }} />,
		},
		{
			contract: eTreesNftContract,
			name: "Tree NFT",
			icon: <NatureIcon sx={{ verticalAlign: "middle", mr: 1 }} />,
		},
	];

	return (
		<Box>
			<Paper sx={classes.filterFormSection}>
				<Typography variant="h6" mb={2}>
					Token Contracts List
				</Typography>
				<Divider sx={{ mb: 3 }} />
				<TableContainer component={Paper} sx={classes.tableContainer}>
					<Table>
						<TableHead>
							<TableRow>
								<TableCell sx={classes.tableHeaderCell}>Name</TableCell>
								<TableCell sx={classes.tableHeaderCell}>Address</TableCell>
								<TableCell sx={classes.tableHeaderCell}>Owner</TableCell>
								<TableCell sx={classes.tableHeaderCell}>Created At</TableCell>
								<TableCell sx={classes.tableHeaderCell}>Action</TableCell>
							</TableRow>
						</TableHead>
						<TableBody>
							{contractDisplayData
								.filter((d) => d.contract)
								.map((data, idx) => (
									<TableRow key={data.contract!.contract_address || idx} sx={classes.tableRow}>
										<TableCell>
											{data.icon}
											{data.name}
										</TableCell>
										<TableCell>{data.contract!.contract_address}</TableCell>
										<TableCell>{data.contract!.owner}</TableCell>
										<TableCell>{formatDate(data.contract!.created_at)}</TableCell>
										<TableCell>
											<MuiLink component={Link} to={`/admin/token-contracts/${data.contract!.contract_address}`}>
												View
											</MuiLink>
										</TableCell>
									</TableRow>
								))}
						</TableBody>
					</Table>
				</TableContainer>
			</Paper>

			{/* New Table for fpTokenContracts */}
			<Paper sx={{ ...classes.filterFormSection, mt: 4 }}>
				<Typography variant="h6" mb={2}>
					Forest Project Token Contracts
				</Typography>
				<Divider sx={{ mb: 3 }} />
				<TableContainer component={Paper} sx={classes.tableContainer}>
					<Table>
						<TableHead>
							<TableRow>
								<TableCell sx={classes.tableHeaderCell}>Symbol</TableCell>
								<TableCell sx={classes.tableHeaderCell}>Project Name</TableCell>
								<TableCell sx={classes.tableHeaderCell}>Contract Type</TableCell>
								<TableCell sx={classes.tableHeaderCell}>Address</TableCell>
								<TableCell sx={classes.tableHeaderCell}>Owner</TableCell>
								<TableCell sx={classes.tableHeaderCell}>Created At</TableCell>
								<TableCell sx={classes.tableHeaderCell}>Action</TableCell>
							</TableRow>
						</TableHead>
						<TableBody>
							{fpTokenContracts?.data?.length ? (
								fpTokenContracts.data.map((contract) => (
									<TableRow key={contract.contract_address} sx={classes.tableRow}>
										<TableCell>{contract.symbol}</TableCell>
										<TableCell>{contract.forest_project_name}</TableCell>
										<TableCell>{contract.contract_type}</TableCell>
										<TableCell>{contract.contract_address}</TableCell>
										<TableCell>{contract.owner}</TableCell>
										<TableCell>{formatDate(contract.created_at)}</TableCell>
										<TableCell>
											<MuiLink component={Link} to={`/admin/fp-token-contracts/${contract.contract_address}`}>
												View
											</MuiLink>
										</TableCell>
									</TableRow>
								))
							) : (
								<TableRow>
									<TableCell colSpan={7} align="center">
										No data
									</TableCell>
								</TableRow>
							)}
						</TableBody>
					</Table>
				</TableContainer>
				<TablePagination
					component="div"
					count={fpTokenContracts?.page_count ? fpTokenContracts.page_count * fpTokenContractsPageSize : 0}
					page={fpTokenContractsPage}
					onPageChange={(_, newPage) => setFpTokenContractsPage(newPage)}
					rowsPerPage={fpTokenContractsPageSize}
					onRowsPerPageChange={(e) => {
						setFpTokenContractsPageSize(parseInt(e.target.value, 10));
						setFpTokenContractsPage(0);
					}}
					rowsPerPageOptions={[5, 10, 25, 50, 100]}
				/>
			</Paper>
		</Box>
	);
}
