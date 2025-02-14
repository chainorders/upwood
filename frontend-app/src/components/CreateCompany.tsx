import { useCallback, useEffect, useState } from "react";
import closeIcon from "../assets/close.svg";
import Button from "./Button";
import OtpInput from "./OtpInput";
interface PopupProps {
	close?: () => void;
}
export default function CreateCompany({ close = () => {} }: PopupProps) {
	const [emailOtpScreen, setEmailOtpScreen] = useState(false);
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
	return (
		<div className="popup-overlay" onClick={handleOverlayClick}>
			<div className="popup" onClick={(e) => e.stopPropagation()}>
				<>
					<img src={closeIcon} alt="Close icon" width={32} height={32} className="close" onClick={close} />
					<div className="heading">Create company</div>
					<div className="cl-area edo">
						<div className="container">
							<div className="container-in">
								<div className="space-20"></div>
								<div className="col-12">
									<div className="letter">J</div>
									<div className="links">
										<span>Download pictures</span>
									</div>
								</div>
								<div className="space-20"></div>
								<div className="col-12">
									<div className="head">Company data</div>
								</div>
								<div className="clr"></div>
								<div className="col-4 fl col-m-full">
									<div className="boxl">Company name</div>
								</div>
								<div className="col-8 fr col-m-full">
									<input type="text" placeholder="Enter company name" className="boxt" />
								</div>
								<div className="clr"></div>
								<div className="space-15"></div>
								<div className="clr"></div>
								<div className="col-4 fl col-m-full">
									<div className="boxl">Company email</div>
								</div>
								<div className="col-8 fr col-m-full">
									<input type="text" placeholder="Enter company email" className="boxt" />
								</div>
								<div className="clr"></div>
								<div className="space-30"></div>
								<div className="col-12">
									<div className="head">Password settings</div>
								</div>
								<div className="space-15"></div>
								<div className="clr"></div>
								<div className="col-4 fl col-m-full">
									<div className="boxl">Password</div>
								</div>
								<div className="col-8 fr col-m-full">
									<input type="password" placeholder="Enter password" className="boxt" />
								</div>
								<div className="clr"></div>
								<div className="space-15"></div>
								<div className="col-4 fl col-m-full">
									<div className="boxl">Confirm password</div>
								</div>
								<div className="col-8 fr col-m-full ">
									<input type="password" placeholder="Confirm password" className="boxt" />
								</div>
								<div className="clr"></div>
								<div className="space-30"></div>
								<div className="clr"></div>
								<div className="col-5 fl col-m-full col-mr-bottom-20">
									<Button text={"CLOSE"} link={""} active={false} call={() => close()} />
								</div>
								<div className="col-5 fr col-m-full">
									<Button text={"SAVE"} link={""} active={true} call={() => setEmailOtpScreen(true)} />
								</div>
								<div className="clr"></div>
							</div>
						</div>
					</div>
				</>
			</div>

			{emailOtpScreen && (
				<div className="popup" onClick={(e) => e.stopPropagation()}>
					<div className="heading">Email verification</div>
					<div className="cl-area edo">
						<div className="container">
							<div className="container-in">
								<div className="space-20"></div>
								<div className="col-12">
									<div className="head text-align-center">To register company, you need to confirm your email address</div>
									<div className="space-15"></div>
									<div className="text-align-center">We’ve sent a code to Jonh23@gmail.com</div>
									<div className="space-30"></div>
									<OtpInput />
								</div>
								<div className="clr"></div>
								<div className="col-5 fl col-m-full col-mr-bottom-20">
									<Button text={"CANCEL"} link={""} active={false} call={() => setEmailOtpScreen(false)} />
								</div>
								<div className="col-5 fr col-m-full">
									<Button text={"CONFIRM"} link={""} active={true} />
								</div>
								<div className="clr"></div>
							</div>
						</div>
					</div>
				</div>
			)}
		</div>
	);
}
