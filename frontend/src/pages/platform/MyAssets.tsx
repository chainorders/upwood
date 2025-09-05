import React, { useState } from 'react';
import { Card, Progress, Chip, Button } from "@nextui-org/react";
import { motion } from 'framer-motion';
import { TreePine, Coins, ArrowUpRight, TrendingUp, Calendar, Leaf, Gift } from 'lucide-react';
import { TradeTokensModal } from '../../components/platform/TradeTokensModal';

const investments = [
  {
    id: 1,
    projectName: "Baltic Pine Forest",
    investmentDate: "2024-01-15",
    amount: 50000,
    tokens: 500,
    dividends: 2500,
    roi: 5,
    maturityDate: "2029-01-15",
    type: "Green Bond",
    carbonCredits: 25,
    status: "Active",
    image: "https://images.unsplash.com/photo-1473448912268-2022ce9509d8?auto=format&fit=crop&q=80"
  },
  {
    id: 2,
    projectName: "Nordic Spruce Estate",
    investmentDate: "2024-02-01",
    amount: 30000,
    tokens: 300,
    dividends: 1200,
    roi: 4,
    maturityDate: "2031-02-01",
    type: "Green Bond",
    carbonCredits: 15,
    status: "Active",
    image: "https://images.unsplash.com/photo-1502920514313-52581002a659?auto=format&fit=crop&q=80"
  }
];

const stats = [
  {
    label: "Total Investment",
    value: "€80,000",
    icon: Coins,
    change: "+12%"
  },
  {
    label: "Total Dividends",
    value: "€3,700",
    icon: Gift,
    change: "+4.6%"
  },
  {
    label: "Carbon Credits",
    value: "40 tons",
    icon: Leaf,
    change: "+8%"
  }
];

export default function MyAssets() {
  const [showTradeModal, setShowTradeModal] = useState(false);
  const [selectedToken, setSelectedToken] = useState<string>("");

  const handleOpenTradeModal = (tokenSymbol: string) => {
    setSelectedToken(tokenSymbol);
    setShowTradeModal(true);
  };

  const handleClaimDividends = (investmentId: number) => {
    // In a real app, this would trigger the dividend claim process
    console.log('Claiming dividends for investment:', investmentId);
  };

  const handleClaimCarbonCredits = (investmentId: number) => {
    // In a real app, this would trigger the carbon credits claim process
    console.log('Claiming carbon credits for investment:', investmentId);
  };

  return (
    <div className="p-4 lg:p-6">
      <div className="mb-8">
        <h1 className="text-2xl lg:text-3xl font-bold text-white mb-4">My Assets</h1>
        <p className="text-sm lg:text-base text-gray-400">
          Track and manage your forest investments
        </p>
      </div>

      {/* Stats Overview */}
      <div className="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 gap-4 lg:gap-6 mb-8">
        {stats.map((stat, index) => (
          <motion.div
            key={stat.label}
            initial={{ opacity: 0, y: 20 }}
            animate={{ opacity: 1, y: 0 }}
            transition={{ duration: 0.6, delay: index * 0.1 }}
          >
            <Card className="p-4 lg:p-6 bg-gray-900/50 border-gray-800">
              <div className="flex items-center gap-4">
                <div className="p-2 lg:p-3 rounded-lg bg-eco-green/10">
                  <stat.icon className="w-5 h-5 lg:w-6 lg:h-6 text-eco-green" />
                </div>
                <div>
                  <p className="text-xs lg:text-sm text-gray-400">{stat.label}</p>
                  <div className="flex items-center gap-2">
                    <p className="text-lg lg:text-2xl font-bold text-white">{stat.value}</p>
                    <Chip
                      className="bg-green-500/10 text-green-500"
                      size="sm"
                    >
                      {stat.change}
                    </Chip>
                  </div>
                </div>
              </div>
            </Card>
          </motion.div>
        ))}
      </div>

      {/* Investments List */}
      <div className="grid grid-cols-1 xl:grid-cols-2 gap-4 lg:gap-6">
        {investments.map((investment, index) => (
          <motion.div
            key={investment.id}
            initial={{ opacity: 0, y: 20 }}
            animate={{ opacity: 1, y: 0 }}
            transition={{ duration: 0.6, delay: index * 0.1 }}
          >
            <Card className="bg-gray-900/50 border-gray-800">
              <div className="aspect-video relative overflow-hidden">
                <img
                  src={investment.image}
                  alt={investment.projectName}
                  className="w-full h-full object-cover"
                />
                <div className="absolute inset-0 bg-gradient-to-t from-black/60 to-transparent" />
                <Chip 
                  className="absolute top-4 right-4 bg-eco-green text-white"
                  startContent={<TreePine className="w-4 h-4" />}
                >
                  {investment.type}
                </Chip>
              </div>

              <div className="p-4 lg:p-6">
                <div className="flex items-start justify-between mb-4">
                  <div>
                    <h3 className="text-lg lg:text-xl font-bold text-white mb-2">
                      {investment.projectName}
                    </h3>
                    <div className="flex items-center text-gray-400 text-sm">
                      <Calendar className="w-4 h-4 mr-1" />
                      Invested: {new Date(investment.investmentDate).toLocaleDateString()}
                    </div>
                  </div>
                  <Chip color="success" variant="flat">
                    {investment.status}
                  </Chip>
                </div>

                <div className="grid grid-cols-1 sm:grid-cols-2 gap-3 lg:gap-4 mb-6">
                  <div className="bg-black/30 p-4 rounded-lg">
                    <div className="text-xs lg:text-sm text-gray-400 mb-1">Investment</div>
                    <div className="text-lg lg:text-xl font-bold text-white">
                      €{investment.amount.toLocaleString()}
                    </div>
                  </div>
                  <div className="bg-black/30 p-4 rounded-lg">
                    <div className="text-xs lg:text-sm text-gray-400 mb-1">Dividends</div>
                    <div className="flex items-center justify-between mb-2">
                      <div className="text-lg lg:text-xl font-bold text-eco-green">
                        €{investment.dividends.toLocaleString()}
                      </div>
                      <Button
                        size="sm"
                        className="bg-eco-green text-white h-6 min-w-0 px-3"
                        onClick={() => handleClaimDividends(investment.id)}
                      >
                        Claim
                      </Button>
                    </div>
                  </div>
                  <div className="bg-black/30 p-4 rounded-lg">
                    <div className="text-xs lg:text-sm text-gray-400 mb-1">Tokens Owned</div>
                    <div className="text-lg lg:text-xl font-bold text-white">
                      {investment.tokens}
                    </div>
                  </div>
                  <div className="bg-black/30 p-4 rounded-lg">
                    <div className="text-xs lg:text-sm text-gray-400 mb-1">Carbon Credits</div>
                    <div className="flex items-center justify-between mb-2">
                      <div className="text-lg lg:text-xl font-bold text-white">
                        {investment.carbonCredits} tons
                      </div>
                      <Button
                        size="sm"
                        className="bg-eco-green text-white h-6 min-w-0 px-3"
                        onClick={() => handleClaimCarbonCredits(investment.id)}
                      >
                        Claim
                      </Button>
                    </div>
                  </div>
                </div>

                <div className="mb-6">
                  <div className="flex justify-between text-sm mb-2">
                    <span className="text-gray-400">ROI Progress</span>
                    <span className="text-white font-medium">{investment.roi}%</span>
                  </div>
                  <Progress 
                    value={investment.roi}
                    className="h-2"
                    color="success"
                  />
                </div>

                <div className="flex flex-col sm:flex-row gap-3">
                  <Button
                    className="flex-1 bg-eco-green text-white font-semibold"
                    endContent={<Coins className="w-4 h-4" />}
                    onClick={() => handleOpenTradeModal("tEUGB")}
                  >
                    Trade Tokens
                  </Button>
                  <Button
                    className="bg-gray-800 text-white"
                    variant="flat"
                    endContent={<ArrowUpRight className="w-4 h-4" />}
                  >
                    Details
                  </Button>
                </div>
              </div>
            </Card>
          </motion.div>
        ))}
      </div>

      <TradeTokensModal
        isOpen={showTradeModal}
        onClose={() => setShowTradeModal(false)}
        defaultToken={selectedToken}
      />
    </div>
  );
}