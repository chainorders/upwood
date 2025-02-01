import { useParams } from "react-router";
import { useEffect, useState } from "react";
import {
	Box,
	Typography,
	CircularProgress,
	Grid,
	Paper,
	List,
	ListItemText,
	ListItemIcon,
	ListItemButton,
	Dialog,
	DialogTitle,
	DialogContent,
} from "@mui/material";
import {
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
import AddIcon from "@mui/icons-material/Add";
import MarketDetails from "./components/MarketDetails";
import FundDetails from "./components/FundDetails";
import AddYieldPopup from "./components/AddYieldPopup";
import Yields from "./components/Yields";
import AddMarketPopup from "./components/AddMarketPopup";
import AddFundPopup from "./components/AddFundPopup";
import TokenDetails from "./components/TokenDetails";
import { AddProjectTokenPopup } from "./components/AddProjectTokenPopup";

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

	const [loading, setLoading] = useState(true);
	const [openYieldPopup, setOpenYieldPopup] = useState(false);
	const [openFundPopup, setOpenFundPopup] = useState(false);
	const [openMarketPopup, setOpenMarketPopup] = useState(false);
	const [openPreSaleTokenPopup, setOpenPreSaleTokenPopup] = useState(false);

	const handleOpenYieldPopup = () => {
		setOpenYieldPopup(true);
	};

	const handleCloseYieldPopup = () => {
		setOpenYieldPopup(false);
	};

	const handleOpenFundPopup = () => {
		setOpenFundPopup(true);
	};

	const handleCloseFundPopup = () => {
		setOpenFundPopup(false);
	};

	const handleOpenMarketPopup = () => {
		setOpenMarketPopup(true);
	};

	const handleCloseMarketPopup = () => {
		setOpenMarketPopup(false);
	};

	const handleOpenPreSaleTokenPopup = () => {
		setOpenPreSaleTokenPopup(true);
	};

	const handleClosePreSaleTokenPopup = () => {
		setOpenPreSaleTokenPopup(false);
	};

	useEffect(() => {
		IndexerService.getAdminIndexerCis2Token(contract_address!, token_id!)
			.then((data) => {
				setToken(data);
				setLoading(false);
			})
			.catch(() => {
				alert("Failed to fetch token details");
				setLoading(false);
			});
		ForestProjectService.getAdminForestProjectsContract(contract_address!).then((data) => {
			setTokenContract(data);
		});
	}, [contract_address, token_id]);
	useEffect(() => {
		IndexerService.getAdminIndexerCis2TokenMarket(contract_address!, token_id!)
			.then((market) => {
				setMarket(market);
			})
			.catch(() => {
				setMarket(undefined);
			});
	}, [contract_address, token_id]);
	useEffect(() => {
		if (market) {
			ForestProjectService.getAdminTokenMetadata(market.currency_token_contract_address, market.currency_token_id)
				.then((metadata) => {
					setMarketCurrencyMetdata(metadata);
				})
				.catch(() => {
					setMarketCurrencyMetdata(undefined);
				});
		}
	}, [market]);
	useEffect(() => {
		IndexerService.getAdminIndexerCis2TokenYieldsList(contract_address!, token_id!)
			.then((data) => {
				setYields(data);
			})
			.catch(() => {
				setYields([]);
			});
	}, [contract_address, token_id]);
	useEffect(() => {
		IndexerService.getAdminIndexerCis2TokenFund(contract_address!, token_id!)
			.then((fund) => {
				setFund(fund);
			})
			.catch(() => {
				setFund(undefined);
			});
	}, [contract_address, token_id]);
	useEffect(() => {
		if (fund) {
			ForestProjectService.getAdminTokenMetadata(fund.currency_token_contract_address, fund.currency_token_id)
				.then((metadata) => {
					setFundCurrencyMetdata(metadata);
				})
				.catch(() => {
					setFundCurrencyMetdata(undefined);
				});
		}
	}, [fund]);
	useEffect(() => {
		UserService.getSystemConfig().then((data) => {
			setContracts(data);
		});
	}, [contract_address, token_id]);
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
			IndexerService.getAdminIndexerCis2Token(preSaleTokenContract.contract_address!, token_id!).then((data) => {
				setPreSaleToken(data);
			});
		}
	}, [preSaleTokenContract, token_id]);

	if (loading) {
		return <CircularProgress />;
	}

	if (!token) {
		return <Typography>No token found</Typography>;
	}

	return (
		<Box sx={{ flexGrow: 1, padding: 2 }}>
			<Grid container spacing={2}>
				<Grid item xs={12} md={8}>
					<Paper sx={{ padding: 2 }}>
						<TokenDetails token={token} tokenContract={tokenContract} />
					</Paper>
					<Paper sx={{ padding: 2, marginTop: 2 }}>
						{market ? (
							<MarketDetails market={market} tokenMetadata={tokenContract} currencyMetadata={marketCurrencyMetdata} />
						) : (
							<Typography>No market details available</Typography>
						)}
					</Paper>
					<Paper sx={{ padding: 2, marginTop: 2 }}>
						{preSaleToken ? (
							<TokenDetails token={preSaleToken} tokenContract={preSaleTokenContract} />
						) : (
							<Typography>No Pre Sale Token Attached</Typography>
						)}
					</Paper>
					<Paper sx={{ padding: 2, marginTop: 2 }}>
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
					</Paper>
					<Paper sx={{ padding: 2, marginTop: 2 }}>
						{yields.length > 0 && tokenContract && contracts ? (
							<Yields
								tokenId={token_id!}
								tokenContract={tokenContract!}
								yielderContract={contracts.yielder_contract_index}
								yields={yields}
							/>
						) : (
							<Typography>No yields available</Typography>
						)}
					</Paper>
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
					<AddYieldPopup contract_address={contract_address!} token_id={token_id!} onDone={() => window.location.reload()} />
				</DialogContent>
			</Dialog>
			{contracts && contracts.mint_funds_contract && tokenContract && preSaleTokenContract && (
				<Dialog open={openFundPopup} onClose={handleCloseFundPopup} fullWidth>
					<DialogTitle>Add Fund</DialogTitle>
					<DialogContent>
						<AddFundPopup
							tokenId={token_id!}
							forestProjectId={id!}
							onDone={() => window.location.reload()}
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
						onDone={() => window.location.reload()}
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
							onDone={() => window.location.reload()}
						/>
					</DialogContent>
				</Dialog>
			)}
		</Box>
	);
}
