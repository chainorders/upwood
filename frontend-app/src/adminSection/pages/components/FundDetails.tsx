import { Typography } from "@mui/material";
import DeleteIcon from "@mui/icons-material/Delete";
import { useEffect, useState } from "react";
import { SecurityMintFund, TokenMetadata, ForestProjectService, ForestProjectTokenContract } from "../../../apiClient";
import TransactionButton from "../../../components/TransactionButton";
import { TxnStatus, updateContract } from "../../../lib/concordium";
import { User } from "../../../lib/user";
import { useOutletContext } from "react-router";
import securityMintFund from "../../../contractClients/generated/securityMintFund";
import { toTokenId } from "../../../lib/conversions";

interface FundDetailsProps {
	fund: SecurityMintFund;
	investmentTokenContract?: ForestProjectTokenContract;
	tokenContract?: ForestProjectTokenContract;
	currencyMetadata?: TokenMetadata;
}

export default function FundDetails({ fund }: FundDetailsProps) {
	const [fundTokenMetadata, setFundTokenMetadata] = useState<ForestProjectTokenContract>();
	const [fundCurrencyMetadata, setFundCurrencyMetadata] = useState<TokenMetadata>();
	const [deleteTxnStatus, setDeleteTxnStatus] = useState<TxnStatus>("none");
	const { user } = useOutletContext<{ user: User }>();

	useEffect(() => {
		ForestProjectService.getAdminForestProjectsContract(fund.token_contract_address)
			.then((metadata) => {
				setFundTokenMetadata(metadata);
			})
			.catch(() => {
				setFundTokenMetadata(undefined);
			});
		ForestProjectService.getAdminTokenMetadata(fund.currency_token_contract_address, fund.currency_token_id)
			.then((metadata) => {
				setFundCurrencyMetadata(metadata);
			})
			.catch(() => {
				setFundCurrencyMetadata(undefined);
			});
	}, [fund]);

	const deleteFund = async () => {
		try {
			await updateContract(
				user.concordiumAccountAddress,
				fund.contract_address,
				securityMintFund.removeFund,
				{
					id: toTokenId(BigInt(fund.investment_token_id), 8),
					contract: {
						index: Number(fund.investment_token_contract_address),
						subindex: 0,
					},
				},
				setDeleteTxnStatus,
			);
			setDeleteTxnStatus("success");
			alert("Deletion successfully");
			window.location.reload();
		} catch (e) {
			console.error(e);
			setDeleteTxnStatus("error");
			alert(`Failed to delete: ${JSON.stringify(e)}`);
		}
	};

	return (
		<>
			<div style={{ display: "flex", justifyContent: "space-between", alignItems: "center" }}>
				<Typography variant="h6">Fund Details</Typography>
				<TransactionButton
					variant="outlined"
					color="error"
					startIcon={<DeleteIcon />}
					txnStatus={deleteTxnStatus}
					defaultText="Delete"
					loadingText="Deleting..."
					onClick={deleteFund}
				/>
			</div>
			<Typography>
				<strong>Contract Address:</strong> {fund.contract_address}
			</Typography>
			<Typography>
				<strong>Investment Token ID:</strong> {fund.investment_token_id}
			</Typography>
			<Typography>
				<strong>Investment Token Contract Address:</strong> {fund.investment_token_contract_address}
			</Typography>
			<Typography>
				<strong>Token ID:</strong> {fund.token_id}
			</Typography>
			<Typography>
				<strong>Token Contract Address:</strong> {fund.token_contract_address}
			</Typography>
			{fundTokenMetadata && (
				<>
					<Typography>
						<strong>Token Symbol:</strong> {fundTokenMetadata.symbol}
					</Typography>
					<Typography>
						<strong>Token Decimals:</strong> {fundTokenMetadata.decimals}
					</Typography>
				</>
			)}
			<Typography>
				<strong>Token Amount:</strong> {fund.token_amount}
			</Typography>
			<Typography>
				<strong>Currency Token ID:</strong> {fund.currency_token_id}
			</Typography>
			<Typography>
				<strong>Currency Token Contract Address:</strong> {fund.currency_token_contract_address}
			</Typography>
			<Typography>
				<strong>Currency Amount:</strong> {fund.currency_amount}
			</Typography>
			{fundCurrencyMetadata && (
				<>
					<Typography>
						<strong>Currency Symbol:</strong> {fundCurrencyMetadata.symbol}
					</Typography>
					<Typography>
						<strong>Currency Decimals:</strong> {fundCurrencyMetadata.decimals}
					</Typography>
				</>
			)}
			<Typography>
				<strong>Token Amount:</strong> {fund.token_amount}
			</Typography>
			<Typography>
				<strong>Receiver Address:</strong> {fund.receiver_address || "N/A"}
			</Typography>
			<Typography>
				<strong>Rate:</strong> {fund.rate_numerator} / {fund.rate_denominator}
			</Typography>
			<Typography>
				<strong>Fund State:</strong> {fund.fund_state}
			</Typography>
			<Typography>
				<strong>Create Time:</strong> {fund.create_time}
			</Typography>
			<Typography>
				<strong>Update Time:</strong> {fund.update_time}
			</Typography>
		</>
	);
}
