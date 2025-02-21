import { useState, useEffect, useCallback } from "react";
import { useForm, Controller } from "react-hook-form";
import closeIcon from "../assets/close.svg";
import Button from "./Button";
import { signMessage, TxnStatus, updateContract } from "../lib/concordium";
import {
	ForestProject,
	ForestProjectService,
	ForestProjectTokenContract,
	Market,
	SystemContractsConfigApiModel,
	TokenMetadata,
	UserService,
} from "../apiClient";
import { User } from "../lib/user";
import euroeStablecoin from "../contractClients/generated/euroeStablecoin";
import concordiumNodeClient from "../contractClients/ConcordiumNodeClient";
import { AccountAddress, CcdAmount, ContractAddress } from "@concordium/web-sdk";
import securitySftMulti from "../contractClients/generated/securitySftMulti";
import { toDisplayAmount, toTokenId } from "../lib/conversions";
import securityP2PTrading from "../contractClients/generated/securityP2PTrading";
import greenTickIcon from "../assets/green-tick.svg";

export interface MarketBuyProps {
	user: User;
	contracts: SystemContractsConfigApiModel;
	project: ForestProject;
	market: Market;
	tokenContract?: ForestProjectTokenContract;
	currencyMetadata?: TokenMetadata;
	supply: string;
	legalContractSigned: boolean;
	userNotified: boolean;
	close?: () => void;
}

interface NotifyFormData {
	investmentAmount: number;
	terms: boolean;
}

export default function MarketBuy({
	close,
	user,
	contracts,
	market,
	tokenContract,
	currencyMetadata,
	project,
	supply,
	legalContractSigned,
	userNotified,
}: MarketBuyProps) {
	const handleKeyDown = useCallback(
		(e: KeyboardEvent) => {
			if (e.key === "Escape" && close) {
				close();
			}
		},
		[close],
	);
	const handleOverlayClick = (e: React.MouseEvent) => {
		e.stopPropagation();
		if (close) {
			close();
		}
	};
	useEffect(() => {
		document.addEventListener("keydown", handleKeyDown);
		return () => {
			document.removeEventListener("keydown", handleKeyDown);
		};
	}, [handleKeyDown]);

	const [popupState, setPopupState] = useState<"buy" | "notify" | "bought">("buy");
	const [price, setPrice] = useState<bigint>(BigInt(0));
	const [totalPayment, setTotalPayment] = useState(BigInt(0));
	const [euroeBalanceBuyer, setEuroeBalanceBuyer] = useState(BigInt(0));
	const [tokenBalanceSeller, setTokenBalanceSeller] = useState(BigInt(0));
	const [txnStatus, setTxnStatus] = useState<TxnStatus>("none");
	const [contractSigned, setContractSigned] = useState(legalContractSigned);
	const [isUserNotified, setIsUserNotified] = useState(userNotified);
	const [isBuying, setIsBuying] = useState(false);

	const {
		control,
		handleSubmit,
		formState: { errors, isValid },
		setError,
		setValue,
		clearErrors,
		watch,
	} = useForm<NotifyFormData>({
		defaultValues: {
			terms: contractSigned,
		},
	});

	const buyer = user.concordiumAccountAddress;
	const seller = market.liquidity_provider;

	useEffect(() => {
		euroeStablecoin.balanceOf
			.invoke(
				concordiumNodeClient,
				ContractAddress.create(BigInt(contracts.euro_e_contract_index), BigInt(0)),
				[
					{
						token_id: "",
						address: { Account: [buyer] },
					},
				],
				AccountAddress.fromBase58(buyer),
			)
			.then((response) => euroeStablecoin.balanceOf.parseReturnValue(response.returnValue!)!)
			.then((balance) => {
				setEuroeBalanceBuyer(BigInt(balance[0]));
			});
	}, [contracts, buyer]);

	useEffect(() => {
		setPrice(BigInt(market.sell_rate_numerator) / BigInt(market.sell_rate_denominator));
		securitySftMulti.balanceOf
			.invoke(
				concordiumNodeClient,
				ContractAddress.create(BigInt(market.token_contract_address), BigInt(0)),
				[
					{
						token_id: toTokenId(Number(market.token_id), 8),
						address: { Account: [seller] },
					},
				],
				AccountAddress.fromBase58(seller),
			)
			.then((response) => securitySftMulti.balanceOf.parseReturnValue(response.returnValue!)!)
			.then((balance) => {
				setTokenBalanceSeller(BigInt(balance[0]));
			});
	}, [market, seller]);

	useEffect(() => {
		if (tokenBalanceSeller === BigInt(0)) {
			setPopupState("notify");
		} else {
			setPopupState("buy");
		}
	}, [tokenBalanceSeller]);

	const onSubmit = async (data: NotifyFormData) => {
		setIsBuying(true);
		try {
			const isOperator = await euroeStablecoin.operatorOf
				.invoke(
					concordiumNodeClient,
					ContractAddress.create(BigInt(market.currency_token_contract_address), BigInt(0)),
					[
						{
							owner: { Account: [buyer] },
							address: {
								Contract: [
									{
										index: Number(market.contract_address),
										subindex: 0,
									},
								],
							},
						},
					],
					AccountAddress.fromBase58(buyer),
					CcdAmount.fromCcd(0),
				)
				.then((response) => euroeStablecoin.operatorOf.parseReturnValue(response.returnValue!)!)
				.then((response) => response[0]);
			if (!isOperator) {
				await updateContract(
					buyer,
					market.currency_token_contract_address,
					euroeStablecoin.updateOperator,
					[
						{
							update: {
								Add: {},
							},
							operator: {
								Contract: [
									{
										index: Number(market.contract_address),
										subindex: 0,
									},
								],
							},
						},
					],
					setTxnStatus,
				);
			}
			await updateContract(
				buyer,
				market.contract_address,
				securityP2PTrading.buy,
				{
					rate: { numerator: BigInt(market.sell_rate_numerator), denominator: BigInt(market.sell_rate_denominator) },
					token: {
						contract: { index: Number(market.token_contract_address), subindex: 0 },
						id: toTokenId(Number(market.token_id), 8),
					},
					amount: data.investmentAmount.toString(),
				},
				setTxnStatus,
			);
			setPopupState("bought");
			setIsBuying(false);
		} catch {
			setIsBuying(false);
		}
	};

	const addProjectUserNotification = async () => {
		UserService.postUserNotifications(project.id).then(() => setIsUserNotified(true));
	};

	const handleInvestmentAmountChange = (value: number) => {
		clearErrors();
		const payment = BigInt(value) * price;
		setTotalPayment(payment);

		if (value > tokenBalanceSeller) {
			setError("investmentAmount", {
				message: "Insufficient liquidity",
			});
			setPopupState("notify");
		} else if (payment > euroeBalanceBuyer) {
			setError("investmentAmount", {
				message: "Insufficient Balance",
			});
			setPopupState("buy");
		} else if (tokenBalanceSeller === BigInt(0)) {
			setError("investmentAmount", {
				message: "Insufficient liquidity",
			});
			setPopupState("notify");
		} else {
			setPopupState("buy");
		}
	};

	const handleTermsChange = (checked: boolean) => {
		clearErrors("terms");
		if (checked && !legalContractSigned) {
			signMessage(user.concordiumAccountAddress, project.id)
				.then((sigs) => {
					return ForestProjectService.postForestProjectsLegalContractSign(project.id, sigs);
				})
				.then(() => {
					setContractSigned(true);
					setValue("terms", true);
				})
				.catch((e) => {
					console.error(e);
					setError("terms", { message: "Failed to sign contract" });
					setContractSigned(false);
					setValue("terms", false);
				});
		}
	};

	const investmentAmountWatch = watch("investmentAmount");

	return (
		<div className="popup-overlay" onClick={handleOverlayClick}>
			{popupState === "bought" ? (
				<div className="popup" onClick={(e) => e.stopPropagation()}>
					<img src={closeIcon} alt="Close icon" width={32} height={32} className="close" onClick={close} />
					<div className="heading">Congratulations!</div>
					<div className="message">
						<img src={greenTickIcon} width={100} height={100} />
						You have successfully purchased{" "}
						<span>{toDisplayAmount(investmentAmountWatch.toString(), tokenContract?.decimals || 0)}</span> shares of the
						&quot;{project.name}&quot; forest plantation.
					</div>
					<div className="space-30"></div>
					<div className="container">
						<div className="container-in">
							<div className="col-12">
								<Button text="GO TO INVESTMENT PORTFOLIO" link={"/portfolio"} active />
							</div>
						</div>
					</div>
				</div>
			) : (
				<div className="popup" onClick={(e) => e.stopPropagation()}>
					<img src={closeIcon} alt="Close icon" width={32} height={32} className="close" onClick={close} />
					<div
						className="heading"
						title={`${tokenContract?.contract_address}-${toTokenId(Number(tokenContract?.market_token_id || "0"), 8)}`}
					>
						{popupState === "notify" ? "Notify me of available tokens" : "Buy shares"}
					</div>
					<div className="title">{project.name}</div>
					<div className="container">
						<div className="container-in">
							<div className="vis col-6 fl">
								<span className="colc">Price per share</span>
								<span className="colb">
									{toDisplayAmount(price.toString(), currencyMetadata?.decimals || 6, 2)}
									{currencyMetadata?.symbol}
								</span>
							</div>
							<div className="vis col-6 fl">
								<span className="colc">Share available</span>
								<span className="colb">
									{toDisplayAmount(tokenBalanceSeller.toString(), tokenContract?.decimals || 0, 0)}
									{tokenContract?.symbol}
								</span>
							</div>
							<div className="clr"></div>
						</div>
					</div>
					<div className="space-30"></div>
					<form onSubmit={handleSubmit(onSubmit)} id="notify-form">
						<div className="field">
							<label className="center">
								<span>Type amount of shares you want to buy</span>
							</label>
							<Controller
								name="investmentAmount"
								control={control}
								rules={{ required: "Amount is required", min: 1 }}
								render={({ field }) => (
									<input
										{...field}
										type="number"
										className={`textField center ${errors.investmentAmount ? "error" : ""}`}
										placeholder="Type the amount shares"
										max={tokenBalanceSeller.toString()}
										onChange={(e) => {
											field.onChange(e);
											handleInvestmentAmountChange(Number(e.target.value));
										}}
									/>
								)}
							/>
							<p className="text-align-center error">{errors.investmentAmount?.message}</p>
						</div>
						<div className="resu">
							<div className="center">
								If another token holder decides to sell their shares,
								<br />
								we will notify you promptly.
							</div>
						</div>
						<div className="space-30"></div>
						<div className="field">
							<div className="checkbox">
								<Controller
									name="terms"
									control={control}
									rules={{ required: "You must accept the terms and conditions" }}
									render={({ field }) => (
										<input
											{...field}
											type="checkbox"
											id="terms"
											checked={contractSigned}
											value={contractSigned.toString()}
											required
											onChange={(e) => {
												handleTermsChange(e.target.checked);
											}}
										/>
									)}
								/>
								<label htmlFor="terms" className="center no-center-mobile">
									<span>
										Accept <a href="">BOND TERMS AND CONDITIONS</a> and <a href="">SUBSCRIPTION AGREEMENT</a>
									</span>
								</label>
							</div>
							{errors.terms && <p className="text-align-center error">{errors.terms.message}</p>}
						</div>
						<div className="space-30"></div>
						<div className="container">
							<div className="container-in">
								<div className="col-5 col-m-full col-mr-bottom-20 fl">
									<Button text="CLOSE" active={false} call={close} />
								</div>
								<div className="col-5 col-m-full fr">
									{popupState === "notify" ? (
										<Button text="NOTIFY ME" call={addProjectUserNotification} disabled={userNotified || isUserNotified} />
									) : (
										<Button text="BUY" active call={handleSubmit(onSubmit)} loading={isBuying} />
									)}
								</div>
								<div className="clr"></div>
							</div>
						</div>
					</form>
				</div>
			)}
		</div>
	);
}
