import { useParams } from "react-router";
import { useEffect, useState } from "react";
import {
	Box,
	Typography,
	CircularProgress,
	Grid,
	Paper,
	List,
	ListItem,
	ListItemText,
	ListItemIcon,
	Accordion,
	AccordionSummary,
	AccordionDetails,
	Breadcrumbs,
} from "@mui/material";
import {
	ForestProject,
	ForestProjectService,
	ForestProjectTokenContract,
	IndexerService,
	Market,
	SecurityMintFund,
	SecurityTokenContractType,
	Token,
	TokenMetadata,
} from "../../apiClient";
import { Link } from "react-router";
import EditIcon from "@mui/icons-material/Edit";
import DeleteIcon from "@mui/icons-material/Delete";
import ExpandMoreIcon from "@mui/icons-material/ExpandMore";
import MarketDetails from "./components/MarketDetails";
import FundDetails from "./components/FundDetails";
import TokenList from "./components/TokenList";

export default function ProjectContractDetails() {
	const { contract_address } = useParams<{ contract_address?: string }>();
	const [contract, setContract] = useState<ForestProjectTokenContract | null>(null);
	const [loading, setLoading] = useState(true);
	const [market, setMarket] = useState<Market>();
	const [marketCurrencyMetdata, setMarketCurrencyMetdata] = useState<TokenMetadata>();
	const [fund, setFund] = useState<SecurityMintFund>();
	const [fundCurrencyMetdata, setFundCurrencyMetdata] = useState<TokenMetadata>();
	const [preSaleTokenContract, setPreSaleTokenContract] = useState<ForestProjectTokenContract>();
	const [tokens, setTokens] = useState<Token[]>([]);
	const [refreshCounter, setRefreshCounter] = useState(0);
	const [project, setProject] = useState<ForestProject | null>(null);

	useEffect(() => {
		ForestProjectService.getAdminForestProjectsContract(contract_address!)
			.then((data) => {
				setContract(data);
				setLoading(false);
			})
			.catch(() => {
				alert("Failed to fetch contract details");
				setLoading(false);
			});
	}, [contract_address, refreshCounter]);

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
	}, [fund]);
	useEffect(() => {
		if (contract && contract.market_token_id) {
			IndexerService.getAdminIndexerCis2TokenMarket(contract.contract_address, contract.market_token_id).then(setMarket);
		}
		if (contract && contract.fund_token_id) {
			IndexerService.getAdminIndexerCis2TokenFund(contract.contract_address, contract.fund_token_id).then(setFund);
		}
		if (contract) {
			switch (contract.contract_type) {
				case SecurityTokenContractType.PROPERTY: {
					ForestProjectService.getAdminForestProjectsContractByType(
						contract.forest_project_id,
						SecurityTokenContractType.PROPERTY_PRE_SALE,
					).then((data) => {
						setPreSaleTokenContract(data);
					});
					break;
				}
				case SecurityTokenContractType.BOND: {
					ForestProjectService.getAdminForestProjectsContractByType(
						contract.forest_project_id,
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

			IndexerService.getAdminIndexerCis2TokenList(contract.contract_address!).then(setTokens);
		}
	}, [contract, refreshCounter]);

	useEffect(() => {
		if (contract) {
			ForestProjectService.getAdminForestProjects(contract.forest_project_id).then(setProject);
		}
	}, [contract]);

	if (loading) {
		return <CircularProgress />;
	}

	if (!contract || !project) {
		return <Typography>No contract found</Typography>;
	}

	return (
		<>
			<Breadcrumbs aria-label="breadcrumb">
				<Link to="/admin">Admin</Link>
				<Link to="/admin/projects">Projects</Link>
				<Link to={`/admin/projects/${project.id}/details`}>{project.name}</Link>
				<Link to={`/admin/projects/${project.id}/contract/${contract_address}/details`}>
					{contract.contract_type} - {contract.contract_address}
				</Link>
				<Typography color="textPrimary">Contract Details</Typography>
			</Breadcrumbs>
			<Box sx={{ flexGrow: 1, padding: 2 }}>
				<Grid container spacing={2}>
					<Grid item xs={12} md={8}>
						<Paper sx={{ padding: 2 }}>
							<Typography variant="h6">Contract Details</Typography>
							<Typography>
								<strong>Contract Address:</strong> {contract.contract_address}
							</Typography>
							<Typography>
								<strong>Forest Project ID:</strong> {contract.forest_project_id}
							</Typography>
							<Typography>
								<strong>Contract Type:</strong> {contract.contract_type}
							</Typography>
							<Typography>
								<strong>Fund Token ID:</strong>
								{contract.fund_token_id ? (
									<Link to={`../token/${contract.fund_token_id}/details`}>{contract.fund_token_id}</Link>
								) : (
									"N/A"
								)}
							</Typography>
							<Typography>
								<strong>Market Token ID:</strong>
								{contract.market_token_id ? (
									<Link to={`../token/${contract.market_token_id}/details`}>{contract.market_token_id}</Link>
								) : (
									"N/A"
								)}
							</Typography>
							<Typography>
								<strong>Symbol:</strong> {contract.symbol || "N/A"}
							</Typography>
							<Typography>
								<strong>Decimals:</strong> {contract.decimals !== undefined ? contract.decimals : "N/A"}
							</Typography>
							<Typography>
								<strong>Created At:</strong> {contract.created_at}
							</Typography>
							<Typography>
								<strong>Updated At:</strong> {contract.updated_at}
							</Typography>
						</Paper>
						<Accordion sx={{ marginTop: 2 }}>
							<AccordionSummary expandIcon={<ExpandMoreIcon />}>
								<Typography>Tokens</Typography>
							</AccordionSummary>
							<AccordionDetails>
								<TokenList tokens={tokens} tokenContract={contract} onTokenAdded={() => setRefreshCounter((c) => c + 1)} />
							</AccordionDetails>
						</Accordion>
						<Accordion sx={{ marginTop: 2 }}>
							<AccordionSummary expandIcon={<ExpandMoreIcon />}>
								<Typography>Market Details</Typography>
							</AccordionSummary>
							<AccordionDetails>
								{market ? (
									<MarketDetails market={market} currencyMetadata={marketCurrencyMetdata} tokenMetadata={contract} />
								) : (
									<Typography>No market details available</Typography>
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
										currencyMetadata={fundCurrencyMetdata}
										investmentTokenContract={contract}
										tokenContract={preSaleTokenContract}
									/>
								) : (
									<Typography>No fund details available</Typography>
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
								<ListItem button component={Link} to={`../update`}>
									<ListItemIcon>
										<EditIcon />
									</ListItemIcon>
									<ListItemText primary="Update Contract" />
								</ListItem>
								<ListItem button onClick={() => alert("Not implemented")}>
									<ListItemIcon>
										<DeleteIcon />
									</ListItemIcon>
									<ListItemText primary="Delete Contract" />
								</ListItem>
							</List>
						</Paper>
					</Grid>
				</Grid>
			</Box>
		</>
	);
}
