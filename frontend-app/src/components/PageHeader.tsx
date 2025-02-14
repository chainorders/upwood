import { Link } from "react-router";
import { User } from "../lib/user";

export default function PageHeader(props: { parts: { name: string; link?: string }[]; user: User }) {
	const { fullName, initialis } = props.user;

	return (
		<>
			<div className="container">
				<div className="container-in">
					<div className="col-12">
						<h1 className="breadcrumb">
							{props.parts.map((part, index) => {
								return (
									<span key={index}>
										{part.link ? (
											<Link type="text" to={part.link}>
												{part.name}
											</Link>
										) : (
											<span>{part.name}</span>
										)}
										{index < props.parts.length - 1 && <span> &gt; </span>}
									</span>
								);
							})}
							<div className="username fr" style={{ marginTop: "-0.5em" }} title={props.user.concordiumAccountAddress}>
								<span>{initialis}</span>
								{fullName}
							</div>
						</h1>
					</div>
				</div>
			</div>
			<div className="space-30"></div>
		</>
	);
}
