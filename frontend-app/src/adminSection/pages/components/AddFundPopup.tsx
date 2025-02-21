import { useState } from "react";
import { useForm, Controller } from "react-hook-form";
import { Box, Typography, Grid, TextField } from "@mui/material";
import { useOutletContext } from "react-router";
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

interface ProjectTokenAddFundPopupProps {
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
	contracts,
	fundContract,
	tokenContract,
	preSaleTokenContract,
	tokenId,
	onDone,
}: ProjectTokenAddFundPopupProps) {
	const user = useOutletContext<{ user: User }>().user;
	const [txnStatus, setTxnStatus] = useState<TxnStatus>("none");
	const { control, handleSubmit } = useForm<FundFormData>({
		defaultValues: {
			price: 1 * 10 ** (contracts.euro_e_metadata.decimals || 6),
		},
	});

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
		<div>
			<form onSubmit={handleSubmit(onSubmit)}>
				<Box mb={2} pt={2}>
					<Grid container spacing={2} alignItems="center">
						<Grid item xs={12}>
							<Controller
								name="price"
								control={control}
								defaultValue={0}
								rules={{ required: true }}
								render={({ field }) => (
									<TextField
										{...field}
										label="Price"
										type="number"
										fullWidth
										autoComplete="off"
										required
										InputProps={{ inputProps: { min: 0 } }}
										helperText={`Price per token unit ${contracts.euro_e_metadata.symbol} ${toDisplayAmount(
											field.value.toString(),
											contracts.euro_e_metadata.decimals || 6,
											contracts.euro_e_metadata.decimals || 6,
										)}`}
									/>
								)}
							/>
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
