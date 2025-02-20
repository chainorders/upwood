import { useEffect, useState } from "react";
import { Link, useParams } from "react-router";

import AddIcon from "@mui/icons-material/Add";
import ExpandMoreIcon from "@mui/icons-material/ExpandMore";
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
	TokenMetadata,
	UserService,
	YieldApiModel,
} from "../../apiClient";
import AddFundPopup from "./components/AddFundPopup";
import AddMarketPopup from "./components/AddMarketPopup";
import { AddProjectTokenPopup } from "./components/AddProjectTokenPopup";
import AddYieldPopup from "./components/AddYieldPopup";
import FundDetails from "./components/FundDetails";
import MarketDetails from "./components/MarketDetails";
import TokenDetails from "./components/TokenDetails";
import Yields from "./components/Yields";

export default function ProjectTokenDetails() {
	const { id, contract_address, token_id } = useParams<{ id: string; token_id: string; contract_address: string }>();
	const [token, setToken] = useState<Token>();
	const [tokenContract, setTokenContract] = useState<ForestProjectTokenContract>();
	const [preSaleToken, setPreSaleToken] = useState<Token>();
	const [preSaleTokenContract, setPreSaleTokenContract] = useState<ForestProjectTokenContract>();
	const [market, setMarket] = useState<Market>();
	const [marketCurrencyMetdata, setMarketCurrencyMetdata] = useState<TokenMetadata>();
	const [fund, setFund] = useState<SecurityMintFund>();
	const [fundCurrencyMetdata, setFundCurrencyMetdata] = useState<TokenMetadata>();
	const [yields, setYields] = useState<YieldApiModel[]>([]);
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
		IndexerService.getAdminIndexerCis2TokenMarket(contract_address!, token_id!).then(setMarket);
		IndexerService.getAdminIndexerCis2TokenYieldsList(contract_address!, token_id!).then(setYields);
		IndexerService.getAdminIndexerCis2TokenFund(contract_address!, token_id!).then(setFund);
		IndexerService.getAdminIndexerCis2Token(contract_address!, token_id!)
			.then(setToken)
			.catch(() => alert("Failed to fetch token details"))
			.finally(() => setLoading(false));
		ForestProjectService.getAdminForestProjectsContract(contract_address!).then(setTokenContract);
		UserService.getSystemConfig().then(setContracts);
	}, [contract_address, token_id, refreshCounter]);

	useEffect(() => {
		if (market) {
			ForestProjectService.getAdminTokenMetadata(market.currency_token_contract_address, market.currency_token_id).then(
				setMarketCurrencyMetdata,
			);
		}
	}, [market, refreshCounter]);

	useEffect(() => {
		if (fund) {
			ForestProjectService.getAdminTokenMetadata(fund.currency_token_contract_address, fund.currency_token_id).then(
				setFundCurrencyMetdata,
			);
		}
	}, [fund, refreshCounter]);

	// Pre Sale Token
	useEffect(() => {
		if (tokenContract) {
			switch (tokenContract.contract_type) {
				case SecurityTokenContractType.PROPERTY: {
					ForestProjectService.getAdminForestProjectsContractByType(
						tokenContract.forest_project_id,
						SecurityTokenContractType.PROPERTY_PRE_SALE,
					).then((data) => {
						setPreSaleTokenContract(data);
					});
					break;
				}
				case SecurityTokenContractType.BOND: {
					ForestProjectService.getAdminForestProjectsContractByType(
						tokenContract.forest_project_id,
						SecurityTokenContractType.BOND_PRE_SALE,
					).then((data) => {
						setPreSaleTokenContract(data);
					});
					break;
				}
				default: {
					console.error("Unsupported contract type");
					setPreSaleTokenContract(undefined);
				}
			}
		}
	}, [tokenContract]);

	useEffect(() => {
		if (preSaleTokenContract) {
			IndexerService.getAdminIndexerCis2Token(preSaleTokenContract.contract_address!, token_id!).then(setPreSaleToken);
		}
	}, [preSaleTokenContract, token_id]);

	useEffect(() => {
		if (tokenContract) {
			ForestProjectService.getAdminForestProjects(tokenContract.forest_project_id).then(setProject);
		}
	}, [tokenContract]);

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
									<MarketDetails market={market} tokenMetadata={tokenContract} currencyMetadata={marketCurrencyMetdata} />
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
										fund={fund}
										tokenContract={preSaleTokenContract}
										investmentTokenContract={tokenContract}
										currencyMetadata={fundCurrencyMetdata}
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
										tokenId={token_id!}
										tokenContract={tokenContract!}
										yielderContract={contracts.yielder_contract_index}
										yields={yields}
										onRemoveYield={() => setRefreshCounter((c) => c + 1)}
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
							contract_address={contract_address!}
							token_id={token_id!}
							onDone={() => setRefreshCounter((c) => c + 1)}
						/>
					</DialogContent>
				</Dialog>
				{contracts && contracts.mint_funds_contract && tokenContract && preSaleTokenContract && (
					<Dialog open={openFundPopup} onClose={handleCloseFundPopup} fullWidth>
						<DialogTitle>Add Fund</DialogTitle>
						<DialogContent>
							<AddFundPopup
								tokenId={token_id!}
								forestProjectId={id!}
								onDone={() => setRefreshCounter((c) => c + 1)}
								fundContract={contracts.mint_funds_contract}
								tokenContract={tokenContract}
								preSaleTokenContract={preSaleTokenContract}
							/>
						</DialogContent>
					</Dialog>
				)}
				<Dialog open={openMarketPopup} onClose={handleCloseMarketPopup}>
					<DialogTitle>Add Market</DialogTitle>
					<DialogContent>
						<AddMarketPopup
							contract_address={contract_address!}
							token_id={token_id!}
							onDone={() => setRefreshCounter((c) => c + 1)}
						/>
					</DialogContent>
				</Dialog>
				{preSaleTokenContract && (
					<Dialog open={openPreSaleTokenPopup} onClose={handleClosePreSaleTokenPopup} fullWidth>
						<DialogTitle>Add Pre Sale Token</DialogTitle>
						<DialogContent>
							<AddProjectTokenPopup
								token_contract={preSaleTokenContract}
								token_id={token_id}
								onDone={() => setRefreshCounter((c) => c + 1)}
							/>
						</DialogContent>
					</Dialog>
				)}
			</Box>
		</>
	);
}
