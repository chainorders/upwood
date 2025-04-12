import { useState } from "react";
import { ForestProjectAggApiModel, SecurityMintFundState, SystemContractsConfigApiModel } from "../apiClient";
import { User } from "../lib/user";
import Button from "./Button";
import FundInvest from "./FundInvest";
import MarketSell from "./MarketSell";
import { LazyLoadImage } from "react-lazy-load-image-component";
import "react-lazy-load-image-component/src/effects/opacity.css";

interface Props {
	project: ForestProjectAggApiModel;
	user: User;
	contracts: SystemContractsConfigApiModel;
}

export default function ProjectCardOwned({ project, user, contracts }: Props) {
	const [fundInvestPopupOpen, setFundInvestPopupOpen] = useState(false);
	const [marketSellPopup, setMarketSellPopup] = useState(false);

	return (
		<>
			<div className="project-card">
				<div className="container">
					<div className="container-in">
						<div className="col-12">
							<div className="image">
								<LazyLoadImage
									src={project.forest_project.image_small_url}
									alt="Project Image"
									effect="opacity"
									width="100%"
									height="100%"
									placeholderSrc="https://placehold.co/600x400?text=Loading"
								/>
								<div className="caption">{project.forest_project.label}</div>
							</div>
						</div>
					</div>
				</div>
				<div className={`container`}>
					<div className="container-in">
						<div className="col-12">
							<div className="project-name">{project.forest_project.name}</div>
							<div className="project-description">{project.forest_project.desc_short}</div>
						</div>
					</div>
					<div className="container-in">
						<div className="col-4 col-m-padding-right-0 fl">
							<span className="colb">AREA</span>
							<span className="colc">{project.forest_project.area}</span>
						</div>
						<div className="col-4 col-m-padding-right-0 fl">
							<span className="colb">ROI</span>
							<span className="colc">{project.forest_project.roi_percent}%</span>
						</div>
						<div className="col-4 col-m-padding-right-0 fl">
							<span className="colb">CARBON CREDITS</span>
							<span className="colc">{project.forest_project.carbon_credits}</span>
						</div>
						<div className="clr"></div>
					</div>
					<div className="container-in">
						<div className="col-5 col-m-full col-mr-bottom-20 fl">
							<Button text="SELL SHARES" call={() => setMarketSellPopup(true)} disabled={!project.property_market} />
						</div>
						<div className="col-5 col-m-full fr">
							<Button
								text="INVEST MORE"
								active
								call={() => setFundInvestPopupOpen(true)}
								disabled={!project.bond_fund || project.bond_fund.fund_state != SecurityMintFundState.OPEN}
							/>
						</div>
						<div className="clr"></div>
					</div>
				</div>
			</div>
			{fundInvestPopupOpen && project.bond_fund ? (
				<FundInvest
					close={() => setFundInvestPopupOpen(false)}
					user={user}
					contracts={contracts}
					fund={project.bond_fund}
					tokenContract={project.bond_contract}
					project={project.forest_project}
					supply={project.supply}
					legalContractSigned={project.contract_signed}
				/>
			) : (
				<></>
			)}
			{marketSellPopup && project.property_market ? (
				<MarketSell
					close={() => setMarketSellPopup(false)}
					user={user}
					contracts={contracts}
					market={project.property_market}
					tokenContract={project.property_contract}
					project={project.forest_project}
					supply={project.supply}
					legalContractSigned={project.contract_signed}
					userNotified={project.user_notified}
				/>
			) : (
				<></>
			)}
		</>
	);
}
