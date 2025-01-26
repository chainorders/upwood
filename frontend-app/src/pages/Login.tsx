import { SubmitHandler, useForm } from "react-hook-form";
import { Link, useLocation, useNavigate, useParams } from "react-router";
import { LoginReq, LoginRes, OpenAPI, UserService } from "../apiClient";
import Button from "../components/Button";
import { useTitle } from "../components/useTitle";

export interface SliderData {
	title: string;
	description: string;
}

export default function Login(props: { setUser: (user: LoginRes) => void }) {
	useTitle("Login");
	const {
		register,
		handleSubmit,
		setError,
		formState: { errors },
	} = useForm<LoginReq>();
	const onLoginFormSubmit: SubmitHandler<LoginReq> = (data, e) => {
		e?.preventDefault();
		UserService.postUserLogin(data)
			.then((res) => {
				console.log("User logged in");
				props.setUser(res);
				OpenAPI.TOKEN = res.id_token;
				navigate(location.state?.from ? location.state.from : "/projects/active");
			})
			.catch(() => {
				console.log("Invalid email or password");
				setError("email", {
					type: "custom",
					message: "Invalid email or password",
				});
			});
	};

	// eslint-disable-next-line @typescript-eslint/no-unused-vars
	const { affiliateAccountAddress } = useParams();
	const navigate = useNavigate();
	const location = useLocation();

	const whenRequestInvitationButtonHit = () => {
		console.log("Request invitation button hit");
		navigate("/login/invite-success");
	};

	return (
		<div>
			<form onSubmit={handleSubmit(onLoginFormSubmit)}>
				<div className="heading">User login</div>
				<div className="field mrbottom">
					<p className="text-align-right error" style={{ minHeight: "3em" }}>
						{errors?.email?.message}
					</p>
					<input
						id="email"
						type="email"
						placeholder="Enter email address"
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
					{errors.password && <p className="text-align-right error">{errors.password.message}</p>}
					<input
						type="password"
						placeholder="Enter your password"
						className={`textField style2 ${errors.password ? "error" : ""}`}
						{...(errors.password && { error: true })}
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
				<div>
					<button className="button style2 active" type="submit">
						LOG IN
					</button>
				</div>
			</form>
			<div className="forgotlink">
				<Link to="/forgot-password">Forgot Password</Link>
			</div>
			<div className="divider"></div>
			<form>
				<div className="heading">Not a user? Request invitation</div>
				<div className="field mrbottom">
					<input type="text" placeholder="Enter email address" className="textField style2" />
				</div>
				<div className="field mrbottom">
					<div className="checkbox">
						<input type="checkbox" id="terms" />
						<label htmlFor="terms" className="no-center-mobile">
							<span>
								By entering your email you agree to receive marketing communications from SIA Upwood. You can unsubscribe at any
								time. For more information, see our <a href="">Privacy Policy</a>.
							</span>
						</label>
					</div>
				</div>
				<div>
					<Button
						style={"style2"}
						text={"REQUEST INVITATION"}
						link={""}
						active={true}
						disabled={true}
						call={whenRequestInvitationButtonHit}
					/>
				</div>
			</form>
		</div>
	);
}
