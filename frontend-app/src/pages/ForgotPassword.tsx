"use client";
import { useState } from "react";
import { useForm, SubmitHandler } from "react-hook-form";
import Button from "../components/Button";
import Vector from "../assets/Vector.svg";
import greenTickIcon from "../assets/green-tick.svg";
import redCrossIcon from "../assets/red-cross.svg";
import { CognitoUser } from "amazon-cognito-identity-js";
import cognitoUserPool from "../lib/cognitoUserPool";

interface EmailFormInputs {
	email: string;
}

interface ConfirmCodeFormInputs {
	code: string;
	password: string; // Added new password field
}

export default function ForgotPassword() {
	const [currentState, setCurrentState] = useState<"email" | "code" | "success" | "error">("email");
	const [error, setError] = useState<string>();
	const [reSubmitting, setResubmitting] = useState(false);

	const {
		register: registerEmail,
		handleSubmit: handleSubmitEmail,
		formState: { errors: emailErrors },
		watch,
	} = useForm<EmailFormInputs>();
	const email = watch("email");

	const {
		register: registerCode,
		handleSubmit: handleSubmitCode,
		formState: { errors: codeErrors },
	} = useForm<ConfirmCodeFormInputs>();

	const onSubmitEmail: SubmitHandler<EmailFormInputs> = (data) => {
		new CognitoUser({
			Pool: cognitoUserPool,
			Username: data.email,
		}).forgotPassword({
			onSuccess: () => {
				setCurrentState("success");
			},
			onFailure: (err) => {
				setError(err.message);
			},
			inputVerificationCode: () => {
				setCurrentState("code");
			},
		});
	};

	const onSubmitCode: SubmitHandler<ConfirmCodeFormInputs> = (data) => {
		if (reSubmitting) return;
		if (!email) return;
		setResubmitting(true);
		new CognitoUser({
			Pool: cognitoUserPool,
			Username: email,
		}).confirmPassword(data.code, data.password, {
			onSuccess: () => {
				setCurrentState("success");
				setResubmitting(false);
			},
			onFailure: (err) => {
				setError(err.message);
				setResubmitting(false);
			},
		});
	};

	const resendEmail = () => {
		if (reSubmitting) return;
		if (!email) return;
		setResubmitting(true);
		onSubmitEmail({ email });
	};

	return (
		<>
			{
				{
					["email"]: (
						<div id="send-email">
							<form onSubmit={handleSubmitEmail(onSubmitEmail)}>
								<div className="heading">Forgot your password?</div>
								<div className="field mrbottom">
									{emailErrors.email && <p className="text-align-right error">{emailErrors.email.message}</p>}
									<input
										type="text"
										placeholder="Enter email address"
										className={`textField style2 ${emailErrors.email ? "error" : ""}`}
										{...registerEmail("email", {
											required: "Email is required",
										})}
									/>
								</div>
								<div className="field mrbottom">
									<Button style="style2" text="CONFIRM EMAIL" active type="submit" />
								</div>
							</form>
							<div className="left-text">
								If your email will be recognized in the system, you will recieve further instructions to reset your password in
								the email. If you don&apos;t see an email from Upwood, please check your spam folder. If you haven&apos;t
								recieved e-mail or forgot your e-mail address please{" "}
								<a href={`mailto:${import.meta.env.VITE_UPWOOD_SUPPORT_MAIL}`}>contact Upwood support</a>.
							</div>
							<div className="space-30"></div>
							<div className="container">
								<div className="container-in">
									<Button icon={Vector} text="BACK TO LOGIN" link="/login" active={false} />
								</div>
							</div>
						</div>
					),
					["code"]: (
						<div id="submit-confirm-code">
							<form onSubmit={handleSubmitCode(onSubmitCode)}>
								<div className="heading">Forgot your password?</div>
								<div className="field mrbottom">
									<input
										type="text"
										placeholder="Enter email address"
										readOnly
										className="textField style2"
										value={email || ""}
									/>
								</div>
								<div className="field mrbottom">
									{codeErrors.code && <p className="text-align-right error">{codeErrors.code?.message}</p>}
									<input
										type="password"
										placeholder="Enter confirmation code"
										className={`textField style2 ${codeErrors.code ? "error" : ""}`}
										{...registerCode("code", {
											required: "Code is required",
										})}
									/>
								</div>
								<div className="field mrbottom">
									{" "}
									{/* Added new password field */}
									{codeErrors.password && <p className="text-align-right error">{codeErrors.password?.message}</p>}
									<input
										type="password"
										placeholder="Enter new password"
										className={`textField style2 ${codeErrors.password ? "error" : ""}`}
										{...registerCode("password", {
											required: "Password is required",
										})}
									/>
								</div>
								<div className="field mrbottom">
									<Button style="style2" text="CONFIRM CODE" active type="submit" />
								</div>
							</form>
							<div className="left-text">
								If your email will be recognized in the system, you will recieve further instructions to reset your password in
								the email. If you don&apos;t see an email from Upwood, please check your spam folder. If you haven&apos;t
								recieved e-mail{" "}
								<a
									href="#"
									onClick={(e) => {
										e.preventDefault();
										resendEmail();
									}}
								>
									try resending
								</a>{" "}
								&nbsp; please or &nbsp;
								<a href={`mailto:${import.meta.env.VITE_UPWOOD_SUPPORT_MAIL}`}>contact Upwood support</a>.
							</div>
							<div className="space-30"></div>
							<div className="container">
								<div className="container-in">
									<Button icon={Vector} text="BACK TO LOGIN" link="/login" active={false} />
								</div>
							</div>
						</div>
					),
					["success"]: (
						<div>
							<div className="image">
								<img src={greenTickIcon} alt="" />
							</div>
							<div className="center-text biger bold">Success</div>
							<div className="space-30"></div>
							<div className="center-text">Your password has been reset successfully.</div>
							<br />
							<div className="container">
								<div className="container-in">
									<Button icon={Vector} text="BACK TO LOGIN" link="/login" active={false} />
								</div>
							</div>
						</div>
					),
					["error"]: (
						<div>
							<div className="image">
								<img src={redCrossIcon} alt="" />
							</div>
							<div className="center-text biger bold">Error</div>
							<div className="space-30"></div>
							<div className="center-text">There was an error resetting your password. {error}</div>
							<br />
							<div className="container">
								<div className="container-in">
									<Button icon={Vector} text="BACK TO LOGIN" link="/login" active={false} />
								</div>
							</div>
						</div>
					),
				}[currentState]
			}
			<div className="divider"></div>
		</>
	);
}
