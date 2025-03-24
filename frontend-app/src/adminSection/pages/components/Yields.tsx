import { useEffect, useState } from "react";
import { Paper, Typography, Grid, Alert, Tooltip, IconButton, ButtonGroup } from "@mui/material";
import {
	Agent,
	ForestProjectTokenContract,
	IndexerService,
	SystemContractsConfigApiModel,
	Treasury,
	Yield,
} from "../../../apiClient";
import DeleteIcon from "@mui/icons-material/Delete";
import AddIcon from "@mui/icons-material/Add";
import RefreshIcon from "@mui/icons-material/Refresh";
import { toTokenId } from "../../../lib/conversions";
import { TxnStatus, updateContract } from "../../../lib/concordium";
import securitySftMultiYielder from "../../../contractClients/generated/securitySftMultiYielder";
import TransactionButton from "../../../components/TransactionButton";
import { User } from "../../../lib/user";
import securitySftMulti from "../../../contractClients/generated/securitySftMulti";
import YieldCard from "./YieldCard";
import euroeStablecoin from "../../../contractClients/generated/euroeStablecoin";
import concordiumNodeClient from "../../../contractClients/ConcordiumNodeClient";
import { AccountAddress, CcdAmount, ContractAddress } from "@concordium/web-sdk";
import securitySftSingle from "../../../contractClients/generated/securitySftSingle";

interface YieldsProps {
	user: User;
	tokenContract: ForestProjectTokenContract;
	tokenId: string;
	yielderContract: string;
	yields: Yield[];
	contracts: SystemContractsConfigApiModel;
	onRefresh: () => void;
}

export default function Yields({
	user,
	yields,
	yielderContract,
	tokenContract,
	tokenId,
	onRefresh,
	contracts,
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

	const removeYield = async () => {
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
					<TransactionButton
						variant="outlined"
						color="secondary"
						type="button"
						startIcon={<DeleteIcon />}
						onClick={async () => {
							console.log("delete");
							removeYield();
						}}
						txnStatus={txnStatus}
						defaultText="Delete"
						loadingText="Deleting..."
						disabled={!yields || yields.length === 0}
					/>
					<IconButton aria-label="refresh" onClick={() => setRefreshCounter((prev) => prev + 1)}>
						<RefreshIcon />
					</IconButton>
				</Grid>
				{yields.length > 0 ? (
					<Grid container spacing={2} mt={1}>
						<Grid item xs={12} md={6}>
							{yields.map((yield_) => (
								<YieldCard yield_={yield_} contracts={contracts} />
							))}
						</Grid>
						<Grid item xs={12} md={6}>
							<Grid container spacing={2} direction={"row-reverse"}>
								<Grid item xs={12} md={12} lg={6}>
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
								<Grid item xs={12} md={12} lg={6}>
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
								<Grid item xs={12} md={12} lg={6}>
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
												Yielder contract is not an agent of the trees contract and therefore will not be able to transfer tokens
												from Treasury to the user.
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
								<Grid item xs={12} md={12} lg={6}>
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
				) : (
					<Grid item xs={12} mt={1}>
						<Typography>No yields available</Typography>
					</Grid>
				)}
			</Grid>
		</Paper>
	);
}
