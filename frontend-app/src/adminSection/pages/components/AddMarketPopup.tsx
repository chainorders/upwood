import { useOutletContext } from "react-router";
import { TxnStatus, updateContract } from "../../../lib/concordium";
import { User } from "../../../lib/user";
import { useState } from "react";
import { SystemContractsConfigApiModel } from "../../../apiClient";
import securityP2PTrading from "../../../contractClients/generated/securityP2PTrading";
import { toDisplayAmount, toTokenId } from "../../../lib/conversions";
import { useForm, Controller } from "react-hook-form";
import { TextField, Typography } from "@mui/material";
import AddIcon from "@mui/icons-material/Add";
import TransactionButton from "../../../components/TransactionButton";

interface Props {
	contracts: SystemContractsConfigApiModel;
	contract_address: string;
	token_id: string;
	onDone: (err?: string) => void;
}

interface AddMarketFormData {
	buy_price: number;
	sell_price: number;
	liquidity_provider: string;
}

export default function AddMarketPopup({ contract_address, token_id, onDone, contracts }: Props) {
	const user = useOutletContext<{ user: User }>().user;
	const [txnStatus, setTxnStatus] = useState<TxnStatus>("none");
	const { control, handleSubmit, watch } = useForm<AddMarketFormData>({
		defaultValues: {
			liquidity_provider: user.concordiumAccountAddress,
			buy_price: 1 * 10 ** (contracts.euro_e_metadata.decimals || 6),
			sell_price: 1 * 10 ** (contracts.euro_e_metadata.decimals || 6),
		},
	});

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
							numerator: BigInt(data.buy_price),
							denominator: BigInt(1),
						},
						sell_rate: {
							numerator: BigInt(data.sell_price),
							denominator: BigInt(1),
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

	const buyPrice = watch("buy_price");
	const sellPrice = watch("sell_price");

	return (
		<form onSubmit={handleSubmit(onSubmitAddMarket)}>
			<div style={{ display: "flex", flexWrap: "wrap", gap: "16px" }}>
				<Controller
					name="liquidity_provider"
					control={control}
					render={({ field }) => <TextField {...field} label="Liquidity Provider" fullWidth margin="normal" />}
				/>
				<Typography variant="body2" style={{ width: "100%" }} mt={2}>
					This is the buy price for the contract. The user will sell at this price.
				</Typography>
				<Controller
					name="buy_price"
					control={control}
					render={({ field }) => (
						<TextField
							{...field}
							label="Buy Price"
							type="number"
							margin="normal"
							fullWidth
							helperText={`Price per token unit ${contracts.euro_e_metadata.symbol} ${toDisplayAmount(
								field.value.toString(),
								contracts.euro_e_metadata.decimals || 6,
								contracts.euro_e_metadata.decimals || 6,
							)}`}
						/>
					)}
				/>
				<Typography variant="body2" style={{ width: "100%" }} mt={2}>
					This is the sell price for the contract. The user will buy at this price.
				</Typography>
				<Controller
					name="sell_price"
					control={control}
					render={({ field }) => (
						<TextField
							{...field}
							label="Sell Price"
							type="number"
							margin="normal"
							fullWidth
							helperText={`Price per token unit ${contracts.euro_e_metadata.symbol} ${toDisplayAmount(
								field.value.toString(),
								contracts.euro_e_metadata.decimals || 6,
								contracts.euro_e_metadata.decimals || 6,
							)}`}
						/>
					)}
				/>
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
