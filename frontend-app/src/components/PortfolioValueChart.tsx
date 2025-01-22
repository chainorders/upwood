import { Line } from 'react-chartjs-2';
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
} from 'chart.js';
import { PortfolioValue } from '../apiClient';

// Register necessary chart.js components
ChartJS.register(CategoryScale, LinearScale, PointElement, LineElement, Title, Tooltip, Filler);

export default function PortfolioValueChart(props: { values: PortfolioValue[] }) {
  // Define data type
  const data = {
    labels: ['Jan', 'Feb', 'Mar', 'Apr', 'May', 'Jun'], // Months
    datasets: [
      {
        label: 'Portfolio Value',
        data: [300, 400, 350, 500, 450, 550], // Sample data points
        borderColor: '#28a745', // Line color
        backgroundColor: 'rgba(40, 167, 69, 0.2)', // Fill color
        pointRadius: 8,
        pointBackgroundColor: '#28a745',
        pointBorderColor: '#fff',
        tension: 0.4, // Smooth curve
        fill: false,
      },
    ],
  };

  // Define options type for Chart.js
  const options: ChartOptions<'line'> = {
    responsive: true,
    plugins: {
      tooltip: {
        callbacks: {
          label: (props:{raw: unknown}) => `€${props.raw}`, // Custom tooltip content
        },
      },
      legend: {
        display: false, // Hide legend
      },
    },
    scales: {
      y: {
        grid: {
          display: true, // Hide grid lines on x-axis
        },
        beginAtZero: true,
        ticks: {
          callback: (value: string | number) => `€${value}`, // Prefix y-axis values with €
        },
      },
      x: {
        grid: {
          display: true, // Hide grid lines on x-axis
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
            <div className="col-4 fl cels first">In 6 months <span>+104€</span></div>
            <div className="col-4 text-align-center fl cels">Last month <span>+104€</span></div>
            <div className="col-4 text-align-right fl cels last">Current month <span>+104€</span></div>
            <div className="clr"></div>
          </div>
        </div>
      </div>
    </div>
  );
};