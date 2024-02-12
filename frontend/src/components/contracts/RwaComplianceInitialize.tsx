import { useState } from "react";
import { useWallet } from "../WalletProvider";
import {
	Button,
	IconButton,
	List,
	ListItem,
	ListItemButton,
	ListItemText,
	Paper,
	Stack,
	TextField,
	Typography,
} from "@mui/material";
import ContractAddressField from "../common/concordium/ContractAddressField";
import SendTransactionButton from "../common/SendTransactionButton";
import { Contract, ContractType } from "./ContractTypes";
import ErrorDisplay from "../common/ErrorDisplay";
import CCDScanContractLink from "../common/concordium/CCDScanContractLink";
import { Delete } from "@mui/icons-material";
import {
	BlockItemSummaryInBlock,
	ContractAddress,
	RejectedInit,
} from "@concordium/web-sdk";
import { parseContractAddress } from "../../lib/common/common";
import rwaCompliance from "../../lib/rwaCompliance";

type ComplianceModule = ContractAddress.Type;

export default function RwaComplianceInitialize(props: {
	onSuccess: (contract: Contract) => void;
	complianceModules: Contract[];
}) {
	const wallet = useWallet();
	const [modules, setModules] = useState<ComplianceModule[]>([]);
	const [name, setName] = useState<string>("");
	const [newModule, setNewModule] = useState<ComplianceModule | undefined>(
		undefined,
	);
	const [error, setError] = useState("");

	const addExistingModule = (module: ComplianceModule) => {
		if (
			modules.find(
				(m) => m.index === module.index && m.subindex === module.subindex,
			)
		) {
			return;
		}
		setModules([...modules, module]);
	};
	const addModule = async () => {
		if (!newModule) {
			return;
		}
		if (
			modules.find(
				(m) => m.index === newModule.index && m.subindex === newModule.subindex,
			)
		) {
			return;
		}

		setModules([...modules, newModule!]);
		setNewModule(undefined);
	};

	const removeModule = (module: ComplianceModule) => {
		setModules(modules.filter((m) => m !== module));
	};
	const resetState = () => {
		setModules([]);
		setName("");
		setNewModule(undefined);
	};

	const handleSuccess = (outcome: BlockItemSummaryInBlock) => {
		try {
			const address = parseContractAddress(outcome);
			props.onSuccess({
				address,
				name,
				type: ContractType.RwaCompliance,
			});

			resetState();
		} catch (error) {
			setError(error instanceof Error ? error.message : "Unknown error");
			return;
		}
	};
	const isValid = modules.length > 0 && name.length > 0;
	return (
		<Stack spacing={2}>
			<Typography variant="h5">Initialize Compliance</Typography>

			<Paper sx={{ padding: 2 }} variant="outlined">
				<Stack spacing={2}>
					<TextField
						id="complianceContractDisplayName"
						name="complianceContractDisplayName"
						label="Compliance Display Name"
						variant="outlined"
						fullWidth
						required
						type="text"
						onChange={(e) => setName(e.target.value)}
						margin="normal"
						value={name}
					/>
					<Typography variant="h6">Added Modules</Typography>
					<List>
						{modules.map((module, index) => (
							<ListItem
								key={index}
								secondaryAction={
									<IconButton
										edge="end"
										aria-label="delete"
										onClick={() => removeModule(module)}
									>
										<Delete />
									</IconButton>
								}
							>
								<ListItemText
									primary={
										<CCDScanContractLink
											index={module.index.toString()}
											subIndex={module.subindex.toString()}
										/>
									}
									secondary="Compliance Module"
								/>
							</ListItem>
						))}
					</List>
				</Stack>
			</Paper>
			<Paper sx={{ padding: 2 }} variant="outlined">
				<Typography variant="h6">Existing Modules</Typography>
				<List dense>
					{props.complianceModules.map((module, index) => (
						<ListItem key={index} disablePadding disableGutters>
							<ListItemButton onClick={() => addExistingModule(module.address)}>
								<ListItemText
									primary={module.name}
									secondary={
										<CCDScanContractLink
											index={module.address.index.toString()}
											subIndex={module.address.subindex.toString()}
										/>
									}
								/>
							</ListItemButton>
						</ListItem>
					))}
				</List>
				<ContractAddressField
					value={newModule}
					onChange={setNewModule}
					indexName="complianceModuleIndex"
					subIndexName="complianceModuleSubIndex"
					indexHelperText="The index of the compliance module to initialize."
					subIndexHelperText="The sub-index of the compliance module to initialize."
				/>
				<Button disabled={!newModule} onClick={addModule} fullWidth>
					Add Module
				</Button>
			</Paper>
			<SendTransactionButton
				onClick={() =>
					rwaCompliance.init.init(wallet.provider!, wallet.currentAccount!, {
						modules: modules.map((m) => ({
							index: Number(m.index),
							subindex: Number(m.subindex),
						})),
					})
				}
				onFinalized={handleSuccess}
				onFinalizedError={(r) =>
					rwaCompliance.init.parseError(r as RejectedInit) || "Unknown Error"
				}
				disabled={!isValid}
			>
				Initialize Compliance Contract
			</SendTransactionButton>
			{error && <ErrorDisplay text={error} />}
		</Stack>
	);
}
