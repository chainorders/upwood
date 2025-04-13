import { useState, useEffect, useCallback } from "react";
import { useForm, Controller } from "react-hook-form";
import closeIcon from "../assets/close.svg";
import Button from "./Button";
import { signMessage, TxnStatus, updateContract } from "../lib/concordium";
import { ForestProject, ForestProjectService, ForestProjectTokenContract, UserService } from "../apiClient";
import { User } from "../lib/user";
import euroeStablecoin from "../contractClients/generated/euroeStablecoin";
import concordiumNodeClient from "../contractClients/ConcordiumNodeClient";
import { AccountAddress, CcdAmount, ContractAddress } from "@concordium/web-sdk";
import securitySftMulti from "../contractClients/generated/securitySftMulti";
import { toDisplayAmount, toTokenId } from "../lib/conversions";
import securityP2PTrading from "../contractClients/generated/securityP2PTrading";
import greenTickIcon from "../assets/green-tick.svg";

export interface TransferMarket {
	liquidity_provider: string;
	token_contract_address: string;
	token_id: string;
	contract_address: string;
	sell_rate_numerator: string;
	sell_rate_denominator: string;
	currency_token_contract_address: string;
	max_token_amount: string;
	max_currency_amount: string;
}

export interface MarketBuyProps {
	user: User;
	project: ForestProject;
	market: TransferMarket;
	tokenContract: ForestProjectTokenContract;
	supply: string;
	legalContractSigned: boolean;
	userNotified: boolean;
	close: () => void;
}

interface NotifyFormData {
	tokenAmount: number;
	terms: boolean;
}

export default function MarketBuy({
	close,
	user,
	market,
	tokenContract,
	project,
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
	const [euroeBalanceBuyer, setEuroeBalanceBuyer] = useState(BigInt(0));
	const [tokenBalanceLp, setTokenBalanceLp] = useState(BigInt(0));
	const [marketMaxTokenAmount] = useState<bigint>(BigInt(market.max_token_amount));
	const [_txnStatus, setTxnStatus] = useState<TxnStatus>("none");
	const [contractSigned, setContractSigned] = useState(legalContractSigned);
	const [isUserNotified, setIsUserNotified] = useState(userNotified);
	const [isBuying, setIsBuying] = useState(false);

	const {
		control,
		handleSubmit,
		formState: { errors },
		setError,
		setValue,
		clearErrors,
		watch,
	} = useForm<NotifyFormData>({
		defaultValues: {
			terms: contractSigned,
			tokenAmount: 1,
		},
		mode: "onChange",
	});

	const buyer = user.concordiumAccountAddress;
	const lp = market.liquidity_provider;

	useEffect(() => {
		euroeStablecoin.balanceOf
			.invoke(
				concordiumNodeClient,
				ContractAddress.create(BigInt(market.currency_token_contract_address), BigInt(0)),
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
			})
			.catch((error) => {
				console.error("Error fetching EuroE balance:", error);
			});
	}, [market.currency_token_contract_address, buyer]);

	useEffect(() => {
		setPrice(BigInt(market.sell_rate_numerator) / BigInt(market.sell_rate_denominator));
		securitySftMulti.balanceOf
			.invoke(
				concordiumNodeClient,
				ContractAddress.create(BigInt(market.token_contract_address), BigInt(0)),
				[
					{
						token_id: toTokenId(Number(market.token_id), 8),
						address: { Account: [lp] },
					},
				],
				AccountAddress.fromBase58(lp),
			)
			.then((response) => securitySftMulti.balanceOf.parseReturnValue(response.returnValue!)!)
			.then((balance) => {
				setTokenBalanceLp(BigInt(balance[0]));
			})
			.catch((error) => {
				console.error("Error fetching token balance:", error);
			});
	}, [market, lp]);

	const tokenAmount = watch("tokenAmount") || 0;
	const totalPayment = BigInt(price) * BigInt(tokenAmount);
	useEffect(() => {
		if (tokenAmount > Number(tokenBalanceLp)) {
			setPopupState("notify");
		} else {
			setPopupState("buy");
		}
	}, [tokenAmount, tokenBalanceLp, euroeBalanceBuyer, setError, marketMaxTokenAmount]);

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
					contract: {
						index: Number(market.token_contract_address),
						subindex: 0,
					},
					amount: data.tokenAmount.toString(),
				},
				setTxnStatus,
			);
			setPopupState("bought");
			setIsBuying(false);
		} catch (e) {
			console.error(e);
			setIsBuying(false);
		}
	};

	const addProjectUserNotification = async () => {
		UserService.postUserNotifications(project.id).then(() => setIsUserNotified(true));
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

	return (
		<div className="popup-overlay" onClick={handleOverlayClick}>
			{popupState === "bought" ? (
				<div className="popup" onClick={(e) => e.stopPropagation()}>
					<img src={closeIcon} alt="Close icon" width={32} height={32} className="close" onClick={close} />
					<div className="heading">Congratulations!</div>
					<div className="message">
						<img src={greenTickIcon} width={100} height={100} />
						You have successfully purchased <span>
							{toDisplayAmount(tokenAmount.toString(), tokenContract.decimals)}
						</span>{" "}
						shares of the &quot;
						{project.name}&quot; forest plantation.
					</div>
					<div className="space-30"></div>
					<div className="container">
						<div className="container-in">
							<div className="col-12">
								<Button text="GO TO INVESTMENT PORTFOLIO" link="/portfolio" active call={close} />
							</div>
						</div>
					</div>
				</div>
			) : (
				<div className="popup" onClick={(e) => e.stopPropagation()}>
					<img src={closeIcon} alt="Close icon" width={32} height={32} className="close" onClick={close} />
					<div className="heading" title={`${tokenContract.contract_address}`}>
						{popupState === "notify" ? "Notify me of available tokens" : "Buy shares"}
					</div>
					<div className="title">{project.name}</div>
					<div className="container">
						<div className="container-in">
							<div className="vis col-6 fl">
								<span className="colc">Price per share</span>
								<span className="colb">{toDisplayAmount(price.toString(), 6, 2)} €</span>
							</div>
							<div className="vis col-6 fl">
								<span className="colc">Share available</span>
								<span className="colb">
									{toDisplayAmount(market.max_token_amount.toString(), tokenContract.decimals, 0)}
									{tokenContract.symbol}
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
								name="tokenAmount"
								control={control}
								rules={{
									required: "Amount is required",
									min: {
										value: 1,
										message: "Minimum amount is 1",
									},
									validate: (value) => {
										if (isNaN(value)) {
											return "Invalid amount";
										}
										if (totalPayment > Number(euroeBalanceBuyer)) {
											return "Insufficient EuroE balance";
										}
										if (value > Number(marketMaxTokenAmount)) {
											return `Investment amount cannot exceed ${toDisplayAmount(
												marketMaxTokenAmount.toString(),
												tokenContract.decimals,
												0,
											)}`;
										}
										if (value > Number(tokenBalanceLp)) {
											return "Insufficient Liquidity";
										}
										if (value < 1) {
											return "Minimum amount is 1";
										}
										return true;
									},
								}}
								render={({ field }) => (
									<input
										{...field}
										type="number"
										className={`textField center ${errors.tokenAmount ? "error" : ""}`}
										placeholder="Type the amount shares"
									/>
								)}
							/>
							<p className="text-align-center error">{errors.tokenAmount?.message}</p>
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
