import { Typography } from "@mui/material";
import DeleteIcon from "@mui/icons-material/Delete";
import { ForestProjectTokenContract, Token } from "../../../apiClient";
import TransactionButton from "../../../components/TransactionButton";
import { TxnStatus, updateContract } from "../../../lib/concordium";
import { useState } from "react";
import { User } from "../../../lib/user";
import { useOutletContext } from "react-router";
import securitySftMulti from "../../../contractClients/generated/securitySftMulti";
import { toTokenId } from "../../../lib/conversions";

interface TokenDetailsProps {
	token: Token;
	tokenContract?: ForestProjectTokenContract;
}

export default function TokenDetails({ token, tokenContract }: TokenDetailsProps) {
	const { user } = useOutletContext<{ user: User }>();
	const [txnStatus, setTxnStatus] = useState<TxnStatus>("none");

	const deleteToken = async () => {
		try {
			await updateContract(
				user.concordiumAccountAddress,
				token.cis2_address,
				securitySftMulti.removeToken,
				toTokenId(BigInt(token.token_id), 8),
				setTxnStatus,
			);
			setTxnStatus("success");
			alert("Deletion successfully");
			window.location.reload();
		} catch (e) {
			console.error(e);
			setTxnStatus("error");
			alert("Failed to delete");
		}
	};
	return (
		<>
			<div style={{ display: "flex", justifyContent: "space-between", alignItems: "center" }}>
				<Typography variant="h6">Token Details ({tokenContract?.contract_type || "No Type"})</Typography>
				<TransactionButton
					variant="outlined"
					color="error"
					startIcon={<DeleteIcon />}
					txnStatus={txnStatus}
					defaultText="Delete"
					loadingText="Deleting..."
					onClick={deleteToken}
				/>
			</div>
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
		</>
	);
}
