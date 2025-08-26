import { TxnStatus, updateContract } from "../../../lib/concordium";
import { User } from "../../../lib/user";
import { ForestProjectTokenContract, Token } from "../../../apiClient";
import securitySftMulti from "../../../contractClients/generated/securitySftMulti";
import { toTokenId } from "../../../lib/conversions";
import { useState } from "react";
import { useForm } from "react-hook-form";
import { Box, TextField, Grid } from "@mui/material";
import TransactionButton from "../../../components/TransactionButton";
import useCommonStyles from "../../../theme/useCommonStyles";

interface AddProjectTokenPopupProps {
	token_contract: ForestProjectTokenContract;
	token_id?: string;
	onDone: (err?: string) => void;
	user: User;
}

export function AddProjectTokenPopup({ token_contract, token_id, onDone, user }: AddProjectTokenPopupProps) {
	const styles = useCommonStyles();
	const [txnStatus, setTxnStatus] = useState<TxnStatus>("none");
	const [tokenIdHex, setTokenIdHex] = useState<string>(toTokenId(BigInt(token_id || ""), 8));
	const {
		register,
		handleSubmit,
		setValue,
		formState: { errors },
	} = useForm<Token>({
		defaultValues: {
			metadata_url: token_contract.metadata_url,
			metadata_hash: token_contract.metadata_hash,
			token_id: token_id,
		},
	});

	const onTokenIdChange = (e: React.ChangeEvent<HTMLInputElement>) => {
		const tokenId = e.target.value;
		setValue("token_id", tokenId);
		setTokenIdHex(toTokenId(BigInt(tokenId), 8));
	};

	const onSubmit = async (data: Token) => {
		try {
			await updateContract(
				user.concordiumAccountAddress,
				token_contract.contract_address,
				securitySftMulti.addToken,
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
			onDone();
		} catch (e) {
			setTxnStatus("error");
			console.error(e);
		}
	};

	return (
		<Box sx={styles.dialogFormContainer}>
			<form onSubmit={handleSubmit(onSubmit)}>
				<Box sx={styles.dialogFormSection}>
					<Grid container spacing={2}>
						<Grid item xs={12}>
							<Box sx={styles.dialogFormField}>
								<TextField
									label="Token Id"
									{...register("token_id", { required: true, valueAsNumber: true })}
									onChange={onTokenIdChange}
									error={!!errors.token_id}
									helperText={errors.token_id ? "This field is required" : ""}
									type="number"
									fullWidth
									variant="outlined"
									size="small"
								/>
							</Box>
						</Grid>
						<Grid item xs={12}>
							<Box sx={styles.dialogFormField}>
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
							</Box>
						</Grid>
						<Grid item xs={12}>
							<Box sx={styles.dialogFormField}>
								<TextField
									label="Metadata Url"
									{...register("metadata_url", { required: true })}
									error={!!errors.metadata_url}
									helperText={errors.metadata_url ? "This field is required" : ""}
									fullWidth
									variant="outlined"
									size="small"
								/>
							</Box>
						</Grid>
						<Grid item xs={12}>
							<Box sx={styles.dialogFormField}>
								<TextField
									label="Metadata Hash"
									{...register("metadata_hash")}
									error={!!errors.metadata_hash}
									helperText={errors.metadata_hash ? "This field is required" : ""}
									InputProps={{
										readOnly: true,
									}}
									InputLabelProps={{ shrink: true }}
									fullWidth
									variant="outlined"
									size="small"
								/>
							</Box>
						</Grid>
					</Grid>
				</Box>
				<Box sx={styles.dialogFormActions}>
					<TransactionButton
						type="submit"
						txnStatus={txnStatus}
						defaultText="Create Token"
						loadingText="Creating..."
						variant="contained"
						fullWidth
						sx={styles.formSubmitButton}
					/>
				</Box>
			</form>
		</Box>
	);
}
