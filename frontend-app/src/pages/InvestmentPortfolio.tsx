import { useEffect, useState } from "react";
import {
	ForestProjectService,
	InvestmentPortfolioService,
	InvestmentPortfolioUserAggregate,
	PagedResponse_ForestProjectAggApiModel,
	PortfolioValue,
	SystemContractsConfigApiModel,
	UserService,
} from "../apiClient";
import PageHeader from "../components/PageHeader";
import ProjectCardOwned from "../components/ProjectCardOwned";
import PortfolioValueChart from "../components/PortfolioValueChart";
import { useOutletContext } from "react-router";
import { User } from "../lib/user";
import { toDisplayAmount } from "../lib/conversions";

export default function InvestmentPortfolio() {
	const { user } = useOutletContext<{ user: User }>();
	const [contracts, setContracts] = useState<SystemContractsConfigApiModel>();
	const [projects, setProjects] = useState<PagedResponse_ForestProjectAggApiModel>();
	const [portfolioValues, setPortfolioValues] = useState<PortfolioValue[]>([]);
	const [portfolioAgg, setPortfolioAgg] = useState<InvestmentPortfolioUserAggregate>();
	useEffect(() => {
		ForestProjectService.getForestProjectsListOwned().then(setProjects);
		InvestmentPortfolioService.getPortfolioValueLastNMonths(6).then((response) => {
			response.forEach((r) => (r.portfolio_value = toDisplayAmount(r.portfolio_value, 6, 0)));
			setPortfolioValues(response.reverse());
		});
		InvestmentPortfolioService.getPortfolioAggregate().then(setPortfolioAgg);
		UserService.getSystemConfig().then(setContracts);
	}, [user]);

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
								<div className="value">
									{toDisplayAmount(
										portfolioAgg?.locked_mint_fund_euro_e_amount || "0",
										contracts?.euro_e_metadata.decimals || 6,
										0,
									)}{" "}
									{contracts?.euro_e_metadata.symbol}
								</div>
							</div>
							<div className="col-20-percent fl investmentms col-m-full col-mr-bottom-30">
								<div className="tag">Portfolio value</div>
								<div className="value">
									{toDisplayAmount(portfolioAgg?.current_portfolio_value || "0", contracts?.euro_e_metadata.decimals || 6, 0)}{" "}
									{contracts?.euro_e_metadata.symbol}
								</div>
							</div>
							<div className="col-20-percent fl investmentms col-m-full col-mr-bottom-30">
								<div className="tag">Yearly portfolio growth</div>
								<div className="value">
									{toDisplayAmount(portfolioAgg?.yearly_return || "0", contracts?.euro_e_metadata.decimals || 6, 0)}{" "}
									{contracts?.euro_e_metadata.symbol}
								</div>
							</div>
							<div className="col-20-percent fl investmentms col-m-full col-mr-bottom-30">
								<div className="tag">Return on investment</div>
								<div className="value">{parseFloat(portfolioAgg?.return_on_investment || "0").toFixed(2)}%</div>
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
						{contracts &&
							projects?.data.map((project) => (
								<div className="col-6 col-m-full fl" key={project.forest_project.id}>
									<ProjectCardOwned project={project} user={user} contracts={contracts} />
								</div>
							))}
						<div className="clr"></div>
					</div>
				</div>
			</div>
		</>
	);
}
