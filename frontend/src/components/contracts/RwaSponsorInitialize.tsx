import {
	AccountAddress,
	BlockItemSummaryInBlock,
	RejectedInit,
} from "@concordium/web-sdk";
import SendTransactionButton from "../common/SendTransactionButton";
import { Contract, ContractType } from "./ContractTypes";
import { Stack, TextField } from "@mui/material";
import { useState } from "react";
import { parseContractAddress } from "../../lib/common/common";
import ErrorDisplay from "../common/ErrorDisplay";
import { WalletApi } from "@concordium/browser-wallet-api-helpers";
import rwaSponsor from "../../lib/rwaSponsor";

export default function RwaSponsorInitialize(props: {
	wallet: WalletApi;
	currentAccount: AccountAddress.Type;
	onSuccess: (contract: Contract) => void;
}) {
	const [form, setForm] = useState({
		contractDisplayName: "",
	});
	const [error, setError] = useState("");

	const handleSuccess = (outcome: BlockItemSummaryInBlock) => {
		try {
			const address = parseContractAddress(outcome);
			props.onSuccess({
				address,
				name: form.contractDisplayName,
				type: ContractType.RwaSponsor,
			});
		} catch (error) {
			setError(error instanceof Error ? error.message : "Unknown error");
			return;
		}
	};

	const isFormValid = () => {
		return form.contractDisplayName.length > 0;
	};

	const setFormValue = (key: keyof typeof form, value: string) => {
		setForm((prev) => ({ ...prev, [key]: value }));
	};

	return (
		<form>
			<Stack spacing={2}>
				<TextField
					id="contractDisplayName"
					name="contractDisplayName"
					label="Contract Display Name"
					variant="outlined"
					fullWidth
					required
					type="text"
					onChange={(e) => setFormValue("contractDisplayName", e.target.value)}
				/>
				<SendTransactionButton
					onClick={() =>
						rwaSponsor.init.init(props.wallet, props.currentAccount)
					}
					onFinalized={handleSuccess}
					onFinalizedError={(r) =>
						rwaSponsor.init.parseError(r as RejectedInit) || "Unknown Error"
					}
					disabled={!isFormValid()}
				>
					Initialize Sponsor Contract
				</SendTransactionButton>
				{error && <ErrorDisplay text={error} />}
			</Stack>
		</form>
	);
}
