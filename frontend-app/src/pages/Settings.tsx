"use client";
import { useState } from "react";
import PageHeader from "../components/PageHeader";
import { Link, useOutletContext } from "react-router";
import { ApiUser } from "../apiClient";
import Button from "../components/Button";
import AccountCross from "../assets/account-not-protected.svg";
import ClaimPopup from "../components/ClaimPopup";
import EditProfile from "../components/EditProfile";
import CreateCompany from "../components/CreateCompany";
import EditCompany from "../components/EditCompany";
export default function Settings() {
	const { user } = useOutletContext<{ user: ApiUser }>();
	const [claim_popup, setClaimPopup] = useState(false);
	const [edit_profile_popup, setEditProfilePopup] = useState(false);
	const [create_company_popup, setCreateCompanyPopup] = useState(false);
	const [edit_company_popup, setEditCompanyPopup] = useState(false);
	const table_data = [
		{
			transaction_hash: "765192..",
			type: "Share purchase",
			sender: "35CJPZ..",
			amount: "500 EuroE",
			status: "Successful",
		},
		{
			transaction_hash: "765192..",
			type: "Share purchase",
			sender: "35CJPZ..",
			amount: "500 EuroE",
			status: "Pending",
		},
		{
			transaction_hash: "765192..",
			type: "Share purchase",
			sender: "35CJPZ..",
			amount: "500 EuroE",
			status: "Failed",
		},
		{
			transaction_hash: "765192..",
			type: "Share purchase",
			sender: "35CJPZ..",
			amount: "500 EuroE",
			status: "Successful",
		},
		{
			transaction_hash: "765192..",
			type: "Share purchase",
			sender: "35CJPZ..",
			amount: "500 EuroE",
			status: "Pending",
		},
		{
			transaction_hash: "765192..",
			type: "Share purchase",
			sender: "35CJPZ..",
			amount: "500 EuroE",
			status: "Failed",
		},
	];
	const table_data2 = [
		{
			number: "1",
			wallet_address: "sfbdsfdsye3267rgdfehsh",
			amount_invested: "5000€",
			your_commission: "3%",
			amount: "150€",
			status: "150€",
		},
		{
			number: "1",
			wallet_address: "sfbdsfdsye3267rgdfehsh",
			amount_invested: "5000€",
			your_commission: "3%",
			amount: "150€",
			status: "150€",
		},
		{
			number: "1",
			wallet_address: "sfbdsfdsye3267rgdfehsh",
			amount_invested: "5000€",
			your_commission: "3%",
			amount: "150€",
			status: "150€",
		},
	];
	const links = [
		{ title: "Portfolio", description: "How to manage your investments portfolio", link: "" },
		{ title: "Wallet", description: "How to manage your wallet", link: "" },
		{ title: "Contracts", description: "How to manage your contracts", link: "" },
	];
	const claim_popup_details = {
		heading: "Claim affiliate earnings",
		list: [
			{
				tag: "Claim affiliate earnings",
				display: "150 EUROe",
			},
		],
	};
	return (
		<>
			<div className="clr"></div>
			<div className="settings">
				<PageHeader userFullName={user.fullName} initials={user.initials} parts={[{ name: "Settings" }]} />
				<div className="outerboxshadow">
					<div className="container">
						<div className="container-in">
							<div className="col-6 fl col-m-full">
								<div className="setting-block text-align-center">
									<div className="heading">Profile settings</div>
									<div className="letter">J</div>
									<div className="name">John Carter</div>
									<div className="email mr">Jonh@gmail.com</div>
									<div className="action">
										<Button
											style={`style3`}
											text={"Edit profile"}
											link={""}
											active={false}
											call={() => setEditProfilePopup(true)}
										/>
									</div>
									<div className="st">
										<span>
											<img src={AccountCross} alt="" />
											Account is not protected by 2FA
										</span>
									</div>
								</div>
							</div>
							<div className="col-6 fl col-m-full">
								<div className="setting-block text-align-center">
									<div className="heading">Legal entity</div>
									{/* <div>
                    <div className="letter">C</div>
                    <div className="name">SIA Upwood</div>
                    <div className="email">esg@upwood.io</div>
                    <div className="reg">Reg. nr. 12343678</div>
                    <div className='action'>
                      <Button style={`style3`} text ={'Edit company profile'} link={''} active={false} call={()=> setEditCompanyPopup(true)} />
                    </div>
                    <div className="st"><Link type="text" to="/"
                        className="guides"
                        style={{ cursor: "pointer" }}
                    >Download account statement</Link></div>
                  </div> */}
									<div>
										<div className="action pdtop">
											<Button text={"Create company profile"} link={""} active={true} call={() => setCreateCompanyPopup(true)} />
										</div>
									</div>
								</div>
							</div>
							<div className="clr"></div>
							{links.map((item, index) => (
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
						</div>
					</div>
				</div>
				<div className="space-30"></div>
				<div className="outerboxshadow">
					<div className="container">
						<div className="container-in">
							<div className="col-12 col-m-full fl">
								<div className="heading">Your transaction history</div>
								<p className="genp">
									Earn up to 5% by sharing Upwood to your network! In this section you will find your unique referral link and
									see your affiliate earnings.{" "}
								</p>
								<div className="space-20"></div>
							</div>
							<div className="clr"></div>
						</div>
					</div>
					<div className="container">
						<div className="container-in">
							<div className="col-12">
								<div className="table">
									<table cellSpacing={0}>
										<thead>
											<tr>
												<th>Transaction hash</th>
												<th>Type</th>
												<th>Sender</th>
												<th>Amount</th>
												<th>Status</th>
											</tr>
										</thead>
										<tbody>
											{table_data.map((item, index) => (
												<tr key={index}>
													<td>{item.transaction_hash}</td>
													<td>{item.type}</td>
													<td>{item.sender}</td>
													<td>{item.amount}</td>
													<td>
														<span className={item.status}>{item.status}</span>
													</td>
												</tr>
											))}
										</tbody>
									</table>
								</div>
								<div className="pagignation">
									<ul>
										<li className="disabled">{"<"}</li>
										<li className="active">{"1"}</li>
										<li>{"2"}</li>
										<li>{"3"}</li>
										<li>{"4"}</li>
										<li>{"5"}</li>
										<li>{">"}</li>
									</ul>
								</div>
								<div className="space-30"></div>
							</div>
						</div>
					</div>
				</div>

				<div className="space-30"></div>
				<div className="outerboxshadow">
					<div className="container">
						<div className="container-in">
							<div className="col-8 col-m-full fl">
								<div className="heading">Affiliate settings</div>
								<p className="genp hideonmobile">
									Your unique link : <Link to="upwood.io/jsdhdsjsdhc1234">upwood.io/jsdhdsjsdhc1234</Link>
								</p>
							</div>
							<div className="col-4 text-align-right fr hideonmobile">
								<Link type="text" to="/" className="guides" style={{ cursor: "pointer" }}>
									Affiliate Terms & Conditions
								</Link>
							</div>
							<div className="clr"></div>
						</div>
					</div>
					<div className="container">
						<div className="container-in">
							<div className="col-12">
								<div className="table">
									<table cellSpacing={0}>
										<thead>
											<tr>
												<th>Number</th>
												<th>Wallet address</th>
												<th>Amount invested</th>
												<th>Your commission %</th>
												<th>Amount</th>
												<th>Status</th>
											</tr>
										</thead>
										<tbody>
											{table_data2.map((item, index) => (
												<tr key={index}>
													<td>{item.number}</td>
													<td>{item.wallet_address}</td>
													<td>{item.amount_invested}</td>
													<td>{item.your_commission}</td>
													<td>{item.amount}</td>
													<td>{item.status}</td>
												</tr>
											))}
										</tbody>
									</table>
								</div>
								<div className="clr"></div>

								<div className="space-30"></div>
							</div>
						</div>
						<div className="container-in">
							<div className="col-4 fl hideonmobile">
								<Link type="text" to="/" className="guides" style={{ cursor: "pointer" }}>
									Export affiliate earning table
								</Link>
							</div>

							<div className="fr col-m-full">
								<div className="pagignation">
									<ul>
										<li className="disabled">{"<"}</li>
										<li className="active">{"1"}</li>
										<li>{"2"}</li>
										<li>{"3"}</li>
										<li>{"4"}</li>
										<li>{"5"}</li>
										<li>{">"}</li>
									</ul>
								</div>
							</div>
							<div className="col-2 text-align-right fr hideonmobile">
								<Button text={"CLAIM EARNINGS"} link={""} active={true} call={() => setClaimPopup(true)} />
							</div>
							<div className="clr"></div>
							<div className="space-20 showonmobile"></div>
							<div className="col-12 text-align-center showonmobile">
								<p className="genp">
									Your unique link :<br />
									<Link to="upwood.io/jsdhdsjsdhc1234">upwood.io/jsdhdsjsdhc1234</Link>
								</p>
							</div>
							<div className="space-20 showonmobile"></div>
							<div className="col-12 text-align-center showonmobile">
								<Link type="text" to="/" className="guides" style={{ cursor: "pointer" }}>
									Affiliate Terms & Conditions
								</Link>
							</div>
							<div className="space-20 showonmobile"></div>
							<div className="col-12 showonmobile">
								<Button text={"CLAIM EARNINGS"} link={""} active={true} call={() => setClaimPopup(true)} />
							</div>
							<div className="space-20 showonmobile"></div>
							<div className="col-12 text-align-center showonmobile">
								<Link type="text" to="/" className="guides" style={{ cursor: "pointer" }}>
									Export affiliate earning table
								</Link>
							</div>
							<div className="clr"></div>
							<div className="space-20"></div>
						</div>
					</div>
				</div>
				<div className="space-30"></div>
			</div>
			{claim_popup ? <ClaimPopup config={claim_popup_details} close={() => setClaimPopup(false)} /> : null}
			{edit_profile_popup ? <EditProfile close={() => setEditProfilePopup(false)} /> : null}
			{create_company_popup ? <CreateCompany close={() => setCreateCompanyPopup(false)} /> : null}
			{edit_company_popup ? <EditCompany close={() => setEditCompanyPopup(false)} /> : null}
		</>
	);
}
