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
import {
	IndexerService,
	SystemContractsConfigApiModel,
	TokenContract,
	UserService,
	Agent,
	Token,
} from "../../apiClient";
import useCommonStyles from "../../theme/useCommonStyles";
import { formatDateField } from "../../lib/conversions";
import AgentsTable from "../components/AgentsTable";
import TokensTable from "../components/TokensTable";
import TokenHoldersTable from "../components/TokenHoldersTable";
import BalanceUpdatesTable from "../components/BalanceUpdatesTable";
import Tabs from "@mui/material/Tabs";
import Tab from "@mui/material/Tab";
import AddAgentPopup from "./components/AddAgentPopup";
import RemoveAgentPopup from "./components/RemoveAgentPopup";
import PauseTokenPopup from "./components/PauseTokenPopup";
import UnpauseTokenPopup from "./components/UnpauseTokenPopup";
import { User } from "../../lib/user";
import securitySftSingle from "../../contractClients/generated/securitySftSingle";

export default function TokenContractDetails({ user }: { user: User }) {
	const { contract_index } = useParams();
	const [contracts, setContracts] = useState<SystemContractsConfigApiModel>();
	const [contract, setContract] = useState<TokenContract>();
	const [refreshCounter, setRefreshCounter] = useState(0);
	const [tab, setTab] = useState(0);
	const classes = useCommonStyles();

	// Popup state
	const [addAgentOpen, setAddAgentOpen] = useState(false);
	const [removeAgentOpen, setRemoveAgentOpen] = useState(false);
	const [removeAgentAddress, setRemoveAgentAddress] = useState<string>("");
	const [roles] = useState<string[]>([
		"SetIdentityRegistry",
		"SetCompliance",
		"AddAgent",
		"Mint",
		"ForcedBurn",
		"ForcedTransfer",
		"Freeze",
		"UnFreeze",
		"HolderRecovery",
		"Pause",
		"UnPause",
		"AddToken",
		"Operator",
	]);

	// Token popup states
	const [pauseTokenOpen, setPauseTokenOpen] = useState(false);
	const [unpauseTokenOpen, setUnpauseTokenOpen] = useState(false);
	const [selectedToken, setSelectedToken] = useState<Token | null>(null);

	const handleRefresh = () => {
		setRefreshCounter((prev) => prev + 1);
	};

	const handlePauseToken = (token: Token) => {
		setSelectedToken(token);
		setPauseTokenOpen(true);
	};

	const handleUnpauseToken = (token: Token) => {
		setSelectedToken(token);
		setUnpauseTokenOpen(true);
	};

	const tabRoutes = [
		{
			label: "Agents",
			component: (
				<>
					<AgentsTable
						contract_index={contract_index!}
						onAddAgent={() => setAddAgentOpen(true)}
						onRemoveAgent={(agent: Agent) => {
							setRemoveAgentAddress(agent.agent_address);
							setRemoveAgentOpen(true);
						}}
						refreshCounter={refreshCounter}
					/>
					<AddAgentPopup
						user={user}
						open={addAgentOpen}
						onClose={() => {
							setAddAgentOpen(false);
							setRefreshCounter((prev) => prev + 1);
						}}
						contractAddress={contract?.contract_address || ""}
						roles={roles}
						method={securitySftSingle.addAgent}
					/>
					<RemoveAgentPopup
						user={user}
						open={removeAgentOpen}
						onClose={() => {
							setRemoveAgentOpen(false);
							setRefreshCounter((prev) => prev + 1);
						}}
						contractAddress={contract?.contract_address || ""}
						agentAddress={removeAgentAddress}
						method={securitySftSingle.removeAgent}
					/>
				</>
			),
		},
		{
			label: "Tokens",
			component: (
				<>
					<TokensTable
						contract_index={contract_index!}
						onPauseToken={handlePauseToken}
						onUnpauseToken={handleUnpauseToken}
						refreshCounter={refreshCounter}
					/>
					{contract && selectedToken && (
						<>
							<PauseTokenPopup
								open={pauseTokenOpen}
								onClose={() => {
									setPauseTokenOpen(false);
									setRefreshCounter((prev) => prev + 1);
								}}
								tokenId={selectedToken.token_id}
								contractAddress={contract_index!}
								user={user}
								onSuccess={handleRefresh}
								method={securitySftSingle.pause}
								tokenIdSize={0}
							/>
							<UnpauseTokenPopup
								open={unpauseTokenOpen}
								onClose={() => {
									setUnpauseTokenOpen(false);
									setRefreshCounter((prev) => prev + 1);
								}}
								tokenId={selectedToken.token_id}
								contractAddress={contract_index!}
								user={user}
								onSuccess={handleRefresh}
								method={securitySftSingle.unPause}
								tokenIdSize={0}
							/>
						</>
					)}
				</>
			),
		},
		{ label: "Holders", component: <TokenHoldersTable contract_index={contract_index!} /> },
		{ label: "Balance Updates", component: <BalanceUpdatesTable contract_index={contract_index!} /> },
	];

	const getDisplayContractName = (contract: TokenContract) => {
		if (contract.contract_address === contracts?.tree_ft_contract_index) {
			return `Fungible Tree (${contract.contract_name})`;
		} else if (contract.contract_address === contracts?.tree_nft_contract_index) {
			return `Non-Fungible Tree (${contract.contract_name})`;
		} else if (contract.contract_address === contracts?.carbon_credit_contract_index) {
			return `Carbon Credit (${contract.contract_name})`;
		}
		return contract.contract_name;
	};

	useEffect(() => {
		UserService.getSystemConfig().then(setContracts).catch(console.error);
	}, []);
	useEffect(() => {
		if (!contract_index) return;
		IndexerService.getAdminIndexerTokenContract(contract_index).then(setContract).catch(console.error);
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
