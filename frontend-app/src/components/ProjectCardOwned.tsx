import { useState } from "react";
import { ForestProjectAggApiModel, MarketType } from "../apiClient";
import { User } from "../lib/user";
import Button from "./Button";
import MarketSell from "./MarketSell";
import { LazyLoadImage } from "react-lazy-load-image-component";
import "react-lazy-load-image-component/src/effects/opacity.css";
import MarketBuyCombined from "./MarketBuyCombined";

interface Props {
	project: ForestProjectAggApiModel;
	user: User;
}

export default function ProjectCardOwned({ project, user }: Props) {
	const [bondMarketBuyPopup, setBondMarketBuyPopup] = useState(false);
	const [propertyMarketSellPopup, setPropertyMarketSellPopup] = useState(false);

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
							<Button
								text="SELL SHARES"
								call={() => setPropertyMarketSellPopup(true)}
								disabled={
									!project.property_contract ||
									!project.property_market ||
									project.property_market.market_type !== MarketType.TRANSFER
								}
							/>
						</div>
						<div className="col-5 col-m-full fr">
							<Button
								text="INVEST MORE"
								active
								call={() => setBondMarketBuyPopup(true)}
								disabled={!project.bond_contract || !project.bond_market}
							/>
						</div>
						<div className="clr"></div>
					</div>
				</div>
			</div>
			{bondMarketBuyPopup && project.bond_contract && project.bond_market && (
				<MarketBuyCombined
					project={project}
					user={user}
					token_contract={project.bond_contract}
					market={project.bond_market}
					close={() => setBondMarketBuyPopup(false)}
				/>
			)}
			{propertyMarketSellPopup &&
			project.property_contract &&
			project.property_market &&
			project.property_market.market_type === MarketType.TRANSFER ? (
				<MarketSell
					close={() => setPropertyMarketSellPopup(false)}
					user={user}
					market={{
						buy_rate_numerator: project.property_market.buy_rate_numerator!,
						buy_rate_denominator: project.property_market.buy_rate_denominator!,
						currency_token_contract_address: project.property_market.currency_token_contract_address!,
						liquidity_provider: project.property_market.liquidity_provider!,
						token_contract_address: project.property_market.token_contract_address!,
						token_id: project.property_market.token_id!,
						contract_address: project.property_contract.contract_address,
						max_currency_amount: project.property_market.max_currency_amount!,
						max_token_amount: project.property_market.max_token_amount,
					}}
					tokenContract={project.property_contract}
					project={project.forest_project}
					supply={project.supply}
					legalContractSigned={project.contract_signed}
					userNotified={project.user_notified}
				/>
			) : null}
		</>
	);
}
