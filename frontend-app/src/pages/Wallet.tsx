import { useEffect, useState } from "react";
import {
	ForestProjectService,
	ForestProjectTokenUserYieldClaim,
	PagedResponse_ForestProjectAggApiModel_,
	PagedResponse_ForestProjectTokenContractAggApiModel_,
	SystemContractsConfigApiModel,
	UserService,
} from "../apiClient";
import PageHeader from "../components/PageHeader";
import { useOutletContext } from "react-router";
import ProjectCardOwned from "../components/ProjectCardOwned";
import ClaimPopup from "../components/ClaimPopup";
import { Link } from "react-router";
import { User } from "../lib/user";
import "./Wallet.css"; // Add this line to import the CSS file
import { toDisplayAmount, toTokenId } from "../lib/conversions";
import { TxnStatus, updateContract } from "../lib/concordium";
import securitySftMultiYielder from "../contractClients/generated/securitySftMultiYielder";

export default function Wallet() {
	const { user } = useOutletContext<{ user: User }>();
	const [carbonCreditsPopup, setCarbonCreditsPopup] = useState(false);
	const [dividendsDetailsPopup, setDividendsPopup] = useState(false);
	const [eTreesPopup, setEtreesPopup] = useState(false);
	const [projects, setProjects] = useState<PagedResponse_ForestProjectAggApiModel_>();
	const [yields, setYields] = useState<{
		carbonCredits: string;
		euroE: string;
		eTrees: string;
	}>();
	const [contracts, setContracts] = useState<SystemContractsConfigApiModel>();
	const [ownedTokenContracts, setOwnedTokenContracts] = useState<PagedResponse_ForestProjectTokenContractAggApiModel_>();
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
	const [yieldTxnStatus, setYieldTxnStatus] = useState<TxnStatus>("none");
	const [refreshCounter, setRefreshCounter] = useState(0);

	useEffect(() => {
		ForestProjectService.getForestProjectsListOwned().then(setProjects);
		UserService.getSystemConfig().then(setContracts);
	}, [user, refreshCounter]);
	useEffect(() => {
		if (contracts) {
			ForestProjectService.getForestProjectsYieldsTotal().then((response) => {
				const carbon_credit_yield = response.find(
					(r) =>
						r.yield_token_id === contracts.carbon_credit_token_id &&
						r.yield_contract_address === contracts.carbon_credit_contract_index,
				);
				const euro_e_yield = response.find(
					(r) =>
						r.yield_token_id === contracts.euro_e_token_id && r.yield_contract_address === contracts.euro_e_contract_index,
				);
				const etrees_yield = response.find((r) => r.yield_contract_address === contracts.tree_ft_contract_index);
				setYields({
					carbonCredits: carbon_credit_yield?.yield_amount || "0",
					euroE: euro_e_yield?.yield_amount || "0",
					eTrees: etrees_yield?.yield_amount || "0",
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

	const claimYields = async () => {
		if (!contracts || !yieldsClaimable) {
			return;
		}

		try {
			await updateContract(
				user.concordiumAccountAddress,
				contracts.yielder_contract_index,
				securitySftMultiYielder.yieldFor,
				{
					owner: user.concordiumAccountAddress,
					yields: yieldsClaimable.map((y) => ({
						token_ver_from: toTokenId(BigInt(y.token_id), 8),
						token_ver_to: toTokenId(BigInt(y.max_token_id), 8),
						token_contract: { index: Number(y.token_contract_address), subindex: 0 },
						amount: y.token_balance,
					})),
				},
				setYieldTxnStatus,
			);
			alert("Yields claimed successfully");
			setRefreshCounter((c) => c + 1);
		} catch (e) {
			console.error(e);
			alert("Failed to claim yields");
		}
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
								<Link type="text" to="/" className="guides" style={{ cursor: "pointer" }}>
									WALLET MANAGEMENT GUIDES
								</Link>
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
								<button onClick={() => setCarbonCreditsPopup(true)}>Change</button>
							</div>
							<div className="col-20-percent fl walletclms col-m-full col-mr-bottom-30">
								<div className="tag">Entity</div>
								<div className="value">SIA UPWOOD</div>
							</div>
							<div className="col-20-percent fl walletclms col-m-full col-mr-bottom-30">
								<div className="tag">Carbon credits</div>
								<div className="value">
									{toDisplayAmount(yields?.carbonCredits || "0", contracts?.carbon_credit_metadata.decimals || 0, 0)} = {0}€
								</div>
							</div>
							<div className="col-10-percent fl walletclms col-m-full col-mr-bottom-30">
								<div className="tag">Dividends</div>
								<div className="value">
									{toDisplayAmount(yields?.euroE || "0", contracts?.euro_e_metadata.decimals || 0, 2)}
								</div>
							</div>
							<div className="col-20-percent fl walletclms col-m-full col-mr-bottom-30">
								<div className="tag">E-trees</div>
								<div className="value">
									{toDisplayAmount(yields?.eTrees || "0", contracts?.tree_ft_metadata.decimals || 0, 0)}
								</div>
							</div>
							<div className="col-10-percent fl walletclms col-m-full">
								<div className="tag">Claim Rewards</div>
								<div className="value"></div>
								<button onClick={claimYields} disabled={yieldsClaimable?.length === 0}>
									Claim <br />
									All
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
								<Link type="text" to="/" className="guides" style={{ cursor: "pointer" }}>
									Export transaction history
								</Link>
								<Link type="text" to="/" className="guides margin" style={{ cursor: "pointer" }}>
									Export token list
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
													<td>{toDisplayAmount(item.user_balance_price, contracts?.euro_e_metadata.decimals || 6, 2)}</td>
													<td>{item.carbon_credit_yield_balance}</td>
													<td>{toDisplayAmount(item.euro_e_yields_balance, contracts?.euro_e_metadata.decimals || 6, 2)}</td>
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
									<ProjectCardOwned project={project} user={user} contracts={contracts} />
								</div>
							))}
						<div className="clr"></div>
					</div>
				</div>
			</div>
			{/* {carbonCreditsPopup ? (
				<ClaimPopup config={__carbon_credits_details} close={() => setCarbonCreditsPopup(false)} />
			) : null} */}
			{/* {dividends_details_popup ? <ClaimPopup config={__dividends_details} close={() => setDividendsPopup(false)} /> : null} */}
			{/* {etrees_popup ? <ClaimPopup config={__etrees_details} close={() => setEtreesPopup(false)} /> : null} */}
		</>
	);
}
