import greenTickIcon from "../assets/green-tick.svg";

export default function RegisterDownloadWallet() {
	return (
		<div>
			<div className="image">
				<img src={greenTickIcon} />
			</div>
			<div className="center-text biger bold">Let’s verify your identity</div>
			<div className="space-30"></div>
			<div className="center-text big">
				To finish your registration you need to create a digital wallet
				<br />
				and verify your identity in few simple steps.
			</div>
			<div className="space-30"></div>
			<div className="center-text">
				Concordium wallet extension will be downloaded to your
				<br />
				device or browser
			</div>
			<div className="space-30"></div>
			<div className="center-text big">
				Once you’re done you will be logged in Upwood platform
				<br />
				<a href="">I understand, let’s continue</a>
			</div>
			<div className="divider"></div>
		</div>
	);
}
