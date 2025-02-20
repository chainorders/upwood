import { useState, useEffect, useCallback } from "react";
import Button from "./Button";
import closeIcon from "../assets/close.svg";
import greenTickIcon from "../assets/green-tick.svg";
import redCrossIcon from "../assets/red-cross.svg";
import { ForestProjectTokenUserYieldClaim, SystemContractsConfigApiModel } from "../apiClient";
import { TxnStatus, updateContract } from "../lib/concordium";
import { User } from "../lib/user";
import securitySftMultiYielder from "../contractClients/generated/securitySftMultiYielder";
import { toDisplayAmount, toTokenId } from "../lib/conversions";
import securitySftSingle from "../contractClients/generated/securitySftSingle";

interface ClaimPopupProps {
	user: User;
	yieldsClaimable: ForestProjectTokenUserYieldClaim[];
	yieldsDisplay: {
		carbonCredits: string;
		euroE: string;
		eTrees: string;
	};
	contracts: SystemContractsConfigApiModel;
	close?: () => void;
	onClaimed: () => void;
}

export default function ClaimPopup({
	yieldsDisplay,
	close,
	yieldsClaimable,
	contracts,
	user,
	onClaimed,
}: ClaimPopupProps) {
	const handleKeyDown = useCallback(
		(e: KeyboardEvent) => {
			if (e.key === "Escape" && close) {
				close();
			}
		},
		[close],
	);
	const handleOverlayClick = (e: React.MouseEvent<HTMLDivElement, MouseEvent>) => {
		if (e.target === e.currentTarget && close) {
			close();
		}
	};
	useEffect(() => {
		document.addEventListener("keydown", handleKeyDown);
		return () => {
			document.removeEventListener("keydown", handleKeyDown);
		};
	}, [handleKeyDown]);

	const [yieldTxnStatus, setYieldTxnStatus] = useState<TxnStatus>("none");
	const [burnTxnStatus, setBurnTxnStatus] = useState<TxnStatus>("none");
	const [popupState, setPopupState] = useState<"claim" | "burn" | "success" | "error">("claim");
	const claimYields = async () => {
		if (!yieldsClaimable.length) {
			return;
		}

		try {
			await updateContract(
				user.concordiumAccountAddress,
				contracts.yielder_contract_index,
				securitySftMultiYielder.yieldFor,
				{
					owner: user.concordiumAccountAddress,
					yields: yieldsClaimable.map((y) => ({
						token_ver_from: toTokenId(BigInt(y.token_id), 8),
						token_ver_to: toTokenId(BigInt(y.max_token_id), 8),
						token_contract: { index: Number(y.token_contract_address), subindex: 0 },
						amount: y.token_balance,
					})),
				},
				setYieldTxnStatus,
			);

			if (Number(yieldsDisplay.carbonCredits) > 0) {
				setPopupState("burn");
			} else {
				setPopupState("success");
			}
		} catch (e) {
			console.error(e);
			setPopupState("error");
		}
	};

	const burnCarbonCredits = async () => {
		if (!yieldsDisplay.carbonCredits) {
			return;
		}

		try {
			await updateContract(
				user.concordiumAccountAddress,
				contracts.carbon_credit_contract_index,
				securitySftSingle.burn,
				[
					{
						token_id: "",
						amount: yieldsDisplay.carbonCredits,
						owner: { Account: [user.concordiumAccountAddress] },
					},
				],
				setBurnTxnStatus,
			);
			setPopupState("success");
		} catch (e) {
			console.error(e);
			setPopupState("error");
		}
	};

	return (
		<div className="popup-overlay" onClick={handleOverlayClick}>
			<div className="popup" onClick={(e) => e.stopPropagation()}>
				<img src={closeIcon} alt="Close icon" width={32} height={32} className="close" onClick={close} />
				{
					{
						claim: (
							<>
								<div className="heading">Claim Yields</div>
								<div className="cl-area">
									<div className="container">
										<div className="container-in">
											<div className="col-4 fl col-m-full">
												<div className="cl-in">
													<div className="tag">Carbon Credits</div>
													<div className="display">
														{toDisplayAmount(yieldsDisplay.carbonCredits, contracts.carbon_credit_metadata.decimals || 0, 0)}{" "}
													</div>
												</div>
											</div>
											<div className="col-4 fl col-m-full">
												<div className="cl-in">
													<div className="tag">EuroE</div>
													<div className="display">
														{toDisplayAmount(yieldsDisplay.euroE, contracts.euro_e_metadata.decimals || 6, 0)}
													</div>
												</div>
											</div>
											<div className="col-4 fl col-m-full">
												<div className="cl-in">
													<div className="tag">E Trees</div>
													<div className="display">
														{toDisplayAmount(yieldsDisplay.eTrees, contracts.tree_ft_metadata.decimals || 0, 0)}
													</div>
												</div>
											</div>
											<div className="clr"></div>
											<Button
												text="CLAIM"
												call={claimYields}
												disabled={yieldTxnStatus === "sending" || yieldTxnStatus === "waiting" || !yieldsClaimable.length}
												active
												loading={yieldTxnStatus === "sending" || yieldTxnStatus === "waiting"}
											/>
										</div>
									</div>
								</div>
							</>
						),
						burn: (
							<>
								<div className="heading">Burn Carbon Credits?</div>
								<div className="cl-area">
									<div className="container">
										<div className="container-in">
											<div className="col-4 auto col-m-full">
												<div className="cl-in">
													<div className="tag">Carbon Credits</div>
													<div className="display">
														{toDisplayAmount(yieldsDisplay.carbonCredits, contracts.carbon_credit_metadata.decimals || 0, 0)}{" "}
													</div>
												</div>
											</div>
											<div className="clr"></div>
											<Button
												text="BURN"
												call={burnCarbonCredits}
												disabled={burnTxnStatus === "sending" || burnTxnStatus === "waiting" || !yieldsDisplay.carbonCredits}
												active
												loading={burnTxnStatus === "sending" || burnTxnStatus === "waiting"}
											/>
										</div>
									</div>
								</div>
							</>
						),
						success: (
							<>
								<div className="heading">Congratulations!</div>
								<div className="message">
									<img src={greenTickIcon} />
									You have successfully claimed your yields.
								</div>
								<div className="space-30"></div>
								<div className="container">
									<div className="container-in">
										<div className="col-12">
											<Button text="Close" call={onClaimed} />
										</div>
									</div>
								</div>
							</>
						),
						error: (
							<>
								<div className="heading">Error</div>
								<div className="message">
									<img src={redCrossIcon} />
									There was an error claiming your yields. <br />
									Please try again later.
								</div>
								<div className="space-30"></div>
								<div className="container">
									<div className="container-in">
										<div className="col-12">
											<Button text="Close" call={close} />
										</div>
									</div>
								</div>
							</>
						),
					}[popupState]
				}
			</div>
		</div>
	);
}
