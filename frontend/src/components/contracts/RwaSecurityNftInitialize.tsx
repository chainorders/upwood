import {
	AccountAddress,
	BlockItemSummaryInBlock,
	ContractAddress,
	RejectedInit,
} from "@concordium/web-sdk";
import { Contract, ContractType } from "./ContractTypes";
import { useState } from "react";
import { parseContractAddress } from "../../lib/common/common";
import {
	List,
	ListItem,
	ListItemButton,
	ListItemText,
	Paper,
	Stack,
	TextField,
	Typography,
} from "@mui/material";
import ErrorDisplay from "../common/ErrorDisplay";
import SendTransactionButton from "../common/SendTransactionButton";
import CCDScanContractLink from "../common/concordium/CCDScanContractLink";
import rwaSecurityNft from "../../lib/rwaSecurityNft";
import { WalletApi } from "@concordium/browser-wallet-api-helpers";

export default function RwaSecurityNftInitialize(props: {
	wallet: WalletApi;
	currentAccount: AccountAddress.Type;
	onSuccess: (contract: Contract) => void;
	identityRegistries: Contract[];
	complianceContracts: Contract[];
	sponsorContracts: Contract[];
}) {
	const [form, setForm] = useState<{
		contractDisplayName: string;
		identityRegistry?: ContractAddress.Type;
		complianceContract?: ContractAddress.Type;
		sponsorContracts: ContractAddress.Type[];
	}>({
		contractDisplayName: "",
		sponsorContracts: [],
	});
	const [error, setError] = useState("");

	const setFormValue = (key: keyof typeof form, value: unknown) => {
		setForm((prev) => ({ ...prev, [key]: value }));
	};
	const isFormValid = () => {
		return (
			form.contractDisplayName.length > 0 &&
			form.identityRegistry !== undefined &&
			form.complianceContract !== undefined
		);
	};
	const handleSuccess = (outcome: BlockItemSummaryInBlock) => {
		try {
			const address = parseContractAddress(outcome);
			props.onSuccess({
				address,
				name: form.contractDisplayName,
				type: ContractType.RwaSecurityNft,
			});
			setError("");
			setForm({
				contractDisplayName: "",
				sponsorContracts: [],
			});
		} catch (error) {
			setError(error instanceof Error ? error.message : "Unknown error");
			return;
		}
	};

	return (
		<Stack spacing={2}>
			<TextField
				label="Contract display name"
				value={form.contractDisplayName}
				onChange={(e) => setFormValue("contractDisplayName", e.target.value)}
				sx={{ width: "100%" }}
				id="nftContractDisplayName"
				name="nftContractDisplayName"
			/>
			<Paper sx={{ padding: 2 }} variant="outlined">
				<Typography variant="caption">Select identity registry</Typography>
				<List>
					{props.identityRegistries.map((contract, i) => (
						<ListItem key={i}>
							<ListItemButton
								selected={
									form.identityRegistry?.index === contract.address.index
								}
								onClick={() =>
									form.identityRegistry?.index === contract.address.index
										? setFormValue("identityRegistry", undefined)
										: setFormValue("identityRegistry", contract.address)
								}
							>
								<ListItemText
									primary={contract.name}
									secondary={CCDScanContractLink({
										index: contract.address.index.toString(),
										subIndex: contract.address.subindex.toString(),
									})}
								/>
							</ListItemButton>
						</ListItem>
					))}
				</List>
			</Paper>
			<Paper sx={{ padding: 2 }} variant="outlined">
				<Typography variant="caption">Select compliance contract</Typography>
				<List>
					{props.complianceContracts.map((contract, i) => (
						<ListItem key={i}>
							<ListItemButton
								selected={form.complianceContract === contract.address}
								onClick={() =>
									form.complianceContract?.index === contract.address.index
										? setFormValue("complianceContract", undefined)
										: setFormValue("complianceContract", contract.address)
								}
							>
								<ListItemText
									primary={contract.name}
									secondary={CCDScanContractLink({
										index: contract.address.index.toString(),
										subIndex: contract.address.subindex.toString(),
									})}
								/>
							</ListItemButton>
						</ListItem>
					))}
				</List>
			</Paper>
			<Paper sx={{ padding: 2 }} variant="outlined">
				<Typography variant="caption">Select Sponsor contracts</Typography>
				<List>
					{props.sponsorContracts.map((contract, i) => (
						<ListItem key={i}>
							<ListItemButton
								selected={form.sponsorContracts
									.map((c) => c.index)
									.includes(contract.address.index)}
								onClick={() =>
									form.sponsorContracts
										.map((c) => c.index)
										.includes(contract.address.index)
										? setFormValue(
												"sponsorContracts",
												form.sponsorContracts.filter(
													(c) => c.index !== contract.address.index,
												),
											)
										: setFormValue("sponsorContracts", [
												...form.sponsorContracts,
												contract.address,
											])
								}
							>
								<ListItemText
									primary={contract.name}
									secondary={CCDScanContractLink({
										index: contract.address.index.toString(),
										subIndex: contract.address.subindex.toString(),
									})}
								/>
							</ListItemButton>
						</ListItem>
					))}
				</List>
			</Paper>
			<SendTransactionButton
				onClick={() =>
					rwaSecurityNft.init.init(props.wallet, props.currentAccount, {
						identity_registry: {
							index: Number(form.identityRegistry!.index),
							subindex: Number(form.identityRegistry!.subindex),
						},
						compliance: {
							index: Number(form.complianceContract!.index),
							subindex: Number(form.complianceContract!.subindex),
						},
						sponsors: form.sponsorContracts.map((s) => ({
							index: Number(s.index),
							subindex: Number(s.subindex),
						})),
					})
				}
				onFinalized={handleSuccess}
				onFinalizedError={(r) =>
					rwaSecurityNft.init.parseError(r as RejectedInit) ||
					"Unknown Finalized error"
				}
				disabled={!isFormValid()}
			>
				Initialize Security NFT
			</SendTransactionButton>
			<ErrorDisplay text={error} />
		</Stack>
	);
}
