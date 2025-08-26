import React, { useState } from "react";
import {
	Dialog,
	DialogTitle,
	DialogContent,
	DialogActions,
	Button,
	Typography,
	Box,
	Paper,
	Divider,
	alpha,
	Grid,
	TextField,
} from "@mui/material";
import AddIcon from "@mui/icons-material/Add";
import { useForm } from "react-hook-form";
import { User } from "../../../lib/user";
import { TxnStatus, updateContract } from "../../../lib/concordium";
import TransactionButton from "../../../components/TransactionButton";
import securitySftMulti from "../../../contractClients/generated/securitySftMulti";
import { toTokenId } from "../../../lib/conversions";
import useCommonStyles from "../../../theme/useCommonStyles";
import { ForestProjectTokenContract, Token } from "../../../apiClient";

interface AddTokenPopupProps {
	open: boolean;
	onClose: () => void;
	onSuccess?: () => void;
	tokenContract: ForestProjectTokenContract;
	user: User;
	method: typeof securitySftMulti.addToken;
	tokenIdSize: number;
}

const AddTokenPopup: React.FC<AddTokenPopupProps> = ({
	open,
	onClose,
	onSuccess,
	tokenContract,
	user,
	method,
	tokenIdSize,
}) => {
	const classes = useCommonStyles();
	const [txnStatus, setTxnStatus] = React.useState<TxnStatus>("none");
	const [error, setError] = React.useState<string>();
	const [tokenIdHex, setTokenIdHex] = useState<string>("");

	const {
		register,
		handleSubmit,
		setValue,
		reset,
		formState: { errors },
	} = useForm<Token>({
		defaultValues: {
			metadata_url: tokenContract.metadata_url,
			metadata_hash: tokenContract.metadata_hash,
			token_id: "",
		},
	});

	const onTokenIdChange = (e: React.ChangeEvent<HTMLInputElement>) => {
		const tokenId = e.target.value;
		setValue("token_id", tokenId);
		if (tokenId) {
			try {
				setTokenIdHex(toTokenId(BigInt(tokenId), tokenIdSize));
			} catch (err) {
				setTokenIdHex("");
			}
		} else {
			setTokenIdHex("");
		}
	};

	const onSubmit = async (data: Token) => {
		setError(undefined);
		try {
			await updateContract(
				user.concordiumAccountAddress,
				tokenContract.contract_address,
				method,
				{
					token_id: toTokenId(BigInt(data.token_id), 8),
					token_metadata: {
						url: data.metadata_url,
						hash: data.metadata_hash ? { Some: [data.metadata_hash] } : { None: {} },
					},
				},
				setTxnStatus,
			);
			setTxnStatus("success");
			if (onSuccess) onSuccess();
			setTimeout(() => {
				handleClose();
			}, 1000);
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
		reset({
			metadata_url: tokenContract.metadata_url,
			metadata_hash: tokenContract.metadata_hash,
			token_id: "",
		});
		setTokenIdHex("");
		onClose();
	};

	return (
		<Dialog open={open} onClose={handleClose} fullWidth maxWidth="sm">
			<DialogTitle sx={{ pb: 1 }}>
				<Typography variant="h6" component="div" fontWeight={600}>
					Add New Token
				</Typography>
			</DialogTitle>
			<Divider />
			<DialogContent sx={classes.dialogFormContainer}>
				<Paper
					elevation={0}
					sx={{
						p: 2,
						mb: 3,
						backgroundColor: (theme) => alpha(theme.palette.primary.main, 0.05),
						borderRadius: 1,
					}}
				>
					<Typography variant="body2" color="text.secondary">
						Contract Address: <b>{tokenContract.contract_address}</b>
					</Typography>
					<Typography variant="body2" color="text.secondary">
						Contract Type: <b>{tokenContract.contract_type || "Unknown"}</b>
					</Typography>
				</Paper>

				<form onSubmit={handleSubmit(onSubmit)}>
					<Box sx={classes.dialogFormSection}>
						<Paper
							elevation={0}
							sx={{
								p: 2,
								backgroundColor: (theme) => theme.palette.grey[50],
								borderRadius: 1,
								mb: 2,
							}}
						>
							<Typography variant="subtitle2" color="primary" fontWeight={600} sx={{ mb: 2 }}>
								Token Information
							</Typography>

							<Grid container spacing={2}>
								<Grid item xs={12}>
									<Box sx={classes.dialogFormField}>
										<TextField
											label="Token ID"
											{...register("token_id", {
												required: "Token ID is required",
												pattern: {
													value: /^\d+$/,
													message: "Please enter a valid numeric ID",
												},
											})}
											onChange={onTokenIdChange}
											error={!!errors.token_id}
											helperText={errors.token_id?.message || "The numeric ID for this token"}
											fullWidth
											variant="outlined"
											size="small"
										/>
									</Box>
								</Grid>
								<Grid item xs={12}>
									<Box sx={classes.dialogFormField}>
										<TextField
											label="Token ID Hex"
											value={tokenIdHex}
											InputProps={{
												readOnly: true,
											}}
											fullWidth
											variant="outlined"
											size="small"
											helperText="Generated hexadecimal representation"
										/>
									</Box>
								</Grid>
							</Grid>
						</Paper>

						<Paper
							elevation={0}
							sx={{
								p: 2,
								backgroundColor: (theme) => theme.palette.grey[50],
								borderRadius: 1,
							}}
						>
							<Typography variant="subtitle2" color="primary" fontWeight={600} sx={{ mb: 2 }}>
								Metadata
							</Typography>

							<Grid container spacing={2}>
								<Grid item xs={12}>
									<Box sx={classes.dialogFormField}>
										<TextField
											label="Metadata URL"
											{...register("metadata_url", { required: "Metadata URL is required" })}
											error={!!errors.metadata_url}
											helperText={errors.metadata_url?.message || "URL pointing to token metadata"}
											fullWidth
											variant="outlined"
											size="small"
										/>
									</Box>
								</Grid>
								<Grid item xs={12}>
									<Box sx={classes.dialogFormField}>
										<TextField
											label="Metadata Hash"
											{...register("metadata_hash")}
											error={!!errors.metadata_hash}
											helperText="Optional hash for metadata verification"
											InputLabelProps={{ shrink: true }}
											fullWidth
											variant="outlined"
											size="small"
										/>
									</Box>
								</Grid>
							</Grid>
						</Paper>
					</Box>

					{error && (
						<Paper
							elevation={0}
							sx={{
								p: 2,
								mb: 2,
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

					<DialogActions sx={classes.dialogFormActions}>
						<Button onClick={handleClose} variant="outlined" size="medium">
							Cancel
						</Button>
						<TransactionButton
							type="submit"
							variant="contained"
							color="primary"
							startIcon={<AddIcon />}
							txnStatus={txnStatus}
							defaultText="Add Token"
							loadingText="Adding Token..."
							size="medium"
						/>
					</DialogActions>
				</form>
			</DialogContent>
		</Dialog>
	);
};

export default AddTokenPopup;
