import { useState } from "react";

import { ForestProjectAggApiModel, ForestProjectState } from "../apiClient";
import { User } from "../lib/user";
import Button from "./Button";
import { LazyLoadImage } from "react-lazy-load-image-component";
import "react-lazy-load-image-component/src/effects/opacity.css";
import MarketBuyCombined from "./MarketBuyCombined";

interface Props {
	project: ForestProjectAggApiModel;
	user: User;
}

export default function ProjectCardFunded({ project, user }: Props) {
	const [propertyMarketBuyPopup, setPropertyMarketBuyPopup] = useState(false);

	const comingSoon = project.forest_project.state === ForestProjectState.DRAFT;
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
							<Button
								text="VIEW DETAILS"
								link={`/projects/funded/${project.forest_project.id}`}
								active={false}
								linkState={{ project }}
							/>
						</div>
						<div className="col-4 col-m-full fl">
							<Button
								text="BUY"
								active
								call={() => setPropertyMarketBuyPopup(true)}
								disabled={!project.property_contract || !project.property_market}
							/>
						</div>
						<div className="clr"></div>
					</div>
				</div>
			</div>
			{propertyMarketBuyPopup && project.property_contract && project.property_market && (
				<MarketBuyCombined
					project={project}
					user={user}
					token_contract={project.property_contract}
					market={project.property_market}
					close={() => setPropertyMarketBuyPopup(false)}
				/>
			)}
		</>
	);
}
