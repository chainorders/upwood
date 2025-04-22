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
	ForestProjectContract,
	IndexerService,
	SystemContractsConfigApiModel,
	UserService,
	Agent,
	Token,
	TokenHolderUser,
} from "../../apiClient";
import useCommonStyles from "../../theme/useCommonStyles";
import { formatDateField } from "../../lib/conversions";
import AgentsTable from "../components/AgentsTable";
import TokensTable from "../components/TokensTable";
import TokenHoldersTable from "../components/TokenHoldersTable";
import BalanceUpdatesTable from "../components/BalanceUpdatesTable";
import FundsTable from "../components/FundsTable";
import InvestorsTable from "../components/InvestorsTable";
import InvestmentRecordsTable from "../components/InvestmentRecordsTable";
import TradersTable from "../components/TradersTable";
import TradesTable from "../components/TradesTable";
import YieldsTab from "../components/YieldsTab";
import UserYieldDistributionsTab from "../components/UserYieldDistributionsTab";
import MarketsTab from "../components/MarketsTab";
import Tabs from "@mui/material/Tabs";
import Tab from "@mui/material/Tab";
import AddAgentPopup from "./components/AddAgentPopup";
import RemoveAgentPopup from "../components/RemoveAgentPopup";
import { User } from "../../lib/user";
import securitySftMulti from "../../contractClients/generated/securitySftMulti";
import PauseTokenPopup from "../components/PauseTokenPopup";
import UnpauseTokenPopup from "../components/UnpauseTokenPopup";
import AddTokenPopup from "./components/AddTokenPopup";
import UpdateTokenMetadataPopup from "../components/UpdateTokenMetadataPopup";
import FreezeHolderBalancePopup from "../components/FreezeHolderBalancePopup";
import UnfreezeHolderBalancePopup from "../components/UnfreezeHolderBalancePopup";
import TransferHolderBalancePopup from "../components/TransferHolderBalancePopup";
import BurnHolderBalancePopup from "../components/BurnHolderBalancePopup";

export default function ForestTokenContractDetails({ user, fileBaseUrl }: { user: User; fileBaseUrl: string }) {
	const { contract_index } = useParams();
	const [contracts, setContracts] = useState<SystemContractsConfigApiModel>();
	const [contract, setContract] = useState<ForestProjectContract>();
	const [refreshCounter, setRefreshCounter] = useState(0);
	const [tab, setTab] = useState(0);
	const classes = useCommonStyles();

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
		"RemoveToken",
	]);

	const [pauseTokenOpen, setPauseTokenOpen] = useState(false);
	const [unpauseTokenOpen, setUnpauseTokenOpen] = useState(false);
	const [addTokenOpen, setAddTokenOpen] = useState(false);
	const [updateTokenMetadataOpen, setUpdateTokenMetadataOpen] = useState(false);
	const [selectedToken, setSelectedToken] = useState<Token>();
	const [freezeHolderOpen, setFreezeHolderOpen] = useState(false);
	const [unfreezeHolderOpen, setUnfreezeHolderOpen] = useState(false);
	const [transferHolderOpen, setTransferHolderOpen] = useState(false);
	const [burnHolderOpen, setBurnHolderOpen] = useState(false);
	const [selectedHolder, setSelectedHolder] = useState<TokenHolderUser>();
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

	const handleUpdateTokenMetadata = (token: Token) => {
		setSelectedToken(token);
		setUpdateTokenMetadataOpen(true);
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
						method={securitySftMulti.addAgent}
					/>
					<RemoveAgentPopup
						contractAddress={contract?.contract_address || ""}
						user={user}
						open={removeAgentOpen}
						onClose={() => {
							setRemoveAgentOpen(false);
							setRefreshCounter((prev) => prev + 1);
						}}
						agentAddress={removeAgentAddress}
						method={securitySftMulti.removeAgent}
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
						onUpdateTokenMetadata={handleUpdateTokenMetadata}
						onAddToken={() => setAddTokenOpen(true)}
						showAddToken={true}
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
								method={securitySftMulti.pause}
								tokenIdSize={8}
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
								method={securitySftMulti.unPause}
								tokenIdSize={8}
							/>
							<AddTokenPopup
								open={addTokenOpen}
								onClose={() => {
									setAddTokenOpen(false);
									setRefreshCounter((prev) => prev + 1);
								}}
								tokenContract={contract}
								user={user}
								onSuccess={handleRefresh}
								method={securitySftMulti.addToken}
								tokenIdSize={8}
							/>
							<UpdateTokenMetadataPopup
								open={updateTokenMetadataOpen}
								onClose={() => {
									setUpdateTokenMetadataOpen(false);
									setRefreshCounter((prev) => prev + 1);
								}}
								tokenId={selectedToken.token_id}
								contractAddress={contract_index!}
								user={user}
								onUpdate={handleRefresh}
								method={securitySftMulti.setTokenMetadata}
								fileBaseUrl={fileBaseUrl}
								initialHash={selectedToken.metadata_hash}
								initialUrl={selectedToken.metadata_url}
								tokenIdSize={8}
							/>
						</>
					)}
				</>
			),
		},
		{
			label: "Holders",
			component: (
				<>
					<TokenHoldersTable
						contract_index={contract_index!}
						onFreezeHolder={(holder: TokenHolderUser) => {
							setSelectedHolder(holder);
							setFreezeHolderOpen(true);
						}}
						onUnfreezeHolder={(holder: TokenHolderUser) => {
							setSelectedHolder(holder);
							setUnfreezeHolderOpen(true);
						}}
						onTransferHolder={(holder: TokenHolderUser) => {
							setSelectedHolder(holder);
							setTransferHolderOpen(true);
						}}
						onBurnHolder={(holder: TokenHolderUser) => {
							setSelectedHolder(holder);
							setBurnHolderOpen(true);
						}}
						refreshCounter={refreshCounter}
					/>
					{contract && selectedHolder && (
						<>
							<FreezeHolderBalancePopup
								open={freezeHolderOpen}
								onClose={() => {
									setFreezeHolderOpen(false);
								}}
								holder={selectedHolder}
								user={user}
								onSuccess={() => {
									setFreezeHolderOpen(false);
									setRefreshCounter((prev) => prev + 1);
								}}
								method={securitySftMulti.freeze}
								tokenIdSize={8}
							/>
							<UnfreezeHolderBalancePopup
								open={unfreezeHolderOpen}
								onClose={() => {
									setUnfreezeHolderOpen(false);
								}}
								holder={selectedHolder}
								user={user}
								onSuccess={() => {
									setUnfreezeHolderOpen(false);
									setRefreshCounter((prev) => prev + 1);
								}}
								method={securitySftMulti.unFreeze}
								tokenIdSize={8}
							/>
							<TransferHolderBalancePopup
								open={transferHolderOpen}
								onClose={() => {
									setTransferHolderOpen(false);
								}}
								holder={selectedHolder}
								user={user}
								onSuccess={() => {
									setTransferHolderOpen(false);
									setRefreshCounter((prev) => prev + 1);
								}}
								method={securitySftMulti.transfer}
								tokenIdSize={8}
							/>
							<BurnHolderBalancePopup
								open={burnHolderOpen}
								onClose={() => {
									setBurnHolderOpen(false);
								}}
								holder={selectedHolder}
								user={user}
								onSuccess={() => {
									setBurnHolderOpen(false);
									setRefreshCounter((prev) => prev + 1);
								}}
								method={securitySftMulti.burn}
								tokenIdSize={8}
							/>
						</>
					)}
				</>
			),
		},
		{ label: "Balance Updates", component: <BalanceUpdatesTable contract_index={contract_index!} /> },
		{ label: "Active Funds", component: <FundsTable contract_index={contract_index!} /> },
		{ label: "Investors", component: <InvestorsTable contract_index={contract_index!} /> },
		{ label: "Investments", component: <InvestmentRecordsTable contract_index={contract_index!} /> },
		{ label: "Markets", component: <MarketsTab contract_index={contract_index!} /> },
		{ label: "Traders", component: <TradersTable contract_index={contract_index!} /> },
		{ label: "Trades", component: <TradesTable contract_index={contract_index!} /> },
		{ label: "Yields", component: <YieldsTab contract_index={contract_index!} /> },
		{ label: "Yield Distributions", component: <UserYieldDistributionsTab contract_index={contract_index!} /> },
	];

	const getDisplayContractName = (contract: ForestProjectContract) => {
		if (contract.contract_address === contracts?.tree_ft_contract_index) {
			return `Fungible Tree (${contract.contract_name})`;
		} else if (contract.contract_address === contracts?.tree_nft_contract_index) {
			return `Non-Fungible Tree (${contract.contract_name})`;
		} else if (contract.contract_address === contracts?.carbon_credit_contract_index) {
			return `Carbon Credit (${contract.contract_name})`;
		} else if (contract.forest_project_name) {
			return `${contract.forest_project_name}/${contract.contract_type} (${contract.contract_name})`;
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
