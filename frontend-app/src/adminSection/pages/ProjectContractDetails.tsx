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
	Button,
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
} from "../../apiClient";
import { Link } from "react-router";
import EditIcon from "@mui/icons-material/Edit";
import DeleteIcon from "@mui/icons-material/Delete";
import ExpandMoreIcon from "@mui/icons-material/ExpandMore";
import AddIcon from "@mui/icons-material/Add";
import MarketDetails from "./components/MarketDetails";
import FundDetails from "./components/FundDetails";
import TokenList from "./components/TokenList";
import AddMarketPopup from "./components/AddMarketPopup";
import { User } from "../../lib/user.ts";

const ProjectContractDetails = ({ user }: { user: User }) => {
	const { contract_address } = useParams<{ contract_address?: string }>();
	const [contract, setContract] = useState<ForestProjectTokenContract | null>(null);
	const [loading, setLoading] = useState(true);
	const [market, setMarket] = useState<Market>();
	const [fund, setFund] = useState<SecurityMintFund>();
	const [preSaleTokenContract, setPreSaleTokenContract] = useState<ForestProjectTokenContract>();
	const [tokens, setTokens] = useState<Token[]>([]);
	const [refreshCounter, setRefreshCounter] = useState(0);
	const [project, setProject] = useState<ForestProject | null>(null);
	const [contracts, setContracts] = useState<SystemContractsConfigApiModel>();
	const [openMarketPopup, setOpenMarketPopup] = useState(false);

	const handleOpenMarketPopup = () => {
		setOpenMarketPopup(true);
	};

	const handleCloseMarketPopup = () => {
		setOpenMarketPopup(false);
		setRefreshCounter((c) => c + 1);
	};

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
		UserService.getSystemConfig().then(setContracts);
	}, [refreshCounter]);
	useEffect(() => {
		if (contract) {
			IndexerService.getAdminIndexerCis2Market(contract.contract_address).then(setMarket);
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
								<TokenList
									user={user}
									tokens={tokens}
									tokenContract={contract}
									onTokenAdded={() => setRefreshCounter((c) => c + 1)}
								/>
							</AccordionDetails>
						</Accordion>
						<Accordion sx={{ marginTop: 2 }}>
							<AccordionSummary expandIcon={<ExpandMoreIcon />}>
								<Typography>Market Details</Typography>
							</AccordionSummary>
							<AccordionDetails>
								{market ? (
									<MarketDetails
										user={user}
										market={market}
										tokenMetadata={contract}
										onRefresh={() => setRefreshCounter((c) => c + 1)}
									/>
								) : (
									<Box sx={{ display: "flex", flexDirection: "column", alignItems: "center", gap: 2 }}>
										<Typography>No market details available</Typography>
										{contracts && (
											<Button variant="contained" color="primary" startIcon={<AddIcon />} onClick={handleOpenMarketPopup}>
												Add Market
											</Button>
										)}
									</Box>
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
										investmentTokenContract={contract}
										tokenContract={preSaleTokenContract}
										onRefresh={() => setRefreshCounter((c) => c + 1)}
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
			{contracts && contract && (
				<AddMarketPopup
					user={user}
					contracts={contracts}
					tokenContract={contract}
					onDone={handleCloseMarketPopup}
					open={openMarketPopup}
					onClose={handleCloseMarketPopup}
				/>
			)}
		</>
	);
};

export default ProjectContractDetails;
