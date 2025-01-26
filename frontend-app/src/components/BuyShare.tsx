import { useState, useEffect, useCallback } from "react";
import closeIcon from "../assets/close.svg";
import Button from "./Button";

export interface BuyShareConfig {
	heading: string;
	title: string;
	share_price: bigint;
	share_available: bigint;
}

interface BuyProps {
	config: BuyShareConfig;
	close?: () => void;
}

export default function BuyShare({ config, close }: BuyProps) {
	const [thankyou, setThankYou] = useState(false);
	const [share] = useState(0);
	const [totalPayment] = useState(0);
	const [investmentAmount] = useState(0);

	const investButtonHit = () => {
		setThankYou(true);
	};

	// Memoized handler for key down to manage closure dependencies
	const handleKeyDown = useCallback(
		(e: KeyboardEvent) => {
			if (e.key === "Escape" && close) {
				close();
			}
		},
		[close],
	);

	// Close modal when overlay is clicked
	const handleOverlayClick = (e: React.MouseEvent) => {
		e.stopPropagation(); // Prevent click event from bubbling up if clicking inside modal content
		if (close) {
			close(); // Close the modal
		}
	};

	// Add event listener for Escape key press when the modal is mounted
	useEffect(() => {
		document.addEventListener("keydown", handleKeyDown);
		// Cleanup the event listener when the component unmounts
		return () => {
			document.removeEventListener("keydown", handleKeyDown);
		};
	}, [handleKeyDown]);

	return (
		<div className="popup-overlay" onClick={handleOverlayClick}>
			{thankyou ? (
				<div className="popup" onClick={(e) => e.stopPropagation()}>
					<img
						src="/Close.svg"
						alt="Close icon"
						// layout="intrinsic"
						width={32}
						height={32}
						className="close"
						onClick={close}
					/>
					<div className="heading">Congratulations!</div>
					<div className="message">
						<img
							src="/Group-1000003068.svg"
							// layout="intrinsic"
							width={100}
							height={100}
						/>
						You have successfully invested in <span>5 shares</span> of the &quot;Oaktree House&quot; forest plantation.
					</div>
					<div className="space-30"></div>
					<div className="container">
						<div className="container-in">
							<div className="col-12">
								<Button text={"GO TO INVESTMENT PORTFOLIO"} link={""} active={true} />
							</div>
						</div>
					</div>
				</div>
			) : (
				<div className="popup" onClick={(e) => e.stopPropagation()}>
					<img src={closeIcon} alt="Close icon" width={32} height={32} className="close" onClick={close} />
					<div className="heading">{config.heading}</div>
					<div className="title">{config.title}</div>
					<div className="container">
						<div className="container-in">
							<div className="vis col-6 fl">
								<span className="colc">Price per share</span>
								<span className="colb">{config.share_price.toString()}€</span>
							</div>
							<div className="vis col-6 fl">
								<span className="colc">Share available</span>
								<span className="colb">{config.share_available.toString()}</span>
							</div>
							<div className="clr"></div>
						</div>
					</div>
					<div className="space-30"></div>
					<div className="field">
						<label>
							<span>
								Investment amount <span className="fr">Balance - {investmentAmount} EUROe</span>
							</span>
						</label>
						<input className="textField error center" />
						<p className="text-align-center error">
							Your wallet balance is not sufficient to buy shares. Please add funds to your wallet.
						</p>
					</div>
					<div className="resu">
						<div className="left col-m-full col-mr-bottom-20 fl">
							Get shares : <span>{share} Share</span>
						</div>
						<div className="right col-m-full fr">
							Total payment : <span>{totalPayment} EUROe</span>
						</div>
						<div className="clr"></div>
					</div>
					<div className="space-30"></div>
					<div className="field">
						<div className="checkbox">
							<input type="checkbox" id="terms" />
							<label htmlFor="terms" className="center no-center-mobile">
								<span>
									Accept <a href="">BOND TERMS AND CONDITIONS</a> and <a href="">SUBSCRIPTION AGREEMENT</a>
								</span>
							</label>
						</div>
					</div>
					<div className="space-30"></div>
					<div className="container">
						<div className="container-in">
							<div className="col-5 col-m-full col-mr-bottom-20 fl">
								<Button text={"CLOSE"} link={""} active={false} call={close} />
							</div>
							<div className="col-5 col-m-full fr">
								<Button text={"INVEST"} link={""} active={true} call={investButtonHit} />
							</div>
							<div className="clr"></div>
						</div>
					</div>
				</div>
			)}
		</div>
	);
}
