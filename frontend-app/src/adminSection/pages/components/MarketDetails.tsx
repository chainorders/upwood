import { useEffect, useState } from "react";
import { Alert, Box, Grid, IconButton, Paper, Typography } from "@mui/material";
import { Market, ForestProjectTokenContract, Agent, IndexerService, TokenHolder } from "../../../apiClient";
import DeleteIcon from "@mui/icons-material/Delete";
import RefreshIcon from "@mui/icons-material/Refresh";
import TransactionButton from "../../../components/TransactionButton";
import { TxnStatus, updateContract } from "../../../lib/concordium";
import securityP2PTrading from "../../../contractClients/generated/securityP2PTrading";
import { User } from "../../../lib/user";
import { toDisplayAmount } from "../../../lib/conversions";
import euroeStablecoin from "../../../contractClients/generated/euroeStablecoin";
import concordiumNodeClient from "../../../contractClients/ConcordiumNodeClient";
import { AccountAddress, CcdAmount, ContractAddress } from "@concordium/web-sdk";
import securitySftMulti from "../../../contractClients/generated/securitySftMulti";
import useCommonStyles from "../../../theme/useCommonStyles";
import DetailRow from "./DetailRow";
import { formatISO, millisecondsToHours, toDate } from "date-fns";

interface MarketDetailsProps {
	user: User;
	market: Market;
	tokenMetadata?: ForestProjectTokenContract;
	onRefresh: () => void;
}

export default function MarketDetails({ market, user, onRefresh }: MarketDetailsProps) {
	const classes = useCommonStyles();
	const [txnStatus, setTxnStatus] = useState<TxnStatus>("none");
	const [isCurrencyOperator, setIsCurrencyOperator] = useState(false);
	const [refreshCounter, setRefreshCounter] = useState(0);
	const [agent, setAgent] = useState<Agent>();
	const [removeAgentTxnStatus, setRemoveAgentTxnStatus] = useState<TxnStatus>("none");
	const [addAgentTxnStatus, setAddAgentTxnStatus] = useState<TxnStatus>("none");
	const [lpTokenHolder, setLpTokenHolder] = useState<TokenHolder>();
	const [lpEuroEBalance, setLpEuroEBalance] = useState<bigint>(BigInt(0));

	useEffect(() => {
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
		euroeStablecoin.balanceOf
			.invoke(
				concordiumNodeClient,
				ContractAddress.create(Number(market.currency_token_contract_address)),
				[
					{
						token_id: "",
						address: { Account: [market.liquidity_provider] },
					},
				],
				AccountAddress.fromBase58(market.liquidity_provider),
				CcdAmount.fromCcd(0),
			)
			.then((res) => euroeStablecoin.balanceOf.parseReturnValue(res.returnValue!)!)
			.then((res) => {
				setLpEuroEBalance(BigInt(res[0]));
			})
			.catch((err) => {
				console.error("Error fetching balanceOf", err);
			});

		if (market.token_id) {
			IndexerService.getAdminIndexerCis2TokenHolder(
				market.token_contract_address,
				market.token_id,
				market.liquidity_provider,
			).then(setLpTokenHolder);
		}
	}, [market, refreshCounter]);

	const deleteMarket = async () => {
		try {
			await updateContract(
				user.concordiumAccountAddress,
				market.contract_address,
				securityP2PTrading.removeMarket,
				{ index: Number(market.token_contract_address), subindex: 0 },
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
					roles: [{ Operator: {} }, { Mint: {} }, { AddToken: {} }],
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
						<DetailRow label="Market Type" value={market.market_type} />
						<DetailRow label="Currency Token ID" value={market.currency_token_id} />
						<DetailRow label="Currency Token Contract Address" value={market.currency_token_contract_address} />
						<DetailRow label="Liquidity Provider" value={market.liquidity_provider} />
						<DetailRow
							title={market.token_contract_address}
							label="Token Contract Address"
							value={market.token_contract_address ? market.token_contract_address : "N/A"}
						/>
						<DetailRow title={market.token_id} label="Token ID" value={market.token_id ? market.token_id : "N/A"} />
						<DetailRow
							title={market.token_id_calculation_start}
							label="Token ID Calculation Start"
							value={market.token_id_calculation_start ? formatISO(toDate(Number(market.token_id_calculation_start))) : "N/A"}
						/>
						<DetailRow
							title={market.token_id_calculation_diff_millis}
							label="Token ID Calculation Diff"
							value={
								market.token_id_calculation_diff_millis
									? `${market.token_id_calculation_diff_millis} Millis / ${millisecondsToHours(Number(market.token_id_calculation_diff_millis))} hours`
									: "N/A"
							}
						/>
						<DetailRow
							title={`${market.buy_rate_numerator} / ${market.buy_rate_denominator}`}
							label="Buy Rate"
							value={market.buy_rate_numerator ? toDisplayAmount(market.buy_rate_numerator, 6, 6) : "N/A"}
						/>
						<DetailRow
							title={`${market.sell_rate_numerator} / ${market.sell_rate_denominator}`}
							label="Sell Rate"
							value={market.sell_rate_numerator ? toDisplayAmount(market.sell_rate_numerator, 6, 6) : "N/A"}
						/>
					</Box>

					<Box sx={classes.detailsSection}>
						<Typography variant="h6" sx={classes.detailsSectionTitle}>
							Trading Information
						</Typography>

						<DetailRow label="Max Token Amount" value={market.max_token_amount} />
						<DetailRow
							label="Max Currency Amount"
							value={
								market.max_currency_amount
									? `${market.max_currency_amount} (${toDisplayAmount(market.max_currency_amount, 6, 2)})`
									: "N/A"
							}
						/>

						<Typography variant="subtitle2" sx={{ mt: 2, mb: 1 }}>
							Trading Statistics
						</Typography>

						<DetailRow
							label="Tokens Bought by Market"
							value={market.token_in_amount}
							title="Total amount of tokens which users have sold to the market"
						/>
						<DetailRow
							label="Currency Paid Out"
							value={`${market.currency_out_amount} (${toDisplayAmount(market.currency_out_amount, 6, 2)})`}
							title="Total amount of currency units which the market has given out when users sold tokens"
						/>
						<DetailRow
							label="Tokens Sold by Market"
							value={market.token_out_amount}
							title="Total amount of tokens which users have bought from the market"
						/>
						<DetailRow
							label="Currency Received"
							value={`${market.currency_in_amount} (${toDisplayAmount(market.currency_in_amount, 6, 2)})`}
							title="Total amount of currency units which the market has received when users bought tokens"
						/>

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
								{!lpTokenHolder || !lpTokenHolder?.un_frozen_balance || lpTokenHolder?.un_frozen_balance === "0" ? (
									<Alert severity="warning" sx={classes.detailsAlert}>
										<Typography>Liquidity Provider is Not holding any tokens.</Typography>
										<Typography>Anyone will not be able to buy.</Typography>
									</Alert>
								) : (
									<Alert severity="success" sx={classes.detailsAlert}>
										<Typography>Token Balance of Liquidity Provider: {lpTokenHolder?.un_frozen_balance}</Typography>
										<Typography>Anyone will be able to buy.</Typography>
									</Alert>
								)}
							</Grid>
							<Grid item xs={12} md={12} lg={6}>
								{lpEuroEBalance > BigInt(0) ? (
									<Alert severity="success" sx={classes.detailsAlert}>
										<Typography>
											Liquidity Provider has {toDisplayAmount(lpEuroEBalance.toString(), 6, 2)} EuroE balance
										</Typography>
									</Alert>
								) : (
									<Alert severity="warning" sx={classes.detailsAlert}>
										<Typography>Liquidity Provider has no EuroE balance</Typography>
										<Typography>No one will be able to sell tokens to the market</Typography>
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
										<Typography>Market contract is NOT an operator of Liquidity Provider for the currency token</Typography>
										<Typography>Users will not be able to sell tokens</Typography>
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
