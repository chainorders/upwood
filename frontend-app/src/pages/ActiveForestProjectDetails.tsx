import { useEffect, useRef, useState } from "react";
import Button from "../components/Button";
import BuyShare, { BuyShareConfig } from "../components/BuyShare";
import {
	ForestProjectAggApiModel,
	ForestProjectPrice,
	ForestProjectService,
	ForestProjectState,
	LoginRes,
} from "../apiClient";
import { useOutletContext, useParams } from "react-router";
import MultiImageLayout from "../components/MultiImageLayout";
import SingleImageLayout from "../components/SingleImageLayout";
import PageHeader from "../components/PageHeader";
import { useTitle } from "../components/useTitle";

export default function ActiveForestProjectDetails() {
	const { id } = useParams<{ id: string }>();
	const { user } = useOutletContext<{ user: LoginRes }>();
	const [project, setProject] = useState<ForestProjectAggApiModel>();
	const [projectPrice, setProjectPrice] = useState<ForestProjectPrice>();
	const [buyShareConfig, setBuyShareConfig] = useState<BuyShareConfig>();
	const [comingSoon, setComingSoon] = useState(false);

	const [buyShare, setBuyShare] = useState(false);
	const openBuyShare = () => {
		setBuyShare(true);
	};
	const closeBuyShare = () => {
		setBuyShare(false);
	};
	useEffect(() => {
		ForestProjectService.getForestProjects(id!).then((response) => {
			setProject(response);
		});
		ForestProjectService.getAdminForestProjectsPriceLatest(id!).then((response) => {
			setProjectPrice(response);
		});
	}, [id]);
	useEffect(() => {
		if (project?.forest_project.state === ForestProjectState.DRAFT) {
			setComingSoon(true);
		} else {
			setComingSoon(false);
		}

		if (project && projectPrice) {
			setBuyShareConfig({
				heading: "Buy shares",
				title: project.forest_project.name,
				share_price: BigInt(projectPrice.price || 0),
				share_available: BigInt(project.forest_project.shares_available),
			});
		} else {
			setBuyShareConfig(undefined);
		}
	}, [project, projectPrice]);
	useTitle(project?.forest_project.name);

	const tabs_data = [
		{
			name: "OFFERING DOCUMENTATION",
			active: true,
		},
		{
			name: "FINANCIAL PROJECTIONS",
			active: false,
		},
		{
			name: "PROPERTY MEDIA",
			active: false,
		},
		{
			name: "GEOSPATIAL DATA",
			active: false,
		},
	];

	const [tabs, setTabs] = useState(tabs_data);
	const contentRef = useRef<HTMLDivElement>(null);
	function changeTab(index: number) {
		setTabs((prevTabs) =>
			prevTabs.map((tab, i) => ({
				...tab,
				active: i === index,
			})),
		);
		if (contentRef.current) {
			contentRef.current.scrollIntoView({ behavior: "smooth" });
		}
	}
	const multiimagedata = {
		title: "Property media",
		short:
			"Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua.",
		large:
			"Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum.",
		images: ["/Photo2.jpg", "/Photo2.jpg", "/Photo2.jpg", "/Photo2.jpg"],
	};
	const singleimagedata = {
		title: "Title 2",
		short:
			"Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua.",
		large:
			"Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum.",
		image: "/Photo2.jpg",
	};

	useEffect(() => {
		const hash = window.location.hash;
		if (hash) {
			const id = hash.substring(1);
			console.log(id);
		}
	}, []);
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
					<img
						src={project?.forest_project.image_large_url}
						// layout="responsive"
						width={100}
						height={100}
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
							<Button text={"INVEST"} link={""} active={true} call={openBuyShare} />
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
					{tabs.map((item_s, index) => (
						<li key={index} className={`${item_s.active ? "active" : ""}`} onClick={() => changeTab(index)}>
							{item_s.name}
						</li>
					))}
				</ul>
				<div className="clr"></div>
				{tabs.map((item_s, index) => (
					<div key={index}>
						{item_s.active ? (
							<div className="tabular-content">
								<MultiImageLayout data={multiimagedata} />
								<div className="space-30"></div>
								<SingleImageLayout data={singleimagedata} />
							</div>
						) : null}
					</div>
				))}
			</div>
			{buyShare && buyShareConfig ? <BuyShare config={buyShareConfig} close={closeBuyShare} /> : null}
		</>
	);
}
