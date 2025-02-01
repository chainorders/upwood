import { useEffect, useRef, useState } from "react";
import Button from "../components/Button";
import NotifyShare, { NotifyShareConfig } from "../components/NotifyShare";
import { ForestProjectAggApiModel, ForestProjectState, LoginRes } from "../apiClient";
import { useOutletContext, useParams } from "react-router";
import NotifyNon from "../assets/notify-non.svg";
import NotifySuccess from "../assets/notify-success.svg";
import PageHeader from "../components/PageHeader";
import { useTitle } from "../components/useTitle";

export default function FundedForestProjectDetails() {
	const { id } = useParams<{ id: string }>();
	const { user } = useOutletContext<{ user: LoginRes }>();
	const [project, setProject] = useState<ForestProjectAggApiModel>();
	const [notifyShareConfig, setNotifyShareConfig] = useState<NotifyShareConfig>();
	const [comingSoon, setComingSoon] = useState(false);

	const [notifyShare, setNotifyShare] = useState(false);
	const openNotifyShare = () => {
		setNotifyShare(true);
	};
	const closeNotifyShare = () => {
		setNotifyShare(false);
	};
	useEffect(() => {
		const project = {
			forest_project: {
				area: "100",
				carbon_credits: 100,
				created_at: "2021-09-07T12:00:00Z",
				desc_long: "This is a long description",
				desc_short: "This is a short description",
				id: id!,
				image_small_url: "https://picsum.photos/id/237/400/200",
				image_large_url: "https://picsum.photos/id/237/1080/600",
				label: "Grow",
				latest_price: "100",
				name: "Project Name",
				property_media_footer: "Property Media Footer",
				property_media_header: "Property Media Header",
				roi_percent: 100,
				shares_available: 100,
				state: ForestProjectState.ACTIVE,
				updated_at: "2021-09-07T12:00:00Z",
				geo_spatial_url: "https://via.placeholder.com/150",
				offering_doc_link: "https://via.placeholder.com/150",
			},
			supply: "100",
			user_balance: "100",
		};
		setProject(project);
		setNotifyShareConfig({
			heading: "Notify of available tokens",
			title: project.forest_project.name,
			share_price: BigInt(project.forest_project.latest_price),
			share_available: BigInt(project.forest_project.shares_available),
		});
		setComingSoon(false);
	}, [id]);
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

	return (
		<>
			<div className="project-detail">
				<PageHeader
					user={user}
					parts={[
						{ name: "Funded Projects", link: "/projects/active" },
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
						<div className="col-10 col-m-full col-mr-bottom-20 fl">
							<div className="project-name">{comingSoon ? "To be announced" : project?.forest_project.name}</div>
						</div>
						<div className="col-2 col-m-full fr">
							<Button
								icon={NotifyNon}
								style={"style4 text-align-left-on-mob"}
								text={"NOTIFY ME"}
								link={""}
								active={false}
								call={openNotifyShare}
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
						<span className="colc">{comingSoon ? "TBA" : project?.forest_project.carbon_credits}%</span>
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
			</div>
			{notifyShare && notifyShareConfig ? <NotifyShare config={notifyShareConfig} close={closeNotifyShare} /> : null}
		</>
	);
}
