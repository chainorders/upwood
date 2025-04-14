import { useState, useEffect, useCallback } from "react";
import { useForm, Controller } from "react-hook-form";
import closeIcon from "../assets/close.svg";
import Button from "./Button";
import euroeStablecoin from "../contractClients/generated/euroeStablecoin";
import concordiumNodeClient from "../contractClients/ConcordiumNodeClient";
import { AccountAddress, CcdAmount, ContractAddress } from "@concordium/web-sdk";
import {
	ForestProject,
	ForestProjectService,
	ForestProjectTokenContract,
	SecurityMintFund,
	SystemContractsConfigApiModel,
} from "../apiClient";
import { User } from "../lib/user";
import { toDisplayAmount, toTokenId } from "../lib/conversions";
import securityMintFund from "../contractClients/generated/securityMintFund";
import { signMessage, TxnStatus, updateContract } from "../lib/concordium";
import greenTickIcon from "../assets/green-tick.svg";
import { Link } from "react-router";

interface FundInvestProps {
	supply: string;
	user: User;
	contracts: SystemContractsConfigApiModel;
	project: ForestProject;
	fund: SecurityMintFund;
	tokenContract: ForestProjectTokenContract;
	legalContractSigned: boolean;
	close?: () => void;
}

interface InvestmentFormData {
	investmentAmount: number;
	terms: boolean;
}

export default function FundInvest({
	close,
	user,
	contracts,
	fund,
	tokenContract,
	project,
	legalContractSigned,
	supply,
}: FundInvestProps) {
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
	const [price, setPrice] = useState(0);
	const [euroeBalance, setEuroeBalance] = useState(0);
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
		},
		mode: "onChange",
	});

	useEffect(() => {
		euroeStablecoin.balanceOf
			.invoke(
				concordiumNodeClient,
				ContractAddress.create(Number(contracts.euro_e_contract_index)),
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
				setEuroeBalance(Number(balance[0]));
			})
			.catch((err) => {
				console.error("Error fetching balanceOf", err);
			});
	}, [contracts, user]);

	useEffect(() => {
		setPrice(Number(fund.rate_numerator) / Number(fund.rate_denominator));
	}, [fund]);

	const onSubmit = async () => {
		setIsInvesting(true);
		try {
			const isOperator = await euroeStablecoin.operatorOf
				.invoke(
					concordiumNodeClient,
					ContractAddress.create(Number(fund.currency_token_contract_address), 0),
					[
						{
							owner: { Account: [user.concordiumAccountAddress] },
							address: {
								Contract: [
									{
										index: Number(fund.contract_address),
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
					fund.currency_token_contract_address,
					euroeStablecoin.updateOperator,
					[
						{
							update: {
								Add: {},
							},
							operator: {
								Contract: [
									{
										index: Number(fund.contract_address),
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
				fund.contract_address,
				securityMintFund.transferInvest,
				{
					amount: totalPayment.toString(),
					security_token: {
						contract: {
							index: Number(fund.investment_token_contract_address),
							subindex: 0,
						},
						id: toTokenId(Number(fund.investment_token_id), 8),
					},
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

	const investmentAmountWatch = watch("investmentAmount") || 0;
	const totalPayment = Number(investmentAmountWatch) * price;
	const availableShares = Number(project.shares_available) - Number(supply);

	return (
		<div className="popup-overlay" onClick={handleOverlayClick}>
			{thankyou ? (
				<div className="popup" onClick={(e) => e.stopPropagation()}>
					<img src={closeIcon} alt="Close icon" width={32} height={32} className="close" onClick={close} />
					<div className="heading">Congratulations!</div>
					<div className="message">
						<img src={greenTickIcon} width={100} height={100} />
						You have successfully invested in{" "}
						<span>{toDisplayAmount(investmentAmountWatch.toString(), tokenContract.decimals)} shares</span> of the &quot;
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
									{toDisplayAmount(project.shares_available.toString(), tokenContract.decimals, 0)}
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
								name="investmentAmount"
								control={control}
								rules={{
									required: "Investment amount is required",
									min: { value: 1, message: "Invalid investment amount" },
									max: {
										value: availableShares,
										message: "Investment amount exceeds available shares",
									},
									validate: (value) => {
										const totalPayment = Number(value) * price;
										if (totalPayment > euroeBalance) {
											return "Insufficient Balance";
										}
										return true;
									},
								}}
								render={({ field }) => (
									<input
										{...field}
										type="number"
										id="investment-amount"
										required
										className={`textField ${errors.investmentAmount ? "error" : ""} center`}
										autoComplete="off"
									/>
								)}
							/>
							<p className="text-align-center error">{errors.investmentAmount?.message}</p>
						</div>
						<div className="resu">
							<div className="left col-m-full col-mr-bottom-20 fl">
								Get shares :{" "}
								<span>
									{toDisplayAmount(investmentAmountWatch.toString(), tokenContract.decimals, tokenContract.decimals)} Share
								</span>
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
