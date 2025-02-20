import { useState } from "react";

import {
	ForestProjectAggApiModel,
	ForestProjectState,
	SecurityMintFundState,
	SystemContractsConfigApiModel,
} from "../apiClient";
import { User } from "../lib/user";
import Button from "./Button";
import FundInvest from "./FundInvest";

interface Props {
	project: ForestProjectAggApiModel;
	user: User;
	contracts: SystemContractsConfigApiModel;
}

export default function ProjectCardBond({ project, user, contracts }: Props) {
	const [fundInvestPopupOpen, setFundInvestPopupOpen] = useState(false);
	const comingSoon = project.forest_project.state === ForestProjectState.DRAFT;

	return (
		<>
			<div className="project-card">
				<div className="container">
					<div className="container-in">
						<div className="col-12">
							<div className="image">
								<img src={project.forest_project.image_small_url} />
								<div className="caption">{comingSoon ? "coming soon" : project.forest_project.label}</div>
							</div>
						</div>
					</div>
				</div>
				<div className={`container ${comingSoon ? "disable-overlay" : ""}`}>
					<div className="container-in">
						<div className="col-12">
							<div className="project-name">{comingSoon ? "To be announced" : project.forest_project.name}</div>
							<div className="project-description">
								{comingSoon ? "Description coming soon, please wait" : project.forest_project.desc_short}
							</div>
						</div>
					</div>
					<div className="container-in">
						<div className="col-4 col-m-padding-right-0 fl">
							<span className="colb">AREA</span>
							<span className="colc">{comingSoon ? "TBA" : project.forest_project.area}</span>
						</div>
						<div className="col-4 col-m-padding-right-0 fl">
							<span className="colb">ROI</span>
							<span className="colc">{comingSoon ? "TBA" : project.forest_project.roi_percent}%</span>
						</div>
						<div className="col-4 col-m-padding-right-0 fl">
							<span className="colb">CARBON CREDITS</span>
							<span className="colc">{comingSoon ? "TBA" : project.forest_project.carbon_credits}</span>
						</div>
						<div className="clr"></div>
					</div>
					<div className="container-in">
						<div className="col-8 col-m-full col-mr-bottom-20 fl">
							<Button text="VIEW DETAILS" link={`/projects/bond/${project.forest_project.id}`} active={false} />
						</div>
						<div className="col-4 col-m-full fl">
							<Button
								text="INVEST"
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
					currencyMetadata={project.property_fund_currency_metadata}
					project={project.forest_project}
					supply={project.supply}
					legalContractSigned={project.contract_signed}
				/>
			) : (
				<></>
			)}
		</>
	);
}
