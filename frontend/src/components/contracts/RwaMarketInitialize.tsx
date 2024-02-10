import {
	BlockItemSummaryInBlock,
	ContractAddress,
	RejectedInit,
} from "@concordium/web-sdk";
import { useWallet } from "../WalletProvider";
import SendTransactionButton from "../common/SendTransactionButton";
import { Contract, ContractType } from "./ContractTypes";
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
import { useState } from "react";
import { parseContractAddress } from "../../lib/common/common";
import ErrorDisplay from "../common/ErrorDisplay";
import rwaMarket from "../../lib/rwaMarket";
import { Delete } from "@mui/icons-material";
import CCDScanContractLink from "../common/concordium/CCDScanContractLink";
import ContractAddressField from "../common/concordium/ContractAddressField";
import TokenUIdField, { TokenUId } from "../common/concordium/TokenUIdField";
import { Rate } from "../market/types";

type TokenContract = ContractAddress.Type;
const toCommission = (value: string) => {
	let commissionPercentage = parseInt(value);
	if (isNaN(commissionPercentage)) {
		commissionPercentage = 0;
	}
	return {
		numerator: BigInt(commissionPercentage) * 100n,
		denominator: 10000n,
	} as Rate;
};

type Props = {
	onSuccess: (contract: Contract) => void;
	existingTokenContracts: Contract[];
};

export default function RwaMarketInitialize(props: Props) {
	const wallet = useWallet();
	const [form, setForm] = useState({
		contractDisplayName: "",
		commission: {
			numerator: 0n,
			denominator: 1n,
		},
	});
	const [tokenContracts, setTokenContracts] = useState<TokenContract[]>([]);
	const [exchangeTokens, setExchangeTokens] = useState<TokenUId[]>([]);
	const [newTokenContract, setNewTokenContract] = useState<
		TokenContract | undefined
	>(undefined);
	const [newExchangeToken, setNewExchangeToken] = useState<
		TokenUId | undefined
	>(undefined);

	const addTokenContract = (contract: TokenContract) => {
		if (
			tokenContracts.find(
				(c) => c.index === contract.index && c.subindex === contract.subindex,
			)
		) {
			return;
		}
		setTokenContracts([...tokenContracts, contract]);
	};
	const removeTokenContract = (contract: TokenContract) => {
		setTokenContracts(tokenContracts.filter((c) => c !== contract));
	};
	const addExchangeToken = (token: TokenUId) => {
		if (
			exchangeTokens.find(
				(t) =>
					t.id === token.id &&
					t.contract.index === token.contract.index &&
					t.contract.subindex === token.contract.subindex,
			)
		) {
			return;
		}
		setExchangeTokens([...exchangeTokens, token]);
	};
	const removeExchangeToken = (token: TokenUId) => {
		setExchangeTokens(exchangeTokens.filter((t) => t !== token));
	};
	const addNewTokenContract = () => {
		if (!newTokenContract) {
			return;
		}
		addTokenContract(newTokenContract);
		setNewTokenContract(undefined);
	};
	const addNewExchangeToken = () => {
		if (!newExchangeToken) {
			return;
		}
		addExchangeToken(newExchangeToken);
		setNewExchangeToken(undefined);
	};

	const [error, setError] = useState("");

	const handleSuccess = (outcome: BlockItemSummaryInBlock) => {
		try {
			const address = parseContractAddress(outcome);
			props.onSuccess({
				address,
				name: form.contractDisplayName,
				type: ContractType.RwaMarket,
			});
		} catch (error) {
			setError(error instanceof Error ? error.message : "Unknown error");
			return;
		}
	};

	const isFormValid = () => {
		return (
			form.contractDisplayName.length > 0 &&
			tokenContracts.length > 0 &&
			exchangeTokens.length > 0
		);
	};

	const setFormValue = (key: keyof typeof form, value: unknown) => {
		setForm((prev) => ({ ...prev, [key]: value }));
	};

	return (
		<form>
			<Stack spacing={2}>
				<Typography variant="h5">Initialize Market / Exchange</Typography>
				<Paper sx={{ padding: 2 }} variant="outlined">
					<Stack spacing={2}>
						<TextField
							id="marketContractDisplayName"
							name="marketContractDisplayName"
							label="Contract Display Name"
							variant="outlined"
							fullWidth
							required
							type="text"
							onChange={(e) =>
								setFormValue("contractDisplayName", e.target.value)
							}
						/>
						<TextField
							id="marketCommission"
							name="marketCommission"
							label="Market Commission %"
							variant="outlined"
							fullWidth
							required
							type="number"
							onChange={(e) =>
								setFormValue("commission", toCommission(e.target.value))
							}
							helperText={`Commission ${form.commission.numerator}/${form.commission.denominator}`}
						/>
						<Typography variant="h6">Added Token Contracts</Typography>
						<List>
							{tokenContracts.map((tokenContract, index) => (
								<ListItem
									key={index}
									secondaryAction={
										<IconButton
											edge="end"
											aria-label="delete"
											onClick={() => removeTokenContract(tokenContract)}
										>
											<Delete />
										</IconButton>
									}
								>
									<ListItemText
										primary={
											<CCDScanContractLink
												index={tokenContract.index.toString()}
												subIndex={tokenContract.subindex.toString()}
											/>
										}
										secondary="Token Contract"
									/>
								</ListItem>
							))}
						</List>
						<Typography variant="h6">Added Exchange Tokens</Typography>
						<List>
							{exchangeTokens.map((exchangeToken, index) => (
								<ListItem
									key={index}
									secondaryAction={
										<IconButton
											edge="end"
											aria-label="delete"
											onClick={() => removeExchangeToken(exchangeToken)}
										>
											<Delete />
										</IconButton>
									}
								>
									<ListItemText
										primary={
											<CCDScanContractLink
												index={exchangeToken.contract.index.toString()}
												subIndex={exchangeToken.contract.subindex.toString()}
												text={`Token Id: ${exchangeToken.id}, Contract: ${exchangeToken.contract.index}/${exchangeToken.contract.subindex}`}
											/>
										}
										secondary="Exchange Token"
									/>
								</ListItem>
							))}
						</List>
					</Stack>
				</Paper>
				<Paper sx={{ padding: 2 }} variant="outlined">
					<Typography variant="h6">Existing Token Contracts</Typography>
					<List dense>
						{props.existingTokenContracts.map((tokenContract, index) => (
							<ListItem key={index} disablePadding disableGutters>
								<ListItemButton
									onClick={() => addTokenContract(tokenContract.address)}
								>
									<ListItemText
										primary={tokenContract.name}
										secondary={
											<CCDScanContractLink
												index={tokenContract.address.index.toString()}
												subIndex={tokenContract.address.subindex.toString()}
											/>
										}
									/>
								</ListItemButton>
							</ListItem>
						))}
					</List>
					<ContractAddressField
						value={newTokenContract}
						onChange={setNewTokenContract}
						indexName="tokenContractIndex"
						subIndexName="tokenContractSubIndex"
						indexHelperText="The index of the Token Contract."
						subIndexHelperText="The sub-index of the Token Contract."
					/>
					<Button
						disabled={!newTokenContract}
						onClick={addNewTokenContract}
						fullWidth
					>
						Add Token Contract
					</Button>
				</Paper>
				<Paper sx={{ padding: 2 }} variant="outlined">
					<Typography variant="h6">Exchange Token</Typography>
					<TokenUIdField
						value={newExchangeToken}
						onChange={setNewExchangeToken}
					/>
					<Button
						disabled={!newExchangeToken}
						onClick={addNewExchangeToken}
						fullWidth
					>
						Add Exchange Token
					</Button>
				</Paper>
				<SendTransactionButton
					onClick={() =>
						rwaMarket.init.init(wallet.provider!, wallet.currentAccount!, {
							token_contracts: tokenContracts.map((c) => ({
								index: Number(c.index),
								subindex: Number(c.subindex),
							})),
							exchange_tokens: exchangeTokens.map((t) => ({
								id: t.id,
								contract: {
									index: Number(t.contract.index),
									subindex: Number(t.contract.subindex),
								},
							})),
							commission: {
								numerator: BigInt(form.commission.numerator),
								denominator: BigInt(form.commission.denominator),
							},
						})
					}
					onFinalized={handleSuccess}
					onFinalizedError={(r) =>
						rwaMarket.init.parseError(r as RejectedInit) || "Unknown Error"
					}
					disabled={!isFormValid()}
				>
					Initialize Market
				</SendTransactionButton>
				{error && <ErrorDisplay text={error} />}
			</Stack>
		</form>
	);
}
