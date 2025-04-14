import { useState, useEffect, useCallback } from "react";
import { useForm, Controller } from "react-hook-form";
import closeIcon from "../assets/close.svg";
import Button from "./Button";
import { signMessage, TxnStatus, updateContract } from "../lib/concordium";
import { ForestProject, ForestProjectService, ForestProjectTokenContract, UserService } from "../apiClient";
import { User } from "../lib/user";
import euroeStablecoin from "../contractClients/generated/euroeStablecoin";
import concordiumNodeClient from "../contractClients/ConcordiumNodeClient";
import { AccountAddress, ContractAddress } from "@concordium/web-sdk";
import securitySftMulti from "../contractClients/generated/securitySftMulti";
import { toDisplayAmount, toTokenId } from "../lib/conversions";
import securityP2PTrading from "../contractClients/generated/securityP2PTrading";
import greenTickIcon from "../assets/green-tick.svg";

export interface TransferMarket {
	liquidity_provider: string;
	token_contract_address: string;
	token_id: string;
	contract_address: string;
	buy_rate_numerator: string;
	buy_rate_denominator: string;
	currency_token_contract_address: string;
	max_currency_amount: string;
	max_token_amount: string;
}

export interface MarketSellProps {
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

export default function MarketSell({
	close,
	user,
	market,
	tokenContract,
	project,
	legalContractSigned,
	userNotified,
}: MarketSellProps) {
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

	const [popupState, setPopupState] = useState<"sell" | "notify" | "sold">("sell");
	const [price, setPrice] = useState(0);
	const [euroeBalanceLp, setEuroeBalanceBuyer] = useState(0);
	const [tokenBalanceSeller, setTokenBalanceSeller] = useState(0);
	const [, setTxnStatus] = useState<TxnStatus>("none");
	const [contractSigned, setContractSigned] = useState(legalContractSigned);
	const [isUserNotified, setIsUserNotified] = useState(userNotified);
	const [isSelling, setIsSelling] = useState(false);
	const [marketMaxCurrencyAmount] = useState(Number(market.max_currency_amount));

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

	const lp = market.liquidity_provider;
	const seller = user.concordiumAccountAddress;
	const tokenAmount = watch("tokenAmount") || 0;
	const totalPayment = Number(tokenAmount) * price;

	useEffect(() => {
		if (totalPayment > euroeBalanceLp) {
			setPopupState("notify");
		} else {
			setPopupState("sell");
		}
	}, [totalPayment, euroeBalanceLp]);

	useEffect(() => {
		euroeStablecoin.balanceOf
			.invoke(
				concordiumNodeClient,
				ContractAddress.create(Number(market.currency_token_contract_address)),
				[
					{
						token_id: "",
						address: { Account: [lp] },
					},
				],
				AccountAddress.fromBase58(lp),
			)
			.then((response) => euroeStablecoin.balanceOf.parseReturnValue(response.returnValue!)!)
			.then((balance) => {
				setEuroeBalanceBuyer(Number(balance[0]));
			});
	}, [market.currency_token_contract_address, lp]);

	useEffect(() => {
		setPrice(Number(market.buy_rate_numerator) / Number(market.buy_rate_denominator));
		securitySftMulti.balanceOf
			.invoke(
				concordiumNodeClient,
				ContractAddress.create(Number(market.token_contract_address)),
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
				setTokenBalanceSeller(Number(balance[0]));
			})
			.catch((error) => {
				console.error("Error fetching token balance:", error);
			});
	}, [market, seller]);

	useEffect(() => {
		if (tokenBalanceSeller === 0) {
			setPopupState("notify");
		} else {
			setPopupState("sell");
		}
	}, [tokenBalanceSeller]);

	const onSubmit = async (data: NotifyFormData) => {
		setIsSelling(true);
		try {
			await updateContract(
				seller,
				market.contract_address,
				securityP2PTrading.sell,
				{
					rate: { numerator: BigInt(market.buy_rate_numerator), denominator: BigInt(market.buy_rate_denominator) },
					contract: {
						index: Number(market.currency_token_contract_address),
						subindex: 0,
					},
					amount: data.tokenAmount.toString(),
				},
				setTxnStatus,
			);
			setPopupState("sold");
			setIsSelling(false);
		} catch (error) {
			console.error("Error during contract update:", error);
			setIsSelling(false);
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
			{popupState === "sold" ? (
				<div className="popup" onClick={(e) => e.stopPropagation()}>
					<img src={closeIcon} alt="Close icon" width={32} height={32} className="close" onClick={close} />
					<div className="heading">Congratulations!</div>
					<div className="message">
						<img src={greenTickIcon} width={100} height={100} />
						You have successfully sold <span>{toDisplayAmount(tokenAmount.toString(), tokenContract.decimals)}</span> shares
						of the &quot;
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
					<div className="heading">{popupState === "notify" ? "Notify me of available tokens" : "Sell shares"}</div>
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
									{toDisplayAmount(tokenBalanceSeller.toString(), tokenContract.decimals, 0)}
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
								<span>Type amount of shares you want to sell</span>
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
									validate: (value: number) => {
										if (isNaN(value)) {
											return "Invalid amount";
										}
										if (value > Number(tokenBalanceSeller)) {
											return `You can only sell up to ${toDisplayAmount(tokenBalanceSeller.toString(), tokenContract.decimals, 0)} ${tokenContract.symbol}`;
										}
										if (totalPayment > marketMaxCurrencyAmount) {
											return `You can only sell up to ${toDisplayAmount(
												marketMaxCurrencyAmount.toString(),
												6,
												2,
											)} € worth of shares`;
										}
										if (totalPayment > euroeBalanceLp) {
											return `Insufficient EuroE liquidity provider balance. You need at least ${toDisplayAmount(
												totalPayment.toString(),
												6,
												2,
											)} €`;
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
								If another token holder decides to buy your shares,
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
											autoComplete="off"
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
										<Button
											text="SELL"
											active
											call={handleSubmit(onSubmit)}
											loading={isSelling}
											disabled={market.buy_rate_numerator === undefined || market.buy_rate_denominator === undefined}
										/>
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
