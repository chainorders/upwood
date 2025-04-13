import { useState, useEffect, useCallback } from "react";
import { useForm, Controller } from "react-hook-form";
import closeIcon from "../assets/close.svg";
import Button from "./Button";
import euroeStablecoin from "../contractClients/generated/euroeStablecoin";
import concordiumNodeClient from "../contractClients/ConcordiumNodeClient";
import { AccountAddress, CcdAmount, ContractAddress } from "@concordium/web-sdk";
import { ForestProject, ForestProjectService, ForestProjectTokenContract } from "../apiClient";
import { User } from "../lib/user";
import { toDisplayAmount } from "../lib/conversions";
import { signMessage, TxnStatus, updateContract } from "../lib/concordium";
import greenTickIcon from "../assets/green-tick.svg";
import { Link } from "react-router";
import securityP2PTrading from "../contractClients/generated/securityP2PTrading";

export interface MintMarket {
	liquidity_provider: string;
	token_contract_address: string;
	contract_address: string;
	sell_rate_numerator: string;
	sell_rate_denominator: string;
	currency_token_contract_address: string;
	max_token_amount: string;
}

interface MarketBuyMintProps {
	user: User;
	project: ForestProject;
	market: MintMarket;
	tokenContract: ForestProjectTokenContract;
	supply: string;
	legalContractSigned: boolean;
	close?: () => void;
}

interface InvestmentFormData {
	tokenAmount: number;
	terms: boolean;
}

export default function MarketBuyMint({
	close,
	user,
	tokenContract,
	project,
	legalContractSigned,
	market,
}: MarketBuyMintProps) {
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

	const [thankyou, setThankYou] = useState(false);
	const [price, setPrice] = useState<bigint>(BigInt(0));
	const [euroeBalance, setEuroeBalance] = useState(BigInt(0));
	const [_txnStatus, setTxnStatus] = useState<TxnStatus>("none");
	const [contractSigned, setContractSigned] = useState(legalContractSigned);
	const [isInvesting, setIsInvesting] = useState(false);

	const {
		control,
		handleSubmit,
		formState: { errors },
		setError,
		setValue,
		clearErrors,
		watch,
	} = useForm<InvestmentFormData>({
		defaultValues: {
			terms: contractSigned,
			tokenAmount: 1,
		},
		mode: "onChange",
	});

	useEffect(() => {
		euroeStablecoin.balanceOf
			.invoke(
				concordiumNodeClient,
				ContractAddress.create(BigInt(market.currency_token_contract_address), BigInt(0)),
				[
					{
						token_id: "",
						address: { Account: [user.concordiumAccountAddress] },
					},
				],
				AccountAddress.fromBase58(user.concordiumAccountAddress),
				CcdAmount.fromCcd(0),
			)
			.then((response) => euroeStablecoin.balanceOf.parseReturnValue(response.returnValue!)!)
			.then((balance) => {
				setEuroeBalance(BigInt(balance[0]));
			});
	}, [market.currency_token_contract_address, user]);

	useEffect(() => {
		setPrice(BigInt(market.sell_rate_numerator) / BigInt(market.sell_rate_denominator));
	}, [market.sell_rate_numerator, market.sell_rate_denominator]);

	const tokenAmount = watch("tokenAmount") || 0;
	const totalPayment = BigInt(tokenAmount) * price;

	const onSubmit = async (data: InvestmentFormData) => {
		setIsInvesting(true);
		try {
			const isOperator = await euroeStablecoin.operatorOf
				.invoke(
					concordiumNodeClient,
					ContractAddress.create(BigInt(market.currency_token_contract_address), BigInt(0)),
					[
						{
							owner: { Account: [user.concordiumAccountAddress] },
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
					AccountAddress.fromBase58(user.concordiumAccountAddress),
					CcdAmount.fromCcd(0),
				)
				.then((response) => euroeStablecoin.operatorOf.parseReturnValue(response.returnValue!)!)
				.then((response) => response[0]);
			if (!isOperator) {
				await updateContract(
					user.concordiumAccountAddress,
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
				user.concordiumAccountAddress,
				market.contract_address,
				securityP2PTrading.mint,
				{
					rate: {
						numerator: BigInt(market.sell_rate_numerator),
						denominator: BigInt(market.sell_rate_denominator),
					},
					token_contract: {
						index: Number(market.token_contract_address),
						subindex: 0,
					},
					amount: data.tokenAmount.toString(),
				},
				setTxnStatus,
			);
			setThankYou(true);
			setIsInvesting(false);
		} catch (e) {
			console.error(e);
			setIsInvesting(false);
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

	return (
		<div className="popup-overlay" onClick={handleOverlayClick}>
			{thankyou ? (
				<div className="popup" onClick={(e) => e.stopPropagation()}>
					<img src={closeIcon} alt="Close icon" width={32} height={32} className="close" onClick={close} />
					<div className="heading">Congratulations!</div>
					<div className="message">
						<img src={greenTickIcon} width={100} height={100} />
						You have successfully invested in{" "}
						<span>{toDisplayAmount(tokenAmount.toString(), tokenContract.decimals)} shares</span> of the &quot;
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
					<div className="heading">Buy shares</div>
					<div className="title">{project.name}</div>
					<div className="container">
						<div className="container-in">
							<div className="vis col-6 fl">
								<span className="colc">Price per share</span>
								<span className="colb" title={price.toString()}>
									{toDisplayAmount(price.toString(), 6, 2)}
								</span>
							</div>
							<div className="vis col-6 fl">
								<span className="colc">Share available</span>
								<span className="colb">
									{toDisplayAmount(market.max_token_amount.toString(), tokenContract.decimals, 0)}
									&nbsp;{tokenContract.symbol}
								</span>
							</div>
							<div className="clr"></div>
						</div>
					</div>
					<div className="space-30"></div>
					<form onSubmit={handleSubmit(onSubmit)} id="investment-form">
						<div className="field">
							<label>
								<span>
									Investment amount{" "}
									<span className="fr" title={`Account Address: ${user.concordiumAccountAddress}`}>
										Balance - {toDisplayAmount(euroeBalance.toString(), 6)}
									</span>
								</span>
							</label>
							<Controller
								name="tokenAmount"
								control={control}
								rules={{
									required: "Investment amount is required",
									validate: (value) => {
										if (value > Number(market.max_token_amount)) {
											return `Investment amount cannot exceed ${toDisplayAmount(
												market.max_token_amount.toString(),
												tokenContract.decimals,
											)}`;
										}
										if (totalPayment > euroeBalance) {
											console.error("Insufficient balance");
											return "Insufficient Balance";
										}
										if (value < 1) {
											return "Invalid investment amount";
										}

										if (value % 1 !== 0) {
											return "Investment amount must be a whole number";
										}
										if (value < 0) {
											return "Investment amount must be a positive number";
										}
										return true;
									},
								}}
								render={({ field, fieldState }) => (
									<input
										{...field}
										type="number"
										id="investment-amount"
										required
										className={`textField ${fieldState.error ? "error" : ""} center`}
										autoComplete="off"
									/>
								)}
							/>
							<p className="text-align-center error">{errors.tokenAmount?.message}</p>
						</div>
						<div className="resu">
							<div className="left col-m-full col-mr-bottom-20 fl">
								Get shares :{" "}
								<span>{toDisplayAmount(tokenAmount.toString(), tokenContract.decimals, tokenContract.decimals)} Share</span>
							</div>
							<div className="right col-m-full fr">
								Total payment : <span>{toDisplayAmount(totalPayment.toString(), 6)} EUROe</span>
							</div>
							<div className="clr"></div>
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
										Accept <Link to={`/contracts/${project.id}`}>BOND TERMS AND CONDITIONS</Link> and{" "}
										<Link to={`/contracts/${project.id}`}>SUBSCRIPTION AGREEMENT</Link>
									</span>
								</label>
							</div>
							{errors.terms && <p className="text-align-center error">{errors.terms.message}</p>}
						</div>
						<div className="space-30"></div>
						<div className="container">
							<div className="container-in">
								<div className="col-5 col-m-full col-mr-bottom-20 fl">
									<Button text="CLOSE" call={close} />
								</div>
								<div className="col-5 col-m-full fr">
									<Button text="INVEST" active call={handleSubmit(onSubmit)} loading={isInvesting} />
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
