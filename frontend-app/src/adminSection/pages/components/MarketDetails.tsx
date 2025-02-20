import { useEffect, useState } from "react";
import { Paper, Typography } from "@mui/material";
import { Market, TokenMetadata, ForestProjectService, ForestProjectTokenContract } from "../../../apiClient";
import DeleteIcon from "@mui/icons-material/Delete";
import CheckIcon from "@mui/icons-material/Check";
import CloseIcon from "@mui/icons-material/Close";
import TransactionButton from "../../../components/TransactionButton";
import { TxnStatus, updateContract } from "../../../lib/concordium";
import securityP2PTrading from "../../../contractClients/generated/securityP2PTrading";
import { User } from "../../../lib/user";
import { useOutletContext } from "react-router";
import { toDisplayAmount, toTokenId } from "../../../lib/conversions";
import euroeStablecoin from "../../../contractClients/generated/euroeStablecoin";
import concordiumNodeClient from "../../../contractClients/ConcordiumNodeClient";
import { ContractAddress } from "@concordium/web-sdk";

interface MarketDetailsProps {
	market: Market;
	tokenMetadata?: ForestProjectTokenContract;
	currencyMetadata?: TokenMetadata;
}

export default function MarketDetails({ market }: MarketDetailsProps) {
	const { user } = useOutletContext<{ user: User }>();
	const [marketCurrencyMetadata, setmarketCurrencyMetadata] = useState<TokenMetadata>();
	const [txnStatus, setTxnStatus] = useState<TxnStatus>("none");
	const [isCurrencyOperator, setIsCurrencyOperator] = useState(false);
	const [refreshCounter, setRefreshCounter] = useState(0);

	useEffect(() => {
		if (market) {
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
		}
	}, [market, user, refreshCounter]);

	const deleteMarket = async () => {
		if (!market) {
			return;
		}

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
			setRefreshCounter((c) => c + 1);
		} catch (e) {
			console.error(e);
			setTxnStatus("error");
			alert("Failed to delete market");
		}
	};

	if (!market) {
		return (
			<Paper sx={{ padding: 2, marginTop: 2 }}>
				<Typography variant="h6">No market details available</Typography>
			</Paper>
		);
	}

	return (
		<>
			<div style={{ display: "flex", justifyContent: "space-between", alignItems: "center" }}>
				<Typography variant="h6">Market Details</Typography>
				<TransactionButton
					variant="outlined"
					color="error"
					startIcon={<DeleteIcon />}
					txnStatus={txnStatus}
					defaultText="Delete"
					loadingText="Deleting..."
					onClick={deleteMarket}
				/>
			</div>
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
				{/* <span style={{ display: "flex", alignItems: "center" }}>
					{isCurrencyOperator ? (
						<CheckIcon style={{ color: "green" }} />
					) : (
						<span>
							<CloseIcon style={{ color: "red" }} />
							<Typography color="error" style={{ marginLeft: 4 }}>
								Contract is not an Operator in currency contract
							</Typography>
						</span>
					)}
				</span> */}
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
		</>
	);
}
