import { useEffect, useState } from "react";
import {
	ForestProjectAggApiModel,
	ForestProjectService,
	InvestmentPortfolioService,
	InvestmentPortfolioUserAggregate,
	LoginRes,
	PortfolioValue,
} from "../apiClient";
import PageHeader from "../components/PageHeader";
import ProjectCardOwned from "../components/ProjectCardOwned";
import PortfolioValueChart from "../components/PortfolioValueChart";
import { useOutletContext } from "react-router";

export default function InvestmentPortfolio() {
	const { user } = useOutletContext<{ user: LoginRes }>();
	const [projects, setProjects] = useState<ForestProjectAggApiModel[]>([]);
	const [portfolioValues, setPortfolioValues] = useState<PortfolioValue[]>([]);
	const [portfolioAgg, setPortfolioAgg] = useState<InvestmentPortfolioUserAggregate>();
	useEffect(() => {
		ForestProjectService.getForestProjectsListOwned().then((response) => {
			setProjects(response.data);
		});
		InvestmentPortfolioService.getPortfolioValueLastNMonths(24).then((response) => {
			setPortfolioValues(response);
		});
		InvestmentPortfolioService.getPortfolioAggregate().then((response) => {
			setPortfolioAgg(response);
		});
	}, []);

	return (
		<>
			<div className="clr"></div>
			<div className="investmentportfolio">
				<PageHeader user={user} parts={[{ name: "Investment Portfolio" }]} />
				<div className="outerboxshadow">
					<div className="container">
						<div className="container-in">
							<div className="col-20-percent fl investmentms col-m-full col-mr-bottom-30">
								<div className="tag">Locked token value</div>
								<div className="value">{portfolioAgg?.locked_mint_fund_euro_e_amount} €</div>
							</div>
							<div className="col-20-percent fl investmentms col-m-full col-mr-bottom-30">
								<div className="tag">Portfolio value</div>
								<div className="value">{portfolioAgg?.current_portfolio_value} €</div>
							</div>
							<div className="col-20-percent fl investmentms col-m-full col-mr-bottom-30">
								<div className="tag">Yearly portfolio growth</div>
								<div className="value">{portfolioAgg?.yearly_return} €</div>
							</div>
							<div className="col-20-percent fl investmentms col-m-full col-mr-bottom-30">
								<div className="tag">Return on investment</div>
								<div className="value">{portfolioAgg?.return_on_investment}%</div>
							</div>
							<div className="col-20-percent fl investmentms col-m-full">
								<div className="tag">Carbon tons offset</div>
								<div className="value">{portfolioAgg?.carbon_tons_offset}</div>
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
							<div className="col-8 col-m-full fl">
								<div className="heading">Portfolio value</div>
							</div>
							<div className="col-4 fr"></div>
							<div className="clr"></div>
						</div>
					</div>
					<div className="space-20"></div>
					<div className="container">
						<div className="container-in">
							<div className="col-12">
								<div className="chart">
									<div className="chart-in">
										<PortfolioValueChart values={portfolioValues} />
									</div>
								</div>
							</div>
						</div>
					</div>
					<div className="space-30"></div>
				</div>
				<div className="space-30"></div>
			</div>
			<div className="projects">
				<div className="container">
					<div className="container-in">
						{projects.map((project, index) => (
							<div className="col-6 col-m-full fl" key={index}>
								<ProjectCardOwned item={project} />
							</div>
						))}
						<div className="clr"></div>
					</div>
				</div>
			</div>
		</>
	);
}
