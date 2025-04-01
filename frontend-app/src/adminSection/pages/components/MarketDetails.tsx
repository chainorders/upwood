import { useEffect, useState } from "react";
import { Alert, Box, Grid, IconButton, Paper, Typography } from "@mui/material";
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
import RefreshIcon from "@mui/icons-material/Refresh";
import TransactionButton from "../../../components/TransactionButton";
import { TxnStatus, updateContract } from "../../../lib/concordium";
import securityP2PTrading from "../../../contractClients/generated/securityP2PTrading";
import { User } from "../../../lib/user";
import { toDisplayAmount, toTokenId } from "../../../lib/conversions";
import euroeStablecoin from "../../../contractClients/generated/euroeStablecoin";
import concordiumNodeClient from "../../../contractClients/ConcordiumNodeClient";
import { ContractAddress } from "@concordium/web-sdk";
import securitySftMulti from "../../../contractClients/generated/securitySftMulti";
import useCommonStyles from "../../../theme/useCommonStyles";
import DetailRow from "./DetailRow";

interface MarketDetailsProps {
	user: User;
	market: Market;
	tokenMetadata?: ForestProjectTokenContract;
	currencyMetadata?: TokenMetadata;
	onRefresh: () => void;
}

export default function MarketDetails({ market, user, onRefresh }: MarketDetailsProps) {
	const classes = useCommonStyles();
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
		<Paper sx={classes.detailsContainer}>
			<Box sx={classes.detailsHeader}>
				<Typography variant="h5" sx={classes.detailsTitle}>
					Market Details
				</Typography>
				<Box sx={classes.detailsActions}>
					<TransactionButton
						variant="outlined"
						color="error"
						startIcon={<DeleteIcon />}
						txnStatus={txnStatus}
						defaultText="Delete"
						loadingText="Deleting..."
						onClick={deleteMarket}
						sx={{ mx: 1 }}
					/>
					<IconButton onClick={onRefresh} color="primary">
						<RefreshIcon />
					</IconButton>
				</Box>
			</Box>

			<Grid container spacing={3} sx={classes.detailsGrid}>
				<Grid item xs={12} md={6}>
					<Box sx={classes.detailsSection}>
						<Typography variant="h6" sx={classes.detailsSectionTitle}>
							Basic Information
						</Typography>

						<DetailRow label="Contract Address" value={market.contract_address} />
						<DetailRow label="Currency Token ID" value={market.currency_token_id} />
						<DetailRow label="Currency Token Contract Address" value={market.currency_token_contract_address} />
						<DetailRow label="Liquidity Provider" value={market.liquidity_provider} />
						<DetailRow label="Buy Rate" value={`${market.buy_rate_numerator} / ${market.buy_rate_denominator}`} />
						<DetailRow label="Sell Rate" value={`${market.sell_rate_numerator} / ${market.sell_rate_denominator}`} />
					</Box>

					<Box sx={classes.detailsSection}>
						<Typography variant="h6" sx={classes.detailsSectionTitle}>
							Trading Information
						</Typography>

						<DetailRow label="Total Sell Token Amount" value={market.total_sell_token_amount} />
						<DetailRow
							label="Total Sell Currency Amount"
							value={`${market.total_sell_currency_amount} (${toDisplayAmount(
								market.total_sell_currency_amount,
								marketCurrencyMetadata?.decimals || 6,
								2,
							)} ${marketCurrencyMetadata?.symbol || ""})`}
						/>

						{marketCurrencyMetadata && (
							<>
								<DetailRow label="Currency Symbol" value={marketCurrencyMetadata.symbol} />
								<DetailRow label="Currency Decimals" value={marketCurrencyMetadata.decimals} />
							</>
						)}

						<DetailRow label="Create Time" value={market.create_time} />
						<DetailRow label="Update Time" value={market.update_time} />
					</Box>
				</Grid>

				<Grid item xs={12} md={6}>
					<Box sx={classes.detailsSection} id="market-checks-section">
						<Typography variant="h6" sx={classes.detailsSectionTitle}>
							Contract Status
						</Typography>

						<Grid container spacing={2}>
							<Grid item xs={12} md={12} lg={6}>
								{agent ? (
									<Alert severity="success" sx={classes.detailsAlert}>
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
								) : (
									<Alert severity="error" sx={classes.detailsAlert}>
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
								{!holder ? (
									<Alert severity="warning" sx={classes.detailsAlert}>
										<Typography>Liquidity Provider is Not holding any tokens.</Typography>
										<Typography>Anyone will not be able to buy.</Typography>
									</Alert>
								) : (
									<Alert severity="info" sx={classes.detailsAlert}>
										<Typography>
											Balance of Liquidity Provider is{" "}
											<Typography component="span" fontWeight="bold">
												{holder.un_frozen_balance}
											</Typography>
										</Typography>
									</Alert>
								)}
							</Grid>

							<Grid item xs={12} md={12} lg={6}>
								{isCurrencyOperator ? (
									<Alert severity="success" sx={classes.detailsAlert}>
										<Typography>Market contract is an operator for the currency token</Typography>
									</Alert>
								) : (
									<Alert severity="warning" sx={classes.detailsAlert}>
										<Typography>Market contract is NOT an operator for the currency token</Typography>
										<Typography>Trading may not work properly</Typography>
									</Alert>
								)}
							</Grid>
						</Grid>
					</Box>
				</Grid>
			</Grid>
		</Paper>
	);
}
