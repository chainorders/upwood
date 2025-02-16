import { useCallback, useEffect, useState } from "react";
import { useForm, SubmitHandler, Controller } from "react-hook-form";
import closeIcon from "../assets/close.svg";
import Button from "./Button";
import { useOutletContext } from "react-router";
import AccountCross from "../assets/account-not-protected.svg";
import AccountProtected from "../assets/account-protected.svg";
import editRow from "../assets/editRow.svg";
import saveRow from "../assets/saveRow.svg";
import Avatar from "../assets/Avatar.svg";
import OtpInput from "./OtpInput";
import { User } from "../lib/user";

interface PopupProps {
	close?: () => void;
}

interface FullNameFormInputs {
	fullName: string;
}

interface EmailFormInputs {
	email: string;
}

interface PasswordFormInputs {
	current_password: string;
	new_password: string;
	confirm_password: string;
}

interface OtpFormInputs {
	otp: string;
}

export default function EditProfile({ close = () => {} }: PopupProps) {
	const { user } = useOutletContext<{ user: User }>();
	const {
		register: registerFullName,
		handleSubmit: handleSubmitFullName,
		formState: { errors: fullNameErrors },
		watch: watchFullName,
	} = useForm<FullNameFormInputs>({
		defaultValues: {
			fullName: user.fullName,
		},
	});
	const {
		register: registerEmail,
		handleSubmit: handleSubmitEmail,
		formState: { errors: emailErrors },
		watch: watchEmailForm,
	} = useForm<EmailFormInputs>({
		defaultValues: {
			email: user.email,
		},
	});
	const {
		register: registerPassword,
		handleSubmit: handleSubmitPassword,
		formState: { errors: passwordErrors, isValid: isPasswordFormValid },
		setError: setPasswordError,
		reset: resetPasswordForm,
	} = useForm<PasswordFormInputs>();

	const {
		register: registerOtp,
		handleSubmit: handleSubmitOtp,
		control: controlOtp,
		formState: { errors: otpErrors, isValid },
		setError: setOtpError,
	} = useForm<OtpFormInputs>();

	const [fullNameEdit, setFullNameEdit] = useState(false);
	const [emailEdit, setEmailEdit] = useState(false);
	const [emailEditOTPScreen, setEmailEditOtpScreen] = useState<{ email: string }>();
	const [forgotScreen, setForgotScreen] = useState(false);
	const [enable2FaScreen, set2FAScreen] = useState(false);
	const [disable2FaScreen, set2FAScreenDisable] = useState(false);
	const [settingPassword, setSettingPassword] = useState(false);

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

	const onSubmitFullName: SubmitHandler<FullNameFormInputs> = ({ fullName }) => {
		const nameParts = fullName.split(" ");
		const givenName = nameParts[0];
		const familyName = nameParts[nameParts.length - 1];
		const middleName = nameParts.length > 2 ? nameParts.slice(1, -1).join(" ") : "";

		user.cognitoUser.updateAttributes(
			[
				{ Name: "family_name", Value: familyName },
				{ Name: "given_name", Value: givenName },
				...(middleName ? [{ Name: "middle_name", Value: middleName }] : []),
			],
			(err) => {
				if (err) {
					console.log(err);
					setFullNameEdit(false);
					return;
				}
				user.fullName = fullName;
			},
		);
		setFullNameEdit(false);
	};

	const onSubmitEmail: SubmitHandler<EmailFormInputs> = ({ email }) => {
		user.cognitoUser.updateAttributes([{ Name: "email", Value: email }], (err) => {
			if (err) {
				console.log(err);
				setEmailEdit(false);
				setEmailEditOtpScreen(undefined);
				return;
			}
			setEmailEditOtpScreen({ email });
		});
	};

	const onSubmitPassword: SubmitHandler<PasswordFormInputs> = (data) => {
		setSettingPassword(true);
		user.cognitoUser.changePassword(data.current_password, data.new_password, (err) => {
			setSettingPassword(false);
			if (!err) {
				resetPasswordForm();
			} else {
				console.log(err);
				setPasswordError("root", { message: "Error changing password" });
			}
		});
	};

	const onSubmitOtp: SubmitHandler<OtpFormInputs> = (data) => {
		console.log("OTP data:", data);
		user.cognitoUser.verifyAttribute("email", data.otp, {
			onSuccess: () => {
				setEmailEditOtpScreen(undefined);
				setEmailEdit(false);
				user.email = watchEmailForm("email");
			},
			onFailure: (err) => {
				setOtpError("otp", { message: err.message });
			},
		});
	};

	const onEnableMfaClicked = async () => {
		console.log("Enabling MFA");
		// user.cognitoUser.setUserMfaPreference({});
		// user.cognitoUser.enableMFA((err) => {
		// 	if (err) {
		// 		console.log(err);
		// 		return;
		// 	}
		// 	user.mfaEnabled = true;
		// });
	};

	const onDisableMfaClicked = async () => {
		console.log("Disabling MFA");
		user.cognitoUser.disableMFA((err) => {
			if (err) {
				console.log(err);
				return;
			}
			user.mfaEnabled = false;
		});
	};

	return (
		<div className="popup-overlay" onClick={handleOverlayClick}>
			<div className="popup" onClick={(e) => e.stopPropagation()}>
				<>
					<img src={closeIcon} alt="Close icon" width={32} height={32} className="close" onClick={close} />
					<div className="heading">Edit profile</div>
					<div className="cl-area edo">
						<div className="container">
							<div className="container-in">
								<div className="space-20"></div>
								<div className="col-12">
									<div className="text-align-center">
										<img src={Avatar} alt="" className="Avatar" />
									</div>
									<div className="space-15"></div>
									<div className="links">
										<span>CHANGE</span>
										<span className="danger">DELETE</span>
									</div>
								</div>
								<div className="space-20"></div>
								<div className="col-12">
									<div className="head">Personal data</div>
								</div>
								<div className="clr"></div>
								<div className="col-4 fl col-m-full">
									<div className="boxl">Full name</div>
								</div>
								<div className="col-8 fr col-m-full">
									{fullNameEdit ? (
										<form onSubmit={handleSubmitFullName(onSubmitFullName)}>
											<input
												type="text"
												placeholder="Enter full name"
												className="boxt withedit fl"
												{...registerFullName("fullName", { required: "Full name is required" })}
											/>
											<button type="submit" className="saverow fr">
												<img src={saveRow} alt="" />
											</button>
											<div className="clr"></div>
											{fullNameErrors.fullName && <p className="error">{fullNameErrors.fullName.message}</p>}
										</form>
									) : (
										<div className="boxl lg">
											{watchFullName("fullName")}{" "}
											<span className="fr">
												<img src={editRow} onClick={() => setFullNameEdit(true)} />
											</span>
										</div>
									)}
								</div>
								<div className="clr"></div>
								<div className="space-15"></div>
								<div className="clr"></div>
								<div className="col-4 fl col-m-full">
									<div className="boxl">Email</div>
								</div>
								<div className="col-8 fr col-m-full">
									{emailEdit ? (
										<form onSubmit={handleSubmitEmail(onSubmitEmail)}>
											<input
												type="text"
												placeholder="Enter email"
												className="boxt withedit fl"
												{...registerEmail("email", {
													required: "Email is required",
													pattern: {
														value: /^[a-zA-Z0-9._-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,6}$/,
														message: "Invalid email address",
													},
												})}
											/>
											<button type="submit" className="saverow fr">
												<img src={saveRow} alt="" />
											</button>
											<div className="clr"></div>
											{emailErrors.email && <p className="error">{emailErrors.email.message}</p>}
										</form>
									) : (
										<div className="boxl lg">
											{watchEmailForm("email")}{" "}
											<span className="fr">
												<img src={editRow} onClick={() => setEmailEdit(true)} />
											</span>
										</div>
									)}
								</div>
								<div className="clr"></div>
								<div className="space-30"></div>
								<div className="col-12">
									<div className="head">
										Change password{" "}
										{/* <span className="headlink fr" onClick={() => setForgotScreen(true)}>
											Forget Password
										</span> */}
									</div>
								</div>
								<div className="clr"></div>
								<form onSubmit={handleSubmitPassword(onSubmitPassword)}>
									<div className="col-4 fl col-m-full col-mr-bottom-10">
										<div className="boxl">Current password</div>
									</div>
									<div className="col-8 fr col-m-full">
										<input
											id="current_password"
											type="password"
											placeholder="Enter current password"
											className="boxt"
											{...registerPassword("current_password", { required: "Current password is required" })}
										/>
										{passwordErrors.current_password && <p className="error">{passwordErrors.current_password.message}</p>}
									</div>
									<div className="clr"></div>
									<div className="space-15"></div>
									<div className="col-4 fl col-m-full col-mr-bottom-10">
										<div className="boxl">New password</div>
									</div>
									<div className="col-8 fr col-m-full">
										<input
											id="new_password"
											type="password"
											placeholder="Enter new password"
											className="boxt"
											{...registerPassword("new_password", { required: "New password is required" })}
										/>
										{passwordErrors.new_password && <p className="error">{passwordErrors.new_password.message}</p>}
									</div>
									<div className="clr"></div>
									<div className="space-15"></div>
									<div className="col-4 fl col-m-full col-mr-bottom-10">
										<div className="boxl">Confirm password</div>
									</div>
									<div className="col-8 fr col-m-full">
										<input
											id="confirm_password"
											type="password"
											placeholder="Confirm new password"
											className="boxt"
											{...registerPassword("confirm_password", { required: "Confirm password is required" })}
										/>
										{passwordErrors.confirm_password && <p className="error">{passwordErrors.confirm_password.message}</p>}
									</div>
									<div className="clr"></div>
									<div className="space-20"></div>
									<div className="clr"></div>
									<div className="space-20"></div>
									<div className="col-12">
										<div className="head">
											Security
											{user.mfaEnabled ? (
												<span className="headlink fr showonmobile" onClick={() => onDisableMfaClicked()}>
													ENABLE
												</span>
											) : (
												<span className="headlink danger fr showonmobile" onClick={() => onEnableMfaClicked()}>
													DISABLE
												</span>
											)}
										</div>
									</div>
									<div className="space-15"></div>
									{user.mfaEnabled ? (
										<div className="col-12">
											<span className="twofactor enabled text-align-center-mob">
												<img src={AccountProtected} alt="" height={13} />
												Account secured with 2FA
											</span>{" "}
											<span className="headlink danger fr hideonmobile" onClick={() => onDisableMfaClicked()}>
												DISABLE
											</span>
										</div>
									) : (
										<div className="col-12">
											<span className="twofactor text-align-center-mob">
												<img src={AccountCross} alt="" height={13} />
												Account is not secured with 2FA
											</span>
											<span className="headlink danger fr hideonmobile" onClick={() => onEnableMfaClicked()}>
												ENABLE
											</span>
										</div>
									)}

									<div className="clr"></div>
									<div className="space-30"></div>
									<div className="clr"></div>
									<div className="col-5 fl col-m-full col-mr-bottom-20">
										<Button type="button" text="CLOSE" call={() => close()} />
									</div>
									<div className="col-5 fr col-m-full">
										<Button
											type="submit"
											text="SAVE"
											active
											loading={settingPassword}
											disabled={settingPassword || !isPasswordFormValid}
										/>
									</div>
									<div className="clr"></div>
								</form>
							</div>
						</div>
					</div>
				</>
			</div>

			{emailEditOTPScreen && (
				<div className="popup" onClick={(e) => e.stopPropagation()}>
					<div className="heading">Change Email</div>
					<div className="cl-area edo">
						<div className="container">
							<div className="container-in">
								<div className="space-20"></div>
								<div className="col-12">
									<div className="head text-align-center">To change, you need to confirm your new email address</div>
									<div className="space-15"></div>
									<div className="text-align-center">We’ve sent a code to {user.email}</div>
									<div className="space-30"></div>
									<form onSubmit={handleSubmitOtp(onSubmitOtp)}>
										<Controller<OtpFormInputs>
											name="otp"
											control={controlOtp}
											render={({ field }) => <OtpInput control={controlOtp} name={field.name} length={6} error={otpErrors.otp} />}
										/>
										<div className="clr"></div>
										<div className="col-5 fl col-m-full col-mr-bottom-20">
											<Button
												text="CANCEL"
												call={() => {
													setEmailEditOtpScreen(undefined);
													setEmailEdit(false);
												}}
											/>
										</div>
										<div className="col-5 fr col-m-full">
											<Button text="CONFIRM" active type="submit" disabled={!isValid} />
										</div>
										<div className="clr"></div>
									</form>
								</div>
							</div>
						</div>
					</div>
				</div>
			)}
			{forgotScreen && (
				<div className="popup" onClick={(e) => e.stopPropagation()}>
					<div className="heading">Forget Password</div>
					<div className="cl-area edo">
						<div className="container">
							<div className="container-in">
								<div className="space-20"></div>
								<div className="col-12">
									<div className="head text-align-center">Please check your email to create</div>
									<div className="head text-align-center">a new password</div>
									<div className="space-30"></div>
								</div>
							</div>
						</div>
					</div>
				</div>
			)}
			{enable2FaScreen && (
				<div className="popup" onClick={(e) => e.stopPropagation()}>
					<div className="heading">Security 2FA</div>
					<div className="cl-area edo">
						<div className="container">
							<div className="container-in">
								<div className="space-20"></div>
								<div className="col-12">
									<div className="head text-align-center">To activate 2FA security, you need to confirm via your email</div>
									<div className="space-15"></div>
									<div className="text-align-center">We’ve sent a code to {user.email}</div>
									<div className="space-30"></div>
									{/* <OtpInput /> */}
								</div>
								<div className="clr"></div>
								<div className="col-5 fl col-m-full col-mr-bottom-20">
									<Button text="CANCEL" call={() => set2FAScreen(false)} />
								</div>
								<div className="col-5 fr col-m-full">
									<Button text="ENABLE 2FA" active />
								</div>
								<div className="clr"></div>
							</div>
						</div>
					</div>
				</div>
			)}
			{disable2FaScreen && (
				<div className="popup" onClick={(e) => e.stopPropagation()}>
					<div className="heading">Security 2FA</div>
					<div className="cl-area edo">
						<div className="container">
							<div className="container-in">
								<div className="space-20"></div>
								<div className="col-12">
									<div className="head text-align-center">To disable 2FA security, you need to confirm it via email</div>
									<div className="space-15"></div>
									<div className="text-align-center">We’ve sent a code to {user.email}</div>
									<div className="space-30"></div>
									{/* <OtpInput /> */}
								</div>
								<div className="clr"></div>
								<div className="col-5 fl col-m-full col-mr-bottom-20">
									<Button text="CANCEL" call={() => set2FAScreenDisable(false)} />
								</div>
								<div className="col-5 fr col-m-full">
									<Button text="DISABLE" active danger={true} />
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
