import { useState } from "react";
import { useForm } from "react-hook-form";
import { Box, Grid, Typography, Paper } from "@mui/material";
import {
	ForestProjectTokenContract,
	SecurityMintFundContract,
	SystemContractsConfigApiModel,
} from "../../../apiClient";
import { TxnStatus, updateContract } from "../../../lib/concordium";
import { toDisplayAmount, toTokenId } from "../../../lib/conversions";
import TransactionButton from "../../../components/TransactionButton";
import { User } from "../../../lib/user";
import securityMintFund from "../../../contractClients/generated/securityMintFund";
import useCommonStyles from "../../../theme/useCommonStyles";
import IntegerInput from "./IntegerInput";

interface ProjectTokenAddFundPopupProps {
	user: User;
	contracts: SystemContractsConfigApiModel;
	fundContract: SecurityMintFundContract;
	tokenContract: ForestProjectTokenContract;
	preSaleTokenContract: ForestProjectTokenContract;
	tokenId: string;
	forestProjectId: string;
	onDone: (err?: string) => void;
}

interface FundFormData {
	price: number;
}

export default function AddFundPopup({
	user,
	contracts,
	fundContract,
	tokenContract,
	preSaleTokenContract,
	tokenId,
	onDone,
}: ProjectTokenAddFundPopupProps) {
	const styles = useCommonStyles();
	const [txnStatus, setTxnStatus] = useState<TxnStatus>("none");
	const { control, handleSubmit, watch } = useForm<FundFormData>({
		mode: "onChange", // Enable validation on change
		defaultValues: {
			price: 1 * 10 ** (contracts.euro_e_metadata.decimals || 6),
		},
	});

	const priceWatch = watch("price");

	const onSubmit = async (data: FundFormData) => {
		try {
			setTxnStatus("sending");
			await updateContract(
				user.concordiumAccountAddress,
				fundContract.contract_address,
				securityMintFund.addFund,
				{
					security_token: {
						id: toTokenId(BigInt(tokenId), 8),
						contract: {
							index: Number(tokenContract.contract_address),
							subindex: 0,
						},
					},
					rate: {
						numerator: BigInt(data.price),
						denominator: BigInt(1),
					},
					token: {
						id: toTokenId(BigInt(tokenId), 8),
						contract: {
							index: Number(preSaleTokenContract.contract_address),
							subindex: 0,
						},
					},
				},
				setTxnStatus,
			);
			onDone();
		} catch (e) {
			setTxnStatus("error");
			console.error(e);
		}
	};

	return (
		<Box sx={styles.dialogFormContainer}>
			<form onSubmit={handleSubmit(onSubmit)}>
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
							{contracts.euro_e_metadata.symbol || "Euro"} Price
						</Typography>
						<Grid container spacing={2}>
							<Grid item xs={12}>
								<IntegerInput
									name="price"
									control={control}
									label="Price"
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
										priceWatch?.toString() || "0",
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
						variant="contained"
						type="submit"
						txnStatus={txnStatus}
						defaultText="Add Fund"
						loadingText="Adding Fund..."
						fullWidth
						sx={styles.formSubmitButton}
					/>
				</Box>
			</form>
		</Box>
	);
}
