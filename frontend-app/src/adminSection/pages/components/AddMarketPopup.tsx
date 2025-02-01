import { useOutletContext } from "react-router";
import { TxnStatus, updateContract } from "../../../lib/concordium";
import { User } from "../../../lib/user";
import { useEffect, useState } from "react";
import {
	ForestProjectService,
	ForestProjectTokenContract,
	SystemContractsConfigApiModel,
	TokenMetadata,
	UserService,
} from "../../../apiClient";
import securityP2PTrading from "../../../contractClients/generated/securityP2PTrading";
import { toTokenId } from "../../../lib/conversions";
import { useForm, Controller, } from "react-hook-form";
import { TextField, Typography } from "@mui/material";
import AddIcon from "@mui/icons-material/Add";
import TransactionButton from "../../../components/TransactionButton";

interface Props {
	contract_address: string;
	token_id: string;
	onDone: (err?: string) => void;
}

interface AddMarketFormData {
	buy_rate_numerator: number;
	buy_rate_denominator: number;
	sell_rate_numerator: number;
	sell_rate_denominator: number;
	liquidity_provider: string;
}

export default function AddMarketPopup({ contract_address, token_id, onDone }: Props) {
	const user = useOutletContext<{ user: User }>().user;
	const [contracts, setContracts] = useState<SystemContractsConfigApiModel>();
	const [projectTokenContract, setProjectTokenContract] = useState<ForestProjectTokenContract>();
	const [currencyMetadata, setCurrencyMetadata] = useState<TokenMetadata>();
	const [txnStatus, setTxnStatus] = useState<TxnStatus>("none");
	const { control, handleSubmit, watch } = useForm<AddMarketFormData>();

	useEffect(() => {
		UserService.getSystemConfig().then((data) => {
			setContracts(data);
		});
	}, [contract_address, token_id]);
	useEffect(() => {
		if (contracts && contracts.trading_contract) {
			ForestProjectService.getAdminTokenMetadata(
				contracts.trading_contract.currency_token_contract_address,
				contracts.trading_contract.currency_token_id,
			).then((data) => {
				setCurrencyMetadata(data);
			});
		}
	}, [contracts]);
	useEffect(() => {
		ForestProjectService.getAdminForestProjectsContract(contract_address).then((data) => {
			setProjectTokenContract(data);
		});
	}, [contract_address]);

	const onSubmitAddMarket = async (data: AddMarketFormData) => {
		try {
			await updateContract(
				user.concordiumAccountAddress,
				contracts!.trading_contract_index,
				securityP2PTrading.addMarket,
				{
					token: {
						contract: {
							index: Number(contract_address),
							subindex: 0,
						},
						id: toTokenId(BigInt(token_id), 8),
					},
					market: {
						buy_rate: {
							numerator: BigInt(data.buy_rate_numerator),
							denominator: BigInt(data.buy_rate_denominator),
						},
						sell_rate: {
							numerator: BigInt(data.sell_rate_numerator),
							denominator: BigInt(data.sell_rate_denominator),
						},
						liquidity_provider: data.liquidity_provider,
					},
				},
				setTxnStatus,
            );
            setTxnStatus("success");
            onDone();
		} catch (error) {
			console.error(error);
            setTxnStatus("error");
            onDone("Failed to add market");
		}
	};

	const buyRateNumerator = watch("buy_rate_numerator");
	const buyRateDenominator = watch("buy_rate_denominator");
	const sellRateNumerator = watch("sell_rate_numerator");
	const sellRateDenominator = watch("sell_rate_denominator");

	return (
		<form onSubmit={handleSubmit(onSubmitAddMarket)}>
			<div style={{ display: "flex", flexWrap: "wrap", gap: "16px" }}>
				<Controller
					name="liquidity_provider"
					control={control}
					defaultValue={user.concordiumAccountAddress}
					render={({ field }) => <TextField {...field} label="Liquidity Provider" fullWidth margin="normal" />}
				/>
				<Typography variant="body2" style={{ width: "100%" }} mt={2}>
					This is the buy rate for the contract. The user will sell at this rate.
				</Typography>
				<div style={{ display: "flex", alignItems: "center", gap: "8px", width: "100%" }}>
					<Controller
						name="buy_rate_numerator"
						control={control}
						defaultValue={0}
						render={({ field }) => (
							<TextField {...field} label="Buy Rate Numerator" type="number" margin="normal" fullWidth />
						)}
					/>
					<span>/</span>
					<Controller
						name="buy_rate_denominator"
						control={control}
						defaultValue={1}
						render={({ field }) => (
							<TextField {...field} label="Buy Rate Denominator" type="number" margin="normal" fullWidth />
						)}
					/>
				</div>
				<Typography variant="body2" style={{ width: "100%" }} align="right">
					Please enter the buy and sell rates for the market.
				</Typography>
				<Typography variant="body2" style={{ width: "100%" }} mt={2}>
					This is the sell rate for the contract. The user will buy at this rate.
				</Typography>
				<div style={{ display: "flex", alignItems: "center", gap: "8px", width: "100%" }}>
					<Controller
						name="sell_rate_numerator"
						control={control}
						defaultValue={0}
						render={({ field }) => (
							<TextField {...field} label="Sell Rate Numerator" type="number" margin="normal" fullWidth />
						)}
					/>
					<span>/</span>
					<Controller
						name="sell_rate_denominator"
						control={control}
						defaultValue={1}
						render={({ field }) => (
							<TextField {...field} label="Sell Rate Denominator" type="number" margin="normal" fullWidth />
						)}
					/>
				</div>
			</div>
			<TransactionButton
				type="submit"
				variant="contained"
				color="primary"
				txnStatus={txnStatus}
				defaultText="Add Market"
				loadingText="Adding Market.."
				fullWidth
				startIcon={<AddIcon />}
			/>
		</form>
	);
}
