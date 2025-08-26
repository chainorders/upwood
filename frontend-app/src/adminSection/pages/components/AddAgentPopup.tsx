import React from "react";
import {
	Dialog,
	DialogTitle,
	DialogContent,
	DialogActions,
	Button,
	TextField,
	FormControl,
	FormLabel,
	RadioGroup,
	FormControlLabel,
	Radio,
	Checkbox,
	Box,
	Typography,
	Paper,
	alpha,
	Grid,
} from "@mui/material";
import { useForm, Controller } from "react-hook-form";
import TransactionButton from "../../../components/TransactionButton";
import { TxnStatus, updateContract } from "../../../lib/concordium";
import securitySftSingle from "../../../contractClients/generated/securitySftSingle";
import { User } from "../../../lib/user";
import { toParamsAddress } from "../../../lib/conversions";
import useCommonStyles from "../../../theme/useCommonStyles";
import securitySftMulti from "../../../contractClients/generated/securitySftMulti";

interface AddAgentPopupProps {
	user: User;
	open: boolean;
	onClose: () => void;
	contractAddress: string;
	roles: string[];
	method: typeof securitySftSingle.addAgent | typeof securitySftMulti.addAgent;
}

export interface AddAgentFormData {
	addressType: "Account" | "Contract";
	accountAddress?: string;
	contractIndex?: string;
	selectedRoles: string[];
}

export default function AddAgentPopup({ open, onClose, contractAddress, roles, user, method }: AddAgentPopupProps) {
	const classes = useCommonStyles();
	const [txnStatus, setTxnStatus] = React.useState<TxnStatus>("none");
	const { control, handleSubmit, watch, reset } = useForm<AddAgentFormData>({
		defaultValues: {
			addressType: "Account",
			accountAddress: "",
			contractIndex: "",
			selectedRoles: [],
		},
		mode: "onChange",
	});
	const [error, setError] = React.useState<string>();
	const addressType = watch("addressType");

	const handleFormSubmit = async (data: AddAgentFormData) => {
		setError(undefined);
		try {
			await updateContract(
				user.concordiumAccountAddress,
				contractAddress,
				method,
				{
					address: toParamsAddress(data.addressType === "Account" ? data.accountAddress! : data.contractIndex!),
					roles: data.selectedRoles.map((role) => ({ [role]: {} })),
				},
				setTxnStatus,
			);
			reset();
			handleClose();
		} catch (error) {
			if (error instanceof Error) {
				setError(error.message);
			} else if (typeof error === "string") {
				setError(error);
			} else {
				setError("An unknown error occurred.");
			}
			setTxnStatus("error");
		}
	};

	const handleClose = () => {
		setError(undefined);
		setTxnStatus("none");
		reset();
		onClose();
	};

	return (
		<Dialog open={open} onClose={handleClose} fullWidth maxWidth="sm">
			<DialogTitle sx={{ pb: 1 }}>
				<Typography variant="h6" component="div" fontWeight={600}>
					Add Agent
				</Typography>
			</DialogTitle>
			<DialogContent sx={classes.dialogFormContainer}>
				<form onSubmit={handleSubmit(handleFormSubmit)}>
					<Box sx={classes.dialogFormSection}>
						<Paper elevation={0} sx={{ p: 2, mb: 2, backgroundColor: (theme) => theme.palette.grey[50] }}>
							<Typography variant="subtitle2" color="primary" fontWeight={600} sx={{ mb: 1 }}>
								Address Information
							</Typography>
							<Box sx={classes.dialogFormField}>
								<FormControl component="fieldset" fullWidth>
									<FormLabel>Address Type</FormLabel>
									<Controller
										name="addressType"
										control={control}
										render={({ field }) => (
											<RadioGroup row {...field}>
												<FormControlLabel value="Account" control={<Radio />} label="Account" />
												<FormControlLabel value="Contract" control={<Radio />} label="Contract" />
											</RadioGroup>
										)}
									/>
								</FormControl>
							</Box>
							<Box sx={classes.dialogFormField}>
								{addressType === "Account" ? (
									<Controller
										name="accountAddress"
										control={control}
										rules={{ required: "Account address is required" }}
										render={({ field, fieldState }) => (
											<TextField
												{...field}
												label="Account Address"
												fullWidth
												size="small"
												error={!!fieldState.error}
												helperText={fieldState.error?.message}
											/>
										)}
									/>
								) : (
									<Controller
										name="contractIndex"
										control={control}
										rules={{ required: "Contract index is required" }}
										render={({ field, fieldState }) => (
											<TextField
												{...field}
												label="Contract Index"
												fullWidth
												size="small"
												error={!!fieldState.error}
												helperText={fieldState.error?.message}
											/>
										)}
									/>
								)}
							</Box>
						</Paper>

						<Paper elevation={0} sx={{ p: 2, backgroundColor: (theme) => theme.palette.grey[50] }}>
							<Typography variant="subtitle2" color="primary" fontWeight={600} sx={{ mb: 1 }}>
								Agent Roles
							</Typography>
							<Box sx={classes.dialogFormField}>
								<FormControl component="fieldset" fullWidth>
									<FormLabel sx={{ mb: 1 }}>Select Agent Permissions</FormLabel>
									<Controller
										name="selectedRoles"
										control={control}
										rules={{ required: "At least one role must be selected" }}
										render={({ field, fieldState }) => (
											<>
												<Grid container spacing={1}>
													{roles.map((role) => (
														<Grid item xs={6} key={role}>
															<FormControlLabel
																control={
																	<Checkbox
																		checked={field.value.includes(role)}
																		onChange={(e) => {
																			if (e.target.checked) {
																				field.onChange([...field.value, role]);
																			} else {
																				field.onChange(field.value.filter((r: string) => r !== role));
																			}
																		}}
																	/>
																}
																label={<Typography variant="body2">{role}</Typography>}
															/>
														</Grid>
													))}
												</Grid>
												{fieldState.error && (
													<Typography variant="body2" color="error" sx={{ mt: 1 }}>
														{fieldState.error.message}
													</Typography>
												)}
											</>
										)}
									/>
								</FormControl>
							</Box>
							{error && (
								<Paper
									elevation={0}
									sx={{
										p: 2,
										mt: 2,
										backgroundColor: (theme) => alpha(theme.palette.error.main, 0.1),
										borderLeft: (theme) => `4px solid ${theme.palette.error.main}`,
										borderRadius: 1,
									}}
								>
									<Typography variant="body2" color="error">
										{error}
									</Typography>
								</Paper>
							)}
						</Paper>
					</Box>

					<DialogActions sx={classes.dialogFormActions}>
						<Button onClick={handleClose} variant="outlined" size="medium">
							Cancel
						</Button>
						<TransactionButton
							type="submit"
							variant="contained"
							color="primary"
							txnStatus={txnStatus}
							defaultText="Add Agent"
							loadingText="Adding Agent..."
							size="medium"
						/>
					</DialogActions>
				</form>
			</DialogContent>
		</Dialog>
	);
}
