import { TxnStatus, updateContract } from "../../../lib/concordium";
import { User } from "../../../lib/user";
import { useState } from "react";
import { SystemContractsConfigApiModel } from "../../../apiClient";
import securityP2PTrading from "../../../contractClients/generated/securityP2PTrading";
import { toDisplayAmount, toTokenId } from "../../../lib/conversions";
import { useForm, Controller } from "react-hook-form";
import { TextField, Typography, Box, Grid, Paper } from "@mui/material";
import TransactionButton from "../../../components/TransactionButton";
import useCommonStyles from "../../../theme/useCommonStyles";
import IntegerInput from "./IntegerInput";

interface Props {
	contracts: SystemContractsConfigApiModel;
	contract_address: string;
	token_id: string;
	user: User;
	onDone: (err?: string) => void;
}

interface AddMarketFormData {
	buy_price: number;
	sell_price: number;
	liquidity_provider: string;
}

export default function AddMarketPopup({ contract_address, token_id, onDone, contracts, user }: Props) {
	const styles = useCommonStyles();
	const [txnStatus, setTxnStatus] = useState<TxnStatus>("none");
	const { control, handleSubmit, watch } = useForm<AddMarketFormData>({
		mode: "onChange", // Enable validation on change
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
		<Box sx={styles.dialogFormContainer}>
			<form onSubmit={handleSubmit(onSubmitAddMarket)}>
				<Box sx={styles.dialogFormSection}>
					<Paper
						elevation={0}
						sx={{
							...styles.dialogFormField,
							p: 2,
							mb: 2,
							backgroundColor: "rgba(0,0,0,0.02)",
						}}
					>
						<Typography variant="h6" mb={1} color="primary">
							Liquidity Provider
						</Typography>
						<Grid container spacing={2}>
							<Grid item xs={12}>
								<Controller
									name="liquidity_provider"
									control={control}
									rules={{
										required: "Liquidity provider is required",
									}}
									render={({ field, fieldState }) => (
										<TextField
											{...field}
											label="Liquidity Provider"
											fullWidth
											variant="outlined"
											size="small"
											error={!!fieldState.error}
											helperText={fieldState.error?.message}
										/>
									)}
								/>
							</Grid>
						</Grid>
					</Paper>

					<Paper
						elevation={0}
						sx={{
							...styles.dialogFormField,
							p: 2,
							mb: 2,
							backgroundColor: "rgba(0,0,0,0.02)",
						}}
					>
						<Typography variant="h6" mb={1} color="primary">
							Buy Price ({contracts.euro_e_metadata.symbol})
						</Typography>
						<Grid container spacing={2}>
							<Grid item xs={12}>
								<Typography variant="body2" color="textSecondary" sx={{ mb: 1 }}>
									This is the buy price for the contract. The user will sell at this price.
								</Typography>
								<IntegerInput
									name="buy_price"
									control={control}
									label="Buy Price"
									min={0}
									textFieldProps={{
										fullWidth: true,
										autoComplete: "off",
										required: true,
									}}
								/>
								<Typography variant="caption" color="textSecondary" sx={{ display: "block", mt: 1, textAlign: "right" }}>
									Price per token unit: {contracts.euro_e_metadata.symbol}{" "}
									{toDisplayAmount(
										buyPrice?.toString() || "0",
										contracts.euro_e_metadata.decimals || 6,
										contracts.euro_e_metadata.decimals || 6,
									)}
								</Typography>
							</Grid>
						</Grid>
					</Paper>

					<Paper
						elevation={0}
						sx={{
							...styles.dialogFormField,
							p: 2,
							mb: 2,
							backgroundColor: "rgba(0,0,0,0.02)",
						}}
					>
						<Typography variant="h6" mb={1} color="primary">
							Sell Price ({contracts.euro_e_metadata.symbol})
						</Typography>
						<Grid container spacing={2}>
							<Grid item xs={12}>
								<Typography variant="body2" color="textSecondary" sx={{ mb: 1 }}>
									This is the sell price for the contract. The user will buy at this price.
								</Typography>
								<IntegerInput
									name="sell_price"
									control={control}
									label="Sell Price"
									min={0}
									textFieldProps={{
										fullWidth: true,
										autoComplete: "off",
										required: true,
									}}
								/>
								<Typography variant="caption" color="textSecondary" sx={{ display: "block", mt: 1, textAlign: "right" }}>
									Price per token unit: {contracts.euro_e_metadata.symbol}{" "}
									{toDisplayAmount(
										sellPrice?.toString() || "0",
										contracts.euro_e_metadata.decimals || 6,
										contracts.euro_e_metadata.decimals || 6,
									)}
								</Typography>
							</Grid>
						</Grid>
					</Paper>
				</Box>

				<Box sx={styles.dialogFormActions}>
					<TransactionButton
						type="submit"
						variant="contained"
						color="primary"
						txnStatus={txnStatus}
						defaultText="Add Market"
						loadingText="Adding Market..."
						fullWidth
						sx={styles.formSubmitButton}
					/>
				</Box>
			</form>
		</Box>
	);
}
