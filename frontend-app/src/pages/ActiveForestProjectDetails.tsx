import { useEffect, useRef, useState } from "react";
import Button from "../components/Button";
import FundInvest from "../components/FundInvest";
import {
	ForestProjectAggApiModel,
	ForestProjectMedia,
	ForestProjectService,
	ForestProjectState,
	PagedResponse_ForestProjectMedia_,
	SecurityMintFundState,
	SystemContractsConfigApiModel,
	UserService,
} from "../apiClient";
import { useOutletContext, useParams } from "react-router";
import SingleImageLayout from "../components/SingleImageLayout";
import PageHeader from "../components/PageHeader";
import { useTitle } from "../components/useTitle";
import { User } from "../lib/user";

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
									<img src={im.image_url} width={100} height={100} />
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

export default function ActiveForestProjectDetails() {
	const { id } = useParams<{ id: string }>();
	const { user } = useOutletContext<{ user: User }>();
	const [project, setProject] = useState<ForestProjectAggApiModel>();
	const [contracts, setContracts] = useState<SystemContractsConfigApiModel>();
	const [medias, setMedias] = useState<PagedResponse_ForestProjectMedia_>();
	const [tabIndex, setTabIndex] = useState(0);

	const [buyShare, setBuyShare] = useState(false);
	const openBuyShare = () => {
		setBuyShare(true);
	};
	const closeBuyShare = () => {
		setBuyShare(false);
	};
	useEffect(() => {
		UserService.getSystemConfig().then(setContracts);
		ForestProjectService.getForestProjects(id!).then(setProject);
		ForestProjectService.getForestProjectsMediaList(id!, 0).then(setMedias);
	}, [id]);
	useTitle(project?.forest_project.name);
	const contentRef = useRef<HTMLDivElement>(null);
	const singleimagedata = {
		title: "Title 2",
		short:
			"Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua.",
		large:
			"Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum.",
		image: "/Photo2.jpg",
	};

	const comingSoon = project?.forest_project.state === ForestProjectState.DRAFT;
	return (
		<>
			<div className="project-detail">
				<PageHeader
					user={user}
					parts={[
						{ name: "Active Projects", link: "/projects/active" },
						{ name: project?.forest_project.name || "", link: "" },
					]}
				/>
				<div className="image">
					<img src={project?.forest_project.image_large_url} width={100} height={100} />
					<div className="caption">{project?.forest_project.label}</div>
				</div>
				<div className="space-30"></div>
				<div className="container">
					<div className="container-in">
						<div className="col-9 col-m-full col-mr-bottom-20 fl">
							<div className="project-name">{comingSoon ? "To be announced" : project?.forest_project.name}</div>
						</div>
						<div className="col-3 col-m-full fr">
							<Button
								text={"INVEST"}
								link={""}
								active={true}
								call={openBuyShare}
								disabled={!project?.property_fund || project.property_fund.fund_state != SecurityMintFundState.OPEN}
							/>
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
									1: <SingleImageLayout data={singleimagedata} />,
									2: <SingleImageLayout data={singleimagedata} />,
									3: <SingleImageLayout data={singleimagedata} />,
								}[tabIndex]
							}
						</div>
						<div className="space-30"></div>
					</div>
				</div>
			</div>
			{buyShare && project?.property_fund && contracts ? (
				<FundInvest
					close={closeBuyShare}
					user={user}
					contracts={contracts}
					fund={project.property_fund}
					tokenContract={project.property_contract}
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
