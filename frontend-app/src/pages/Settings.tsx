import { useEffect, useState } from "react";
import { Link, useOutletContext } from "react-router";

import {
	PagedResponse_ForestProjectFundsAffiliateRewardRecord,
	PagedResponse_UserTransaction,
	SystemContractsConfigApiModel,
	UserService,
	WalletService,
} from "../apiClient";
import AccountCross from "../assets/account-not-protected.svg";
import Button from "../components/Button";
// import ClaimPopup from "../components/ClaimPopup";
import CreateCompany from "../components/CreateCompany";
import EditCompany from "../components/EditCompany";
import EditProfile from "../components/EditProfile";
import PageHeader from "../components/PageHeader";
import { User } from "../lib/user";
import { toDisplayAmount } from "../lib/conversions";

function Pagination({
	pageCount,
	currentPage,
	onPageChange,
}: {
	pageCount: number;
	currentPage: number;
	onPageChange: (page: number) => void;
}) {
	return (
		<div className="pagignation">
			<ul>
				<li className={currentPage <= 0 ? "disabled" : ""} onClick={() => onPageChange(Math.max(currentPage - 1, 0))}>
					{"<"}
				</li>
				{[...Array(pageCount).keys()].map((i) => (
					<li key={i} className={i === currentPage ? "active" : ""} onClick={() => onPageChange(i)}>
						{i + 1}
					</li>
				))}
				<li
					className={currentPage >= pageCount - 1 ? "disabled" : ""}
					onClick={() => onPageChange(Math.min(currentPage + 1, pageCount - 1))}
				>
					{">"}
				</li>
			</ul>
		</div>
	);
}

export default function Settings() {
	const { user } = useOutletContext<{ user: User }>();
	const [transactions, setTransactions] = useState<PagedResponse_UserTransaction>();
	const [trasactionsPage, setTransactionsPage] = useState(0);
	const [affiliateRewards, setAffiliateRewards] = useState<PagedResponse_ForestProjectFundsAffiliateRewardRecord>();
	const [affiliateRewardsPage, setAffiliateRewardsPage] = useState(0);
	const [contracts, setContracts] = useState<SystemContractsConfigApiModel>();

	const [, setClaimPopup] = useState(false);
	const [edit_profile_popup, setEditProfilePopup] = useState(false);
	const [create_company_popup, setCreateCompanyPopup] = useState(false);
	const [edit_company_popup, setEditCompanyPopup] = useState(false);

	useEffect(() => {
		UserService.getSystemConfig().then(setContracts);
	}, [user]);
	useEffect(() => {
		WalletService.getUserTransactionsList(trasactionsPage).then(setTransactions);
	}, [user, trasactionsPage]);
	useEffect(() => {
		WalletService.getUserAffiliateRewardsList(affiliateRewardsPage).then(setAffiliateRewards);
	}, [user, affiliateRewardsPage]);
	const links = [
		{ title: "Portfolio", description: "How to manage your investments portfolio", link: "" },
		{ title: "Wallet", description: "How to manage your wallet", link: "" },
		{ title: "Contracts", description: "How to manage your contracts", link: "" },
	];
	// const claim_popup_details = {
	// 	heading: "Claim affiliate earnings",
	// 	list: [
	// 		{
	// 			tag: "Claim affiliate earnings",
	// 			display: "150 EUROe",
	// 		},
	// 	],
	// };
	return (
		<>
			<div className="clr"></div>
			<div className="settings">
				<PageHeader user={user} parts={[{ name: "Settings" }]} />
				<div className="outerboxshadow">
					<div className="container">
						<div className="container-in">
							<div className="col-6 fl col-m-full">
								<div className="setting-block text-align-center">
									<div className="heading">Profile settings</div>
									<div className="letter">{user.initialis}</div>
									<div className="name">{user.fullName}</div>
									<div className="email mr">{user.email}</div>
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
											</tr>
										</thead>
										<tbody>
											{transactions?.data.map((item) => (
												<tr key={item.transaction_hash + item.transaction_type}>
													<td style={{ whiteSpace: "nowrap", overflow: "hidden", textOverflow: "ellipsis", maxWidth: "6ch" }}>
														{item.transaction_hash}
													</td>
													<td>{item.transaction_type}</td>
													<td>{item.account_address}</td>
													<td>
														{toDisplayAmount(item.currency_amount, contracts?.euro_e_metadata.decimals || 6, 2)}
														{contracts?.euro_e_metadata.symbol}
													</td>
												</tr>
											))}
										</tbody>
									</table>
								</div>
								<Pagination
									pageCount={transactions?.page_count ?? 0}
									currentPage={trasactionsPage}
									onPageChange={setTransactionsPage}
								/>
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
									Your unique link :{" "}
									<a
										href={`${window.location.protocol}//${window.location.host}/login/${user.concordiumAccountAddress}`}
										target="_blank"
									>{`${window.location.protocol}//${window.location.host}/login/${user.concordiumAccountAddress}`}</a>
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
											{affiliateRewards?.data.map((item, index) => (
												<tr key={index}>
													<td>{item.investment_record_id}</td>
													<td>{item.investor_account_address}</td>
													<td>{toDisplayAmount(item.currency_amount, contracts?.euro_e_metadata.decimals || 6)}</td>
													<td>{item.affiliate_commission}%</td>
													<td>{toDisplayAmount(item.reward_amount, contracts?.euro_e_metadata.decimals || 6)}</td>
													<td>{toDisplayAmount(item.remaining_reward_amount, contracts?.euro_e_metadata.decimals || 6)}</td>
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
								<Pagination
									pageCount={affiliateRewards?.page_count ?? 0}
									currentPage={affiliateRewardsPage}
									onPageChange={setAffiliateRewardsPage}
								/>
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
			{/* {claim_popup ? <ClaimPopup config={claim_popup_details} close={() => setClaimPopup(false)} /> : null} */}
			{edit_profile_popup ? <EditProfile close={() => setEditProfilePopup(false)} /> : null}
			{create_company_popup ? <CreateCompany close={() => setCreateCompanyPopup(false)} /> : null}
			{edit_company_popup ? <EditCompany close={() => setEditCompanyPopup(false)} /> : null}
		</>
	);
}
