import { Alert, Grid, Typography } from "@mui/material";
import DeleteIcon from "@mui/icons-material/Delete";
import { ForestProjectTokenContract, Token } from "../../../apiClient";
import TransactionButton from "../../../components/TransactionButton";
import { TxnStatus, updateContract } from "../../../lib/concordium";
import { useState } from "react";
import { User } from "../../../lib/user";
import securitySftMulti from "../../../contractClients/generated/securitySftMulti";
import { toTokenId } from "../../../lib/conversions";

interface TokenDetailsProps {
	user: User;
	token: Token;
	tokenContract?: ForestProjectTokenContract;
	onDeleteToken: () => void;
}

export default function TokenDetails({ user, token, tokenContract, onDeleteToken }: TokenDetailsProps) {
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
		<Grid container>
			<Grid item xs={12} md={6}>
				<Typography variant="h6">Token Details ({tokenContract?.contract_type || "No Type"})</Typography>
			</Grid>
			<Grid item xs={12} md={6} style={{ display: "flex", justifyContent: "flex-end" }}>
				<TransactionButton
					variant="outlined"
					color="error"
					startIcon={<DeleteIcon />}
					txnStatus={deleteTxnStatus}
					defaultText="Delete"
					loadingText="Deleting..."
					onClick={deleteToken}
				/>
			</Grid>
			<Grid item xs={12} md={6}>
				<Typography>
					<strong>Token Contract Address:</strong> {token.cis2_address}
				</Typography>
				<Typography>
					<strong>Token Contract Type:</strong> {tokenContract?.contract_type}
				</Typography>
				<Typography>
					<strong>Symbol:</strong> {tokenContract?.symbol || "N/A"}
				</Typography>
				<Typography>
					<strong>Decimals:</strong> {tokenContract?.decimals !== undefined ? tokenContract.decimals : "N/A"}
				</Typography>
				<Typography>
					<strong>Token ID:</strong> {token.token_id} ({toTokenId(BigInt(token.token_id), 8)})
				</Typography>
				<Typography>
					<strong>Metadata URL:</strong> {token.metadata_url}
				</Typography>
				<Typography>
					<strong>Is Paused:</strong> {token.is_paused ? "Yes" : "No"}
				</Typography>
				<Typography>
					<strong>Supply:</strong> {token.supply}
				</Typography>
			</Grid>
			<Grid item xs={12} md={6}>
				<Grid container spacing={2} direction={"row-reverse"}>
					<Grid item xs={12} md={12} lg={6}>
						{token.is_paused && (
							<Alert severity="warning" style={{ marginTop: "10px" }}>
								<Typography>This token is paused. You cannot transfer or mint tokens until it is unpaused.</Typography>
								<TransactionButton
									variant="outlined"
									color="primary"
									defaultText="Unpause Token"
									loadingText="Unpausing..."
									onClick={unpauseToken}
									txnStatus={unpauseTxnStatus}
									style={{ marginTop: "10px" }}
								/>
							</Alert>
						)}
						{!token.is_paused && (
							<Alert severity="success" style={{ marginTop: "10px" }}>
								<Typography>This token is active. You can transfer and mint tokens.</Typography>
								<TransactionButton
									variant="outlined"
									color="primary"
									defaultText="Pause Token"
									loadingText="Pausing..."
									onClick={pauseToken}
									txnStatus={pauseTxnStatus}
									style={{ marginTop: "10px" }}
								/>
							</Alert>
						)}
					</Grid>
				</Grid>
			</Grid>
		</Grid>
	);
}
