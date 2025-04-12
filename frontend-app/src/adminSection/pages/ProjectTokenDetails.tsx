import { useEffect, useState } from "react";
import { Link, useParams } from "react-router";

import AddIcon from "@mui/icons-material/Add";
import ExpandMoreIcon from "@mui/icons-material/ExpandMore";
import RefreshIcon from "@mui/icons-material/Refresh";
import {
	Accordion,
	AccordionDetails,
	AccordionSummary,
	Box,
	Breadcrumbs,
	Dialog,
	DialogContent,
	DialogTitle,
	Grid,
	List,
	ListItemButton,
	ListItemIcon,
	ListItemText,
	Paper,
	Typography,
} from "@mui/material";

import {
	ForestProject,
	ForestProjectService,
	ForestProjectTokenContract,
	IndexerService,
	Market,
	SecurityMintFund,
	SecurityTokenContractType,
	SystemContractsConfigApiModel,
	Token,
	UserService,
	Yield,
} from "../../apiClient";
import AddFundPopup from "./components/AddFundPopup";
import AddMarketPopup from "./components/AddMarketPopup";
import { AddProjectTokenPopup } from "./components/AddProjectTokenPopup";
import AddYieldPopup from "./components/AddYieldPopup";
import FundDetails from "./components/FundDetails";
import MarketDetails from "./components/MarketDetails";
import TokenDetails from "./components/TokenDetails";
import Yields from "./components/Yields";
import { User } from "../../lib/user.ts";

const ProjectTokenDetails = ({ user }: { user: User }) => {
	const { id, contract_address, token_id } = useParams<{ id: string; token_id: string; contract_address: string }>();
	const [token, setToken] = useState<Token>();
	const [tokenContract, setTokenContract] = useState<ForestProjectTokenContract>();
	const [preSaleToken, setPreSaleToken] = useState<Token>();
	const [preSaleTokenContract, setPreSaleTokenContract] = useState<ForestProjectTokenContract>();
	const [market, setMarket] = useState<Market>();
	const [fund, setFund] = useState<SecurityMintFund>();
	const [yields, setYields] = useState<Yield[]>([]);
	const [contracts, setContracts] = useState<SystemContractsConfigApiModel>();
	const [refreshCounter, setRefreshCounter] = useState(0);
	const [project, setProject] = useState<ForestProject | null>(null);

	const [, setLoading] = useState(true);
	const [openYieldPopup, setOpenYieldPopup] = useState(false);
	const [openFundPopup, setOpenFundPopup] = useState(false);
	const [openMarketPopup, setOpenMarketPopup] = useState(false);
	const [openPreSaleTokenPopup, setOpenPreSaleTokenPopup] = useState(false);

	const handleOpenYieldPopup = () => {
		setOpenYieldPopup(true);
	};

	const handleCloseYieldPopup = () => {
		setOpenYieldPopup(false);
		setRefreshCounter((c) => c + 1);
	};

	const handleOpenFundPopup = () => {
		setOpenFundPopup(true);
	};

	const handleCloseFundPopup = () => {
		setOpenFundPopup(false);
		setRefreshCounter((c) => c + 1);
	};

	const handleOpenMarketPopup = () => {
		setOpenMarketPopup(true);
	};

	const handleCloseMarketPopup = () => {
		setOpenMarketPopup(false);
		setRefreshCounter((c) => c + 1);
	};

	const handleOpenPreSaleTokenPopup = () => {
		setOpenPreSaleTokenPopup(true);
	};

	const handleClosePreSaleTokenPopup = () => {
		setOpenPreSaleTokenPopup(false);
		setRefreshCounter((c) => c + 1);
	};

	useEffect(() => {
		setLoading(true);
		IndexerService.getAdminIndexerCis2Market(contract_address!, token_id!).then(setMarket);
		IndexerService.getAdminIndexerCis2TokenYieldsList(contract_address!, token_id!).then(setYields);
		IndexerService.getAdminIndexerCis2TokenFund(contract_address!, token_id!).then(setFund);
		IndexerService.getAdminIndexerCis2Token(contract_address!, token_id!)
			.then(setToken)
			.catch(() => alert("Failed to fetch token details"))
			.finally(() => setLoading(false));
		ForestProjectService.getAdminForestProjectsContract(contract_address!).then(setTokenContract);
		UserService.getSystemConfig().then(setContracts);
	}, [contract_address, token_id, refreshCounter]);

	// Pre Sale Token
	useEffect(() => {
		if (tokenContract) {
			switch (tokenContract.contract_type) {
				case SecurityTokenContractType.PROPERTY: {
					ForestProjectService.getAdminForestProjectsContractByType(
						tokenContract.forest_project_id,
						SecurityTokenContractType.PROPERTY_PRE_SALE,
					).then(setPreSaleTokenContract);
					break;
				}
				case SecurityTokenContractType.BOND: {
					ForestProjectService.getAdminForestProjectsContractByType(
						tokenContract.forest_project_id,
						SecurityTokenContractType.BOND_PRE_SALE,
					).then(setPreSaleTokenContract);
					break;
				}
				default: {
					console.error("Unsupported contract type");
					setPreSaleTokenContract(undefined);
				}
			}
		}
	}, [tokenContract, refreshCounter]);

	useEffect(() => {
		if (preSaleTokenContract) {
			IndexerService.getAdminIndexerCis2Token(preSaleTokenContract.contract_address!, token_id!).then(setPreSaleToken);
		}
	}, [preSaleTokenContract, token_id, refreshCounter]);

	useEffect(() => {
		if (tokenContract) {
			ForestProjectService.getAdminForestProjects(tokenContract.forest_project_id).then(setProject);
		}
	}, [tokenContract, refreshCounter]);

	if (!token || !project || !tokenContract) {
		return <div>Loading...</div>;
	}

	return (
		<>
			<Breadcrumbs aria-label="breadcrumb">
				<Link to="/admin">Admin</Link>
				<Link to="/admin/projects">Projects</Link>
				<Link to={`/admin/projects/${project.id}/details`}>{project.name}</Link>
				<Link to={`/admin/projects/${project.id}/contract/${contract_address}/details`}>
					{tokenContract.contract_type} - {tokenContract.contract_address}
				</Link>
				<Typography color="textPrimary">Token ID: {token_id}</Typography>
			</Breadcrumbs>
			<Box sx={{ flexGrow: 1, padding: 2 }}>
				<Grid container spacing={2}>
					<Grid item xs={12} md={8}>
						<Paper sx={{ padding: 2 }}>
							<TokenDetails
								user={user}
								token={token}
								tokenContract={tokenContract}
								onDeleteToken={() => setRefreshCounter((c) => c + 1)}
							/>
						</Paper>
						<Accordion sx={{ marginTop: 2 }}>
							<AccordionSummary expandIcon={<ExpandMoreIcon />}>
								<Typography>Market Details</Typography>
							</AccordionSummary>
							<AccordionDetails>
								{market ? (
									<MarketDetails
										user={user}
										market={market}
										tokenMetadata={tokenContract}
										onRefresh={() => setRefreshCounter((c) => c + 1)}
									/>
								) : (
									<Typography>No market details available</Typography>
								)}
							</AccordionDetails>
						</Accordion>
						<Accordion sx={{ marginTop: 2 }}>
							<AccordionSummary expandIcon={<ExpandMoreIcon />}>
								<Typography>Pre Sale Token</Typography>
							</AccordionSummary>
							<AccordionDetails>
								{preSaleToken ? (
									<TokenDetails
										user={user}
										token={preSaleToken}
										tokenContract={preSaleTokenContract}
										onDeleteToken={() => setRefreshCounter((c) => c + 1)}
									/>
								) : (
									<Typography>No Pre Sale Token Attached</Typography>
								)}
							</AccordionDetails>
						</Accordion>
						<Accordion sx={{ marginTop: 2 }}>
							<AccordionSummary expandIcon={<ExpandMoreIcon />}>
								<Typography>Fund Details</Typography>
							</AccordionSummary>
							<AccordionDetails>
								{fund ? (
									<FundDetails
										user={user}
										fund={fund}
										tokenContract={preSaleTokenContract}
										investmentTokenContract={tokenContract}
										onRefresh={() => setRefreshCounter((c) => c + 1)}
									/>
								) : (
									<Typography>No fund details available</Typography>
								)}
							</AccordionDetails>
						</Accordion>
						<Accordion sx={{ marginTop: 2 }}>
							<AccordionSummary expandIcon={<ExpandMoreIcon />}>
								<Typography>Yields</Typography>
							</AccordionSummary>
							<AccordionDetails>
								{yields.length > 0 && tokenContract && contracts ? (
									<Yields
										user={user}
										tokenId={token_id!}
										tokenContract={tokenContract!}
										yielderContract={contracts.yielder_contract_index}
										yields={yields}
										onRefresh={() => setRefreshCounter((c) => c + 1)}
										contracts={contracts}
									/>
								) : (
									<Typography>No yields available</Typography>
								)}
							</AccordionDetails>
						</Accordion>
					</Grid>
					<Grid item xs={12} md={4}>
						<Paper sx={{ padding: 2 }}>
							<Typography variant="h6" gutterBottom>
								Actions
							</Typography>
							<List>
								<ListItemButton onClick={() => setRefreshCounter((c) => c + 1)}>
									<ListItemIcon>
										<RefreshIcon />
									</ListItemIcon>
									<ListItemText primary="Refresh" />
								</ListItemButton>
								<ListItemButton onClick={handleOpenYieldPopup}>
									<ListItemIcon>
										<AddIcon />
									</ListItemIcon>
									<ListItemText primary="Add Yield" />
								</ListItemButton>
								<ListItemButton onClick={handleOpenPreSaleTokenPopup} disabled={!!preSaleToken || !preSaleTokenContract}>
									<ListItemIcon>
										<AddIcon />
									</ListItemIcon>
									<ListItemText primary="Add Pre Sale Token" />
								</ListItemButton>
								<ListItemButton
									onClick={handleOpenFundPopup}
									disabled={!!fund || !(contracts && contracts.mint_funds_contract && tokenContract && preSaleTokenContract)}
								>
									<ListItemIcon>
										<AddIcon />
									</ListItemIcon>
									<ListItemText primary="Add Fund" />
								</ListItemButton>
								<ListItemButton onClick={handleOpenMarketPopup} disabled={!!market}>
									<ListItemIcon>
										<AddIcon />
									</ListItemIcon>
									<ListItemText primary="Add Market" />
								</ListItemButton>
							</List>
						</Paper>
					</Grid>
				</Grid>
				<Dialog open={openYieldPopup} onClose={handleCloseYieldPopup}>
					<DialogTitle>Add Yield</DialogTitle>
					<DialogContent>
						<AddYieldPopup
							user={user}
							contract_address={contract_address!}
							token_id={token_id!}
							onDone={() => {
								handleCloseYieldPopup();
								setRefreshCounter((c) => c + 1);
							}}
						/>
					</DialogContent>
				</Dialog>
				{contracts && contracts.mint_funds_contract && tokenContract && preSaleTokenContract && (
					<Dialog open={openFundPopup} onClose={handleCloseFundPopup} fullWidth>
						<DialogTitle>Add Fund</DialogTitle>
						<DialogContent>
							<AddFundPopup
								user={user}
								contracts={contracts}
								tokenId={token_id!}
								forestProjectId={id!}
								onDone={handleCloseFundPopup}
								fundContract={contracts.mint_funds_contract}
								tokenContract={tokenContract}
								preSaleTokenContract={preSaleTokenContract}
							/>
						</DialogContent>
					</Dialog>
				)}
				<AddMarketPopup
					user={user}
					contracts={contracts!}
					tokenContract={tokenContract}
					token_id={token_id!}
					onDone={handleCloseMarketPopup}
					open={openMarketPopup}
					onClose={handleCloseMarketPopup}
				/>
				{preSaleTokenContract && (
					<Dialog open={openPreSaleTokenPopup} onClose={handleClosePreSaleTokenPopup} fullWidth>
						<DialogTitle>Add Pre Sale Token</DialogTitle>
						<DialogContent>
							<AddProjectTokenPopup
								user={user}
								token_contract={preSaleTokenContract}
								token_id={token_id}
								onDone={handleClosePreSaleTokenPopup}
							/>
						</DialogContent>
					</Dialog>
				)}
			</Box>
		</>
	);
};

export default ProjectTokenDetails;
