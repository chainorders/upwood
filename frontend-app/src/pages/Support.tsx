"use client";
import { useEffect, useState } from "react";
import Button from "../components/Button";
import PageHeader from "../components/PageHeader";
import { Link } from "react-router";
import { User } from "../lib/user";

interface SupportProps {
	user: User;
}

export default function Support({ user }: SupportProps) {
	const [showAll, setShowAll] = useState(false);
	const links = [
		{
			title: "Portfolio",
			description: "How to manage your investments portfolio",
			link: "",
		},
		{ title: "Wallet", description: "How to manage your wallet", link: "" },
		{
			title: "Contracts",
			description: "How to manage your contracts",
			link: "",
		},
		{
			title: "Contracts",
			description: "How to manage your contracts",
			link: "",
		},
		{ title: "Wallet", description: "How to manage your wallet", link: "" },
		{
			title: "Contracts",
			description: "How to manage your contracts",
			link: "",
		},
		{
			title: "Portfolio",
			description: "How to manage your investments portfolio",
			link: "",
		},
		{ title: "Wallet", description: "How to manage your wallet", link: "" },
		{
			title: "Contracts",
			description: "How to manage your contracts",
			link: "",
		},
		{
			title: "Contracts",
			description: "How to manage your contracts",
			link: "",
		},
		{ title: "Wallet", description: "How to manage your wallet", link: "" },
		{
			title: "Contracts",
			description: "How to manage your contracts",
			link: "",
		},
	];

	const displayedLinks = showAll ? links : links.slice(0, 6);
	const submitclick = () => {};
	return (
		<>
			<div className="clr"></div>
			<div className="support">
				<PageHeader user={user} parts={[{ name: "Active Projects" }]} />
				<div className="outerboxshadow">
					<div className="container">
						<div className="container-in">
							<div className="col-8 col-m-full fl">
								<div className="heading">Platform how to guides</div>
							</div>
							<div className="col-4 text-align-right fr hideonmobile">
								<span className="seeall" onClick={() => setShowAll(!showAll)} style={{ cursor: "pointer" }}>
									{showAll ? "SEE LESS" : "SEE ALL"}
								</span>
							</div>

							<div className="clr"></div>
							{displayedLinks.map((item, index) => (
								<div className="col-4 col-m-full fl" key={index}>
									<div className="linkbox">
										<Link type="text" to={item.link}>
											<div className="title">{item.title}</div>
											<div className="description">{item.description}</div>
										</Link>
									</div>
								</div>
							))}
							<div className="clr"></div>
							<div className="col-12 showonmobile text-align-center">
								<span className="seeall" onClick={() => setShowAll(!showAll)} style={{ cursor: "pointer" }}>
									{showAll ? "SEE LESS" : "SEE ALL"}
								</span>
							</div>
							<div className="space-20 showonmobile"></div>
						</div>
					</div>
				</div>
				<div className="space-30"></div>
				<div className="outerboxshadow">
					<div className="container">
						<div className="container-in">
							<div className="col-12">
								<div className="heading">Write to support</div>
								<div className="sub">
									Our support hours are 10:00 to 16:00 (UTC +2) Mon to Fri. Please expect an answer during those times.
								</div>
							</div>
							<div className="space-20"></div>
							<div className="col-12">
								<div className="field">
									<textarea placeholder="Type your message here..." rows={10} className="textareaField style2"></textarea>
								</div>
							</div>
							<div className="clr"></div>
							<div className="space-20"></div>
							<div className="col-3 col-m-full fr">
								<Button text={"SUBMIT"} link={""} active={true} call={submitclick} />
							</div>
							<div className="clr"></div>
							<div className="space-20"></div>
						</div>
					</div>
				</div>
			</div>
		</>
	);
}
