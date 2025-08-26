import Button from "../components/Button";
import greenTIckIcon from "../assets/green-tick.svg";

export default function LoginInviteSuccess() {
	return (
		<div>
			<div className="image">
				<img src={greenTIckIcon} />
			</div>
			<div className="center-text biger bold">Success</div>
			<div className="space-30"></div>
			<div className="center-text">
				We have recieved your
				<br />
				invitation request!
			</div>
			<div className="space-30"></div>
			<div className="center-text big">Please check your e-mail</div>
			<div className="space-30"></div>
			<div>
				<Button icon={"/Vector.svg"} text={"BACK TO LOGIN"} link="/login" active={false} />
			</div>
			<div className="divider"></div>
		</div>
	);
}
