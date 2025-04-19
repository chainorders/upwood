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
import { User } from "../lib/user";
import { formatDate, toDisplayAmount } from "../lib/conversions";

interface InvestmentPortfolioProps {
	user: User;
}

export default function InvestmentPortfolio({ user }: InvestmentPortfolioProps) {
	const [contracts, setContracts] = useState<SystemContractsConfigApiModel>();
	const [projects, setProjects] = useState<PagedResponse_ForestProjectAggApiModel>();
	const [portfolioValues, setPortfolioValues] = useState<PortfolioValue[]>([]);
	const [portfolioAgg, setPortfolioAgg] = useState<InvestmentPortfolioUserAggregate>();
	const [valSixMonthsAgo, setValSixMonthsAgo] = useState<string>("0");
	const [valLastMonth, setValLastMonth] = useState<string>("0");
	const [valCurrentMonth, setValCurrentMonth] = useState<string>("0");

	useEffect(() => {
		UserService.getSystemConfig().then(setContracts);
	}, [user]);

	useEffect(() => {
		if (!user || !contracts) return;

		const fetchPortfolioValues = async () => {
			const sixMonthsAgo = new Date();
			sixMonthsAgo.setMonth(sixMonthsAgo.getMonth() - 6);
			const currentMonth = new Date();
			const lastMonth = new Date();
			lastMonth.setMonth(lastMonth.getMonth() - 1);

			const [projects, portfolioValues, valSixMonthsAgo, valLastMonth, valCurrentMonth, portfolioAgg] = await Promise.all([
				ForestProjectService.getForestProjectsListOwned(),
				InvestmentPortfolioService.getPortfolioValueLastNMonths(6),
				InvestmentPortfolioService.getPortfolioValue(formatDate(sixMonthsAgo)),
				InvestmentPortfolioService.getPortfolioValue(formatDate(lastMonth)),
				InvestmentPortfolioService.getPortfolioValue(formatDate(currentMonth)),
				InvestmentPortfolioService.getPortfolioAggregate(),
			]);

			setProjects(projects);
			setPortfolioValues(
				portfolioValues
					.map((r) => ({
						...r,
						portfolio_value: toDisplayAmount(r.portfolio_value, 6),
					}))
					.reverse(),
			);
			setValSixMonthsAgo(toDisplayAmount(valSixMonthsAgo, 6));
			setValLastMonth(toDisplayAmount(valLastMonth, 6));
			setValCurrentMonth(toDisplayAmount(valCurrentMonth, 6));
			setPortfolioAgg(portfolioAgg);
		};

		fetchPortfolioValues();
	}, [user, contracts]);

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
								<div className="value">{toDisplayAmount(portfolioAgg?.locked_mint_fund_euro_e_amount || "0", 6)} €</div>
							</div>
							<div className="col-20-percent fl investmentms col-m-full col-mr-bottom-30">
								<div className="tag">Portfolio value</div>
								<div className="value">{toDisplayAmount(portfolioAgg?.current_portfolio_value || "0", 6)} €</div>
							</div>
							<div className="col-20-percent fl investmentms col-m-full col-mr-bottom-30">
								<div className="tag">Yearly portfolio growth</div>
								<div className="value">{toDisplayAmount(portfolioAgg?.yearly_return || "0", 6)} €</div>
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
										<PortfolioValueChart
											values={portfolioValues}
											sixMonthsAgo={valSixMonthsAgo}
											currentMonth={valCurrentMonth}
											lastMonth={valLastMonth}
											currencySymbol="€"
										/>
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
									<ProjectCardOwned project={project} user={user} />
								</div>
							))}
						<div className="clr"></div>
					</div>
				</div>
			</div>
		</>
	);
}
