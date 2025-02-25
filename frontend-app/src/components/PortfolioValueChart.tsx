import { Line } from "react-chartjs-2";
import {
	Chart as ChartJS,
	CategoryScale,
	LinearScale,
	PointElement,
	LineElement,
	Title,
	Tooltip,
	Filler,
	ChartOptions,
	TooltipItem,
} from "chart.js";
import { PortfolioValue } from "../apiClient";

ChartJS.register(CategoryScale, LinearScale, PointElement, LineElement, Title, Tooltip, Filler);

export default function PortfolioValueChart(props: {
	values: PortfolioValue[];
	sixMonthsAgo: string;
	currentMonth: string;
	lastMonth: string;
	currencySymbol: string;
}) {
	const chartLabels = props.values.map((value) => new Date(value.at).toLocaleDateString("en-US", { month: "short" }));
	const chartData = props.values.map((value) => value.portfolio_value);

	const data = {
		labels: chartLabels,
		datasets: [
			{
				label: "Portfolio Value",
				data: chartData,
				borderColor: "#28a745",
				backgroundColor: "rgba(40, 167, 69, 0.2)",
				pointRadius: 8,
				pointBackgroundColor: "#28a745",
				pointBorderColor: "#fff",
				tension: 0.4,
				fill: false,
			},
		],
	};

	const options: ChartOptions<"line"> = {
		responsive: true,
		plugins: {
			tooltip: {
				callbacks: {
					label: (props: TooltipItem<"line">) => `€${props.raw}`,
				},
			},
			legend: {
				display: false,
			},
		},
		scales: {
			y: {
				grid: {
					display: true,
				},
				beginAtZero: true,
				ticks: {
					callback: (value: string | number) => `€${value}`,
				},
			},
			x: {
				grid: {
					display: true,
				},
			},
		},
	};

	return (
		<div className="chart">
			<div className="chart-in">
				<div className="container">
					<div className="container-in">
						<div className="col-12">
							<Line data={data} options={options} height={70} />
						</div>
						<div className="clr"></div>
						<div className="col-4 fl cels first">
							In 6 months{" "}
							<span>
								{props.currencySymbol}
								{props.sixMonthsAgo}
							</span>
						</div>
						<div className="col-4 text-align-center fl cels">
							Last month{" "}
							<span>
								{props.currencySymbol}
								{props.lastMonth}
							</span>
						</div>
						<div className="col-4 text-align-right fl cels last">
							Current month{" "}
							<span>
								{props.currencySymbol}
								{props.currentMonth}
							</span>
						</div>
						<div className="clr"></div>
					</div>
				</div>
			</div>
		</div>
	);
}
