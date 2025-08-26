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
	TokenHolderUser,
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
import RemoveAgentPopup from "../components/RemoveAgentPopup";
import PauseTokenPopup from "../components/PauseTokenPopup";
import UnpauseTokenPopup from "../components/UnpauseTokenPopup";
import { User } from "../../lib/user";
import securitySftSingle from "../../contractClients/generated/securitySftSingle";
import UpdateTokenMetadataPopup from "../components/UpdateTokenMetadataPopup";
import FreezeHolderBalancePopup from "../components/FreezeHolderBalancePopup";
import UnfreezeHolderBalancePopup from "../components/UnfreezeHolderBalancePopup";
import TransferHolderBalancePopup from "../components/TransferHolderBalancePopup";
import BurnHolderBalancePopup from "../components/BurnHolderBalancePopup";

export default function TokenContractDetails({ user, fileBaseUrl }: { user: User; fileBaseUrl: string }) {
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

	const [selectedToken, setSelectedToken] = useState<Token>();
	const [selectedHolder, setSelectedHolder] = useState<TokenHolderUser>();

	// Token popup states
	const [pauseTokenOpen, setPauseTokenOpen] = useState(false);
	const [unpauseTokenOpen, setUnpauseTokenOpen] = useState(false);
	const [updateTokenMetadataOpen, setUpdateTokenMetadataOpen] = useState(false);
	const [freezeHolderOpen, setFreezeHolderOpen] = useState(false);
	const [unfreezeHolderOpen, setUnfreezeHolderOpen] = useState(false);
	const [transferHolderOpen, setTransferHolderOpen] = useState(false);
	const [burnHolderOpen, setBurnHolderOpen] = useState(false);

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
						onPauseToken={(token) => {
							setSelectedToken(token);
							setPauseTokenOpen(true);
						}}
						onUnpauseToken={(token) => {
							setSelectedToken(token);
							setUnpauseTokenOpen(true);
						}}
						onUpdateTokenMetadata={(token: Token) => {
							setSelectedToken(token);
							setUpdateTokenMetadataOpen(true);
						}}
						refreshCounter={refreshCounter}
					/>
					{contract && selectedToken && (
						<>
							<PauseTokenPopup
								open={pauseTokenOpen}
								onClose={() => {
									setPauseTokenOpen(false);
								}}
								tokenId={selectedToken.token_id}
								contractAddress={contract_index!}
								user={user}
								onSuccess={() => {
									setPauseTokenOpen(false);
								}}
								method={securitySftSingle.pause}
								tokenIdSize={0}
							/>
							<UnpauseTokenPopup
								open={unpauseTokenOpen}
								onClose={() => {
									setUnpauseTokenOpen(false);
								}}
								tokenId={selectedToken.token_id}
								contractAddress={contract_index!}
								user={user}
								onSuccess={() => {
									setUnpauseTokenOpen(false);
									setRefreshCounter((prev) => prev + 1);
								}}
								method={securitySftSingle.unPause}
								tokenIdSize={contract_index === contracts?.tree_nft_contract_index ? 8 : 0}
							/>
							<UpdateTokenMetadataPopup
								open={updateTokenMetadataOpen}
								onClose={() => {
									setUpdateTokenMetadataOpen(false);
								}}
								tokenId={selectedToken.token_id}
								contractAddress={contract_index!}
								user={user}
								onUpdate={() => {
									setUpdateTokenMetadataOpen(false);
									setRefreshCounter((prev) => prev + 1);
								}}
								method={securitySftSingle.setTokenMetadata}
								fileBaseUrl={fileBaseUrl}
								initialHash={selectedToken.metadata_hash}
								initialUrl={selectedToken.metadata_url}
								tokenIdSize={contract_index === contracts?.tree_nft_contract_index ? 8 : 0}
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
								method={securitySftSingle.freeze}
								tokenIdSize={contract_index === contracts?.tree_nft_contract_index ? 8 : 0}
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
								method={securitySftSingle.unFreeze}
								tokenIdSize={contract_index === contracts?.tree_nft_contract_index ? 8 : 0}
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
								method={securitySftSingle.transfer}
								tokenIdSize={contract_index === contracts?.tree_nft_contract_index ? 8 : 0}
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
								method={securitySftSingle.burn}
								tokenIdSize={contract_index === contracts?.tree_nft_contract_index ? 8 : 0}
							/>
						</>
					)}
				</>
			),
		},
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
