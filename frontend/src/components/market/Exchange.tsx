import {
	AccountAddress,
	CIS2,
	CIS2Contract,
	CcdAmount,
	ConcordiumGRPCWebClient,
	ContractAddress,
	Energy,
	EntrypointName,
	serializeTypeValue,
	toBuffer,
} from "@concordium/web-sdk";
import { useLocation, Location, useNavigate } from "react-router-dom";
import { MarketToken } from "../../lib/contracts-api-client";
import { useNodeClient } from "../NodeClientProvider";
import { useEffect, useState } from "react";
import { Buffer } from "buffer/";

import rwaMarket, {
	ExchangeRequest,
	GetListedResponse,
} from "../../lib/rwaMarket";
import {
	Button,
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
import ErrorDisplay from "../common/ErrorDisplay";
import { Cancel, CheckCircle } from "@mui/icons-material";
import CCDScanAccountLink from "../common/concordium/CCDScanAccountLink";
import {
	Cis2PaymentToken,
	PaymentToken,
	arePaymentTokensEqual,
	fromContractExchangeRate,
	toContractExchangeRate,
	toContractRate,
} from "./types";
import SendTransactionButton from "../common/SendTransactionButton";
import { WalletApi } from "@concordium/browser-wallet-api-helpers";

type Props = {
	contract: ContractAddress.Type;
	walletApi: WalletApi;
	currentAccount: AccountAddress.Type;
};
export default function Exchange(props: Props) {
	const navigate = useNavigate();
	const { contract, walletApi, currentAccount } = props;
	const { state: token }: Location<MarketToken | undefined> = useLocation();
	const { provider: grpcClient } = useNodeClient();
	const [error, setError] = useState<string | undefined>(undefined);
	const [loading, setLoading] = useState(false);
	const [listedToken, setListedToken] = useState<GetListedResponse | undefined>(
		undefined,
	);

	if (!token) {
		navigate(-1);
	}

	useEffect(() => {
		setLoading(true);
		rwaMarket.getListed
			.invoke(grpcClient, contract, {
				owner: token!.owner,
				token_id: {
					id: token!.token_id,
					contract: {
						index: token!.token_contract.index,
						subindex: token!.token_contract.subindex,
					},
				},
			})
			.then((result) =>
				rwaMarket.getListed.parseReturnValue(result.returnValue!),
			)
			.then((listedToken) => {
				if (!listedToken) {
					throw new Error("Token is not listed, got undefined from getListed");
				}

				return listedToken;
			})
			.then((listedToken) => setListedToken(listedToken))
			.catch((error) => {
				setError(error.message);
			})
			.finally(() => {
				setLoading(false);
			});
	}, [contract, token, grpcClient]);

	const sendTransaction = async (
		request: ExchangeRequest,
		paymentToken: PaymentToken,
		paymentAmount: number,
		transfer: boolean,
	) => {
		if (paymentToken.type === "Cis2") {
			if (transfer) {
				const exchangeRequestSerialized = serializeTypeValue(
					request,
					toBuffer(rwaMarket.exchange.paramsSchemaBase64!, "base64"),
				);
				const cis2CLient = await CIS2Contract.create(
					grpcClient,
					paymentToken.contract,
				);
				const transfer = cis2CLient.createTransfer(
					{
						energy: Energy.create(
							rwaMarket.exchange.maxExecutionEnergy.value * BigInt(2),
						),
					},
					{
						from: currentAccount!,
						to: {
							address: contract,
							hookName: EntrypointName.fromString("deposit"),
						},
						amount: BigInt(0),
						tokenId: paymentToken.id,
						tokenAmount: BigInt(paymentAmount),
						data: Buffer.from(exchangeRequestSerialized.buffer).toString("hex"),
					} as CIS2.Transfer,
				);
				return walletApi!.sendTransaction(
					currentAccount!,
					transfer.type,
					transfer.payload,
					transfer.parameter.json,
					transfer.schema,
				);
			} else {
				return rwaMarket.exchange.update(
					walletApi!,
					currentAccount!,
					props.contract,
					request,
				);
			}
		} else if (paymentToken.type === "Ccd") {
			return rwaMarket.exchange.update(
				walletApi!,
				currentAccount!,
				props.contract,
				request,
				CcdAmount.fromCcd(paymentAmount),
			);
		} else {
			throw new Error("Unknown payment token");
		}
	};

	return (
		<Stack spacing={2}>
			{error && <ErrorDisplay text={error} />}
			{loading && <Typography variant="body1">Loading...</Typography>}
			{listedToken && (
				<ExchangeRequestForm
					contract={contract}
					walletApi={walletApi}
					currentAccount={currentAccount!}
					grpcClient={grpcClient}
					listed={listedToken}
					onSendTransaction={sendTransaction}
				/>
			)}
		</Stack>
	);
}

function ExchangeRequestForm(props: {
	contract: ContractAddress.Type;
	walletApi: WalletApi;
	currentAccount: AccountAddress.Type;
	listed: GetListedResponse;
	grpcClient: ConcordiumGRPCWebClient;
	onSendTransaction: (
		request: ExchangeRequest,
		paymentToken: PaymentToken,
		paymentAmount: number,
		transfer: boolean,
	) => Promise<string>;
}) {
	const { contract, listed, onSendTransaction, currentAccount, grpcClient } =
		props;
	const navigate = useNavigate();
	const exchangeRates = listed.exchange_rates
		.map((r) => fromContractExchangeRate(r))
		.filter((r) => !!r)
		.map((r) => r!);
	const [selectedPaymentToken, setSelectedPaymentToken] = useState<
		PaymentToken | undefined
	>(undefined);
	const [buyAmount, setBuyAmount] = useState(0);
	const [actualBuyAmount, setActualBuyAmount] = useState(0);
	const [payAmount, setPayAmount] = useState<number>(0);
	const [commissionAmount, setCommissionAmount] = useState<number>(0);
	const [totalPayAmount, setTotalAmount] = useState<number>(0);

	const [error, setError] = useState<string | undefined>(undefined);

	useEffect(() => {
		setPayAmount(0);
		setCommissionAmount(0);
		setTotalAmount(0);

		if (!buyAmount || !selectedPaymentToken || !selectedPaymentToken.rate) {
			return;
		}

		rwaMarket.calculateAmounts
			.invoke(grpcClient, contract, {
				amount: buyAmount.toString(),
				owner: listed.owner,
				payer: currentAccount.address,
				rate: toContractExchangeRate(selectedPaymentToken)!,
				token_id: {
					id: listed.token_id.id,
					contract: listed.token_id.contract,
				},
			})
			.then((result) =>
				rwaMarket.calculateAmounts.parseReturnValue(result.returnValue!),
			)
			.then((amounts) => {
				if (!amounts) {
					throw new Error("Amounts is undefined");
				}

				return amounts;
			})
			.then((amounts) => {
				let payAmount = 0;
				let commissionAmount = 0;
				if ("Cis2" in amounts.pay) {
					payAmount = Number(amounts.pay.Cis2[0]);
				} else if ("CCD" in amounts.pay) {
					payAmount = Number(
						CcdAmount.microCcdToCcd(BigInt(amounts.pay.CCD[0])),
					);
				}
				setPayAmount(payAmount);

				if ("Cis2" in amounts.commission) {
					commissionAmount = Number(amounts.commission.Cis2[0]);
				} else if ("CCD" in amounts.commission) {
					commissionAmount = Number(
						CcdAmount.microCcdToCcd(BigInt(amounts.commission.CCD[0])),
					);
				}
				setCommissionAmount(commissionAmount);
				setTotalAmount(payAmount + commissionAmount);
				setActualBuyAmount(Number(amounts.buy));
			})
			.catch((error) => {
				console.log("calculate amounts", error);
				setError(error.message);
			});
	}, [
		buyAmount,
		selectedPaymentToken,
		contract,
		listed,
		grpcClient,
		currentAccount,
	]);

	const buyViaCcd = (
		listed: GetListedResponse,
		rate: { numerator: bigint; denominator: bigint },
		payAmount: number,
		buyAmount: number,
	) => {
		const request: ExchangeRequest = {
			amount: buyAmount.toString(),
			rate: {
				Ccd: [toContractRate(rate)],
			},
			token_id: {
				id: listed.token_id.id,
				contract: listed.token_id.contract,
			},
			owner: listed.owner,
			payer: currentAccount.address,
		};

		return onSendTransaction(request, { type: "Ccd" }, payAmount, false);
	};

	const buyViaToken = (
		listed: GetListedResponse,
		token: Cis2PaymentToken,
		payAmount: number,
		buyAmount: number,
		viaTokenTransfer: boolean,
	) => {
		const request: ExchangeRequest = {
			amount: buyAmount.toString(),
			rate: toContractExchangeRate(token)!,
			token_id: {
				id: listed.token_id.id,
				contract: listed.token_id.contract,
			},
			owner: listed.owner,
			payer: currentAccount.address,
		};

		return onSendTransaction(request, token, payAmount, viaTokenTransfer);
	};

	return (
		<Paper variant="outlined">
			<Grid container spacing={2}>
				<Grid item xs={12} md={2}>
					<Stack spacing={2} p={2}>
						<Typography variant="caption">Select Payment Token</Typography>
						<List>
							{exchangeRates.map((token, index) => (
								<ListItem key={index}>
									<ListItemButton
										selected={arePaymentTokensEqual(
											token,
											selectedPaymentToken,
										)}
										onClick={() => setSelectedPaymentToken(token)}
									>
										<ListItemIcon>
											{arePaymentTokensEqual(token, selectedPaymentToken) ? (
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
																? `${token.rate.numerator} for ${token.rate.denominator}`
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
																? `${token.rate.numerator} for ${token.rate.denominator}`
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
					<Stack spacing={2} p={2}>
						<TextField
							name="buyTokenId"
							label="Buy Token Id"
							value={listed.token_id.id}
							disabled
						/>
						<TextField
							name="buyTokenContract"
							label="Buy Token Contract"
							value={`${listed.token_id.contract.index}/${listed.token_id.contract.subindex}`}
							disabled
						/>
						<Stack direction="row" p={1}>
							<Typography variant="caption"> Buying From : </Typography>
							<CCDScanAccountLink account={listed.owner} />
						</Stack>
						<TextField
							name="buyAmount"
							label="Buy Amount"
							type="number"
							value={buyAmount}
							onChange={(e) => setBuyAmount(Number(e.target.value))}
							helperText={`Actual Amount ${actualBuyAmount}`}
						/>
						<TextField
							name="ownerAmount"
							label="Owner Amount"
							type="number"
							value={payAmount}
							disabled
							helperText={
								selectedPaymentToken
									? `In ${selectedPaymentToken.type}`
									: undefined
							}
						/>
						<TextField
							name="commissionAmount"
							label="Commission Amount"
							type="number"
							value={commissionAmount}
							disabled
							helperText={
								selectedPaymentToken
									? `In ${selectedPaymentToken.type}`
									: undefined
							}
						/>
						<TextField
							name="payAmount"
							label="Pay Amount"
							type="number"
							value={totalPayAmount}
							disabled
							helperText={
								selectedPaymentToken
									? `In ${selectedPaymentToken.type}`
									: undefined
							}
						/>
					</Stack>
				</Grid>
				<Grid item xs={12} md={2}></Grid>
				<Grid item xs={12} md={10}>
					<Stack p={2} direction="row" spacing={2}>
						{selectedPaymentToken &&
							{
								Cis2: (
									<>
										<SendTransactionButton
											disabled={!totalPayAmount}
											onClick={() =>
												buyViaToken(
													listed,
													selectedPaymentToken as Cis2PaymentToken,
													totalPayAmount,
													actualBuyAmount,
													true,
												)
											}
										>
											Pay By Token Via Transfer&nbsp;${totalPayAmount}
										</SendTransactionButton>
									</>
								),
								Ccd: (
									<SendTransactionButton
										disabled={!totalPayAmount}
										onClick={() =>
											buyViaCcd(
												listed,
												selectedPaymentToken.rate!,
												totalPayAmount,
												actualBuyAmount,
											)
										}
									>
										Pay By CCD&nbsp;${totalPayAmount}
									</SendTransactionButton>
								),
							}[selectedPaymentToken.type]}
						<Button onClick={() => navigate(-1)}>Cancel</Button>
					</Stack>
					{error && <ErrorDisplay text={error} />}
				</Grid>
			</Grid>
		</Paper>
	);
}
