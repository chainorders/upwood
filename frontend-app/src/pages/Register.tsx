import Button from "../components/Button";
export default function Register() {
	return (
		<div>
			<div className="heading">User Registration</div>
			<div className="field mrbottom">
				<input type="text" placeholder="Enter your email" className="textField style2" />
			</div>
			<div className="field mrbottom">
				<p className="text-align-right error">error will display here.</p>
				<input type="text" placeholder="Enter your password" className="textField style2 error" />
			</div>
			<div className="field mrbottom">
				<input type="text" placeholder="Repeat your password" className="textField style2" />
			</div>
			<div className="field mrbottom">
				<div className="lab">Desired investment amount € (optional)</div>
				<input type="text" placeholder="Enter amount €" className="textField style2" />
			</div>
			<div className="field mrbottom">
				<div className="checkbox">
					<input type="checkbox" id="terms" />
					<label htmlFor="terms" className="no-center-mobile">
						<span>I accept Terms & condition....</span>
					</label>
				</div>
			</div>
			<div className="field mrbottom">
				<Button text={"CONTINUE WITH DIGITAL WALLET CREATION"} link="/register/download-wallet" active={true} />
			</div>
			<div className="or">
				<span>OR</span>
			</div>
			<div className="field mrbottom">
				<Button style={"style3"} text={"CONNECT IF YOU HAVE CONCORDIUM WALLET"} link={""} active={false} />
			</div>
			<div className="divider"></div>
		</div>
	);
}
