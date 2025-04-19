import { useForm, Controller } from "react-hook-form";
import { Dialog, DialogTitle, DialogContent, TextField, Button, Typography, Box, Paper, Grid } from "@mui/material";
import { ForestProjectPrice, ForestProjectService } from "../../../apiClient";
import { formatDate, toDisplayAmount } from "../../../lib/conversions";
import useCommonStyles from "../../../theme/useCommonStyles";
import CurrencyInput from "./CurrencyInput";

interface AddPricePopupProps {
	open: boolean;
	onClose: () => void;
	currency_token_id: string;
	currency_token_contract_address: string;
	projectId: string;
}

export default function AddPricePopup({
	open,
	onClose,
	projectId,
	currency_token_id,
	currency_token_contract_address,
}: AddPricePopupProps) {
	const styles = useCommonStyles();
	const { control, handleSubmit, reset, watch } = useForm<ForestProjectPrice>({
		mode: "onChange", // Enable validation on change
		defaultValues: {
			currency_token_id: currency_token_id,
			currency_token_contract_address: currency_token_contract_address,
			project_id: projectId,
			price_at: formatDate(new Date()),
		},
	});

	const priceWatch = watch("price");

	const onSubmit = async (data: ForestProjectPrice) => {
		try {
			data.price_at = formatDate(new Date(data.price_at));
			data.currency_token_id = currency_token_id;
			data.currency_token_contract_address = currency_token_contract_address;
			data.project_id = projectId;
			await ForestProjectService.postAdminForestProjectsPrice(projectId, data);
			onClose();
			reset();
		} catch (error) {
			console.error("Failed to add price", error);
		}
	};

	return (
		<Dialog open={open} onClose={onClose} fullWidth maxWidth="sm">
			<DialogTitle>
				<Typography variant="h5" component="div" sx={{ fontWeight: 600 }}>
					Add Price
				</Typography>
			</DialogTitle>
			<DialogContent>
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
									€ Price
								</Typography>
								<Grid container spacing={2}>
									<Grid item xs={12}>
										<CurrencyInput
											name="price"
											control={control}
											label="Price"
											textFieldProps={{
												fullWidth: true,
												autoComplete: "off",
												required: true,
											}}
										/>
										<Typography variant="caption" color="textSecondary" sx={{ display: "block", mt: 1, textAlign: "right" }}>
											Price: € {toDisplayAmount(priceWatch?.toString() || "0", 6)}
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
									Price At
								</Typography>
								<Grid container spacing={2}>
									<Grid item xs={12}>
										<Controller
											name="price_at"
											control={control}
											render={({ field }) => (
												<TextField
													{...field}
													label="Price At"
													type="datetime-local"
													fullWidth
													margin="normal"
													variant="outlined"
													size="small"
												/>
											)}
										/>
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
									Reference Information
								</Typography>
								<Grid container spacing={2}>
									<Grid item xs={12}>
										<Controller
											name="currency_token_id"
											control={control}
											disabled
											render={({ field }) => (
												<TextField
													{...field}
													label="Currency Token ID"
													fullWidth
													margin="normal"
													variant="outlined"
													size="small"
													disabled
													sx={{ opacity: 0.7 }}
												/>
											)}
										/>
									</Grid>
									<Grid item xs={12}>
										<Controller
											name="currency_token_contract_address"
											control={control}
											disabled
											render={({ field }) => (
												<TextField
													{...field}
													label="Currency Token Contract Address"
													fullWidth
													margin="normal"
													variant="outlined"
													size="small"
													disabled
													sx={{ opacity: 0.7 }}
												/>
											)}
										/>
									</Grid>
									<Grid item xs={12}>
										<Controller
											name="project_id"
											control={control}
											render={({ field }) => (
												<TextField
													{...field}
													label="Project ID"
													fullWidth
													margin="normal"
													variant="outlined"
													size="small"
													disabled
													sx={{ opacity: 0.7 }}
												/>
											)}
										/>
									</Grid>
								</Grid>
							</Paper>
						</Box>

						<Box sx={styles.dialogFormActions}>
							<Button type="submit" variant="contained" color="primary" fullWidth sx={styles.formSubmitButton}>
								Add Price
							</Button>
						</Box>
					</form>
				</Box>
			</DialogContent>
		</Dialog>
	);
}
