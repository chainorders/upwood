import { AccountAddress, ContractAddress } from "@concordium/web-sdk";
import {
	Button,
	ButtonGroup,
	Grid,
	List,
	ListItem,
	ListItemButton,
	ListItemIcon,
	ListItemText,
	Paper,
	Stack,
	TextField,
	Typography,
} from "@mui/material";
import { useEffect, useState } from "react";
import rwaMarket, { GetListedResponse, ListRequest } from "../../lib/rwaMarket";
import { useNodeClient } from "../NodeClientProvider";
import {
	Cancel,
	CheckCircle,
	Sell,
	SubscriptionsRounded,
	Token,
} from "@mui/icons-material";
import ErrorDisplay from "../common/ErrorDisplay";
import { useNavigate } from "react-router-dom";
import {
	PaymentToken,
	Rate,
	fromContractExchangeRate,
	toContractExchangeRate,
} from "./types";
import SendTransactionButton from "../common/SendTransactionButton";

export type NonListedToken = {
	id: string;
	contract: ContractAddress.Type;
	amount: number;
};

type Props = {
	contract: ContractAddress.Type;
	listed?: GetListedResponse;
	nonListed?: NonListedToken;
	onSendTransaction: (
		request: ListRequest,
		sponsorContract?: ContractAddress.Type,
	) => Promise<string>;
	currentAccount: AccountAddress.Type;
	sponsorContract?: ContractAddress.Type;
};
export default function ListRequestForm(props: Props) {
	const {
		contract,
		listed,
		onSendTransaction,
		currentAccount,
		nonListed,
		sponsorContract,
	} = props;
	const { provider: grpcClient } = useNodeClient();
	const navigate = useNavigate();

	const [loadingAllowedToList, setLoadingAllowedToList] = useState(false);
	const [errorAllowedToList, setErrorAllowedToList] = useState("");
	const [allowedToListContracts, setAllowedToListContracts] = useState<
		ContractAddress.Type[]
	>([]);
	const selectedListTokenContract =
		(listed &&
			ContractAddress.create(
				listed.token_id.contract.index,
				listed.token_id.contract.subindex,
			)) ||
		nonListed?.contract ||
		undefined;
	const [listTokenContract, setListTokenContract] = useState<
		ContractAddress.Type | undefined
	>(selectedListTokenContract);

	const [listTokenId, setListTokenId] = useState(
		listed?.token_id.id || nonListed?.id || "",
	);
	const [listAmount, setListAmount] = useState(
		Number(listed?.supply || nonListed?.amount) || 0,
	);

	const [loadingPaymentTokens, setLoadingPaymentTokens] = useState(false);
	const [errorPaymentTokens, setErrorPaymentTokens] = useState("");
	const [paymentTokens, setPaymentTokens] = useState<Array<PaymentToken>>([]);
	const [selectedPaymentToken, setSelectedPaymentToken] = useState<
		PaymentToken | undefined
	>(undefined);

	const updatePaymentTokenRate = (token: PaymentToken, rate?: Rate) => {
		const paymentTokenIndex = paymentTokens.findIndex(
			(t) =>
				t.type === token.type &&
				t.id === token.id &&
				t.contract?.index === token.contract?.index &&
				t.contract?.subindex === token.contract?.subindex,
		);

		if (paymentTokenIndex > -1) {
			paymentTokens[paymentTokenIndex] = { ...token, rate };
			setPaymentTokens([...paymentTokens]);
		}
	};

	// load allowed to list contracts
	useEffect(() => {
		setLoadingAllowedToList(true);
		rwaMarket.allowedToList
			.invoke(grpcClient, contract)
			.then((response) => {
				if (!response.returnValue) {
					setErrorAllowedToList("Could not get allowed to list contracts");
				}

				return rwaMarket.allowedToList.parseReturnValue(response.returnValue!)!;
			})
			.then((contracts) => {
				const parsedContracts = contracts.map((c) =>
					ContractAddress.create(c.index, c.subindex),
				);
				setAllowedToListContracts(parsedContracts);
			})
			.catch((error) => setErrorAllowedToList(error.message))
			.finally(() => setLoadingAllowedToList(false));
	}, [contract, grpcClient]);

	// load payment tokens
	useEffect(() => {
		setLoadingPaymentTokens(true);
		rwaMarket.paymentTokens
			.invoke(grpcClient, contract)
			.then((response) => {
				if (!response.returnValue) {
					setErrorPaymentTokens("Could not get payment tokens");
				}

				return rwaMarket.paymentTokens.parseReturnValue(response.returnValue!)!;
			})
			.then((tokens) =>
				tokens.map(
					(t) =>
						({
							type: "Cis2",
							id: t.id,
							contract: ContractAddress.create(
								t.contract.index,
								t.contract.subindex,
							),
						}) as PaymentToken,
				),
			)
			.then((tokens) => {
				if (!listed) {
					return tokens;
				}
				const listedExchangeRates = listed.exchange_rates
					.map((r) => fromContractExchangeRate(r))
					.filter((r) => !!r)
					.map((r) => r!);
				tokens.forEach((token) => {
					if (token.rate) {
						return;
					}

					const exchangeRate = listedExchangeRates.find(
						(r) =>
							r.type === token.type &&
							r.id === token.id &&
							r.contract?.index === token.contract?.index &&
							r.contract?.subindex === token.contract?.subindex,
					);
					if (exchangeRate) {
						token.rate = exchangeRate.rate;
					}
				});

				return tokens;
			})
			.then((tokens) => {
				setPaymentTokens([{ type: "Ccd" }, ...tokens]);
				if (tokens.length > 0) {
					setSelectedPaymentToken(tokens[0]);
				}
			})
			.catch((error) => setErrorPaymentTokens(error.message))
			.finally(() => setLoadingPaymentTokens(false));
	}, [contract, grpcClient, listed]);

	const isValid = () => {
		return (
			listTokenContract !== undefined &&
			listTokenId !== undefined &&
			paymentTokens.filter((t) => !!t.rate).length > 0
		);
	};

	const sendTransaction = async (sponsorContract?: ContractAddress.Type) => {
		const request: ListRequest = {
			owner: currentAccount!.address,
			token_id: {
				id: listTokenId,
				contract: {
					index: Number(listTokenContract!.index),
					subindex: Number(listTokenContract!.subindex),
				},
			},
			supply: listAmount.toString(),
			exchange_rates: paymentTokens
				.filter((t) => !!t.rate)
				.map((t) => toContractExchangeRate(t))
				.filter((t) => !!t)
				.map((t) => t!),
		};

		return onSendTransaction(request, sponsorContract);
	};

	return (
		<Paper variant="outlined">
			<Grid container spacing={2}>
				<Grid item xs={12} md={2}>
					<Stack p={1} spacing={2}>
						<Typography variant="caption">
							Token Contract to Sell / List
						</Typography>
						{loadingAllowedToList && (
							<Typography variant="caption">Loading...</Typography>
						)}
						{errorAllowedToList && <ErrorDisplay text={errorAllowedToList} />}
						<List>
							{allowedToListContracts.map((contract, index) => (
								<ListItem key={index}>
									<ListItemButton
										disabled={!!listed}
										selected={contract.index === listTokenContract?.index}
										onClick={() => setListTokenContract(contract)}
									>
										<ListItemIcon>
											<Token />
										</ListItemIcon>
										<ListItemText
											primary={
												contract.index.toString() +
												"/" +
												contract.subindex.toString()
											}
										/>
									</ListItemButton>
								</ListItem>
							))}
						</List>
					</Stack>
				</Grid>
				<Grid item xs={12} md={10}>
					<Stack p={1} spacing={2}>
						<Typography variant="caption">Token Id To Sell</Typography>
						<TextField
							disabled={!!listed}
							name="tokenId"
							value={listTokenId}
							onChange={(e) => setListTokenId(e.target.value)}
						/>
						<Typography variant="caption">Token Amount To Sell</Typography>
						<TextField
							name="tokenAmount"
							value={listAmount}
							type="number"
							onChange={(e) => setListAmount(Number(e.target.value))}
						/>
					</Stack>
				</Grid>
				<Grid item xs={12} md={2}>
					<Stack p={1} spacing={2}>
						<Typography variant="caption">Exchange Tokens</Typography>
						{loadingPaymentTokens && (
							<Typography variant="caption">Loading...</Typography>
						)}
						{errorPaymentTokens && <ErrorDisplay text={errorPaymentTokens} />}
						<List>
							{paymentTokens.map((token, index) => (
								<ListItem key={index}>
									<ListItemButton
										selected={
											token.type === selectedPaymentToken?.type &&
											token.contract?.index ===
												selectedPaymentToken?.contract?.index &&
											token.id === selectedPaymentToken?.id
										}
										onClick={() => setSelectedPaymentToken(token)}
									>
										<ListItemIcon>
											{token.rate ? (
												<CheckCircle color="success" />
											) : (
												<Cancel />
											)}
										</ListItemIcon>
										{
											{
												Ccd: (
													<ListItemText
														primary="CCD"
														secondary={
															token.rate
																? `${token.rate.denominator} for ${token.rate.numerator}`
																: ""
														}
													/>
												),
												Cis2: (
													<ListItemText
														primary={`${
															token.id
														} ${token.contract?.index.toString()}/${token.contract?.subindex.toString()}`}
														secondary={
															token.rate
																? `${token.rate.denominator} for ${token.rate.numerator}`
																: ""
														}
													/>
												),
											}[token.type]
										}
									</ListItemButton>
								</ListItem>
							))}
						</List>
					</Stack>
				</Grid>
				<Grid item xs={12} md={10}>
					<Stack p={1} spacing={2}>
						<Typography variant="caption">Exchange Rate</Typography>
						<TextField
							name="Exchange Rate"
							helperText="Exchange Rate"
							type="number"
							disabled={!selectedPaymentToken}
							value={(selectedPaymentToken?.rate?.numerator || 1).toString()}
							onChange={(e) => {
								const rate: Rate = {
									numerator: BigInt(e.target.value),
									denominator: 1n,
								};
								setSelectedPaymentToken({
									...selectedPaymentToken!,
									rate,
								});
								updatePaymentTokenRate(selectedPaymentToken!, rate);
							}}
						/>
						<ButtonGroup>
							<Button
								disabled={!selectedPaymentToken}
								variant="text"
								onClick={() => {
									setSelectedPaymentToken({
										...selectedPaymentToken!,
										rate: undefined,
									});
									updatePaymentTokenRate(selectedPaymentToken!, undefined);
								}}
							>
								Remove Rate
							</Button>
						</ButtonGroup>
					</Stack>
				</Grid>
				<Grid item xs={12} md={2}></Grid>
				<Grid item xs={12} md={10}>
					<ButtonGroup>
						<Button onClick={() => navigate(-1)}>Cancel</Button>
						<SendTransactionButton
							disabled={!isValid()}
							onClick={() => sendTransaction()}
						>
							<Sell sx={{ mr: 1 }} />
							List
						</SendTransactionButton>
						{sponsorContract && (
							<SendTransactionButton
								disabled={!isValid()}
								onClick={() => sendTransaction(sponsorContract!)}
							>
								<SubscriptionsRounded sx={{ mr: 1 }} />
								List Sponsored
							</SendTransactionButton>
						)}
					</ButtonGroup>
				</Grid>
			</Grid>
		</Paper>
	);
}
