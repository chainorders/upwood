import React from "react";
import {
	Dialog,
	DialogTitle,
	DialogContent,
	DialogActions,
	Button,
	Typography,
	Paper,
	Divider,
	alpha,
} from "@mui/material";
import TransactionButton from "../../components/TransactionButton";
import { TxnStatus, updateContract } from "../../lib/concordium";
import { User } from "../../lib/user";
import { toParamsAddress } from "../../lib/conversions";
import useCommonStyles from "../../theme/useCommonStyles";
import securitySftSingle from "../../contractClients/generated/securitySftSingle";
import securitySftMulti from "../../contractClients/generated/securitySftMulti";

interface RemoveAgentPopupProps {
	user: User;
	contractAddress: string;
	open: boolean;
	onClose: () => void;
	agentAddress: string;
	method: typeof securitySftSingle.removeAgent | typeof securitySftMulti.removeAgent;
}

export default function RemoveAgentPopup({
	open,
	onClose,
	agentAddress,
	user,
	contractAddress,
	method,
}: RemoveAgentPopupProps) {
	const classes = useCommonStyles();
	const [txnStatus, setTxnStatus] = React.useState<TxnStatus>("none");
	const [error, setError] = React.useState<string>();
	const handleSubmit = async () => {
		setError(undefined);
		try {
			await updateContract(
				user.concordiumAccountAddress,
				contractAddress,
				method,
				toParamsAddress(agentAddress),
				setTxnStatus,
			);
			setTxnStatus("success");
			handleClose();
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
		onClose();
	};

	return (
		<Dialog open={open} onClose={handleClose} fullWidth maxWidth="sm">
			<DialogTitle sx={{ pb: 1 }}>
				<Typography variant="h6" component="div" fontWeight={600}>
					Remove Agent
				</Typography>
			</DialogTitle>
			<Divider />
			<DialogContent sx={classes.dialogFormContainer}>
				<Paper
					elevation={0}
					sx={{
						p: 3,
						mb: 2,
						backgroundColor: (theme) => alpha(theme.palette.warning.main, 0.05),
						borderRadius: 1,
						border: (theme) => `1px solid ${alpha(theme.palette.warning.main, 0.2)}`,
					}}
				>
					<Typography variant="subtitle2" color="warning.main" fontWeight={600} gutterBottom>
						Warning: This action cannot be undone
					</Typography>
					<Typography variant="body2" color="text.secondary" sx={{ mb: 2 }}>
						You are about to remove an agent from this contract. This will revoke all of their permissions.
					</Typography>

					<Paper
						elevation={0}
						sx={{
							p: 2,
							backgroundColor: (theme) => theme.palette.grey[50],
							borderRadius: 1,
						}}
					>
						<Typography variant="body2" color="text.secondary">
							Agent Address:
						</Typography>
						<Typography variant="body1" fontWeight={500}>
							{agentAddress}
						</Typography>
					</Paper>
					{error && (
						<Paper
							elevation={0}
							sx={{
								p: 2,
								ml: 1,
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
				</Paper>
			</DialogContent>
			<DialogActions sx={classes.dialogFormActions}>
				<Button onClick={handleClose} variant="outlined" size="medium">
					Cancel
				</Button>
				<TransactionButton
					onClick={handleSubmit}
					variant="contained"
					color="error"
					txnStatus={txnStatus}
					defaultText="Remove Agent"
					loadingText="Removing Agent..."
					size="medium"
				/>
			</DialogActions>
		</Dialog>
	);
}
