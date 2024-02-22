import { useState } from "react";
import {
	AccountAddress,
	BlockItemSummaryInBlock,
	ContractAddress,
	RejectedInit,
} from "@concordium/web-sdk";
import {
	List,
	ListItem,
	ListItemButton,
	ListItemText,
	Paper,
	Stack,
	TextField,
} from "@mui/material";
import SendTransactionButton from "../common/SendTransactionButton";
import ContractAddressField from "../common/concordium/ContractAddressField";
import { parseContractAddress } from "../../lib/common/common";
import { Contract, ContractType } from "./ContractTypes";
import ErrorDisplay from "../common/ErrorDisplay";
import rwaComplianceModuleAllowedNationalities from "../../lib/rwaComplianceModuleAllowedNationalities";
import { WalletApi } from "@concordium/browser-wallet-api-helpers";

export default function RwaComplianceModuleAllowedNationalitiesInitialize(props: {
	wallet: WalletApi;
	currentAccount: AccountAddress.Type;
	onSuccess: (contract: Contract) => void;
	identityRegistries: Contract[];
}) {
	const [form, setForm] = useState<{
		contractDisplayName: string;
		identityRegistry?: ContractAddress.Type;
		nationalities: string[];
	}>({
		contractDisplayName: "",
		nationalities: [],
	});
	const [error, setError] = useState("");

	const setFormValue = (key: keyof typeof form, value: unknown) => {
		setForm((prev) => ({ ...prev, [key]: value }));
	};
	const isFormValid = () => {
		return (
			form.contractDisplayName.length > 0 &&
			form.identityRegistry !== undefined &&
			form.nationalities.length > 0
		);
	};
	const setNationalities = (nationalities: string) => {
		const trimmed = nationalities.trim();
		if (trimmed.length === 0) {
			setFormValue("nationalities", []);
			return;
		}
		const values = trimmed.split(",").map((s) => s.trim());

		setFormValue("nationalities", values);
	};
	const getNationalitiesString = (nationalities: string[]) => {
		return nationalities.join(", ");
	};
	const handleSuccess = (outcome: BlockItemSummaryInBlock) => {
		try {
			const address = parseContractAddress(outcome);
			props.onSuccess({
				address,
				name: form.contractDisplayName,
				type: ContractType.RwaComplianceModule,
			});
		} catch (error) {
			setError(error instanceof Error ? error.message : "Unknown error");
			return;
		}
	};
	return (
		<Stack spacing={2}>
			<TextField
				id="complianceModuleContractDisplayName"
				name="complianceModuleContractDisplayName"
				label="Compliance Module Contract Display Name"
				variant="outlined"
				fullWidth
				required
				type="text"
				onChange={(e) => setFormValue("contractDisplayName", e.target.value)}
			/>
			<Paper variant="outlined" sx={{ p: 1 }}>
				<Stack spacing={1}>
					<List>
						{props.identityRegistries.map((i) => (
							<ListItem
								key={i.address.index.toString() + i.address.subindex.toString()}
							>
								<ListItemButton
									onClick={() => setFormValue("identityRegistry", i.address)}
								>
									<ListItemText
										primary={i.name}
										secondary={`${i.address.index.toString()}/${i.address.subindex.toString()}`}
									/>
								</ListItemButton>
							</ListItem>
						))}
					</List>
					<ContractAddressField
						value={form.identityRegistry}
						onChange={(value) => setFormValue("identityRegistry", value)}
						indexName="Identity Registry Index"
						subIndexName="Identity Registry SubIndex"
						indexHelperText="Identity Registry Index"
						subIndexHelperText="Identity Registry Sub Index"
					/>
				</Stack>
			</Paper>

			<TextField
				helperText="Comma Separated Values"
				onChange={(e) => setNationalities(e.target.value)}
				value={getNationalitiesString(form.nationalities)}
				id="nationalities"
				name="nationalities"
				label="Nationalities"
			/>
			<SendTransactionButton
				onClick={() =>
					rwaComplianceModuleAllowedNationalities.init.init(
						props.wallet,
						props.currentAccount,
						{
							identity_registry: {
								index: Number(form.identityRegistry!.index),
								subindex: Number(form.identityRegistry!.subindex),
							},
							nationalities: form.nationalities,
						},
					)
				}
				onFinalized={handleSuccess}
				onFinalizedError={(r) =>
					rwaComplianceModuleAllowedNationalities.init.parseError(
						r as RejectedInit,
					) || "Unknown Error"
				}
				disabled={!isFormValid()}
			>
				Initialize Compliance Module Allowed Nationalities
			</SendTransactionButton>
			{error && <ErrorDisplay text={error} />}
		</Stack>
	);
}
