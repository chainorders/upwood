import { useEffect, useState } from "react";
import { Alert, Grid, Typography } from "@mui/material";
import {
	Market,
	TokenMetadata,
	ForestProjectService,
	ForestProjectTokenContract,
	Agent,
	IndexerService,
	TokenHolder,
} from "../../../apiClient";
import DeleteIcon from "@mui/icons-material/Delete";
import TransactionButton from "../../../components/TransactionButton";
import { TxnStatus, updateContract } from "../../../lib/concordium";
import securityP2PTrading from "../../../contractClients/generated/securityP2PTrading";
import { User } from "../../../lib/user";
import { toDisplayAmount, toTokenId } from "../../../lib/conversions";
import euroeStablecoin from "../../../contractClients/generated/euroeStablecoin";
import concordiumNodeClient from "../../../contractClients/ConcordiumNodeClient";
import { ContractAddress } from "@concordium/web-sdk";
import securitySftMulti from "../../../contractClients/generated/securitySftMulti";

interface MarketDetailsProps {
	user: User;
	market: Market;
	tokenMetadata?: ForestProjectTokenContract;
	currencyMetadata?: TokenMetadata;
	onRefresh: () => void;
}

export default function MarketDetails({ market, user, onRefresh }: MarketDetailsProps) {
	const [marketCurrencyMetadata, setmarketCurrencyMetadata] = useState<TokenMetadata>();
	const [txnStatus, setTxnStatus] = useState<TxnStatus>("none");
	const [isCurrencyOperator, setIsCurrencyOperator] = useState(false);
	const [refreshCounter, setRefreshCounter] = useState(0);
	const [agent, setAgent] = useState<Agent>();
	const [removeAgentTxnStatus, setRemoveAgentTxnStatus] = useState<TxnStatus>("none");
	const [addAgentTxnStatus, setAddAgentTxnStatus] = useState<TxnStatus>("none");
	const [holder, setHolder] = useState<TokenHolder>();

	useEffect(() => {
		ForestProjectService.getAdminTokenMetadata(market.currency_token_contract_address, market.currency_token_id)
			.then((metadata) => {
				setmarketCurrencyMetadata(metadata);
			})
			.catch(() => {
				setmarketCurrencyMetadata(undefined);
			});
		euroeStablecoin.operatorOf
			.invoke(concordiumNodeClient, ContractAddress.create(Number(market.currency_token_contract_address)), [
				{
					owner: {
						Account: [market.liquidity_provider],
					},
					address: {
						Contract: [
							{
								index: Number(market.contract_address),
								subindex: 0,
							},
						],
					},
				},
			])
			.then((res) => euroeStablecoin.operatorOf.parseReturnValue(res.returnValue!)!)
			.then((res) => setIsCurrencyOperator(res[0]));
		IndexerService.getAdminIndexerCis2Agent(market.token_contract_address, market.contract_address, true).then(setAgent);
		IndexerService.getAdminIndexerCis2TokenHolder(
			market.token_contract_address,
			market.token_id,
			market.liquidity_provider,
		).then(setHolder);
	}, [market, refreshCounter]);

	const deleteMarket = async () => {
		try {
			await updateContract(
				user.concordiumAccountAddress,
				market.contract_address,
				securityP2PTrading.removeMarket,
				{
					id: toTokenId(BigInt(market.token_id), 8),
					contract: { index: Number(market.token_contract_address), subindex: 0 },
				},
				setTxnStatus,
			);
			setTxnStatus("success");
			alert("Market deleted successfully");
			onRefresh();
		} catch (e) {
			console.error(e);
			setTxnStatus("error");
			alert("Failed to delete market");
		}
	};

	const removeAgent = async () => {
		try {
			await updateContract(
				user.concordiumAccountAddress,
				market.token_contract_address,
				securitySftMulti.removeAgent,
				{ Contract: [{ index: Number(market.contract_address), subindex: 0 }] },
				setRemoveAgentTxnStatus,
			);
			setRemoveAgentTxnStatus("success");
			alert("Agent removed successfully");
			setRefreshCounter((prev) => prev + 1);
		} catch (e) {
			console.error(e);
			setRemoveAgentTxnStatus("error");
			alert("Failed to remove agent");
		}
	};

	const addAgent = async () => {
		try {
			await updateContract(
				user.concordiumAccountAddress,
				market.token_contract_address,
				securitySftMulti.addAgent,
				{
					address: { Contract: [{ index: Number(market.contract_address), subindex: 0 }] },
					roles: [{ Operator: {} }],
				},
				setAddAgentTxnStatus,
			);
			setAddAgentTxnStatus("success");
			alert("Agent added successfully");
			setRefreshCounter((prev) => prev + 1);
		} catch (e) {
			console.error(e);
			setAddAgentTxnStatus("error");
			alert("Failed to add agent");
		}
	};

	return (
		<Grid container spacing={2} style={{ marginTop: "1rem" }}>
			<Grid item xs={12} md={6}>
				<Typography variant="h6">Market Details</Typography>
			</Grid>
			<Grid item xs={12} md={6} style={{ display: "flex", justifyContent: "flex-end" }}>
				<TransactionButton
					variant="outlined"
					color="error"
					startIcon={<DeleteIcon />}
					txnStatus={txnStatus}
					defaultText="Delete"
					loadingText="Deleting..."
					onClick={deleteMarket}
				/>
			</Grid>
			<Grid item xs={12} md={6}>
				<Typography>
					<strong>Contract Address:</strong> {market.contract_address}
				</Typography>
				<Typography>
					<strong>Currency Token ID:</strong> {market.currency_token_id}
				</Typography>
				<Typography>
					<strong>Currency Token Contract Address:</strong> {market.currency_token_contract_address}
				</Typography>
				<Typography style={{ display: "flex", justifyContent: "space-between", alignItems: "center" }}>
					<span>
						<strong>Liquidity Provider:</strong> {market.liquidity_provider}
					</span>
				</Typography>
				<Typography>
					<strong>Buy Rate:</strong> {market.buy_rate_numerator} / {market.buy_rate_denominator}
				</Typography>
				<Typography>
					<strong>Sell Rate:</strong> {market.sell_rate_numerator} / {market.sell_rate_denominator}
				</Typography>
				<Typography>
					<strong>Total Sell Token Amount:</strong> {market.total_sell_token_amount}
				</Typography>
				<Typography>
					<strong>Total Sell Currency Amount:</strong> {market.total_sell_currency_amount} (
					{toDisplayAmount(market.total_sell_currency_amount, marketCurrencyMetadata?.decimals || 6, 2)}
					{marketCurrencyMetadata?.symbol})
				</Typography>
				{marketCurrencyMetadata && (
					<>
						<Typography>
							<strong>Currency Symbol:</strong> {marketCurrencyMetadata.symbol}
						</Typography>
						<Typography>
							<strong>Currency Decimals:</strong> {marketCurrencyMetadata.decimals}
						</Typography>
					</>
				)}
				<Typography>
					<strong>Create Time:</strong> {market.create_time}
				</Typography>
				<Typography>
					<strong>Update Time:</strong> {market.update_time}
				</Typography>
			</Grid>
			<Grid item xs={12} md={6}>
				<Grid container spacing={2} direction={"row-reverse"}>
					<Grid item xs={12} md={12} lg={6}>
						{isCurrencyOperator && (
							<Alert severity="success" style={{ marginTop: "10px" }}>
								<Typography>Market contract is operator of Liquidity Provider in the currency token contract</Typography>
							</Alert>
						)}
						{!isCurrencyOperator && (
							<Alert severity="error" style={{ marginTop: "10px" }}>
								<Typography>Market contract is not operator of Liquidity Provider in the currency token contract</Typography>
							</Alert>
						)}
					</Grid>
					<Grid item xs={12} md={12} lg={6}>
						{agent && (
							<Alert severity="success" style={{ marginTop: "10px" }}>
								<Typography>Market contract is agent of token contract. With the roles {agent.roles.join(", ")}</Typography>
								<TransactionButton
									txnStatus={removeAgentTxnStatus}
									defaultText="Remove Agent"
									loadingText="Removing Agent..."
									variant="outlined"
									color="primary"
									onClick={removeAgent}
									sx={{ mt: 2 }}
								/>
							</Alert>
						)}
						{!agent && (
							<Alert severity="error" style={{ marginTop: "10px" }}>
								<Typography>Market contract is not agent of token contract</Typography>
								<TransactionButton
									txnStatus={addAgentTxnStatus}
									defaultText="Add Agent"
									loadingText="Adding Agent..."
									variant="outlined"
									color="primary"
									onClick={addAgent}
									sx={{ mt: 2 }}
								/>
							</Alert>
						)}
					</Grid>
					<Grid item xs={12} md={12} lg={6}>
						{!holder && (
							<Alert severity="warning">
								<Typography>Liquidity Provider is Not holding any tokens.</Typography>
								<Typography>Anyone will not be able to buy.</Typography>
							</Alert>
						)}
						{holder && (
							<Alert severity="info">
								<Typography>
									Balance of Liquidity Provider is{" "}
									<Typography variant="body2" fontWeight="bold">
										{holder.un_frozen_balance}
									</Typography>
								</Typography>
							</Alert>
						)}
					</Grid>
				</Grid>
			</Grid>
		</Grid>
	);
}
