import { TxnStatus, updateContract } from "../../../lib/concordium";
import { User } from "../../../lib/user";
import { useState } from "react";
import { ForestProjectTokenContract, SystemContractsConfigApiModel } from "../../../apiClient";
import securityP2PTrading from "../../../contractClients/generated/securityP2PTrading";
import { toDisplayAmount, toTokenId } from "../../../lib/conversions";
import { useForm, Controller } from "react-hook-form";
import {
	TextField,
	Typography,
	Box,
	Grid,
	Paper,
	RadioGroup,
	FormControlLabel,
	Radio,
	FormControl,
	DialogTitle,
	DialogContent,
	Dialog,
} from "@mui/material";
import TransactionButton from "../../../components/TransactionButton";
import useCommonStyles from "../../../theme/useCommonStyles";
import CurrencyInput from "./CurrencyInput";
import { format, toDate } from "date-fns";

interface Props {
	contracts: SystemContractsConfigApiModel;
	token_id?: string;
	tokenContract: ForestProjectTokenContract;
	user: User;
	onDone: (err?: string) => void;
	open: boolean;
	onClose: () => void;
}

interface AddMarketFormData {
	buy_rate_numerator?: number;
	sell_rate_numerator?: number;
	liquidity_provider: string;
	marketType: string;
	token_id_start?: string;
	mint_rate_numerator?: number;
	token_id_diff_millis?: number;
	token_id?: string;
}

export default function AddMarketPopup({ token_id, onDone, contracts, user, tokenContract, open, onClose }: Props) {
	const styles = useCommonStyles();
	const [txnStatus, setTxnStatus] = useState<TxnStatus>("none");
	const { control, handleSubmit, watch } = useForm<AddMarketFormData>({
		mode: "onChange", // Enable validation on change
		defaultValues: {
			liquidity_provider: user.concordiumAccountAddress,
			buy_rate_numerator: 1 * 10 ** 6,
			sell_rate_numerator: 1 * 10 ** 6,
			token_id_start: format(toDate(tokenContract.created_at), "yyyy-MM-dd"), // Convert to UTC date string
			token_id_diff_millis: 24 * 60 * 60 * 1000,
			marketType: token_id ? "transfer" : "mint",
			mint_rate_numerator: 1 * 10 ** 6,
			token_id,
		},
	});

	const onSubmitAddMarket = async (data: AddMarketFormData) => {
		console.log("Form data:", data);
		try {
			await updateContract(
				user.concordiumAccountAddress,
				contracts!.trading_contract_index,
				securityP2PTrading.addMarket,
				{
					token_contract: {
						index: Number(tokenContract.contract_address),
						subindex: 0,
					},
					market:
						data.marketType === "mint"
							? {
									Mint: [
										{
											liquidity_provider: data.liquidity_provider,
											rate: {
												numerator: BigInt(data.mint_rate_numerator!),
												denominator: BigInt(1),
											},
											token_metadata_url: {
												url: tokenContract.metadata_url,
												hash: tokenContract.metadata_hash ? { Some: [tokenContract.metadata_hash] } : { None: {} },
											},
											token_id: {
												start: format(toDate(data.token_id_start!), "yyyy-MM-dd'T'HH:mm:ss.SSS'Z'"),
												diff_millis: BigInt(data.token_id_diff_millis!),
											},
										},
									],
								}
							: {
									Transfer: [
										{
											buy_rate: {
												numerator: BigInt(data.buy_rate_numerator!),
												denominator: BigInt(1),
											},
											sell_rate: {
												numerator: BigInt(data.sell_rate_numerator!),
												denominator: BigInt(1),
											},
											liquidity_provider: data.liquidity_provider,
											token_id: toTokenId(BigInt(data.token_id!), 8),
										},
									],
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

	const buyPrice = watch("buy_rate_numerator");
	const sellPrice = watch("sell_rate_numerator");
	const mintPrice = watch("mint_rate_numerator");
	const marketType = watch("marketType");

	return (
		<Dialog open={open} onClose={onClose}>
			<DialogTitle>Add Market</DialogTitle>
			<DialogContent>
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
									Market Type
								</Typography>
								<Grid container spacing={2}>
									<Grid item xs={12}>
										<Controller
											name="marketType"
											control={control}
											render={({ field }) => (
												<FormControl component="fieldset">
													<RadioGroup row {...field}>
														<FormControlLabel value="transfer" control={<Radio />} label="Transfer Market" />
														<FormControlLabel value="mint" control={<Radio />} label="Mint Market" />
													</RadioGroup>
												</FormControl>
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

							{marketType === "transfer" ? (
								<>
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
											Token ID
										</Typography>
										<Grid container spacing={2}>
											<Grid item xs={12}>
												<Typography variant="body2" color="textSecondary" sx={{ mb: 1 }}>
													The specific token ID this market applies to.
												</Typography>
												<Controller
													name="token_id"
													control={control}
													rules={{
														required: marketType === "transfer" ? "Token ID is required" : false,
													}}
													render={({ field, fieldState }) => (
														<TextField
															{...field}
															label="Token ID"
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
											Buy Price €
										</Typography>
										<Grid container spacing={2}>
											<Grid item xs={12}>
												<Typography variant="body2" color="textSecondary" sx={{ mb: 1 }}>
													This is the buy price for the contract. The user will sell at this price.
												</Typography>
												<CurrencyInput
													name="buy_rate_numerator"
													control={control}
													label="Buy Price"
													textFieldProps={{
														fullWidth: true,
														autoComplete: "off",
														required: marketType === "transfer",
													}}
												/>
												<Typography variant="caption" color="textSecondary" sx={{ display: "block", mt: 1, textAlign: "right" }}>
													Price per token unit: € {toDisplayAmount(buyPrice?.toString() || "0", 6, 6)}
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
											Sell Price €
										</Typography>
										<Grid container spacing={2}>
											<Grid item xs={12}>
												<Typography variant="body2" color="textSecondary" sx={{ mb: 1 }}>
													This is the sell price for the contract. The user will buy at this price.
												</Typography>
												<CurrencyInput
													name="sell_rate_numerator"
													control={control}
													label="Sell Price"
													textFieldProps={{
														fullWidth: true,
														autoComplete: "off",
														required: marketType === "transfer",
													}}
												/>
												<Typography variant="caption" color="textSecondary" sx={{ display: "block", mt: 1, textAlign: "right" }}>
													Price per token unit: € {toDisplayAmount(sellPrice?.toString() || "0", 6, 6)}
												</Typography>
											</Grid>
										</Grid>
									</Paper>
								</>
							) : (
								<>
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
											Mint Rate €
										</Typography>
										<Grid container spacing={2}>
											<Grid item xs={12}>
												<Typography variant="body2" color="textSecondary" sx={{ mb: 1 }}>
													This is the mint rate for the contract.
												</Typography>
												<CurrencyInput
													name="mint_rate_numerator"
													control={control}
													label="Mint Rate"
													textFieldProps={{
														fullWidth: true,
														autoComplete: "off",
														required: marketType === "mint",
													}}
												/>
												<Typography variant="caption" color="textSecondary" sx={{ display: "block", mt: 1, textAlign: "right" }}>
													Price per token unit: € {toDisplayAmount(mintPrice?.toString() || "0", 6, 6)}
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
											Token ID Configuration
										</Typography>
										<Grid container spacing={2}>
											<Grid item xs={12}>
												<Controller
													name="token_id_start"
													control={control}
													rules={{
														required: marketType === "mint" ? "Start date is required" : false,
													}}
													render={({ field, fieldState }) => (
														<TextField
															{...field}
															type="date"
															label="Token ID Start Date"
															fullWidth
															variant="outlined"
															size="small"
															InputLabelProps={{ shrink: true }}
															error={!!fieldState.error}
															helperText={
																fieldState.error?.message ||
																(field.value && format(toDate(field.value), "yyyy-MM-dd'T'HH:mm:ss.SSS'Z'"))
															}
															sx={{ mb: 2 }}
														/>
													)}
												/>
												<Controller
													name="token_id_diff_millis"
													control={control}
													rules={{
														required: marketType === "mint" ? "Time difference is required" : false,
													}}
													render={({ field, fieldState }) => (
														<TextField
															{...field}
															label="Token ID Time Difference (milliseconds)"
															fullWidth
															type="number"
															variant="outlined"
															size="small"
															error={!!fieldState.error}
															helperText={fieldState.error?.message || "Time difference in milliseconds (e.g., 86400000 for 1 day)"}
														/>
													)}
												/>
											</Grid>
										</Grid>
									</Paper>
								</>
							)}
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
			</DialogContent>
		</Dialog>
	);
}
