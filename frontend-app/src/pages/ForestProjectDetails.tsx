import { useEffect, useRef, useState } from "react";
import Button from "../components/Button";
import FundInvest from "../components/FundInvest";
import {
	ForestProjectAggApiModel,
	ForestProjectMedia,
	ForestProjectService,
	ForestProjectState,
	PagedResponse_ForestProjectMedia,
	SecurityMintFundState,
	SystemContractsConfigApiModel,
	UserService,
} from "../apiClient";
import { useLocation, useParams } from "react-router";
import SingleImageLayout from "../components/SingleImageLayout";
import PageHeader from "../components/PageHeader";
import { useTitle } from "../components/useTitle";
import { User } from "../lib/user";
import MarketBuy from "../components/MarketBuy";
import { LazyLoadImage } from "react-lazy-load-image-component";
import "react-lazy-load-image-component/src/effects/opacity.css";

function ForestProjectMediaSection({
	project,
	medias,
}: {
	project: ForestProjectAggApiModel;
	medias: ForestProjectMedia[];
}) {
	return (
		<>
			<div className="title">Property Media</div>
			<div className="description">{project.forest_project.property_media_header}</div>
			<div className="images">
				<div className="container">
					<div className="container-in">
						{medias.map((im, index) => (
							<div className="col-3 col-m-full col-mr-bottom-10 fl" key={index}>
								<div className="im">
									<LazyLoadImage
										src={im.image_url}
										alt=""
										width="100%"
										height="auto"
										style={{ objectFit: "cover", maxHeight: "200px" }}
										effect="opacity"
									/>
								</div>
							</div>
						))}
						<div className="clr"></div>
					</div>
				</div>
			</div>
			<div className="description">{project.forest_project.property_media_footer}</div>
		</>
	);
}

interface Props {
	source: ForestProjectState;
	user: User;
}

export default function ForestProjectDetails({ user, source }: Props) {
	const { id } = useParams<{ id: string }>();
	const location = useLocation();
	const [project, setProject] = useState<ForestProjectAggApiModel>(location.state?.project);
	const [contracts, setContracts] = useState<SystemContractsConfigApiModel>();
	const [medias, setMedias] = useState<PagedResponse_ForestProjectMedia>();
	const [tabIndex, setTabIndex] = useState(0);

	const [bondFundPopup, setBondFundPopup] = useState(false);
	const openBondFundPopup = () => {
		setBondFundPopup(true);
	};
	const closeBondFundPopup = () => {
		setBondFundPopup(false);
	};
	const [propertyFundPopup, setPropertyFundPopup] = useState(false);
	const openPropertyFundPopup = () => {
		setPropertyFundPopup(true);
	};
	const closePropertyFundPopup = () => {
		setPropertyFundPopup(false);
	};
	const [marketBuyPopup, setMarketBuyPopup] = useState(false);
	const openMarketBuyPopup = () => {
		setMarketBuyPopup(true);
	};
	const closeMarketBuyPopup = () => {
		setMarketBuyPopup(false);
	};
	useEffect(() => {
		UserService.getSystemConfig().then(setContracts);
		ForestProjectService.getForestProjects(id!).then(setProject);
		ForestProjectService.getForestProjectsMediaList(id!, 0, 5).then(setMedias);
	}, [id]);
	useTitle(project?.forest_project.name);
	const contentRef = useRef<HTMLDivElement>(null);

	const headerParts = [];
	if (source === ForestProjectState.ACTIVE) {
		headerParts.push({ name: "Active Projects", link: "/projects/active" });
	} else if (source === ForestProjectState.FUNDED) {
		headerParts.push({ name: "Funded Projects", link: "/projects/funded" });
	} else if (source === ForestProjectState.BOND) {
		headerParts.push({ name: "Investment Bonds", link: "/projects/bond" });
	}
	headerParts.push({ name: project?.forest_project.name || "", link: "" });

	const comingSoon = project?.forest_project.state === ForestProjectState.DRAFT;
	return (
		<>
			<div className="project-detail">
				<PageHeader user={user} parts={headerParts} />
				<div className="image">
					<LazyLoadImage
						src={project?.forest_project.image_large_url}
						alt="Project Image"
						effect="opacity"
						width="100%"
						height="100"
						placeholderSrc="https://placehold.co/600x400?text=Loading"
					/>
					<div className="caption">{project?.forest_project.label}</div>
				</div>
				<div className="space-30"></div>
				<div className="container">
					<div className="container-in">
						<div className="col-9 col-m-full col-mr-bottom-20 fl">
							<div className="project-name">{comingSoon ? "To be announced" : project?.forest_project.name}</div>
						</div>
						<div className="col-3 col-m-full fr">
							{
								{
									[ForestProjectState.DRAFT]: <Button text="COMING SOON" disabled={true} />,
									[ForestProjectState.ACTIVE]: (
										<Button
											text="INVEST"
											active
											call={openPropertyFundPopup}
											disabled={!project?.property_fund || project.property_fund.fund_state != SecurityMintFundState.OPEN}
										/>
									),
									[ForestProjectState.FUNDED]: (
										<Button text="BUY" active call={openMarketBuyPopup} disabled={!project?.property_market} />
									),
									[ForestProjectState.ARCHIVED]: <Button text="ARCHIVED" disabled={true} />,
									[ForestProjectState.BOND]: (
										<Button
											text="INVEST"
											active
											call={openBondFundPopup}
											disabled={!project?.bond_fund || project.bond_fund.fund_state != SecurityMintFundState.OPEN}
										/>
									),
								}[source]
							}
						</div>
						<div className="clr"></div>
					</div>
				</div>
				<div className="container">
					<div className="container-in">
						<div className="col-12">
							<div className="project-description">{project?.forest_project.desc_long}</div>
						</div>
					</div>
				</div>
				<div className="space-30"></div>
				<div className="container-in">
					<div className="col-20-percent col-m-padding-right-0 fl">
						<span className="colb">AREA</span>
						<span className="colc">{comingSoon ? "TBA" : project?.forest_project.area}</span>
					</div>
					<div className="col-20-percent col-m-padding-right-0 fl">
						<span className="colb">ROI</span>
						<span className="colc">{comingSoon ? "TBA" : project?.forest_project.roi_percent}%</span>
					</div>
					<div className="col-20-percent col-m-padding-right-0 fl">
						<span className="colb">CARBON CREDITS</span>
						<span className="colc">{comingSoon ? "TBA" : project?.forest_project.carbon_credits}</span>
					</div>
					<div className="col-20-percent col-m-padding-right-0 fl">
						<span className="colb">SHARES AVAILABLE</span>
						<span className="colc">{comingSoon ? "TBA" : project?.forest_project.shares_available}</span>
					</div>
					<div className="col-20-percent col-m-padding-right-0 fl">
						<span className="colb">SHARES RESERVED</span>
						<span className="colc">{comingSoon ? "TBA" : project?.supply}</span>
					</div>
					<div className="clr"></div>
				</div>
				<div className="space-30" ref={contentRef}></div>
				<ul className="tabular">
					<li className={tabIndex === 0 ? "active" : ""} onClick={() => setTabIndex(0)}>
						PROPERTY MEDIA
					</li>
					<li className={tabIndex === 1 ? "active" : ""} onClick={() => setTabIndex(1)}>
						OFFERING DOCUMENTATION
					</li>
					<li className={tabIndex === 2 ? "active" : ""} onClick={() => setTabIndex(2)}>
						FINANCIAL PROJECTIONS
					</li>
					<li className={tabIndex === 3 ? "active" : ""} onClick={() => setTabIndex(3)}>
						GEOSPATIAL DATA
					</li>
				</ul>
				<div className="clr"></div>
				<div>
					<div className="tabular-content">
						<div className="multiimage">
							{
								{
									0: project && <ForestProjectMediaSection project={project!} medias={medias?.data || []} />,
									1: (
										<SingleImageLayout
											data={{
												title: project?.forest_project.offering_doc_title || "Offering Documentation",
												header: project?.forest_project.offering_doc_header,
												footer: project?.forest_project.offering_doc_footer,
												image: project?.forest_project.offering_doc_img_url,
											}}
										/>
									),
									2: (
										<SingleImageLayout
											data={{
												title: project?.forest_project.financial_projection_title || "Financial Projections",
												header: project?.forest_project.financial_projection_header,
												footer: project?.forest_project.financial_projection_footer,
												image: project?.forest_project.financial_projection_img_url,
											}}
										/>
									),
									3: (
										<SingleImageLayout
											data={{
												title: project?.forest_project.geo_title || "Geospatial Data",
												header: project?.forest_project.geo_header,
												footer: project?.forest_project.geo_img_url,
												image: project?.forest_project.geo_img_url,
											}}
										/>
									),
								}[tabIndex]
							}
						</div>
						<div className="space-30"></div>
					</div>
				</div>
			</div>
			{bondFundPopup && project?.bond_fund && contracts ? (
				<FundInvest
					close={closeBondFundPopup}
					user={user}
					contracts={contracts}
					fund={project.bond_fund}
					tokenContract={project.bond_contract}
					project={project.forest_project}
					supply={project.supply}
					legalContractSigned={project.contract_signed}
				/>
			) : null}
			{propertyFundPopup && project?.property_fund && contracts ? (
				<FundInvest
					close={closePropertyFundPopup}
					user={user}
					contracts={contracts}
					fund={project.property_fund}
					tokenContract={project.property_contract}
					project={project.forest_project}
					supply={project.supply}
					legalContractSigned={project.contract_signed}
				/>
			) : null}
			{marketBuyPopup && project?.property_market && contracts ? (
				<MarketBuy
					close={closeMarketBuyPopup}
					user={user}
					contracts={contracts}
					market={project.property_market}
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
