import { useState } from "react";
import { Paper, Typography, Table, TableBody, TableCell, TableContainer, TableHead, TableRow } from "@mui/material";
import { YieldApiModel, ForestProjectTokenContract } from "../../../apiClient";
import DeleteIcon from "@mui/icons-material/Delete";
import { toDisplayAmount, toTokenId } from "../../../lib/conversions";
import { TxnStatus, updateContract } from "../../../lib/concordium";
import securitySftMultiYielder from "../../../contractClients/generated/securitySftMultiYielder";
import TransactionButton from "../../../components/TransactionButton";
import { User } from "../../../lib/user";

interface YieldsProps {
	user: User;
	tokenContract: ForestProjectTokenContract;
	tokenId: string;
	yielderContract: string;
	yields: YieldApiModel[];
	onRemoveYield: () => void;
}

export default function Yields({ user, yields, yielderContract, tokenContract, tokenId, onRemoveYield }: YieldsProps) {
	const [txnStatus, setTxnStatus] = useState<TxnStatus>("none");

	const removeYield = async () => {
		try {
			await updateContract(
				user.concordiumAccountAddress,
				yielderContract,
				securitySftMultiYielder.removeYield,
				{
					token_id: toTokenId(BigInt(tokenId), 8),
					token_contract: {
						index: Number(tokenContract.contract_address),
						subindex: 0,
					},
				},
				setTxnStatus,
			);
			onRemoveYield();
		} catch (e) {
			setTxnStatus("error");
			console.error(e);
			alert("Failed to delete yield");
		}
	};

	return (
		<Paper sx={{ padding: 2, marginTop: 2 }}>
			<div style={{ display: "flex", justifyContent: "space-between", alignItems: "center" }}>
				<Typography variant="h6" gutterBottom>
					Yields
				</Typography>
				<TransactionButton
					variant="outlined"
					color="secondary"
					type="button"
					startIcon={<DeleteIcon />}
					onClick={async () => {
						console.log("delete");
						removeYield();
					}}
					txnStatus={txnStatus}
					defaultText="Delete"
					loadingText="Deleting..."
					disabled={!yields || yields.length === 0}
				/>
			</div>
			{yields.length > 0 ? (
				<TableContainer component={Paper}>
					<Table>
						<TableHead>
							<TableRow>
								<TableCell>Yielded Token</TableCell>
								<TableCell>Symbol</TableCell>
								<TableCell>Decimals</TableCell>
								<TableCell>Rate</TableCell>
								<TableCell>Rate</TableCell>
								<TableCell>Yield Type</TableCell>
							</TableRow>
						</TableHead>
						<TableBody>
							{yields.map((yieldItem) => (
								<TableRow key={yieldItem.yield_.yield_token_id + yieldItem.yield_.yield_contract_address}>
									<TableCell>
										{yieldItem.yield_.yield_token_id}-{yieldItem.yield_.yield_contract_address}
									</TableCell>
									<TableCell>{yieldItem.yield_token_metadata?.symbol}</TableCell>
									<TableCell>{yieldItem.yield_token_metadata?.decimals}</TableCell>
									<TableCell>
										{yieldItem.yield_.yield_rate_numerator} / {yieldItem.yield_.yield_rate_denominator}
									</TableCell>
									<TableCell>
										{toDisplayAmount(yieldItem.yield_.yield_rate_numerator, yieldItem.yield_token_metadata?.decimals || 0)} Yeild
										tokens for {toDisplayAmount(yieldItem.yield_.yield_rate_denominator, tokenContract?.decimals || 0)} security
										tokens
									</TableCell>
									<TableCell>{yieldItem.yield_.yield_type}</TableCell>
								</TableRow>
							))}
						</TableBody>
					</Table>
				</TableContainer>
			) : (
				<Typography>No yields available</Typography>
			)}
		</Paper>
	);
}
