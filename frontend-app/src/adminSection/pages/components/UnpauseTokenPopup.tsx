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
	alpha,
} from "@mui/material";
import PlayArrowIcon from "@mui/icons-material/PlayArrow";
import { User } from "../../../lib/user";
import { TxnStatus, updateContract } from "../../../lib/concordium";
import TransactionButton from "../../../components/TransactionButton";
import securitySftMulti from "../../../contractClients/generated/securitySftMulti";
import { toTokenId } from "../../../lib/conversions";
import useCommonStyles from "../../../theme/useCommonStyles";
import securitySftSingle from "../../../contractClients/generated/securitySftSingle";

interface UnpauseTokenPopupProps {
	open: boolean;
	onClose: () => void;
	onSuccess?: () => void;
	tokenId: string;
	contractAddress: string;
	user: User;
	method: typeof securitySftMulti.unPause | typeof securitySftSingle.unPause;
	tokenIdSize: number;
}

const UnpauseTokenPopup: React.FC<UnpauseTokenPopupProps> = ({
	open,
	onClose,
	onSuccess,
	tokenId,
	contractAddress,
	user,
	method,
	tokenIdSize,
}) => {
	const classes = useCommonStyles();
	const [txnStatus, setTxnStatus] = React.useState<TxnStatus>("none");
	const [error, setError] = React.useState<string>();

	const handleUnpauseToken = async () => {
		setError(undefined);
		try {
			await updateContract(
				user.concordiumAccountAddress,
				contractAddress,
				method,
				{
					tokens: [{ token_id: toTokenId(BigInt(tokenId), tokenIdSize) }],
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
		onClose();
	};

	return (
		<Dialog open={open} onClose={handleClose} fullWidth maxWidth="sm">
			<DialogTitle sx={{ pb: 1 }}>
				<Typography variant="h6" component="div" fontWeight={600}>
					Unpause Token
				</Typography>
			</DialogTitle>
			<Divider />
			<DialogContent sx={classes.dialogFormContainer}>
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
						<PlayArrowIcon color="success" sx={{ mr: 1 }} />
						<Typography variant="subtitle2" color="success.main" fontWeight={600}>
							You are about to activate a token
						</Typography>
					</Box>

					<Typography variant="body2" color="text.secondary" sx={{ mb: 2 }}>
						Unpausing this token will enable transfers and minting operations.
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
							Contract Address:
						</Typography>
						<Typography variant="body1" fontWeight={500} sx={{ mb: 1 }}>
							{contractAddress}
						</Typography>

						<Typography variant="body2" color="text.secondary">
							Token ID:
						</Typography>
						<Typography variant="body1" fontWeight={500} sx={{ mb: 1 }}>
							{tokenId} ({toTokenId(BigInt(tokenId), 8)})
						</Typography>
					</Paper>
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
			</DialogContent>
			<DialogActions sx={classes.dialogFormActions}>
				<Button onClick={handleClose} variant="outlined" size="medium">
					Cancel
				</Button>
				<TransactionButton
					onClick={handleUnpauseToken}
					variant="contained"
					color="success"
					startIcon={<PlayArrowIcon />}
					txnStatus={txnStatus}
					defaultText="Unpause Token"
					loadingText="Unpausing Token..."
					size="medium"
				/>
			</DialogActions>
		</Dialog>
	);
};

export default UnpauseTokenPopup;
