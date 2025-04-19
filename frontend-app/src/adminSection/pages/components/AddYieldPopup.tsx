import { useEffect, useState } from "react";
import {
	ForestProjectService,
	ForestProjectTokenContract,
	SystemContractsConfigApiModel,
	UserService,
} from "../../../apiClient";
import { TxnStatus, updateContract } from "../../../lib/concordium";
import securitySftMultiYielder, {
	UpsertYieldRequest,
} from "../../../contractClients/generated/securitySftMultiYielder";
import { toDisplayAmount, toTokenId } from "../../../lib/conversions";
import { useForm, Controller } from "react-hook-form";
import { Checkbox, FormControlLabel, Box, Typography, Grid, Paper } from "@mui/material";
import TransactionButton from "../../../components/TransactionButton";
import { User } from "../../../lib/user";
import useCommonStyles from "../../../theme/useCommonStyles";
import IntegerInput from "./IntegerInput";
import CurrencyInput from "./CurrencyInput";

interface ProjectTokenAddYieldPopupProps {
	contract_address: string;
	token_id: string;
	onDone: (err?: string) => void;
	user: User;
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

export default function AddYieldPopup({ contract_address, token_id, onDone, user }: ProjectTokenAddYieldPopupProps) {
	const styles = useCommonStyles();
	const [contracts, setContracts] = useState<SystemContractsConfigApiModel>();
	const [projectTokenContract, setProjectTokenContract] = useState<ForestProjectTokenContract>();
	const [txnStatus, setTxnStatus] = useState<TxnStatus>("none");
	const { control, handleSubmit, watch } = useForm<YieldFormData>({
		mode: "onChange", // This makes validation run on every change
		defaultValues: {
			carbonCredit: { numerator: 1, denominator: 1, added: false },
			euro: { numerator: 1000000, denominator: 1, added: false },
			eTrees: { numerator: 1, denominator: 1, added: false },
			euroIntrest: { numerator: 1000000, denominator: 1, added: false },
		},
	});

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

	const onSubmit = async (data: YieldFormData) => {
		if (!contracts) {
			return;
		}

		const yields = [];
		if (data.carbonCredit.added) {
			yields.push({
				contract: { index: Number(contracts.carbon_credit_contract_index), subindex: 0 },
				token_id: "",
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
				user.concordiumAccountAddress,
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
		} catch (e) {
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
		<Box sx={styles.dialogFormContainer}>
			<form onSubmit={handleSubmit(onSubmit)}>
				<Box sx={styles.dialogFormSection}>
					<Paper
						elevation={0}
						sx={{
							...styles.dialogFormField,
							p: 2,
							mb: 2,
							backgroundColor: "rgba(0,0,0,0.02)",
						}}
					>
						<Typography variant="h6" mb={1} color="primary">
							Carbon Credits
						</Typography>
						<Grid container spacing={2} alignItems="center">
							<Grid item xs={2}>
								<Controller
									name="carbonCredit.added"
									control={control}
									defaultValue={false}
									render={({ field }) => (
										<FormControlLabel control={<Checkbox {...field} checked={field.value} />} label="Added" />
									)}
								/>
							</Grid>
							<Grid item xs={10}>
								<IntegerInput name="carbonCredit.numerator" control={control} label="Numerator" />
							</Grid>
							<Grid item xs={12}>
								<Typography variant="body2" align="right" color="textSecondary" sx={{ fontWeight: "medium" }}>
									{toDisplayAmount(carbonCreditRateNumeratorWatch.toString(), 0)} Carbon Credits per{" "}
									{toDisplayAmount(carbonCreditRateDenominatorWatch.toString(), projectTokenContract?.decimals || 0)}{" "}
									{projectTokenContract?.symbol || "Project"} Tokens
								</Typography>
							</Grid>
						</Grid>
					</Paper>

					<Paper
						elevation={0}
						sx={{
							...styles.dialogFormField,
							p: 2,
							mb: 2,
							backgroundColor: "rgba(0,0,0,0.02)",
						}}
					>
						<Typography variant="h6" mb={1} color="primary">
							Euro
						</Typography>
						<Grid container spacing={2} alignItems="center">
							<Grid item xs={2}>
								<Controller
									name="euro.added"
									control={control}
									defaultValue={false}
									render={({ field }) => (
										<FormControlLabel control={<Checkbox {...field} checked={field.value} />} label="Added" />
									)}
								/>
							</Grid>
							<Grid item xs={10}>
								<CurrencyInput name="euro.numerator" control={control} label="EuroE" />
							</Grid>
							<Grid item xs={12}>
								<Typography variant="body2" align="right" color="textSecondary" sx={{ fontWeight: "medium" }}>
									{toDisplayAmount(euroRateNumeratorWatch.toString(), 6)} Euro per{" "}
									{toDisplayAmount(euroRateDenominatorWatch.toString(), projectTokenContract?.decimals || 0)}{" "}
									{projectTokenContract?.symbol || "Project"} Tokens
								</Typography>
							</Grid>
						</Grid>
					</Paper>

					<Paper
						elevation={0}
						sx={{
							...styles.dialogFormField,
							p: 2,
							mb: 2,
							backgroundColor: "rgba(0,0,0,0.02)",
						}}
					>
						<Typography variant="h6" mb={1} color="primary">
							E-Trees
						</Typography>
						<Grid container spacing={2} alignItems="center">
							<Grid item xs={2}>
								<Controller
									name="eTrees.added"
									control={control}
									defaultValue={false}
									render={({ field }) => (
										<FormControlLabel control={<Checkbox {...field} checked={field.value} />} label="Added" />
									)}
								/>
							</Grid>
							<Grid item xs={10}>
								<IntegerInput name="eTrees.numerator" control={control} label="Etrees" />
							</Grid>
							<Grid item xs={12}>
								<Typography variant="body2" align="right" color="textSecondary" sx={{ fontWeight: "medium" }}>
									{toDisplayAmount(eTreesRateNumeratorWatch.toString(), 0)} E-Trees per{" "}
									{toDisplayAmount(eTreesRateDenominatorWatch.toString(), projectTokenContract?.decimals || 0)}{" "}
									{projectTokenContract?.symbol || "Project"} Tokens
								</Typography>
							</Grid>
						</Grid>
					</Paper>

					<Paper
						elevation={0}
						sx={{
							...styles.dialogFormField,
							p: 2,
							mb: 2,
							backgroundColor: "rgba(0,0,0,0.02)",
						}}
					>
						<Typography variant="h6" mb={1} color="primary">
							Euro Interest
						</Typography>
						<Grid container spacing={2} alignItems="center">
							<Grid item xs={2}>
								<Controller
									name="euroIntrest.added"
									control={control}
									defaultValue={false}
									render={({ field }) => (
										<FormControlLabel control={<Checkbox {...field} checked={field.value} />} label="Added" />
									)}
								/>
							</Grid>
							<Grid item xs={10}>
								<CurrencyInput name="euroIntrest.numerator" control={control} label="EuroE Interest" />
							</Grid>
							<Grid item xs={12}>
								<Typography variant="body2" align="right" color="textSecondary" sx={{ fontWeight: "medium" }}>
									{toDisplayAmount(euroIntrestRateNumeratorWatch.toString(), 6)} Euro per{" "}
									{toDisplayAmount(euroIntrestRateDenominatorWatch.toString(), projectTokenContract?.decimals || 0)}{" "}
									{projectTokenContract?.symbol || "Project"} Tokens per token version
								</Typography>
							</Grid>
						</Grid>
					</Paper>
				</Box>

				<Box sx={styles.dialogFormActions}>
					<TransactionButton
						variant="contained"
						type="submit"
						txnStatus={txnStatus}
						defaultText="Add Yield"
						loadingText="Adding Yield..."
						fullWidth
						sx={styles.formSubmitButton}
					/>
				</Box>
			</form>
		</Box>
	);
}
