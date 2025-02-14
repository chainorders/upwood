import { useOutletContext } from "react-router";
import { TxnStatus, updateContract } from "../../../lib/concordium";
import { User } from "../../../lib/user";
import { ForestProjectTokenContract, Token } from "../../../apiClient";
import securitySftMulti from "../../../contractClients/generated/securitySftMulti";
import { toTokenId } from "../../../lib/conversions";
import { useState } from "react";
import Typography from "@mui/material/Typography";
import { useForm } from "react-hook-form";
import { Box, TextField } from "@mui/material";
import TransactionButton from "../../../components/TransactionButton";

interface AddProjectTokenPopupProps {
	token_contract: ForestProjectTokenContract;
	token_id?: string;
	onDone: (err?: string) => void;
}

export function AddProjectTokenPopup({ token_contract, token_id, onDone }: AddProjectTokenPopupProps) {
	const { user } = useOutletContext<{ user: User }>();
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
		<Box component="form" onSubmit={handleSubmit(onSubmit)} sx={{ display: "flex", flexDirection: "column", gap: 2 }}>
			<Typography variant="h4" gutterBottom>
				Create Token
			</Typography>
			<TextField
				label="Token Id"
				{...register("token_id", { required: true, valueAsNumber: true })}
				onChange={onTokenIdChange}
				error={!!errors.token_id}
				helperText={errors.token_id ? "This field is required" : ""}
				type="number"
			/>
			<TextField
				label="Token Id Hex"
				value={tokenIdHex}
				InputProps={{
					readOnly: true,
				}}
			/>
			<TextField
				label="Metadata Url"
				{...register("metadata_url", { required: true })}
				error={!!errors.metadata_url}
				helperText={errors.metadata_url ? "This field is required" : ""}
			/>
			<TextField
				label="Metadata Hash"
				InputProps={{
					readOnly: true,
				}}
			/>
			<TransactionButton
				type="submit"
				txnStatus={txnStatus}
				defaultText="Create Token"
				loadingText="Creating..."
				variant="contained"
			/>
		</Box>
	);
}
