"use client";
import { useState } from "react";
import Button from "../components/Button";
import Vector from "../assets/Vector.svg";
import greenTickIcon from "../assets/green-tick.svg";
export default function ForgotPassword() {
	const [forgotSuccess, setforgotSuccess] = useState(false);
	const whenconfirmbuttonhit = () => {
		setforgotSuccess(true);
	};
	return (
		<>
			{forgotSuccess ? (
				<div>
					<div className="image">
						<img src={greenTickIcon} alt="" />
					</div>
					<div className="center-text biger bold">Success</div>
					<div className="space-30"></div>
					<div className="center-text">
						Please check your email
						<br />
						for create a new password
					</div>
					<div className="space-30"></div>
					<div className="center-text big">
						Can&apos;t get email? <a href="">Resubmit</a>
					</div>
					<div className="space-30"></div>
					<div>
						<Button icon={Vector} text={"BACK TO LOGIN"} link={"/login"} active={false} />
					</div>
					<div className="divider"></div>
				</div>
			) : (
				<div>
					<div className="heading">Forgot your password?</div>
					<div className="field mrbottom">
						<p className="text-align-right error">Email is not recognized in our system, please contact support!</p>
						<input type="text" placeholder="Enter email address" className="textField style2 error" />
					</div>
					<div className="field mrbottom">
						<Button style={"style2"} text={"CONFIRM"} link={""} active={true} call={whenconfirmbuttonhit} />
					</div>
					<div className="left-text">
						If your email will be recognized in the system, you will recieve further instructions to reset your password in
						the email. If you don&apos;t see an email from Upwood, please check your spam folder. If you haven&apos;t recieved
						e-mail or forgot your e-mail address please contact Upwood support.
					</div>
					<div className="space-30"></div>
					<div className="container">
						<div className="container-in">
							<div className="col-6 fl col-m-full col-mr-bottom-20">
								<Button icon={Vector} text={"BACK TO LOGIN"} link={"/login"} active={false} />
							</div>
							<div className="col-6 fr col-m-full">
								<Button style={"style3"} text={"CONTACT SUPPORT"} link={""} active={false} />
							</div>
							<div className="clr"></div>
						</div>
					</div>
					<div className="divider"></div>
				</div>
			)}
		</>
	);
}
