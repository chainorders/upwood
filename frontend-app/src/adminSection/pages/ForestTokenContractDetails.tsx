import { useEffect, useState } from "react";
import { useParams } from "react-router";
import {
	Box,
	Paper,
	Typography,
	Divider,
	Table,
	TableBody,
	TableRow,
	TableCell,
	TableContainer,
	CircularProgress,
} from "@mui/material";
import { ForestProjectContract, IndexerService, SystemContractsConfigApiModel, UserService } from "../../apiClient";
import useCommonStyles from "../../theme/useCommonStyles";
import { formatDateField } from "../../lib/conversions";
import AgentsTable from "../components/AgentsTable";
import TokensTable from "../components/TokensTable";
import TokenHoldersTable from "../components/TokenHoldersTable";
import BalanceUpdatesTable from "../components/BalanceUpdatesTable";
import FundsTable from "../components/FundsTable";
import InvestorsTable from "../components/InvestorsTable";
import InvestmentRecordsTable from "../components/InvestmentRecordsTable";
import Tabs from "@mui/material/Tabs";
import Tab from "@mui/material/Tab";

export default function ForestTokenContractDetails() {
	const { contract_index } = useParams();
	const [contracts, setContracts] = useState<SystemContractsConfigApiModel>();
	const [contract, setContract] = useState<ForestProjectContract>();
	const [tab, setTab] = useState(0);
	const classes = useCommonStyles();

	const tabRoutes = [
		{ label: "Agents", component: <AgentsTable contract_index={contract_index!} /> },
		{ label: "Tokens", component: <TokensTable contract_index={contract_index!} /> },
		{ label: "Holders", component: <TokenHoldersTable contract_index={contract_index!} /> },
		{ label: "Balance Updates", component: <BalanceUpdatesTable contract_index={contract_index!} /> },
		{ label: "Active Funds", component: <FundsTable contract_index={contract_index!} /> },
		{ label: "Investors", component: <InvestorsTable contract_index={contract_index!} /> },
		{ label: "Investments", component: <InvestmentRecordsTable contract_index={contract_index!} /> },
	];

	const getDisplayContractName = (contract: ForestProjectContract) => {
		if (contract.contract_address === contracts?.tree_ft_contract_index) {
			return "Fungible Tree";
		} else if (contract.contract_address === contracts?.tree_nft_contract_index) {
			return "Non-Fungible Tree";
		} else if (contract.contract_address === contracts?.carbon_credit_contract_index) {
			return "Carbon Credits";
		} else if (contract.forest_project_name) {
			return `${contract.forest_project_name} (${contract.contract_type})`;
		}
		return contract.contract_name;
	};

	useEffect(() => {
		UserService.getSystemConfig().then(setContracts).catch(console.error);
	}, []);
	useEffect(() => {
		if (!contract_index) return;
		IndexerService.getAdminIndexerFpTokenContract(contract_index).then(setContract).catch(console.error);
	}, [contract_index]);

	if (!contract_index) {
		return <div>Invalid contract index</div>;
	}
	if (!contract) {
		return <CircularProgress />;
	}

	return (
		<Box>
			<Paper sx={classes.filterFormSection}>
				<Typography variant="h6" mb={2}>
					{getDisplayContractName(contract)} Contract Details
				</Typography>
				<Divider sx={{ mb: 3 }} />

				<Box sx={{ mb: 2 }}>
					<TableContainer>
						<Table>
							<TableBody>
								<TableRow>
									<TableCell sx={classes.tableHeaderCell}>Address</TableCell>
									<TableCell>{contract.contract_address}</TableCell>
								</TableRow>
								<TableRow>
									<TableCell sx={classes.tableHeaderCell}>Name</TableCell>
									<TableCell>{getDisplayContractName(contract)}</TableCell>
								</TableRow>
								<TableRow>
									<TableCell sx={classes.tableHeaderCell}>Owner</TableCell>
									<TableCell>{contract.owner}</TableCell>
								</TableRow>
								<TableRow>
									<TableCell sx={classes.tableHeaderCell}>Created At</TableCell>
									<TableCell>{formatDateField(contract.created_at)}</TableCell>
								</TableRow>
								<TableRow>
									<TableCell sx={classes.tableHeaderCell}>Module Ref</TableCell>
									<TableCell>{contract.module_ref}</TableCell>
								</TableRow>
								<TableRow>
									<TableCell sx={classes.tableHeaderCell}>Identity Registry</TableCell>
									<TableCell>{contract.identity_registry || "-"}</TableCell>
								</TableRow>
								<TableRow>
									<TableCell sx={classes.tableHeaderCell}>Compliance Contract</TableCell>
									<TableCell>{contract.compliance_contract || "-"}</TableCell>
								</TableRow>
							</TableBody>
						</Table>
					</TableContainer>
				</Box>
				<Tabs value={tab} onChange={(_, v) => setTab(v)} sx={{ mb: 2 }}>
					{tabRoutes.map((tabItem, idx) => (
						<Tab key={tabItem.label} label={tabItem.label} value={idx} />
					))}
				</Tabs>
				{tabRoutes[tab].component}
			</Paper>
		</Box>
	);
}
