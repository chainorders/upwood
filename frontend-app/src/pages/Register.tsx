import { SubmitHandler, useForm } from "react-hook-form";
import Button from "../components/Button";
import { useEffect, useState } from "react";
import { UserService } from "../apiClient";
import { detectConcordiumProvider, WalletApi } from "@concordium/browser-wallet-api-helpers";
import { Web3StatementBuilder } from "@concordium/web-sdk";
import { useNavigate } from "react-router";
import { sha256 } from "hash-wasm";

export interface RegisterReq {
	email: string;
	password: string;
	confirmPassword: string;
	tempPassword: string;
	investmentAmount?: number;
	terms: boolean;
}

export default function Register() {
	const navigate = useNavigate();
	const [wallepApi, setWallepApi] = useState<WalletApi>();
	const [loading, setLoading] = useState(false);

	useEffect(() => {
		detectConcordiumProvider().then((provider) => setWallepApi(provider));
	}, []);

	const {
		register,
		handleSubmit,
		formState: { errors, isValid },
		watch,
	} = useForm<RegisterReq>({
		mode: "onChange",
		defaultValues: {
			terms: false,
		},
	});

	const onRegisterFormSubmit: SubmitHandler<RegisterReq> = async (data, e) => {
		e?.preventDefault();
		if (!wallepApi) {
			alert("Please connect with Concordium Wallet");
			return;
		}

		setLoading(true);
		let account = await wallepApi.getMostRecentlySelectedAccount();
		if (!account) {
			const accounts = await wallepApi.requestAccounts();
			account = accounts[0];
		}
		if (!account) {
			alert("No account selected");
			return;
		}

		try {
			const emailHash = await sha256(data.email);
			const identityResponse = await wallepApi.requestVerifiablePresentation(
				emailHash,
				new Web3StatementBuilder()
					.addForIdentityCredentials([0, 1, 2], (b) =>
						b.revealAttribute("firstName").revealAttribute("lastName").revealAttribute("nationality"),
					)
					.getStatements(),
			);
			await UserService.postUserRegister({
				account_address: account!,
				email: data.email,
				temp_password: data.tempPassword,
				password: data.password,
				proof: identityResponse,
				desired_investment_amount: data.investmentAmount,
			});
			navigate("/login", {
				state: {
					email: data.email,
					password: data.password,
				},
			});
			setLoading(false);
		} catch (e) {
			console.error(e);
			alert("Failed to register");
			setLoading(false);
		}
	};

	const password = watch("password");

	return (
		<div>
			<form onSubmit={handleSubmit(onRegisterFormSubmit)}>
				<div className="heading">User Registration</div>
				<div className="field mrbottom">
					{errors?.email && <p className="text-align-right error">{errors?.email?.message}</p>}
					<input
						type="email"
						placeholder="Enter your email"
						className={`textField style2 ${errors.email ? "error" : ""}`}
						{...register("email", {
							required: {
								message: "Email is required",
								value: true,
							},
							pattern: {
								value: /^[a-zA-Z0-9._-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,6}$/,
								message: "Invalid email address",
							},
						})}
					/>
				</div>
				<div className="field mrbottom">
					{errors?.tempPassword && <p className="text-align-right error">{errors?.tempPassword?.message}</p>}
					<input
						type="password"
						placeholder="Enter your temporary password"
						className={`textField style2 ${errors.tempPassword ? "error" : ""}`}
						{...register("tempPassword", {
							required: {
								message: "Temporary password is required",
								value: true,
							},
							minLength: {
								value: 8,
								message: "Temporary password must have at least 8 characters",
							},
						})}
					/>
				</div>
				<div className="field mrbottom">
					{errors?.password && <p className="text-align-right error">{errors?.password?.message}</p>}
					<input
						type="password"
						placeholder="Enter your password"
						className={`textField style2 ${errors.password ? "error" : ""}`}
						{...register("password", {
							required: {
								message: "Password is required",
								value: true,
							},
							minLength: {
								value: 8,
								message: "Password must have at least 8 characters",
							},
						})}
					/>
				</div>
				<div className="field mrbottom">
					{errors?.confirmPassword && <p className="text-align-right error">{errors?.confirmPassword?.message}</p>}
					<input
						type="password"
						placeholder="Repeat your password"
						className={`textField style2 ${errors.confirmPassword ? "error" : ""}`}
						{...register("confirmPassword", {
							required: {
								message: "Please confirm your password",
								value: true,
							},
							validate: (value) => value === password || "Passwords do not match",
						})}
					/>
				</div>
				<div className="field mrbottom">
					<div className="lab">Desired investment amount € (optional)</div>
					<input
						type="text"
						placeholder="Enter amount €"
						className="textField style2"
						{...register("investmentAmount", {
							valueAsNumber: true,
						})}
					/>
				</div>
				<div className="field mrbottom">
					<div className="checkbox">
						<input
							type="checkbox"
							id="terms"
							{...register("terms", {
								required: {
									message: "You must accept the terms and conditions",
									value: true,
								},
							})}
						/>
						<label htmlFor="terms" className="no-center-mobile">
							<span>I accept Terms & condition....</span>
						</label>
					</div>
				</div>
				<div className="field mrbottom">
					<Button
						style="style3"
						radius16
						text="CONTINUE WITH DIGITAL WALLET CREATION"
						active
						type="button"
						link="https://chromewebstore.google.com/detail/concordium-wallet/mnnkpffndmickbiakofclnpoiajlegmg?hl=en"
					/>
				</div>
				<div className="or">
					<span>OR</span>
				</div>
				<div className="field mrbottom">
					<Button
						style="style3"
						type="submit"
						radius16
						text="CONNECT IF YOU HAVE CONCORDIUM WALLET"
						disabled={!wallepApi || !isValid}
						loading={loading}
					/>
				</div>
			</form>
			<div className="divider"></div>
		</div>
	);
}
