import React from "react";
import {
	Dialog,
	DialogTitle,
	DialogContent,
	DialogActions,
	Button,
	Typography,
	Box,
	Paper,
	Divider,
	TextField,
	alpha,
} from "@mui/material";
import LocalFireDepartmentIcon from "@mui/icons-material/LocalFireDepartment";
import { User } from "../../lib/user";
import { TxnStatus, updateContract } from "../../lib/concordium";
import TransactionButton from "../../components/TransactionButton";
import securitySftMulti from "../../contractClients/generated/securitySftMulti";
import useCommonStyles from "../../theme/useCommonStyles";
import securitySftSingle from "../../contractClients/generated/securitySftSingle";
import { useForm, Controller } from "react-hook-form";
import { TokenHolderUser } from "../../apiClient";
import { toParamsAddress, toTokenId } from "../../lib/conversions";

interface UnfreezeHolderBalancePopupProps {
	open: boolean;
	onClose: () => void;
	onSuccess?: () => void;
	holder: TokenHolderUser;
	user: User;
	method: typeof securitySftMulti.unFreeze | typeof securitySftSingle.unFreeze;
	tokenIdSize: number;
}

interface UnfreezeFormData {
	amount: string;
}

const UnfreezeHolderBalancePopup: React.FC<UnfreezeHolderBalancePopupProps> = ({
	open,
	onClose,
	onSuccess,
	holder,
	user,
	method,
	tokenIdSize,
}) => {
	const classes = useCommonStyles();
	const [txnStatus, setTxnStatus] = React.useState<TxnStatus>("none");
	const [error, setError] = React.useState<string>();

	const { control, handleSubmit, reset } = useForm<UnfreezeFormData>({
		defaultValues: {
			amount: holder.frozen_balance,
		},
		mode: "onChange",
	});

	const handleUnfreezeBalance = async (data: UnfreezeFormData) => {
		setError(undefined);
		try {
			await updateContract(
				user.concordiumAccountAddress,
				holder.cis2_address,
				method,
				{
					owner: toParamsAddress(holder.holder_address),
					tokens: [
						{
							token_id: toTokenId(BigInt(holder.token_id), tokenIdSize),
							token_amount: data.amount,
						},
					],
				},
				setTxnStatus,
			);
			setTxnStatus("success");
			if (onSuccess) onSuccess();
			setTimeout(() => {
				handleClose();
			}, 1000);
		} catch (error) {
			if (error instanceof Error) {
				setError(error.message);
			} else if (typeof error === "string") {
				setError(error);
			} else {
				setError("An unknown error occurred.");
			}
			setTxnStatus("error");
		}
	};

	const handleClose = () => {
		setError(undefined);
		setTxnStatus("none");
		reset();
		onClose();
	};

	return (
		<Dialog open={open} onClose={handleClose} fullWidth maxWidth="sm">
			<DialogTitle sx={{ pb: 1 }}>
				<Typography variant="h6" component="div" fontWeight={600}>
					Unfreeze Holder Balance
				</Typography>
			</DialogTitle>
			<Divider />
			<DialogContent sx={classes.dialogFormContainer}>
				<form onSubmit={handleSubmit(handleUnfreezeBalance)}>
					<Paper
						elevation={0}
						sx={{
							p: 3,
							mb: 2,
							backgroundColor: (theme) => alpha(theme.palette.success.main, 0.05),
							borderRadius: 1,
							border: (theme) => `1px solid ${alpha(theme.palette.success.main, 0.2)}`,
						}}
					>
						<Box display="flex" alignItems="center" mb={2}>
							<LocalFireDepartmentIcon color="success" sx={{ mr: 1 }} />
							<Typography variant="subtitle2" color="success.main" fontWeight={600}>
								You are about to unfreeze a holder's balance
							</Typography>
						</Box>

						<Typography variant="body2" color="text.secondary" sx={{ mb: 2 }}>
							Unfreezing this holder's balance will allow them to transfer the specified amount of tokens.
						</Typography>

						<Paper
							elevation={0}
							sx={{
								p: 2,
								mb: 2,
								backgroundColor: (theme) => theme.palette.grey[50],
								borderRadius: 1,
							}}
						>
							<Typography variant="body2" color="text.secondary">
								Contract Address:
							</Typography>
							<Typography variant="body1" fontWeight={500} sx={{ mb: 1 }}>
								{holder.cis2_address}
							</Typography>

							<Typography variant="body2" color="text.secondary">
								Holder Address:
							</Typography>
							<Typography variant="body1" fontWeight={500} sx={{ mb: 1 }}>
								{holder.holder_address}
							</Typography>

							<Typography variant="body2" color="text.secondary">
								Token ID:
							</Typography>
							<Typography variant="body1" fontWeight={500} sx={{ mb: 1 }}>
								{holder.token_id} ({tokenIdSize ? toTokenId(BigInt(holder.token_id), tokenIdSize) : "Unit Token"})
							</Typography>
						</Paper>

						<Box sx={classes.dialogFormField}>
							<Controller
								name="amount"
								control={control}
								rules={{
									required: "Amount is required",
									pattern: {
										value: /^[0-9]+$/,
										message: "Please enter a valid amount",
									},
								}}
								render={({ field, fieldState }) => (
									<TextField
										{...field}
										label="Amount to Unfreeze"
										fullWidth
										size="small"
										error={!!fieldState.error}
										helperText={fieldState.error?.message}
									/>
								)}
							/>
						</Box>
					</Paper>

					{error && (
						<Paper
							elevation={0}
							sx={{
								p: 2,
								mb: 2,
								backgroundColor: (theme) => alpha(theme.palette.error.main, 0.1),
								borderLeft: (theme) => `4px solid ${theme.palette.error.main}`,
								borderRadius: 1,
							}}
						>
							<Typography variant="body2" color="error">
								{error}
							</Typography>
						</Paper>
					)}

					<DialogActions sx={classes.dialogFormActions}>
						<Button onClick={handleClose} variant="outlined" size="medium">
							Cancel
						</Button>
						<TransactionButton
							type="submit"
							variant="contained"
							color="success"
							startIcon={<LocalFireDepartmentIcon />}
							txnStatus={txnStatus}
							defaultText="Unfreeze Balance"
							loadingText="Unfreezing Balance..."
							size="medium"
						/>
					</DialogActions>
				</form>
			</DialogContent>
		</Dialog>
	);
};

export default UnfreezeHolderBalancePopup;
