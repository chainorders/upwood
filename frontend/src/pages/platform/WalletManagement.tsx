import React from 'react';
import { Card, Button, Chip, Input } from "@nextui-org/react";
import { motion } from 'framer-motion';
import { Wallet, Copy, ExternalLink, Shield, Activity, Clock, Building2, CreditCard, Download } from 'lucide-react';

export default function WalletManagement() {
  const walletInfo = {
    address: "0x71C7656EC7ab88b098defB751B7401B5f6d8976F",
    connectedSince: "2024-03-15 14:30",
    lastActivity: "2024-03-15 16:45",
    securityLevel: "High",
    status: "Connected"
  };

  const [bankInfo, setBankInfo] = React.useState({
    accountName: "John Doe",
    bankName: "Deutsche Bank",
    iban: "DE89 3704 0044 0532 0130 00",
    swiftCode: "DEUTDEFF",
    status: "Verified"
  });

  const recentTransactions = [
    {
      id: 1,
      type: "Investment",
      project: "Baltic Pine Forest",
      amount: "0.5 ETH",
      status: "Completed",
      timestamp: "2024-03-15 16:45"
    },
    {
      id: 2,
      type: "Token Purchase",
      project: "Nordic Spruce Estate",
      amount: "0.3 ETH",
      status: "Pending",
      timestamp: "2024-03-15 16:30"
    }
  ];

  const copyToClipboard = (text: string) => {
    navigator.clipboard.writeText(text);
    // In a real app, show a toast notification
  };

  const handleBankInfoChange = (field: keyof typeof bankInfo) => (e: React.ChangeEvent<HTMLInputElement>) => {
    setBankInfo(prev => ({
      ...prev,
      [field]: e.target.value
    }));
  };

  const handleUpdateBankDetails = () => {
    // In a real app, this would submit the form to the backend
    console.log('Updating bank details:', bankInfo);
  };

  const handleDownloadHistory = () => {
    // In a real app, this would trigger the transaction history download
    console.log('Downloading transaction history');
  };

  return (
    <div className="p-4 lg:p-6">
      <div className="mb-8">
        <h1 className="text-2xl lg:text-3xl font-bold text-white mb-4">Wallet Management</h1>
        <p className="text-sm lg:text-base text-gray-400">
          Manage your connected digital wallet and view transaction history
        </p>
      </div>

      <div className="grid grid-cols-1 xl:grid-cols-3 gap-4 lg:gap-6">
        {/* Main Wallet Info */}
        <motion.div
          initial={{ opacity: 0, y: 20 }}
          animate={{ opacity: 1, y: 0 }}
          transition={{ duration: 0.6 }}
          className="xl:col-span-2 flex flex-col"
        >
          <Card className="bg-gray-900/50 border-gray-800 flex-1">
            <div className="p-4 lg:p-6">
              <div className="flex items-center gap-4 mb-6">
                <div className="p-2 lg:p-3 rounded-lg bg-eco-green/10">
                  <Wallet className="w-5 h-5 lg:w-6 lg:h-6 text-eco-green" />
                </div>
                <div>
                  <h2 className="text-lg lg:text-xl font-bold text-white">Connected Wallet</h2>
                  <p className="text-xs lg:text-sm text-gray-400">Manage your wallet settings and information</p>
                </div>
                <div className="ml-auto">
                  <Chip
                    className="bg-eco-green/10 text-eco-green"
                    size="sm"
                  >
                    {walletInfo.status}
                  </Chip>
                </div>
              </div>

              <div className="space-y-6">
                <div>
                  <label className="block text-sm text-gray-400 mb-2">Wallet Address</label>
                  <div className="flex flex-col sm:flex-row gap-2">
                    <div className="flex-1 bg-black/30 border border-gray-800 rounded-lg px-3 lg:px-4 py-2 font-mono text-sm lg:text-base text-white overflow-hidden text-ellipsis">
                      {walletInfo.address}
                    </div>
                    <div className="flex gap-2">
                      <Button
                        className="bg-gray-800 text-white flex-1 sm:flex-none"
                        variant="flat"
                        size="sm"
                        startContent={<Copy className="w-4 h-4" />}
                        onClick={() => copyToClipboard(walletInfo.address)}
                      >
                        Copy
                      </Button>
                      <Button
                        className="bg-gray-800 text-white flex-1 sm:flex-none"
                        variant="flat"
                        size="sm"
                        startContent={<ExternalLink className="w-4 h-4" />}
                        onClick={() => window.open(`https://etherscan.io/address/${walletInfo.address}`, '_blank')}
                      >
                        View
                      </Button>
                    </div>
                  </div>
                </div>

                <div className="grid grid-cols-1 sm:grid-cols-3 gap-2 lg:gap-4">
                  <div className="flex items-center gap-2 text-sm text-gray-400">
                    <Shield className="w-4 h-4" />
                    <span className="hidden sm:inline">Security Level:</span>
                    <span className="sm:hidden">Security:</span> {walletInfo.securityLevel}
                  </div>
                  <div className="flex items-center gap-2 text-sm text-gray-400">
                    <Clock className="w-4 h-4" />
                    <span className="hidden sm:inline">Connected:</span>
                    <span className="sm:hidden">Since:</span> {new Date(walletInfo.connectedSince).toLocaleDateString()}
                  </div>
                  <div className="flex items-center gap-2 text-sm text-gray-400">
                    <Activity className="w-4 h-4" />
                    <span className="hidden sm:inline">Last Activity:</span>
                    <span className="sm:hidden">Activity:</span> {new Date(walletInfo.lastActivity).toLocaleDateString()}
                  </div>
                </div>

                {/* Nominated Bank Account Section */}
                <div className="pt-6 border-t border-gray-800">
                  <div className="flex items-center gap-4 mb-6">
                    <div className="p-2 lg:p-3 rounded-lg bg-eco-green/10">
                      <Building2 className="w-5 h-5 lg:w-6 lg:h-6 text-eco-green" />
                    </div>
                    <div>
                      <h2 className="text-lg lg:text-xl font-bold text-white">Nominated Bank Account</h2>
                      <p className="text-xs lg:text-sm text-gray-400">Link your bank account for withdrawals</p>
                    </div>
                    <div className="ml-auto">
                      <Chip
                        className="bg-eco-green/10 text-eco-green"
                        size="sm"
                      >
                        {bankInfo.status}
                      </Chip>
                    </div>
                  </div>

                  <div className="grid grid-cols-1 lg:grid-cols-2 gap-4">
                    <div>
                      <label className="block text-sm font-medium text-gray-400 mb-2">Account Holder Name</label>
                      <Input
                        value={bankInfo.accountName}
                        onChange={handleBankInfoChange('accountName')}
                        placeholder="Enter account holder name"
                        variant="bordered"
                        classNames={{
                          base: "max-w-full",
                          mainWrapper: "h-12",
                          input: "text-white",
                          inputWrapper: "bg-black/30 border-gray-800 hover:border-eco-green/50 h-12"
                        }}
                      />
                    </div>
                    <div>
                      <label className="block text-sm font-medium text-gray-400 mb-2">Bank Name</label>
                      <Input
                        value={bankInfo.bankName}
                        onChange={handleBankInfoChange('bankName')}
                        placeholder="Enter bank name"
                        variant="bordered"
                        startContent={<Building2 className="w-4 h-4 text-gray-400 flex-shrink-0" />}
                        classNames={{
                          base: "max-w-full",
                          mainWrapper: "h-12",
                          input: "text-white",
                          inputWrapper: "bg-black/30 border-gray-800 hover:border-eco-green/50 h-12"
                        }}
                      />
                    </div>
                    <div>
                      <label className="block text-sm font-medium text-gray-400 mb-2">IBAN</label>
                      <Input
                        value={bankInfo.iban}
                        onChange={handleBankInfoChange('iban')}
                        placeholder="Enter IBAN"
                        variant="bordered"
                        startContent={<CreditCard className="w-4 h-4 text-gray-400 flex-shrink-0" />}
                        classNames={{
                          base: "max-w-full",
                          mainWrapper: "h-12",
                          input: "text-white font-mono",
                          inputWrapper: "bg-black/30 border-gray-800 hover:border-eco-green/50 h-12"
                        }}
                      />
                    </div>
                    <div>
                      <label className="block text-sm font-medium text-gray-400 mb-2">SWIFT/BIC Code</label>
                      <Input
                        value={bankInfo.swiftCode}
                        onChange={handleBankInfoChange('swiftCode')}
                        placeholder="Enter SWIFT/BIC code"
                        variant="bordered"
                        classNames={{
                          base: "max-w-full",
                          mainWrapper: "h-12",
                          input: "text-white font-mono",
                          inputWrapper: "bg-black/30 border-gray-800 hover:border-eco-green/50 h-12"
                        }}
                      />
                    </div>
                  </div>

                  <div className="mt-6 flex justify-end">
                    <Button
                      className="bg-eco-green text-white font-medium px-6"
                      size="lg"
                      onClick={handleUpdateBankDetails}
                    >
                      Update Bank Details
                    </Button>
                  </div>
                </div>
              </div>
            </div>
          </Card>
        </motion.div>

        {/* Recent Activity */}
        <motion.div
          initial={{ opacity: 0, y: 20 }}
          animate={{ opacity: 1, y: 0 }}
          transition={{ duration: 0.6, delay: 0.2 }}
          className="flex flex-col"
        >
          <Card className="bg-gray-900/50 border-gray-800 flex-1">
            <div className="p-4 lg:p-6">
              <div className="flex items-center justify-between mb-4">
                <h3 className="text-base lg:text-lg font-bold text-white">Recent Activity</h3>
                <Button
                  className="bg-gray-800 text-white"
                  variant="flat"
                  size="sm"
                  startContent={<Download className="w-4 h-4" />}
                  onClick={handleDownloadHistory}
                >
                  Download History
                </Button>
              </div>
              <div className="space-y-4">
                {recentTransactions.map((transaction) => (
                  <div
                    key={transaction.id}
                    className="p-4 bg-black/30 rounded-lg"
                  >
                    <div className="flex justify-between items-start mb-2">
                      <div>
                        <div className="font-medium text-white">{transaction.type}</div>
                        <div className="text-sm text-gray-400">{transaction.project}</div>
                      </div>
                      <Chip
                        size="sm"
                        className={transaction.status === "Completed" 
                          ? "bg-eco-green/10 text-eco-green"
                          : "bg-amber-500/10 text-amber-500"
                        }
                      >
                        {transaction.status}
                      </Chip>
                    </div>
                    <div className="flex justify-between items-center text-sm">
                      <div className="text-eco-green font-medium">{transaction.amount}</div>
                      <div className="text-gray-400">{new Date(transaction.timestamp).toLocaleString()}</div>
                    </div>
                  </div>
                ))}
              </div>
            </div>
          </Card>
        </motion.div>
      </div>
    </div>
  );
}