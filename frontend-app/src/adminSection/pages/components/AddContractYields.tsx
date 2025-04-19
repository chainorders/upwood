import { useCallback, useEffect, useState } from "react";
import {
	Box,
	Button,
	Dialog,
	DialogContent,
	DialogTitle,
	DialogActions,
	Grid,
	Paper,
	Step,
	StepLabel,
	Stepper,
	TextField,
	Typography,
	FormControlLabel,
	Checkbox,
	IconButton,
	Tabs,
	Tab,
} from "@mui/material";
import { useForm, Controller } from "react-hook-form";
import { DatePicker } from "@mui/x-date-pickers/DatePicker";
import { LocalizationProvider } from "@mui/x-date-pickers/LocalizationProvider";
import { AdapterDayjs } from "@mui/x-date-pickers/AdapterDayjs";
import dayjs from "dayjs";

import CloseIcon from "@mui/icons-material/Close";

import { ForestProjectTokenContract, IndexerService, SystemContractsConfigApiModel } from "../../../apiClient";
import { TxnStatus, updateContract } from "../../../lib/concordium";
import securitySftMulti from "../../../contractClients/generated/securitySftMulti";
import securitySftMultiYielder, {
	UpsertYieldRequest,
} from "../../../contractClients/generated/securitySftMultiYielder";
import { toDisplayAmount, toTokenId } from "../../../lib/conversions";
import { User } from "../../../lib/user";
import TransactionButton from "../../../components/TransactionButton";
import useCommonStyles from "../../../theme/useCommonStyles";
import IntegerInput from "./IntegerInput";
import CurrencyInput from "./CurrencyInput";

interface AddContractYieldsProps {
	contracts: SystemContractsConfigApiModel;
	open: boolean;
	onClose: () => void;
	user: User;
	tokenContract: ForestProjectTokenContract;
	onDone: (err?: string) => void;
}

interface TokenFormData {
	tokenId: number;
	metadataUrl: string;
	metadataHash: string;
	startDate: Date;
	endDate: Date;
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

export default function AddContractYields({
	open,
	onClose,
	user,
	tokenContract,
	onDone,
	contracts,
}: AddContractYieldsProps) {
	const styles = useCommonStyles();
	const [activeStep, setActiveStep] = useState(0);
	const [activeYieldTab, setActiveYieldTab] = useState(0);
	const [tokenIdHex, setTokenIdHex] = useState<string>("");
	const [lastTokenId, setLastTokenId] = useState<number>(0);
	const [addTokenTxnStatus, setAddTokenTxnStatus] = useState<TxnStatus>("none");
	const [addYieldTxnStatus, setAddYieldTxnStatus] = useState<TxnStatus>("none");
	const [createdTokenId, setCreatedTokenId] = useState<string>("");
	const [existingTokens, setExistingTokens] = useState<number[]>([]);
	const [tokenExists, setTokenExists] = useState<boolean>(false);

	const tokenForm = useForm<TokenFormData>({
		defaultValues: {
			tokenId: 0,
			metadataUrl: tokenContract.metadata_url || "",
			metadataHash: tokenContract.metadata_hash || "",
			startDate: new Date(tokenContract.created_at || Date.now()),
			endDate: new Date(),
		},
	});

	const yieldForm = useForm<YieldFormData>({
		mode: "onChange",
		defaultValues: {
			carbonCredit: { numerator: 1, denominator: 1, added: false },
			euro: { numerator: 1000000, denominator: 1, added: false },
			eTrees: { numerator: 1, denominator: 1, added: false },
			euroIntrest: { numerator: 1000000, denominator: 1, added: false },
		},
	});

	const steps = ["Calculate & Add Token", "Configure Yields"];

	useEffect(() => {
		if (open) {
			IndexerService.getAdminIndexerTokens(0, 1000, tokenContract.contract_address).then((tokens) => {
				const ids = tokens.data.map((t) => parseInt(t.token_id));
				setExistingTokens(ids);
			});
			IndexerService.getAdminIndexerYields(0, 1000, tokenContract.contract_address).then((yields) => {
				const ids = yields.data.map((t) => parseInt(t.token_id));
				const maxId = Math.max(...ids, 0);
				setLastTokenId(maxId);
			});
		}
	}, [open, tokenContract]);

	// Helper function to get date with time set to start of day
	const getStartOfDay = (date: Date): Date => {
		const newDate = new Date(date);
		newDate.setHours(0, 0, 0, 0);
		return newDate;
	};

	// Helper function to calculate days difference between two dates
	const calculateDaysDifference = useCallback((startDate: Date, endDate: Date): number => {
		const start = getStartOfDay(startDate);
		const end = getStartOfDay(endDate);
		const diffTime = Math.abs(end.getTime() - start.getTime());
		return Math.floor(diffTime / (1000 * 60 * 60 * 24));
	}, []);

	// Calculate token id whenever dates change and check if it exists
	const startDateValue = tokenForm.watch("startDate");
	const endDateValue = tokenForm.watch("endDate");
	useEffect(() => {
		if (startDateValue && endDateValue) {
			const startDate = new Date(startDateValue);
			const endDate = new Date(endDateValue);
			const daysDiff = calculateDaysDifference(startDate, endDate);
			const calculatedTokenId = (lastTokenId || 0) + daysDiff;
			tokenForm.setValue("tokenId", calculatedTokenId);
			setTokenIdHex(toTokenId(BigInt(calculatedTokenId), 8));
			// Check if token already exists
			setTokenExists(existingTokens.includes(calculatedTokenId));
		}
	}, [lastTokenId, tokenForm, startDateValue, endDateValue, existingTokens, calculateDaysDifference]);

	// Check if token exists when manually changing token ID
	const onTokenIdChange = (e: React.ChangeEvent<HTMLInputElement>) => {
		const tokenId = parseInt(e.target.value);
		tokenForm.setValue("tokenId", tokenId);
		setTokenIdHex(toTokenId(BigInt(tokenId), 8));

		// Check if token already exists
		setTokenExists(existingTokens.includes(tokenId));
	};

	const handleNext = () => {
		if (activeStep === 0) {
			tokenForm.handleSubmit(addToken)();
		} else {
			yieldForm.handleSubmit(addYields)();
		}
	};

	const handleBack = () => {
		setActiveStep((prevStep) => prevStep - 1);
	};

	// const handleReset = () => {
	// 	setActiveStep(0);
	// 	setAddTokenTxnStatus("none");
	// 	setAddYieldTxnStatus("none");
	// 	setCreatedTokenId("");
	// 	tokenForm.reset({
	// 		tokenId: lastTokenId,
	// 		metadataUrl: tokenContract.metadata_url || "",
	// 		metadataHash: tokenContract.metadata_hash || "",
	// 		startDate: new Date(tokenContract.created_at || Date.now()),
	// 		endDate: new Date(),
	// 	});
	// 	yieldForm.reset({
	// 		carbonCredit: { numerator: 1, denominator: 1, added: false },
	// 		euro: { numerator: 1000000, denominator: 1, added: false },
	// 		eTrees: { numerator: 1, denominator: 1, added: false },
	// 		euroIntrest: { numerator: 1000000, denominator: 1, added: false },
	// 	});
	// };

	const addToken = async (data: TokenFormData) => {
		// If token already exists, just set the token ID and move to the next step
		if (tokenExists) {
			console.log(`Token ID ${data.tokenId} already exists. Skipping token creation.`);
			setCreatedTokenId(data.tokenId.toString());
			setActiveStep(1);
			return;
		}

		try {
			await updateContract(
				user.concordiumAccountAddress,
				tokenContract.contract_address,
				securitySftMulti.addToken,
				{
					token_id: toTokenId(BigInt(data.tokenId), 8),
					token_metadata: {
						url: data.metadataUrl,
						hash: data.metadataHash ? { Some: [data.metadataHash] } : { None: {} },
					},
				},
				setAddTokenTxnStatus,
			);
			setAddTokenTxnStatus("success");
			setCreatedTokenId(data.tokenId.toString());
			setActiveStep(1);
		} catch (e) {
			setAddTokenTxnStatus("error");
			console.error(e);
			alert("Failed to add token");
		}
	};

	const addYields = async (data: YieldFormData) => {
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
					token_id: toTokenId(BigInt(createdTokenId), 8),
					token_contract: { index: Number(tokenContract.contract_address), subindex: 0 },
					yields,
				} as UpsertYieldRequest,
				setAddYieldTxnStatus,
			);
			setAddYieldTxnStatus("success");
			onDone();
			onClose();
		} catch (e) {
			console.error(e);
			setAddYieldTxnStatus("error");
			onDone("Failed to add yield");
		}
	};

	const renderTokenStep = () => {
		// Calculate days difference for display using the helper function
		const startDate = new Date(tokenForm.watch("startDate"));
		const endDate = new Date(tokenForm.watch("endDate"));
		const daysDiff = calculateDaysDifference(startDate, endDate);

		return (
			<Box sx={styles.dialogFormContainer}>
				<form>
					<Box sx={styles.dialogFormSection}>
						<Grid container spacing={2}>
							<Grid item xs={12} md={6}>
								<LocalizationProvider dateAdapter={AdapterDayjs}>
									<Controller
										name="startDate"
										control={tokenForm.control}
										render={({ field }) => (
											<DatePicker
												label="Start Date"
												value={dayjs(field.value)}
												minDate={dayjs(new Date(tokenContract.created_at))}
												maxDate={dayjs(endDate)}
												onChange={(newValue) => {
													field.onChange(newValue ? newValue.toDate() : null);
												}}
												slotProps={{ textField: { fullWidth: true, size: "small" } }}
											/>
										)}
									/>
								</LocalizationProvider>
							</Grid>
							<Grid item xs={12} md={6}>
								<LocalizationProvider dateAdapter={AdapterDayjs}>
									<Controller
										name="endDate"
										control={tokenForm.control}
										render={({ field }) => (
											<DatePicker
												label="End Date"
												value={dayjs(field.value)}
												maxDate={dayjs(new Date())}
												minDate={dayjs(startDate)}
												onChange={(newValue) => {
													field.onChange(newValue ? newValue.toDate() : null);
												}}
												slotProps={{ textField: { fullWidth: true, size: "small" } }}
											/>
										)}
									/>
								</LocalizationProvider>
							</Grid>
							<Grid item xs={12}>
								<Box
									sx={{
										backgroundColor: "rgba(0,0,0,0.03)",
										p: 1.5,
										borderRadius: 1,
										mb: 1,
										display: "flex",
										justifyContent: "space-between",
									}}
								>
									<Typography variant="body2">
										<strong>Days Difference:</strong> {daysDiff} days
									</Typography>
									<Typography variant="body2">
										<strong>Last Token ID:</strong> {lastTokenId}
									</Typography>
								</Box>
							</Grid>
							<Grid item xs={12}>
								<TextField
									label="Token Id"
									value={tokenForm.watch("tokenId")}
									onChange={onTokenIdChange}
									type="number"
									fullWidth
									variant="outlined"
									size="small"
									helperText={
										tokenExists
											? "Token with this ID already exists. Will skip token creation."
											: "Calculated based on days difference + last token ID"
									}
								/>
							</Grid>
							<Grid item xs={12}>
								<TextField
									label="Token Id Hex"
									value={tokenIdHex}
									InputProps={{
										readOnly: true,
									}}
									fullWidth
									variant="outlined"
									size="small"
								/>
							</Grid>
							<Grid item xs={12}>
								<TextField
									label="Metadata Url"
									{...tokenForm.register("metadataUrl", { required: true })}
									error={!!tokenForm.formState.errors.metadataUrl}
									helperText={tokenForm.formState.errors.metadataUrl ? "This field is required" : ""}
									fullWidth
									variant="outlined"
									size="small"
								/>
							</Grid>
							<Grid item xs={12}>
								<TextField
									label="Metadata Hash"
									{...tokenForm.register("metadataHash")}
									error={!!tokenForm.formState.errors.metadataHash}
									InputProps={{
										readOnly: true,
									}}
									InputLabelProps={{ shrink: true }}
									fullWidth
									variant="outlined"
									size="small"
								/>
							</Grid>
						</Grid>
					</Box>
				</form>
			</Box>
		);
	};

	const handleYieldTabChange = (_event: React.SyntheticEvent, newValue: number) => {
		setActiveYieldTab(newValue);
	};

	const renderQuantityYields = () => {
		const carbonCreditRateNumeratorWatch = yieldForm.watch("carbonCredit.numerator");
		const carbonCreditRateDenominatorWatch = yieldForm.watch("carbonCredit.denominator");
		const euroRateNumeratorWatch = yieldForm.watch("euro.numerator");
		const euroRateDenominatorWatch = yieldForm.watch("euro.denominator");
		const eTreesRateNumeratorWatch = yieldForm.watch("eTrees.numerator");
		const eTreesRateDenominatorWatch = yieldForm.watch("eTrees.denominator");

		return (
			<>
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
								control={yieldForm.control}
								defaultValue={false}
								render={({ field }) => (
									<FormControlLabel control={<Checkbox {...field} checked={field.value} />} label="Added" />
								)}
							/>
						</Grid>
						<Grid item xs={10}>
							<IntegerInput name="carbonCredit.numerator" control={yieldForm.control} label="Numerator" />
						</Grid>
						<Grid item xs={12}>
							<Typography variant="body2" align="right" color="textSecondary" sx={{ fontWeight: "medium" }}>
								{toDisplayAmount(carbonCreditRateNumeratorWatch.toString(), 0)} Carbon Credits per{" "}
								{toDisplayAmount(carbonCreditRateDenominatorWatch.toString(), tokenContract.decimals || 0)}{" "}
								{tokenContract.symbol || "Project"} Tokens
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
								control={yieldForm.control}
								defaultValue={false}
								render={({ field }) => (
									<FormControlLabel control={<Checkbox {...field} checked={field.value} />} label="Added" />
								)}
							/>
						</Grid>
						<Grid item xs={10}>
							<CurrencyInput name="euro.numerator" control={yieldForm.control} label="EuroE" />
						</Grid>
						<Grid item xs={12}>
							<Typography variant="body2" align="right" color="textSecondary" sx={{ fontWeight: "medium" }}>
								{toDisplayAmount(euroRateNumeratorWatch.toString(), 6)} Euro per{" "}
								{toDisplayAmount(euroRateDenominatorWatch.toString(), tokenContract.decimals || 0)}{" "}
								{tokenContract.symbol || "Project"} Tokens
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
								control={yieldForm.control}
								defaultValue={false}
								render={({ field }) => (
									<FormControlLabel control={<Checkbox {...field} checked={field.value} />} label="Added" />
								)}
							/>
						</Grid>
						<Grid item xs={10}>
							<IntegerInput name="eTrees.numerator" control={yieldForm.control} label="Etrees" />
						</Grid>
						<Grid item xs={12}>
							<Typography variant="body2" align="right" color="textSecondary" sx={{ fontWeight: "medium" }}>
								{toDisplayAmount(eTreesRateNumeratorWatch.toString(), 0)} E-Trees per{" "}
								{toDisplayAmount(eTreesRateDenominatorWatch.toString(), tokenContract.decimals || 0)}{" "}
								{tokenContract.symbol || "Project"} Tokens
							</Typography>
						</Grid>
					</Grid>
				</Paper>
			</>
		);
	};

	const renderInterestYields = () => {
		const euroIntrestRateNumeratorWatch = yieldForm.watch("euroIntrest.numerator");
		const euroIntrestRateDenominatorWatch = yieldForm.watch("euroIntrest.denominator");

		return (
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
							control={yieldForm.control}
							defaultValue={false}
							render={({ field }) => (
								<FormControlLabel control={<Checkbox {...field} checked={field.value} />} label="Added" />
							)}
						/>
					</Grid>
					<Grid item xs={10}>
						<CurrencyInput name="euroIntrest.numerator" control={yieldForm.control} label="EuroE Interest" />
					</Grid>
					<Grid item xs={12}>
						<Typography variant="body2" align="right" color="textSecondary" sx={{ fontWeight: "medium" }}>
							{toDisplayAmount(euroIntrestRateNumeratorWatch.toString(), 6)} Euro per{" "}
							{toDisplayAmount(euroIntrestRateDenominatorWatch.toString(), tokenContract.decimals || 0)}{" "}
							{tokenContract.symbol || "Project"} Tokens per token version
						</Typography>
					</Grid>
				</Grid>
			</Paper>
		);
	};

	const renderYieldStep = () => {
		return (
			<Box sx={styles.dialogFormContainer}>
				<form>
					<Box sx={styles.dialogFormSection}>
						<Tabs
							value={activeYieldTab}
							onChange={handleYieldTabChange}
							variant="fullWidth"
							sx={{ mb: 3, borderBottom: 1, borderColor: "divider" }}
						>
							<Tab label="Quantity Based Yields" />
							<Tab label="Interest Based Yields" />
						</Tabs>

						{activeYieldTab === 0 ? renderQuantityYields() : renderInterestYields()}
					</Box>
				</form>
			</Box>
		);
	};

	return (
		<Dialog open={open} onClose={onClose} maxWidth="md" fullWidth>
			<DialogTitle>
				Add Token with Yields
				<IconButton
					aria-label="close"
					onClick={onClose}
					sx={{
						position: "absolute",
						right: 8,
						top: 8,
					}}
				>
					<CloseIcon />
				</IconButton>
			</DialogTitle>
			<DialogContent>
				<Box sx={{ width: "100%", mt: 2 }}>
					<Stepper activeStep={activeStep} alternativeLabel>
						{steps.map((label) => (
							<Step key={label}>
								<StepLabel>{label}</StepLabel>
							</Step>
						))}
					</Stepper>

					<Box sx={{ mt: 4, mb: 2 }}>{activeStep === 0 ? renderTokenStep() : renderYieldStep()}</Box>
				</Box>
			</DialogContent>
			<DialogActions>
				<Button color="inherit" disabled={activeStep === 0} onClick={handleBack} sx={{ mr: 1 }}>
					Back
				</Button>
				<Box sx={{ flex: "1 1 auto" }} />
				<TransactionButton
					variant="contained"
					onClick={handleNext}
					txnStatus={activeStep === 0 ? addTokenTxnStatus : addYieldTxnStatus}
					defaultText={activeStep === steps.length - 1 ? "Finish" : tokenExists ? "Skip to Yields" : "Next"}
					loadingText={activeStep === 0 ? "Adding Token..." : "Adding Yields..."}
				/>
			</DialogActions>
		</Dialog>
	);
}
