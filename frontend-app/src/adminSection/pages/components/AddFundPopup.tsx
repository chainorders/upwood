import { useState } from "react";
import { useForm, Controller } from "react-hook-form";
import { Box, Typography, Grid, TextField } from "@mui/material";
import { useOutletContext } from "react-router";
import { ForestProjectTokenContract, SecurityMintFundContract } from "../../../apiClient";
import { TxnStatus, updateContract } from "../../../lib/concordium";
import { toDisplayAmount, toTokenId } from "../../../lib/conversions";
import TransactionButton from "../../../components/TransactionButton";
import { User } from "../../../lib/user";
import securityMintFund from "../../../contractClients/generated/securityMintFund";

interface ProjectTokenAddFundPopupProps {
	fundContract: SecurityMintFundContract;
	tokenContract: ForestProjectTokenContract;
	preSaleTokenContract: ForestProjectTokenContract;
	tokenId: string;
	forestProjectId: string;
	onDone: (err?: string) => void;
}

interface FundFormData {
	preSaleTokenId: string;
	preSaleTokenContractAddress: string;
	rateNumerator: number;
	rateDenominator: number;
}

export default function AddFundPopup({
	fundContract,
	tokenContract,
	preSaleTokenContract,
	tokenId,
	onDone,
}: ProjectTokenAddFundPopupProps) {
	const user = useOutletContext<{ user: User }>().user;
	const [txnStatus, setTxnStatus] = useState<TxnStatus>("none");
	const { control, handleSubmit, watch } = useForm<FundFormData>();

	const rateNumeratorWatch = watch("rateNumerator");
	const rateDenominatorWatch = watch("rateDenominator");

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
						numerator: BigInt(data.rateNumerator),
						denominator: BigInt(data.rateDenominator),
					},
					token: {
						id: toTokenId(BigInt(data.preSaleTokenId), 8),
						contract: {
							index: Number(data.preSaleTokenContractAddress),
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
		<div>
			<form onSubmit={handleSubmit(onSubmit)}>
				<Box mb={2}>
					<Grid container spacing={2} alignItems="center">
						<Grid item xs={12}>
							<Controller
								name="preSaleTokenId"
								control={control}
								defaultValue={tokenId}
								rules={{ required: true }}
								render={({ field }) => <TextField {...field} label="Pre-Sale Token ID" fullWidth />}
							/>
						</Grid>
						<Grid item xs={12}>
							<Controller
								name="preSaleTokenContractAddress"
								control={control}
								defaultValue={preSaleTokenContract?.contract_address || ""}
								rules={{ required: true }}
								render={({ field }) => <TextField {...field} label="Pre-Sale Token Contract Address" fullWidth />}
							/>
						</Grid>
						<Grid item xs={12}>
							<Box display="flex" alignItems="center">
								<Controller
									name="rateNumerator"
									control={control}
									defaultValue={0}
									rules={{ required: true }}
									render={({ field }) => <TextField {...field} label="Rate Numerator" type="number" fullWidth />}
								/>
								<Typography variant="h6" mx={2}>
									/
								</Typography>
								<Controller
									name="rateDenominator"
									control={control}
									defaultValue={1}
									rules={{ required: true }}
									render={({ field }) => <TextField {...field} label="Rate Denominator" type="number" fullWidth />}
								/>
							</Box>
						</Grid>
						<Grid item xs={12}>
							<Typography variant="body2" align="right">
								{toDisplayAmount((rateNumeratorWatch || "0").toString(), tokenContract?.decimals || 0)} per{" "}
								{toDisplayAmount((rateDenominatorWatch || "1").toString(), tokenContract?.decimals || 0)}{" "}
								{tokenContract?.symbol || "Project"} Tokens
							</Typography>
						</Grid>
					</Grid>
				</Box>
				<TransactionButton
					variant="contained"
					type="submit"
					txnStatus={txnStatus}
					defaultText="Add Fund"
					loadingText="Adding Fund..."
					fullWidth
				/>
			</form>
		</div>
	);
}
