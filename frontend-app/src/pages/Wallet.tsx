import { useEffect, useState } from "react";
import {
	ForestProjectService,
	ForestProjectTokenUserYieldClaim,
	OpenAPI,
	PagedResponse_ForestProjectAggApiModel,
	PagedResponse_ForestProjectTokenContractAggApiModel,
	SystemContractsConfigApiModel,
	UserService,
} from "../apiClient";
import PageHeader from "../components/PageHeader";
import ProjectCardOwned from "../components/ProjectCardOwned";
import ClaimPopup from "../components/ClaimPopup";
import { Link } from "react-router";
import { User } from "../lib/user";
import "./Wallet.css";
import { toDisplayAmount } from "../lib/conversions";
import useDownloader from "react-use-downloader";
import { FILE_DOWNLOAD_TIMEOUT } from "../lib/constants";

interface WalletProps {
	user: User;
}

export default function Wallet({ user }: WalletProps) {
	const [claimsPopup, setClaimsPopup] = useState(false);
	const [projects, setProjects] = useState<PagedResponse_ForestProjectAggApiModel>();
	const [yields, setYields] = useState<{
		carbonCredits: string;
		euroE: string;
		eTrees: string;
	}>();
	const [contracts, setContracts] = useState<SystemContractsConfigApiModel>();
	const [ownedTokenContracts, setOwnedTokenContracts] = useState<PagedResponse_ForestProjectTokenContractAggApiModel>();
	const [ownedTokenContractsPages, setOwnedTokenContractsPages] = useState<{
		onPreviousClick?: () => void;
		pages: {
			pageNum: number;
			isActive: boolean;
			onClick: (pageNum: number) => void;
		}[];
		onNextClick?: () => void;
	}>({ pages: [] });
	const [ownedTokenContractPage, setOwnedTokenContractPage] = useState(0);
	const [yieldsClaimable, setYieldsClaimable] = useState<ForestProjectTokenUserYieldClaim[]>();
	const [refreshCounter, setRefreshCounter] = useState(0);

	useEffect(() => {
		ForestProjectService.getForestProjectsListOwned().then(setProjects);
		UserService.getSystemConfig().then(setContracts);
	}, [user, refreshCounter]);
	useEffect(() => {
		if (contracts) {
			ForestProjectService.getForestProjectsYieldsTotal().then((response) => {
				const carbonCreditYield = response.find(
					(r) =>
						r.yield_token_id === contracts.carbon_credit_token_id &&
						r.yield_contract_address === contracts.carbon_credit_contract_index,
				);
				const euroEYield = response.find(
					(r) =>
						r.yield_token_id === contracts.euro_e_token_id && r.yield_contract_address === contracts.euro_e_contract_index,
				);
				const eTreesYield = response.find((r) => r.yield_contract_address === contracts.tree_ft_contract_index);
				setYields({
					carbonCredits: carbonCreditYield?.yield_amount || "0",
					euroE: euroEYield?.yield_amount || "0",
					eTrees: eTreesYield?.yield_amount || "0",
				});
			});
			ForestProjectService.getForestProjectsYieldsClaimable().then(setYieldsClaimable);
		}
	}, [user, contracts, refreshCounter]);
	useEffect(() => {
		ForestProjectService.getForestProjectsContractListOwned().then(setOwnedTokenContracts);
	}, [user, ownedTokenContractPage, refreshCounter]);
	useEffect(() => {
		const pages = [];
		if (ownedTokenContracts) {
			for (let index = 0; index < ownedTokenContracts.page_count; index++) {
				pages.push({
					pageNum: index,
					isActive: index === ownedTokenContractPage,
					onClick: () => setOwnedTokenContractPage(index),
				});
			}

			setOwnedTokenContractsPages({
				pages,
				onPreviousClick: ownedTokenContractPage > 0 ? () => setOwnedTokenContractPage((page) => page--) : undefined,
				onNextClick:
					ownedTokenContractPage < ownedTokenContracts.page_count - 1
						? () => {
								console.log("next page");
								setOwnedTokenContractPage((page) => page++);
							}
						: undefined,
			});
		}
	}, [ownedTokenContracts, ownedTokenContractPage, refreshCounter]);

	const { download, isInProgress } = useDownloader();
	const onTxnListDownload = async () => {
		if (isInProgress) return;
		await download(`${OpenAPI.BASE}/user/transactions/list/download`, "transactions_list.csv", FILE_DOWNLOAD_TIMEOUT, {
			headers: {
				Authorization: `Bearer ${user.idToken}`,
			},
		});
	};
	const onTokensListDownload = async () => {
		if (isInProgress) return;
		await download(`${OpenAPI.BASE}/forest_projects/list/owned/download`, "tokens_list.csv", FILE_DOWNLOAD_TIMEOUT, {
			headers: {
				Authorization: `Bearer ${user.idToken}`,
			},
		});
	};

	return (
		<>
			<div className="clr"></div>
			<div className="walletmanagement">
				<PageHeader user={user} parts={[{ name: "Wallet Management" }]} />
				<div className="outerboxshadow">
					<div className="container">
						<div className="container-in">
							<div className="col-8 fl">
								<div className="heading">Balance</div>
							</div>
							<div className="col-4 text-align-right fr hideonmobile">
								<a href={import.meta.env.VITE_WALLET_MANAGEMENT_GUIDE_URL} className="guides" target="_blank" rel="noreferrer">
									WALLET MANAGEMENT GUIDES
								</a>
							</div>
							<div className="col-4 text-align-right fr showonmobile">
								<Link type="text" to="/" className="guides" style={{ cursor: "pointer" }}>
									GUIDES
								</Link>
							</div>

							<div className="clr"></div>
						</div>
						<div className="space-20"></div>
						<div className="container-in">
							<div className="col-20-percent fl walletclms col-m-full col-mr-bottom-30">
								<div className="tag">Wallet</div>
								<div className="value address-ellipsis">{user.concordiumAccountAddress || "NA"}</div>
							</div>
							<div className="col-20-percent fl walletclms col-m-full col-mr-bottom-30">
								<div className="tag">Entity</div>
								<div className="value">SIA UPWOOD</div>
							</div>
							<div className="col-20-percent fl walletclms col-m-full col-mr-bottom-30">
								<div className="tag">Carbon credits</div>
								<div className="value">
									{toDisplayAmount(yields?.carbonCredits || "0", 0, 0)} = {0}
								</div>
							</div>
							<div className="col-10-percent fl walletclms col-m-full col-mr-bottom-30">
								<div className="tag">Dividends</div>
								<div className="value">{toDisplayAmount(yields?.euroE || "0", 6, 2)}</div>
							</div>
							<div className="col-20-percent fl walletclms col-m-full col-mr-bottom-30">
								<div className="tag">E-trees</div>
								<div className="value">{toDisplayAmount(yields?.eTrees || "0", 0, 0)}</div>
							</div>
							<div className="col-10-percent fl walletclms col-m-full">
								<div className="tag">Yields</div>
								<div className="value"></div>
								<button
									onClick={() => setClaimsPopup(true)}
									disabled={!contracts || !yieldsClaimable || yieldsClaimable.length === 0 || !yields}
								>
									Claim
								</button>
							</div>
							<div className="clr"></div>
						</div>
						<div className="space-30"></div>
					</div>
				</div>
				<div className="space-30"></div>
				<div className="outerboxshadow">
					<div className="container">
						<div className="container-in">
							<div className="col-6 col-m-full fl">
								<div className="heading">Token list</div>
							</div>
							<div className="col-6 text-align-right fr hideonmobile">
								<a href="#" className={`guides ${isInProgress ? "disabled" : ""}`} onClick={onTxnListDownload}>
									Export transaction history
								</a>
								<a
									href="#"
									className={`guides margin ${isInProgress ? "disabled" : ""}`}
									onClick={(e) => {
										e.preventDefault();
										onTokensListDownload();
									}}
								>
									Export token list
								</a>
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
												<th>Forest Project Id</th>
												<th>Asset name</th>
												<th>Smart contract address</th>
												<th>Share amount</th>
												<th>Share value</th>
												<th>Carbon credits</th>
												<th>Dividends earned</th>
											</tr>
										</thead>
										<tbody>
											{ownedTokenContracts?.data.map((item) => (
												<tr key={item.token_contract_address}>
													<td>{item.forest_project_id}</td>
													<td>{item.forest_project_name}</td>
													<td>
														{item.token_contract_address}({item.token_contract_type})
													</td>
													<td>{item.user_balance}</td>
													<td>{toDisplayAmount(item.user_balance_price, 6, 2)}</td>
													<td>{item.carbon_credit_yield_balance}</td>
													<td>{toDisplayAmount(item.euro_e_yields_balance, 6, 2)}</td>
												</tr>
											))}
										</tbody>
									</table>
								</div>
								<div className="pagignation">
									<ul>
										<li
											className={!ownedTokenContractsPages.onPreviousClick ? "disabled" : ""}
											onClick={ownedTokenContractsPages.onPreviousClick}
										>
											{"<"}
										</li>
										{ownedTokenContractsPages.pages.map((page) => (
											<li className={page.isActive ? "active" : ""} key={page.pageNum}>
												{page.pageNum + 1}
											</li>
										))}
										<li
											className={!ownedTokenContractsPages.onNextClick ? "disabled" : ""}
											onClick={ownedTokenContractsPages.onNextClick}
										>
											{">"}
										</li>
									</ul>
								</div>
								<div className="space-30"></div>
							</div>
							<div className="col-12 showonmobile text-align-center">
								<Link type="text" to="/" className="guides" style={{ cursor: "pointer" }}>
									Export transaction history
								</Link>
								<Link type="text" to="/" className="guides margin" style={{ cursor: "pointer" }}>
									Export token list
								</Link>
							</div>
							<div className="space-20 showonmobile"></div>
						</div>
					</div>
				</div>
				<div className="space-30"></div>
			</div>
			<div className="projects">
				<div className="container">
					<div className="container-in">
						{contracts &&
							projects?.data.map((project) => (
								<div className="col-6 col-m-full fl" key={project.forest_project.id}>
									<ProjectCardOwned project={project} user={user} />
								</div>
							))}
						<div className="clr"></div>
					</div>
				</div>
			</div>
			{contracts && claimsPopup && yieldsClaimable && yields ? (
				<ClaimPopup
					user={user}
					contracts={contracts}
					yieldsClaimable={yieldsClaimable}
					yieldsDisplay={yields}
					close={() => setClaimsPopup(false)}
					onClaimed={() => {
						setRefreshCounter((c) => c + 1);
						setClaimsPopup(false);
					}}
				/>
			) : null}
		</>
	);
}
