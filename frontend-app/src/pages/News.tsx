import { useEffect, useState } from "react";
import { Link } from "react-router";
import PageHeader from "../components/PageHeader";
import NewsCard from "../components/NewsCard";
import { User } from "../lib/user";
import { MaintenanceMessage, PagedResponse_NewsArticle, PlatformUpdate, UserCommunicationService } from "../apiClient";

interface NewsProps {
	user: User;
}

export default function News({ user }: NewsProps) {
	const [showAll, setShowAll] = useState(false);
	const [newsArticles, setNewsArticles] = useState<PagedResponse_NewsArticle>();
	const [latestMaintenanceMessage, setLatestMaintenanceMessage] = useState<MaintenanceMessage>();

	useEffect(() => {
		UserCommunicationService.getNewsArticlesList(0).then(setNewsArticles);
		UserCommunicationService.getMaintenanceMessagesLatest().then(setLatestMaintenanceMessage);
	}, [user]);

	const links = [
		{
			title: "Update: 1.13",
			description: "Proactively incubate innovative processes for high-payoff architectures. Globally benchmark flexible.",
			link: "",
		},
		{
			title: "Update: 1.13",
			description: "Proactively incubate innovative processes for high-payoff architectures. Globally benchmark flexible.",
			link: "",
		},
		{
			title: "Update: 1.13",
			description: "Proactively incubate innovative processes for high-payoff architectures. Globally benchmark flexible.",
			link: "",
		},
		{
			title: "Update: 1.13",
			description: "Proactively incubate innovative processes for high-payoff architectures. Globally benchmark flexible.",
			link: "",
		},
		{
			title: "Update: 1.13",
			description: "Proactively incubate innovative processes for high-payoff architectures. Globally benchmark flexible.",
			link: "",
		},
		{
			title: "Update: 1.13",
			description: "Proactively incubate innovative processes for high-payoff architectures. Globally benchmark flexible.",
			link: "",
		},
		{
			title: "Update: 1.13",
			description: "Proactively incubate innovative processes for high-payoff architectures. Globally benchmark flexible.",
			link: "",
		},
		{
			title: "Update: 1.13",
			description: "Proactively incubate innovative processes for high-payoff architectures. Globally benchmark flexible.",
			link: "",
		},
		{
			title: "Update: 1.13",
			description: "Proactively incubate innovative processes for high-payoff architectures. Globally benchmark flexible.",
			link: "",
		},
		{
			title: "Update: 1.13",
			description: "Proactively incubate innovative processes for high-payoff architectures. Globally benchmark flexible.",
			link: "",
		},
		{
			title: "Update: 1.13",
			description: "Proactively incubate innovative processes for high-payoff architectures. Globally benchmark flexible.",
			link: "",
		},
	];

	const displayedLinks = showAll ? links : links.slice(0, 3);
	return (
		<>
			<div className="clr"></div>
			<div className="news">
				<PageHeader user={user} parts={[{ name: "News & Updates" }]} />
				<div className="outerboxshadow">
					<div className="container">
						<div className="container-in">
							{latestMaintenanceMessage && (
								<div className="col-4 fr text-align-right col-m-full col-mr-bottom-10 text-align-left-mobile">
									{new Date(latestMaintenanceMessage.created_at).toLocaleDateString()}
								</div>
							)}
							<div className="col-8 fl col-m-full">
								<div className="heading">Planned platform maintenance</div>
							</div>
							<div className="clr"></div>
						</div>
						<div className="container-in">
							<div className="col-12">
								<div className="content">{latestMaintenanceMessage?.message || "No planned platform maintenance"}</div>
							</div>
						</div>
					</div>
				</div>
				<div className="space-30"></div>
				<div className="outerboxshadow">
					<div className="container">
						<div className="container-in">
							<div className="col-8 col-m-full fl">
								<div className="heading">Platform updates</div>
							</div>
							<div className="col-4 text-align-right fr hideonmobile">
								<span className="seeall" onClick={() => setShowAll(!showAll)} style={{ cursor: "pointer" }}>
									{showAll ? "SEE LESS" : "SEE ALL"}
								</span>
							</div>

							<div className="clr"></div>
							{displayedLinks.map((item, index) => (
								<div className="col-4 col-m-full fl" key={index}>
									<div className="linkbox">
										<Link to={item.link}>
											<div className="title">{item.title}</div>
											<div className="description">{item.description}</div>
										</Link>
									</div>
								</div>
							))}
							<div className="clr"></div>
							<div className="col-12 showonmobile text-align-center">
								<span className="seeall" onClick={() => setShowAll(!showAll)} style={{ cursor: "pointer" }}>
									{showAll ? "SEE LESS" : "SEE ALL"}
								</span>
							</div>
							<div className="space-20 showonmobile"></div>
						</div>
					</div>
				</div>
				<div className="space-30"></div>
				<div className="container">
					<div className="container-in">
						{newsArticles?.data.map((item, index) => (
							<div className="col-6 col-m-full fl" key={item.id}>
								<NewsCard article={item} />
							</div>
						))}
						<div className="clr"></div>
					</div>
				</div>
			</div>
		</>
	);
}
