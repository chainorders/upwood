import { Alert, Box, Grid, IconButton, Paper, Typography } from "@mui/material";
import DeleteIcon from "@mui/icons-material/Delete";
import RefreshIcon from "@mui/icons-material/Refresh";
import PauseIcon from "@mui/icons-material/Pause";
import PlayArrowIcon from "@mui/icons-material/PlayArrow";
import { ForestProjectTokenContract, Token } from "../../../apiClient";
import TransactionButton from "../../../components/TransactionButton";
import { TxnStatus, updateContract } from "../../../lib/concordium";
import { useState } from "react";
import { User } from "../../../lib/user";
import securitySftMulti from "../../../contractClients/generated/securitySftMulti";
import { toTokenId } from "../../../lib/conversions";
import useCommonStyles from "../../../theme/useCommonStyles";
import DetailRow from "./DetailRow";

interface TokenDetailsProps {
	user: User;
	token: Token;
	tokenContract?: ForestProjectTokenContract;
	onDeleteToken: () => void;
	onRefresh?: () => void;
}

export default function TokenDetails({ user, token, tokenContract, onDeleteToken, onRefresh }: TokenDetailsProps) {
	const classes = useCommonStyles();
	const [deleteTxnStatus, setDeleteTxnStatus] = useState<TxnStatus>("none");
	const [unpauseTxnStatus, setUnpauseTxnStatus] = useState<TxnStatus>("none");
	const [pauseTxnStatus, setPauseTxnStatus] = useState<TxnStatus>("none");

	const unpauseToken = async () => {
		try {
			await updateContract(
				user.concordiumAccountAddress,
				token.cis2_address,
				securitySftMulti.unPause,
				{
					tokens: [{ token_id: toTokenId(BigInt(token.token_id), 8) }],
				},
				setUnpauseTxnStatus,
			);
			setUnpauseTxnStatus("success");
			alert("Unpause successfully");
			token.is_paused = false;
			if (onRefresh) onRefresh();
		} catch (e) {
			console.error(e);
			setUnpauseTxnStatus("error");
			alert("Failed to unpause");
		}
	};

	const pauseToken = async () => {
		try {
			await updateContract(
				user.concordiumAccountAddress,
				token.cis2_address,
				securitySftMulti.pause,
				{
					tokens: [{ token_id: toTokenId(BigInt(token.token_id), 8) }],
				},
				setPauseTxnStatus,
			);
			setPauseTxnStatus("success");
			alert("Pause successfully");
			token.is_paused = true;
			if (onRefresh) onRefresh();
		} catch (e) {
			console.error(e);
			setPauseTxnStatus("error");
			alert("Failed to pause");
		}
	};

	const deleteToken = async () => {
		try {
			await updateContract(
				user.concordiumAccountAddress,
				token.cis2_address,
				securitySftMulti.removeToken,
				toTokenId(BigInt(token.token_id), 8),
				setDeleteTxnStatus,
			);
			setDeleteTxnStatus("success");
			alert("Deletion successfully");
			onDeleteToken();
		} catch (e) {
			console.error(e);
			setDeleteTxnStatus("error");
			alert("Failed to delete");
		}
	};

	return (
		<Paper sx={classes.detailsContainer}>
			<Box sx={classes.detailsHeader}>
				<Typography variant="h5" sx={classes.detailsTitle}>
					Token Details {tokenContract?.contract_type && `(${tokenContract.contract_type})`}
				</Typography>
				<Box sx={classes.detailsActions}>
					{!token.is_paused ? (
						<TransactionButton
							variant="outlined"
							color="warning"
							startIcon={<PauseIcon />}
							txnStatus={pauseTxnStatus}
							defaultText="Pause"
							loadingText="Pausing..."
							onClick={pauseToken}
							sx={{ mx: 1 }}
						/>
					) : (
						<TransactionButton
							variant="outlined"
							color="success"
							startIcon={<PlayArrowIcon />}
							txnStatus={unpauseTxnStatus}
							defaultText="Unpause"
							loadingText="Unpausing..."
							onClick={unpauseToken}
							sx={{ mx: 1 }}
						/>
					)}
					<TransactionButton
						variant="outlined"
						color="error"
						startIcon={<DeleteIcon />}
						txnStatus={deleteTxnStatus}
						defaultText="Delete"
						loadingText="Deleting..."
						onClick={deleteToken}
						sx={{ mx: 1 }}
					/>
					{onRefresh && (
						<IconButton onClick={onRefresh} color="primary">
							<RefreshIcon />
						</IconButton>
					)}
				</Box>
			</Box>

			<Grid container spacing={3} sx={classes.detailsGrid}>
				<Grid item xs={12} md={6}>
					<Box sx={classes.detailsSection}>
						<Typography variant="h6" sx={classes.detailsSectionTitle}>
							Basic Information
						</Typography>

						<DetailRow label="Token Contract Address" value={token.cis2_address} />
						<DetailRow label="Token Contract Type" value={tokenContract?.contract_type || "N/A"} />
						<DetailRow label="Symbol" value={tokenContract?.symbol || "N/A"} />
						<DetailRow label="Decimals" value={tokenContract?.decimals !== undefined ? tokenContract.decimals : "N/A"} />
						<DetailRow label="Token ID" value={`${token.token_id} (${toTokenId(BigInt(token.token_id), 8)})`} />
					</Box>
				</Grid>

				<Grid item xs={12} md={6}>
					<Box sx={classes.detailsSection}>
						<Typography variant="h6" sx={classes.detailsSectionTitle}>
							Additional Details
						</Typography>

						<DetailRow label="Metadata URL" value={token.metadata_url} />
						<DetailRow label="Is Paused" value={token.is_paused ? "Yes" : "No"} />
						<DetailRow label="Supply" value={token.supply} />
					</Box>

					<Box sx={classes.detailsSection}>
						<Typography variant="h6" sx={classes.detailsSectionTitle}>
							Token Status
						</Typography>

						{token.is_paused ? (
							<Alert severity="warning" sx={classes.detailsAlert}>
								<Typography>This token is paused. You cannot transfer or mint tokens until it is unpaused.</Typography>
								<TransactionButton
									variant="outlined"
									color="primary"
									defaultText="Unpause Token"
									loadingText="Unpausing..."
									onClick={unpauseToken}
									txnStatus={unpauseTxnStatus}
									sx={{ mt: 2 }}
								/>
							</Alert>
						) : (
							<Alert severity="success" sx={classes.detailsAlert}>
								<Typography>This token is active. You can transfer and mint tokens.</Typography>
								<TransactionButton
									variant="outlined"
									color="primary"
									defaultText="Pause Token"
									loadingText="Pausing..."
									onClick={pauseToken}
									txnStatus={pauseTxnStatus}
									sx={{ mt: 2 }}
								/>
							</Alert>
						)}
					</Box>
				</Grid>
			</Grid>
		</Paper>
	);
}
