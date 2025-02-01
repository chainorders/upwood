import { useEffect, useState } from "react";
import {
	ForestProjectService,
	ForestProjectTokenContract,
	SystemContractsConfigApiModel,
	TokenMetadata,
	User,
	UserService,
} from "../../../apiClient";
import { TxnStatus, updateContract } from "../../../lib/concordium";
import securitySftMultiYielder, {
	UpsertYieldRequest,
} from "../../../contractClients/generated/securitySftMultiYielder";
import { toDisplayAmount, toTokenId } from "../../../lib/conversions";
import { useOutletContext } from "react-router";
import { useForm, Controller } from "react-hook-form";
import { TextField, Checkbox, FormControlLabel, Box, Typography, Grid } from "@mui/material";
import TransactionButton from "../../../components/TransactionButton";

interface ProjectTokenAddYieldPopupProps {
	contract_address: string;
	token_id: string;
	onDone: (err?: string) => void;
}

interface YieldFormData {
	carbonCredit: {
		numerator: number;
		denominator: number;
		added: boolean;
	};
	euro: {
		numerator: number;
		denominator: number;
		added: boolean;
	};
	eTrees: {
		numerator: number;
		denominator: number;
		added: boolean;
	};
	euroIntrest: {
		numerator: number;
		denominator: number;
		added: boolean;
	};
}

export default function AddYieldPopup({ contract_address, token_id, onDone }: ProjectTokenAddYieldPopupProps) {
	const user = useOutletContext<{ user: User }>().user;
	const [contracts, setContracts] = useState<SystemContractsConfigApiModel>();
	const [projectTokenContract, setProjectTokenContract] = useState<ForestProjectTokenContract>();
	const [euroeMetdata, setEuroMetadata] = useState<TokenMetadata>();
	const [carbonCreditsMetadata, setCarbonCreditsMetadata] = useState<TokenMetadata>();
	const [eTreesMetadata, setETreesMetadata] = useState<TokenMetadata>();
	const [txnStatus, setTxnStatus] = useState<TxnStatus>("none");
	const { control, handleSubmit, watch } = useForm<YieldFormData>();

	useEffect(() => {
		UserService.getSystemConfig()
			.then((data) => {
				setContracts(data);
			})
			.catch(() => {
				alert("Failed to fetch system contracts config");
			});
		ForestProjectService.getAdminForestProjectsContract(contract_address).then((data) => {
			setProjectTokenContract(data);
		});
	}, [contract_address]);
	useEffect(() => {
		if (contracts) {
			ForestProjectService.getAdminTokenMetadata(contracts.euro_e_contract_index, contracts.euro_e_token_id).then(
				(data) => {
					setEuroMetadata(data);
				},
			);
			ForestProjectService.getAdminTokenMetadata(
				contracts.carbon_credit_contract_index,
				contracts.carbon_credit_token_id,
			).then((data) => {
				setCarbonCreditsMetadata(data);
			});
			ForestProjectService.getAdminTokenMetadata(contracts.tree_ft_contract_index, "0").then((data) => {
				setETreesMetadata(data);
			});
		}
	}, [contracts]);

	const onSubmit = async (data: YieldFormData) => {
		if (!contracts) {
			return;
		}

		const yields = [];
		if (data.carbonCredit.added) {
			yields.push({
				contract: { index: Number(contracts.carbon_credit_contract_index), subindex: 0 },
				token_id: toTokenId(BigInt(contracts.carbon_credit_token_id), 8),
				calculation: {
					Quantity: [{ numerator: BigInt(data.carbonCredit.numerator), denominator: BigInt(data.carbonCredit.denominator) }],
				},
			});
		}
		if (data.euro.added) {
			yields.push({
				contract: { index: Number(contracts.euro_e_contract_index), subindex: 0 },
				token_id: "",
				calculation: {
					Quantity: [{ numerator: BigInt(data.euro.numerator), denominator: BigInt(data.euro.denominator) }],
				},
			});
		}
		if (data.eTrees.added) {
			yields.push({
				contract: { index: Number(contracts.tree_ft_contract_index), subindex: 0 },
				token_id: "",
				calculation: {
					Quantity: [{ numerator: BigInt(data.eTrees.numerator), denominator: BigInt(data.eTrees.denominator) }],
				},
			});
		}
		if (data.euroIntrest.added) {
			yields.push({
				contract: { index: Number(contracts.euro_e_contract_index), subindex: 0 },
				token_id: "",
				calculation: {
					SimpleInterest: [
						{ numerator: BigInt(data.euroIntrest.numerator), denominator: BigInt(data.euroIntrest.denominator) },
					],
				},
			});
		}

		try { 
			await updateContract(
				user.account_address,
				contracts.yielder_contract_index,
				securitySftMultiYielder.upsertYield,
				{
					token_id: toTokenId(BigInt(token_id), 8),
					token_contract: { index: Number(contract_address), subindex: 0 },
					yields,
				} as UpsertYieldRequest,
				setTxnStatus,
			);
			setTxnStatus("success");
			onDone();
		}
		catch (e) {
			console.error(e);
			setTxnStatus("error");
			onDone("Failed to add yield");
		}
	};

	const carbonCreditRateNumeratorWatch = watch("carbonCredit.numerator");
	const carbonCreditRateDenominatorWatch = watch("carbonCredit.denominator");
	const euroRateNumeratorWatch = watch("euro.numerator");
	const euroRateDenominatorWatch = watch("euro.denominator");
	const eTreesRateNumeratorWatch = watch("eTrees.numerator");
	const eTreesRateDenominatorWatch = watch("eTrees.denominator");
	const euroIntrestRateNumeratorWatch = watch("euroIntrest.numerator");
	const euroIntrestRateDenominatorWatch = watch("euroIntrest.denominator");

	return (
		<div>
			<form onSubmit={handleSubmit(onSubmit)}>
				<Box mb={2}>
					<Typography variant="h6" mb={1}>
						{carbonCreditsMetadata?.symbol ? `(${carbonCreditsMetadata?.symbol})` : ""} Carbon Credits
					</Typography>
					<Grid container spacing={2} alignItems="center">
						<Grid item>
							<Controller
								name="carbonCredit.added"
								control={control}
								defaultValue={false}
								render={({ field }) => (
									<FormControlLabel control={<Checkbox {...field} checked={field.value} />} label="Added" />
								)}
							/>
						</Grid>
						<Grid item>
							<Controller
								name="carbonCredit.numerator"
								control={control}
								defaultValue={0}
								rules={{ required: true }}
								render={({ field }) => <TextField {...field} label="Numerator" type="number" />}
							/>
						</Grid>
						<Grid item>
							<Controller
								name="carbonCredit.denominator"
								control={control}
								defaultValue={1}
								render={({ field }) => <TextField {...field} label="Denominator" type="number" />}
							/>
						</Grid>
						<Grid item xs={12}>
							<Typography variant="body2" align="right">
								{toDisplayAmount((carbonCreditRateNumeratorWatch || "0").toString(), carbonCreditsMetadata?.decimals || 0)}{" "}
								{carbonCreditsMetadata?.symbol || "Carbon Credits"} per{" "}
								{toDisplayAmount((carbonCreditRateDenominatorWatch || "1").toString(), projectTokenContract?.decimals || 0)}{" "}
								{projectTokenContract?.symbol || "Project"} Tokens
							</Typography>
						</Grid>
					</Grid>
				</Box>
				<Box mb={2}>
					<Typography variant="h6" mb={1}>
						{euroeMetdata?.symbol ? `(${euroeMetdata?.symbol})` : ""} Euro
					</Typography>
					<Grid container spacing={2} alignItems="center">
						<Grid item>
							<Controller
								name="euro.added"
								control={control}
								defaultValue={false}
								render={({ field }) => (
									<FormControlLabel control={<Checkbox {...field} checked={field.value} />} label="Added" />
								)}
							/>
						</Grid>
						<Grid item>
							<Controller
								name="euro.numerator"
								control={control}
								defaultValue={0}
								render={({ field }) => <TextField {...field} label="Numerator" type="number" />}
							/>
						</Grid>
						<Grid item>
							<Controller
								name="euro.denominator"
								control={control}
								defaultValue={1}
								render={({ field }) => <TextField {...field} label="Denominator" type="number" />}
							/>
						</Grid>
						<Grid item xs={12}>
							<Typography variant="body2" align="right">
								{toDisplayAmount((euroRateNumeratorWatch || "0").toString(), euroeMetdata?.decimals || 0)}{" "}
								{euroeMetdata?.symbol || "Euro"} per{" "}
								{toDisplayAmount((euroRateDenominatorWatch || "1").toString(), projectTokenContract?.decimals || 0)}{" "}
								{projectTokenContract?.symbol || "Project"} Tokens
							</Typography>
						</Grid>
					</Grid>
				</Box>
				<Box mb={2}>
					<Typography variant="h6" mb={1}>
						{eTreesMetadata?.symbol ? `(${eTreesMetadata?.symbol})` : ""} E-Trees
					</Typography>
					<Grid container spacing={2} alignItems="center">
						<Grid item>
							<Controller
								name="eTrees.added"
								control={control}
								defaultValue={false}
								render={({ field }) => (
									<FormControlLabel control={<Checkbox {...field} checked={field.value} />} label="Added" />
								)}
							/>
						</Grid>
						<Grid item>
							<Controller
								name="eTrees.numerator"
								control={control}
								defaultValue={0}
								render={({ field }) => <TextField {...field} label="Numerator" type="number" />}
							/>
						</Grid>
						<Grid item>
							<Controller
								name="eTrees.denominator"
								control={control}
								defaultValue={1}
								render={({ field }) => <TextField {...field} label="Denominator" type="number" />}
							/>
						</Grid>
						<Grid item xs={12}>
							<Typography variant="body2" align="right">
								{toDisplayAmount((eTreesRateNumeratorWatch || "0").toString(), eTreesMetadata?.decimals || 0)}{" "}
								{eTreesMetadata?.symbol || "E-Trees"} per{" "}
								{toDisplayAmount((eTreesRateDenominatorWatch || "1").toString(), projectTokenContract?.decimals || 0)}{" "}
								{projectTokenContract?.symbol || "Project"} Tokens
							</Typography>
						</Grid>
					</Grid>
				</Box>
				<Box mb={2}>
					<Typography variant="h6" mb={1}>
						{euroeMetdata?.symbol ? `(${euroeMetdata?.symbol})` : ""} Euro Interest
					</Typography>
					<Grid container spacing={2} alignItems="center">
						<Grid item>
							<Controller
								name="euroIntrest.added"
								control={control}
								defaultValue={false}
								render={({ field }) => (
									<FormControlLabel control={<Checkbox {...field} checked={field.value} />} label="Added" />
								)}
							/>
						</Grid>
						<Grid item>
							<Controller
								name="euroIntrest.numerator"
								control={control}
								defaultValue={0}
								render={({ field }) => <TextField {...field} label="Numerator" type="number" />}
							/>
						</Grid>
						<Grid item>
							<Controller
								name="euroIntrest.denominator"
								control={control}
								defaultValue={1}
								render={({ field }) => <TextField {...field} label="Denominator" type="number" />}
							/>
						</Grid>
						<Grid item xs={12}>
							<Typography variant="body2" align="right">
								{toDisplayAmount((euroIntrestRateNumeratorWatch || "0").toString(), euroeMetdata?.decimals || 0)}{" "}
								{euroeMetdata?.symbol || "Euro"} per{" "}
								{toDisplayAmount((euroIntrestRateDenominatorWatch || "1").toString(), projectTokenContract?.decimals || 0)}{" "}
								{projectTokenContract?.symbol || "Project"} Tokens per token version
							</Typography>
						</Grid>
					</Grid>
				</Box>
				<TransactionButton
					variant="contained"
					type="submit"
					txnStatus={txnStatus}
					defaultText="Add Yield"
					loadingText="Adding Yield..."
					fullWidth
				/>
			</form>
		</div>
	);
}
