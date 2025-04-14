import { useEffect, useState } from "react";
import {
	Paper,
	Typography,
	Grid,
	Alert,
	Tooltip,
	IconButton,
	CircularProgress,
	Table,
	TableBody,
	TableCell,
	TableContainer,
	TableHead,
	TableRow,
	TablePagination,
} from "@mui/material";
import {
	Agent,
	ForestProjectTokenContract,
	IndexerService,
	SystemContractsConfigApiModel,
	Treasury,
	Yield,
	YieldType,
	PagedResponse_Yield,
} from "../../../apiClient";
import DeleteIcon from "@mui/icons-material/Delete";
import AddIcon from "@mui/icons-material/Add";
import RefreshIcon from "@mui/icons-material/Refresh";
import { toDisplayAmount, toTokenId } from "../../../lib/conversions";
import { TxnStatus, updateContract } from "../../../lib/concordium";
import securitySftMultiYielder from "../../../contractClients/generated/securitySftMultiYielder";
import TransactionButton from "../../../components/TransactionButton";
import { User } from "../../../lib/user";
import securitySftMulti from "../../../contractClients/generated/securitySftMulti";
import euroeStablecoin from "../../../contractClients/generated/euroeStablecoin";
import concordiumNodeClient from "../../../contractClients/ConcordiumNodeClient";
import { AccountAddress, CcdAmount, ContractAddress } from "@concordium/web-sdk";
import securitySftSingle from "../../../contractClients/generated/securitySftSingle";

interface YieldsProps {
	user: User;
	tokenContract: ForestProjectTokenContract;
	tokenId?: string;
	yielderContract: string;
	contracts: SystemContractsConfigApiModel;
	onRefresh: () => void;
	page?: number;
	pageSize?: number;
	yieldTokenContractAddress?: string;
	yieldTokenId?: string;
	yieldType?: YieldType;
}

export default function Yields({
	user,
	yielderContract,
	tokenContract,
	tokenId,
	onRefresh,
	contracts,
	page = 0,
	pageSize = 10, // Changed default to 10 for better table pagination
	yieldTokenContractAddress,
	yieldTokenId,
	yieldType,
}: YieldsProps) {
	const [txnStatus, setTxnStatus] = useState<TxnStatus>("none");
	const [agentTokenContract, setAgentTokenContract] = useState<Agent>();
	const [agentCarbonCreditsContract, setAgentCarbonCreditsContract] = useState<Agent>();
	const [agentTreesContract, setAgentTreesContract] = useState<Agent>();
	const [removeAgentTokenContractTxnStatus, setRemoveAgentTokenContractTxnStatus] = useState<TxnStatus>("none");
	const [addAgentTokenContractTxnStatus, setAddAgentTokenContractTxnStatus] = useState<TxnStatus>("none");
	const [removeAgentCarbonCreditsContractTxnStatus, setRemoveAgentCarbonCreditsContractTxnStatus] =
		useState<TxnStatus>("none");
	const [addAgentCarbonCreditsContractTxnStatus, setAddAgentCarbonCreditsContractTxnStatus] =
		useState<TxnStatus>("none");
	const [removeAgentTreesContractTxnStatus, setRemoveAgentTreesContractTxnStatus] = useState<TxnStatus>("none");
	const [addAgentTreesContractTxnStatus, setAddAgentTreesContractTxnStatus] = useState<TxnStatus>("none");
	const [refreshCounter, setRefreshCounter] = useState(0);
	const [isCurrencyOperator, setIsCurrencyOperator] = useState(false);
	const [treasury, setTreasury] = useState<Treasury>();
	const [addCurrencyOperatorTxnStatus, setAddCurrencyOperatorTxnStatus] = useState<TxnStatus>("none");
	const [removeCurrencyOperatorTxnStatus, setRemoveCurrencyOperatorTxnStatus] = useState<TxnStatus>("none");
	const [yields, setYields] = useState<Yield[]>([]);
	const [yieldsResponse, setYieldsResponse] = useState<PagedResponse_Yield>();
	const [loading, setLoading] = useState(true);
	const [currentPage, setCurrentPage] = useState(page);
	const [rowsPerPage, setRowsPerPage] = useState(pageSize);

	// Fetch yields data
	useEffect(() => {
		setLoading(true);
		IndexerService.getAdminIndexerYieldList(
			currentPage,
			rowsPerPage,
			tokenContract.contract_address,
			tokenId,
			yieldTokenContractAddress,
			yieldTokenId,
			yieldType,
		)
			.then((response) => {
				setYieldsResponse(response);
				setYields(response.data);
			})
			.catch((error) => {
				console.error("Failed to fetch yields:", error);
			})
			.finally(() => {
				setLoading(false);
			});
	}, [
		tokenContract.contract_address,
		tokenId,
		currentPage,
		rowsPerPage,
		yieldTokenContractAddress,
		yieldTokenId,
		yieldType,
		refreshCounter,
	]);

	// Handle page change
	const handleChangePage = (_event: unknown, newPage: number) => {
		setCurrentPage(newPage);
	};

	// Handle rows per page change
	const handleChangeRowsPerPage = (event: React.ChangeEvent<HTMLInputElement>) => {
		setRowsPerPage(parseInt(event.target.value, 10));
		setCurrentPage(0);
	};

	const addCurrencyOperator = async () => {
		try {
			await updateContract(
				user.concordiumAccountAddress,
				contracts.euro_e_contract_index,
				euroeStablecoin.updateOperator,
				[
					{
						update: {
							Add: {},
						},
						operator: {
							Contract: [
								{
									index: Number(contracts.yielder_contract_index),
									subindex: 0,
								},
							],
						},
					},
				],
				setAddCurrencyOperatorTxnStatus,
			);
			setAddCurrencyOperatorTxnStatus("success");
			alert("Added yielder contract as operator of treasury");
			setRefreshCounter((prev) => prev + 1);
		} catch (e) {
			setAddCurrencyOperatorTxnStatus("error");
			console.error(e);
			alert("Failed to add yielder contract as operator of treasury");
		}
	};

	const removeCurrencyOperator = async () => {
		try {
			await updateContract(
				user.concordiumAccountAddress,
				contracts.euro_e_contract_index,
				euroeStablecoin.updateOperator,
				[
					{
						update: {
							Remove: {},
						},
						operator: {
							Contract: [
								{
									index: Number(contracts.yielder_contract_index),
									subindex: 0,
								},
							],
						},
					},
				],
				setRemoveCurrencyOperatorTxnStatus,
			);
			setRemoveCurrencyOperatorTxnStatus("success");
			alert("Removed yielder contract as operator of treasury");
			setRefreshCounter((prev) => prev + 1);
		} catch (e) {
			setRemoveCurrencyOperatorTxnStatus("error");
			console.error(e);
			alert("Failed to remove yielder contract as operator of treasury");
		}
	};

	const removeYield = async (tokenId: string) => {
		try {
			await updateContract(
				user.concordiumAccountAddress,
				yielderContract,
				securitySftMultiYielder.removeYield,
				{
					token_id: toTokenId(BigInt(tokenId), 8),
					token_contract: {
						index: Number(tokenContract.contract_address),
						subindex: 0,
					},
				},
				setTxnStatus,
			);
			setTxnStatus("success");
			onRefresh();
		} catch (e) {
			setTxnStatus("error");
			console.error(e);
			alert("Failed to delete yield");
		}
	};

	const removeAgentTokenContract = async () => {
		try {
			await updateContract(
				user.concordiumAccountAddress,
				tokenContract.contract_address,
				securitySftMulti.removeAgent,
				{ Contract: [{ index: Number(yielderContract), subindex: 0 }] },
				setRemoveAgentTokenContractTxnStatus,
			);
			setRemoveAgentTokenContractTxnStatus("success");
			setRefreshCounter((prev) => prev + 1);
		} catch (e) {
			setRemoveAgentTokenContractTxnStatus("error");
			console.error(e);
			alert("Failed to remove agent");
		}
	};

	const addAgentTokenContract = async () => {
		try {
			await updateContract(
				user.concordiumAccountAddress,
				tokenContract.contract_address,
				securitySftMulti.addAgent,
				{
					address: { Contract: [{ index: Number(yielderContract), subindex: 0 }] },
					roles: [{ Operator: {} }, { Mint: {} }],
				},
				setAddAgentTokenContractTxnStatus,
			);
			setAddAgentTokenContractTxnStatus("success");
			setRefreshCounter((prev) => prev + 1);
		} catch (e) {
			setAddAgentTokenContractTxnStatus("error");
			console.error(e);
			alert("Failed to add agent");
		}
	};

	const removeAgentCarbonCreditsContract = async () => {
		try {
			await updateContract(
				user.concordiumAccountAddress,
				contracts.carbon_credit_contract_index,
				securitySftSingle.removeAgent,
				{ Contract: [{ index: Number(contracts.yielder_contract_index), subindex: 0 }] },
				setRemoveAgentCarbonCreditsContractTxnStatus,
			);
			setRemoveAgentCarbonCreditsContractTxnStatus("success");
			setRefreshCounter((prev) => prev + 1);
		} catch (e) {
			setRemoveAgentCarbonCreditsContractTxnStatus("error");
			console.error(e);
			alert("Failed to remove agent");
		}
	};

	const addAgentCarbonCreditsContract = async () => {
		try {
			await updateContract(
				user.concordiumAccountAddress,
				contracts.carbon_credit_contract_index,
				securitySftSingle.addAgent,
				{
					address: { Contract: [{ index: Number(contracts.yielder_contract_index), subindex: 0 }] },
					roles: [{ Operator: {} }],
				},
				setAddAgentCarbonCreditsContractTxnStatus,
			);
			setAddAgentCarbonCreditsContractTxnStatus("success");
			setRefreshCounter((prev) => prev + 1);
		} catch (e) {
			setAddAgentCarbonCreditsContractTxnStatus("error");
			console.error(e);
			alert("Failed to add agent");
		}
	};

	const removeAgentTreesContract = async () => {
		try {
			await updateContract(
				user.concordiumAccountAddress,
				contracts.tree_ft_contract_index,
				securitySftSingle.removeAgent,
				{ Contract: [{ index: Number(contracts.yielder_contract_index), subindex: 0 }] },
				setRemoveAgentTreesContractTxnStatus,
			);
			setRemoveAgentTreesContractTxnStatus("success");
			setRefreshCounter((prev) => prev + 1);
		} catch (e) {
			setRemoveAgentTreesContractTxnStatus("error");
			console.error(e);
			alert("Failed to remove agent");
		}
	};

	const addAgentTreesContract = async () => {
		try {
			await updateContract(
				user.concordiumAccountAddress,
				contracts.tree_ft_contract_index,
				securitySftSingle.addAgent,
				{
					address: { Contract: [{ index: Number(contracts.yielder_contract_index), subindex: 0 }] },
					roles: [{ Operator: {} }],
				},
				setAddAgentTreesContractTxnStatus,
			);
			setAddAgentTreesContractTxnStatus("success");
			setRefreshCounter((prev) => prev + 1);
		} catch (e) {
			setAddAgentTreesContractTxnStatus("error");
			console.error(e);
			alert("Failed to add agent");
		}
	};

	useEffect(() => {
		IndexerService.getAdminIndexerCis2Agent(tokenContract.contract_address, yielderContract, true).then(
			setAgentTokenContract,
		);
		IndexerService.getAdminIndexerYielderTreasury(yielderContract).then(setTreasury);
	}, [refreshCounter, tokenContract.contract_address, yielderContract]);

	useEffect(() => {
		if (!treasury) return;

		euroeStablecoin.operatorOf
			.invoke(
				concordiumNodeClient,
				ContractAddress.create(BigInt(contracts.euro_e_contract_index), BigInt(0)),
				[
					{
						//TODO: treasury address can also be a contract address
						owner: { Account: [treasury.treasury_address] },
						address: {
							Contract: [
								{
									index: Number(contracts.yielder_contract_index),
									subindex: 0,
								},
							],
						},
					},
				],
				AccountAddress.fromBase58(user.concordiumAccountAddress),
				CcdAmount.fromCcd(0),
			)
			.then((response) => euroeStablecoin.operatorOf.parseReturnValue(response.returnValue!)!)
			.then((response) => setIsCurrencyOperator(response[0]));

		IndexerService.getAdminIndexerCis2Agent(contracts.carbon_credit_contract_index, yielderContract, true).then(
			setAgentCarbonCreditsContract,
		);
		IndexerService.getAdminIndexerCis2Agent(contracts.tree_ft_contract_index, yielderContract, true).then(
			setAgentTreesContract,
		);
	}, [treasury, yielderContract, contracts, user.concordiumAccountAddress, refreshCounter]);

	return (
		<Paper sx={{ padding: 2, marginTop: 2 }}>
			<Grid container spacing={2}>
				<Grid item xs={12} md={4}>
					<Typography variant="h6">Yields</Typography>
				</Grid>
				<Grid item xs={12} md={8} justifyContent="flex-end" display="flex">
					{tokenContract && tokenId && (
						<TransactionButton
							variant="outlined"
							color="secondary"
							type="button"
							startIcon={<DeleteIcon />}
							onClick={() => tokenId && removeYield(tokenId)}
							txnStatus={txnStatus}
							defaultText="Delete"
							loadingText="Deleting..."
							disabled={!yields || yields.length === 0 || !tokenId || !tokenContract}
						/>
					)}
					<IconButton aria-label="refresh" onClick={() => setRefreshCounter((prev) => prev + 1)}>
						<RefreshIcon />
					</IconButton>
				</Grid>

				{loading ? (
					<Grid item xs={12} display="flex" justifyContent="center" mt={2}>
						<CircularProgress />
					</Grid>
				) : yields.length > 0 ? (
					<Grid container spacing={2} mt={1}>
						<Grid item xs={12}>
							<TableContainer>
								<Table sx={{ minWidth: 650 }} aria-label="yields table">
									<TableHead>
										<TableRow>
											<TableCell>Token</TableCell>
											<TableCell>Yield Type</TableCell>
											<TableCell>Yield Contract</TableCell>
											<TableCell align="right">Rate</TableCell>
										</TableRow>
									</TableHead>
									<TableBody>
										{yields.map((yield_, index) => (
											<TableRow key={index} sx={{ "&:last-child td, &:last-child th": { border: 0 } }}>
												<TableCell component="th" scope="row">
													<Typography fontWeight="">
														{yield_.contract_address} / {yield_.token_id}
													</Typography>
												</TableCell>
												<TableCell>{yield_.yield_type}</TableCell>
												<TableCell>
													<Typography variant="subtitle1" fontWeight="bold">
														{
															{
																[contracts.euro_e_contract_index]: "Euro E",
																[contracts.tree_ft_contract_index]: "Tree",
																[contracts.carbon_credit_contract_index]: "Carbon Credit",
															}[yield_.yield_contract_address]
														}
													</Typography>
													<Typography variant="body2" noWrap sx={{ maxWidth: 150 }}>
														{yield_.yield_contract_address} / {yield_.yield_token_id}
													</Typography>
												</TableCell>
												<TableCell align="right">
													{yield_.yield_contract_address === contracts.euro_e_contract_index
														? toDisplayAmount(yield_.yield_rate_numerator, 6)
														: yield_.yield_rate_numerator}
												</TableCell>
											</TableRow>
										))}
									</TableBody>
								</Table>
							</TableContainer>
							<TablePagination
								rowsPerPageOptions={[5, 10, 25]}
								component="div"
								count={(yieldsResponse?.page_count || 0) * rowsPerPage}
								rowsPerPage={rowsPerPage}
								page={currentPage}
								onPageChange={handleChangePage}
								onRowsPerPageChange={handleChangeRowsPerPage}
							/>
						</Grid>
					</Grid>
				) : (
					<Grid item xs={12} mt={1}>
						<Typography>No yields available</Typography>
					</Grid>
				)}
				<Grid item xs={12} md={12}>
					<Grid container spacing={2} direction={"row-reverse"}>
						<Grid item xs={12} md={6} lg={3}>
							{agentTokenContract && (
								<Alert severity="success">
									<Typography>
										Yielder contract is an agent of the token contract with roles {agentTokenContract.roles.join(", ")}
									</Typography>
									<TransactionButton
										defaultText="Remove Agent"
										loadingText="Removing..."
										txnStatus={removeAgentTokenContractTxnStatus}
										onClick={removeAgentTokenContract}
										variant="outlined"
										color="primary"
										startIcon={<DeleteIcon />}
										sx={{ mt: 2 }}
									/>
								</Alert>
							)}
							{!agentTokenContract && (
								<Alert severity="error">
									<Typography>
										Yielder contract is not an agent of the token contract and therefore will not be able to burn older token
										version or mint new token version.
									</Typography>
									<TransactionButton
										defaultText="Add Agent"
										loadingText="Adding..."
										txnStatus={addAgentTokenContractTxnStatus}
										onClick={addAgentTokenContract}
										variant="outlined"
										color="primary"
										startIcon={<AddIcon />}
										sx={{ mt: 2 }}
									/>
								</Alert>
							)}
						</Grid>
						<Grid item xs={12} md={6} lg={3}>
							{agentCarbonCreditsContract && (
								<Alert severity="success">
									<Typography>
										Yielder contract is an agent of the carbon credits contract with roles{" "}
										{agentCarbonCreditsContract.roles.join(", ")}
									</Typography>
									<TransactionButton
										defaultText="Remove Agent"
										loadingText="Removing..."
										txnStatus={removeAgentCarbonCreditsContractTxnStatus}
										onClick={removeAgentCarbonCreditsContract}
										variant="outlined"
										color="primary"
										startIcon={<DeleteIcon />}
										sx={{ mt: 2 }}
									/>
								</Alert>
							)}
							{!agentCarbonCreditsContract && (
								<Alert severity="error">
									<Typography>
										Yielder contract is not an agent of the carbon credits contract and therefore will not be able to transfer
										tokens from Treasury to the user.
									</Typography>
									<TransactionButton
										defaultText="Add Agent"
										loadingText="Adding..."
										txnStatus={addAgentCarbonCreditsContractTxnStatus}
										onClick={addAgentCarbonCreditsContract}
										variant="outlined"
										color="primary"
										startIcon={<AddIcon />}
										sx={{ mt: 2 }}
									/>
								</Alert>
							)}
						</Grid>
						<Grid item xs={12} md={6} lg={3}>
							{agentTreesContract && (
								<Alert severity="success">
									<Typography>
										Yielder contract is an agent of the trees contract with roles {agentTreesContract.roles.join(", ")}
									</Typography>
									<TransactionButton
										defaultText="Remove Agent"
										loadingText="Removing..."
										txnStatus={removeAgentTreesContractTxnStatus}
										onClick={removeAgentTreesContract}
										variant="outlined"
										color="primary"
										startIcon={<DeleteIcon />}
										sx={{ mt: 2 }}
									/>
								</Alert>
							)}
							{!agentTreesContract && (
								<Alert severity="error">
									<Typography>
										Yielder contract is not an agent of the trees contract and therefore will not be able to transfer tokens from
										Treasury to the user.
									</Typography>
									<TransactionButton
										defaultText="Add Agent"
										loadingText="Adding..."
										txnStatus={addAgentTreesContractTxnStatus}
										onClick={addAgentTreesContract}
										variant="outlined"
										color="primary"
										startIcon={<AddIcon />}
										sx={{ mt: 2 }}
									/>
								</Alert>
							)}
						</Grid>
						<Grid item xs={12} md={6} lg={3}>
							{isCurrencyOperator && (
								<Alert severity="success">
									<Typography>
										<Tooltip title={contracts.yielder_contract_index}>
											<Typography fontWeight="bold" component="span">
												Yielder Contract
											</Typography>
										</Tooltip>{" "}
										is an Operator of{" "}
										<Tooltip title={treasury?.treasury_address}>
											<Typography fontWeight="bold" component="span">
												Treasury
											</Typography>
										</Tooltip>{" "}
										for{" "}
										<Tooltip title={contracts.euro_e_contract_index}>
											<Typography fontWeight="bold" component="span">
												Euroe Stablecoin
											</Typography>
										</Tooltip>
									</Typography>
									{user.concordiumAccountAddress === treasury?.treasury_address && (
										<TransactionButton
											defaultText="Remove Operator"
											loadingText="Removing..."
											txnStatus={removeCurrencyOperatorTxnStatus}
											onClick={removeCurrencyOperator}
											variant="outlined"
											color="primary"
											startIcon={<DeleteIcon />}
											sx={{ mt: 2 }}
										/>
									)}
								</Alert>
							)}
							{!isCurrencyOperator && (
								<Alert severity="error">
									<Typography>
										<Tooltip title={contracts.yielder_contract_index}>
											<Typography fontWeight="bold" component="span">
												Yielder Contract
											</Typography>
										</Tooltip>{" "}
										is not an Operator of{" "}
										<Tooltip title={treasury?.treasury_address}>
											<Typography fontWeight="bold" component="span">
												Treasury
											</Typography>
										</Tooltip>{" "}
										for{" "}
										<Tooltip title={contracts.euro_e_contract_index}>
											<Typography fontWeight="bold" component="span">
												Euroe Stablecoin
											</Typography>
										</Tooltip>
									</Typography>
									{user.concordiumAccountAddress === treasury?.treasury_address && (
										<TransactionButton
											defaultText="Add Operator"
											loadingText="Adding..."
											txnStatus={addCurrencyOperatorTxnStatus}
											onClick={addCurrencyOperator}
											variant="outlined"
											color="primary"
											startIcon={<AddIcon />}
											sx={{ mt: 2 }}
										/>
									)}
								</Alert>
							)}
						</Grid>
					</Grid>
				</Grid>
			</Grid>
		</Paper>
	);
}
