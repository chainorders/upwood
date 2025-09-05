import React, { useState, useEffect } from 'react';
import { Modal, ModalContent, ModalHeader, ModalBody, ModalFooter, Button, Input, Select, SelectItem, CircularProgress, Link } from "@nextui-org/react";
import { motion } from 'framer-motion';
import { ArrowDown, Wallet, RefreshCw, Settings, Info, Circle, ExternalLink } from 'lucide-react';

interface TradeTokensModalProps {
  isOpen: boolean;
  onClose: () => void;
  defaultToken?: string;
}

interface Token {
  symbol: string;
  name: string;
  balance: string;
  color: string;
  icon: string;
}

export function TradeTokensModal({ isOpen, onClose, defaultToken = "tEUGB" }: TradeTokensModalProps) {
  const [fromToken, setFromToken] = useState<string>(defaultToken);
  const [toToken, setToToken] = useState<string>("EureE");
  const [fromAmount, setFromAmount] = useState<string>("");
  const [toAmount, setToAmount] = useState<string>("");
  const [slippage, setSlippage] = useState<number>(0.5);
  const [isLoading, setIsLoading] = useState(false);
  const [showSettings, setShowSettings] = useState(false);

  const tokens: Token[] = [
    { 
      symbol: "tEUGB", 
      name: "GreenBond Token", 
      balance: "500.00",
      color: "#3a5a40",
      icon: "https://raw.githubusercontent.com/spothq/cryptocurrency-icons/master/svg/color/eth.svg"
    },
    { 
      symbol: "EureE", 
      name: "Euro E-Stablecoin", 
      balance: "1000.00",
      color: "#2664ec",
      icon: "https://raw.githubusercontent.com/spothq/cryptocurrency-icons/master/svg/color/eur.svg"
    },
    { 
      symbol: "ETH", 
      name: "Ethereum", 
      balance: "1.5",
      color: "#627eea",
      icon: "https://raw.githubusercontent.com/spothq/cryptocurrency-icons/master/svg/color/eth.svg"
    },
    { 
      symbol: "USDC", 
      name: "USD Coin", 
      balance: "2000.00",
      color: "#2775ca",
      icon: "https://raw.githubusercontent.com/spothq/cryptocurrency-icons/master/svg/color/usdc.svg"
    }
  ];

  const getExchangeRate = (from: string, to: string) => {
    const rates: { [key: string]: { [key: string]: number } } = {
      "tEUGB": { "EureE": 100, "ETH": 0.05, "USDC": 105 },
      "EureE": { "tEUGB": 0.01, "ETH": 0.0005, "USDC": 1.05 },
      "ETH": { "tEUGB": 20, "EureE": 2000, "USDC": 2100 },
      "USDC": { "tEUGB": 0.0095, "EureE": 0.95, "ETH": 0.00048 }
    };
    return rates[from]?.[to] || 0;
  };

  useEffect(() => {
    if (fromAmount && fromToken && toToken) {
      const rate = getExchangeRate(fromToken, toToken);
      const calculated = (parseFloat(fromAmount) * rate).toFixed(6);
      setToAmount(calculated);
    } else {
      setToAmount("");
    }
  }, [fromAmount, fromToken, toToken]);

  const handleSwapTokens = () => {
    const tempToken = fromToken;
    setFromToken(toToken);
    setToToken(tempToken);
    setFromAmount("");
    setToAmount("");
  };

  const handleTrade = () => {
    setIsLoading(true);
    setTimeout(() => {
      setIsLoading(false);
      onClose();
    }, 2000);
  };

  const handleMaxAmount = (token: string) => {
    const tokenInfo = tokens.find(t => t.symbol === token);
    if (tokenInfo) {
      setFromAmount(tokenInfo.balance);
    }
  };

  return (
    <Modal 
      isOpen={isOpen} 
      onClose={onClose}
      className="dark"
      size="lg"
    >
      <ModalContent>
        <ModalHeader className="flex flex-col gap-1">
          <div className="flex items-center justify-between">
            <h3 className="text-xl font-bold text-white">Swap Tokens</h3>
            <Button
              isIconOnly
              variant="light"
              className="text-gray-400 hover:text-white"
              onClick={() => setShowSettings(!showSettings)}
            >
              <Settings className="w-5 h-5" />
            </Button>
          </div>
        </ModalHeader>
        <ModalBody>
          {showSettings ? (
            <motion.div
              initial={{ opacity: 0 }}
              animate={{ opacity: 1 }}
              exit={{ opacity: 0 }}
              className="space-y-4"
            >
              <div>
                <label className="block text-sm text-gray-400 mb-2">Slippage Tolerance</label>
                <div className="flex gap-2">
                  {[0.1, 0.5, 1.0].map((value) => (
                    <Button
                      key={value}
                      className={`flex-1 ${
                        slippage === value 
                          ? 'bg-eco-green text-white' 
                          : 'bg-gray-800 text-gray-400'
                      }`}
                      onClick={() => setSlippage(value)}
                    >
                      {value}%
                    </Button>
                  ))}
                </div>
              </div>
              <div className="flex items-start gap-2 text-xs text-gray-500">
                <Info className="w-4 h-4 flex-shrink-0" />
                <p>Your transaction will revert if the price changes unfavorably by more than this percentage.</p>
              </div>
            </motion.div>
          ) : (
            <div className="space-y-6">
              <div className="bg-gray-900/50 p-4 rounded-lg border border-gray-800">
                <div className="flex justify-between items-center text-sm text-gray-400 mb-2">
                  <span>From</span>
                  <div className="flex items-center gap-2">
                    <span>Balance: {tokens.find(t => t.symbol === fromToken)?.balance}</span>
                    <Button
                      size="sm"
                      variant="flat"
                      className="text-eco-green h-6 px-2 min-w-0"
                      onClick={() => handleMaxAmount(fromToken)}
                    >
                      MAX
                    </Button>
                  </div>
                </div>
                <div className="flex gap-4">
                  <Input
                    type="number"
                    value={fromAmount}
                    onChange={(e) => setFromAmount(e.target.value)}
                    placeholder="0.0"
                    classNames={{
                      input: "text-white text-2xl font-medium",
                      inputWrapper: "bg-black/30 border-gray-800"
                    }}
                  />
                  <Select
                    selectedKeys={[fromToken]}
                    onChange={(e) => setFromToken(e.target.value)}
                    className="min-w-[180px]"
                    classNames={{
                      trigger: "bg-black/30 border-gray-800 h-[56px]",
                      value: "text-white",
                      popover: "bg-gray-900 border border-gray-800",
                    }}
                    renderValue={(items) => {
                      const token = tokens.find(t => t.symbol === items[0]?.key);
                      return token ? (
                        <div className="flex items-center gap-2">
                          <img 
                            src={token.icon} 
                            alt={token.symbol}
                            className="w-6 h-6"
                          />
                          <div>
                            <div className="font-medium">{token.symbol}</div>
                            <div className="text-xs text-gray-400">{token.name}</div>
                          </div>
                        </div>
                      ) : null;
                    }}
                  >
                    {tokens.map((token) => (
                      <SelectItem 
                        key={token.symbol} 
                        value={token.symbol}
                        className="data-[selected=true]:bg-eco-green/20"
                        startContent={
                          <img 
                            src={token.icon} 
                            alt={token.symbol}
                            className="w-6 h-6"
                          />
                        }
                      >
                        <div className="flex flex-col">
                          <span className="font-medium">{token.symbol}</span>
                          <span className="text-xs text-gray-400">{token.name}</span>
                        </div>
                      </SelectItem>
                    ))}
                  </Select>
                </div>
              </div>

              <div className="flex justify-center -my-3 relative z-10">
                <Button
                  isIconOnly
                  className="bg-eco-green text-white rounded-full w-10 h-10 shadow-lg hover:scale-105 transition-transform"
                  onClick={handleSwapTokens}
                >
                  <ArrowDown className="w-5 h-5" />
                </Button>
              </div>

              <div className="bg-gray-900/50 p-4 rounded-lg border border-gray-800">
                <div className="flex justify-between items-center text-sm text-gray-400 mb-2">
                  <span>To</span>
                  <span>Balance: {tokens.find(t => t.symbol === toToken)?.balance}</span>
                </div>
                <div className="flex gap-4">
                  <Input
                    type="number"
                    value={toAmount}
                    readOnly
                    placeholder="0.0"
                    classNames={{
                      input: "text-white text-2xl font-medium",
                      inputWrapper: "bg-black/30 border-gray-800"
                    }}
                  />
                  <Select
                    selectedKeys={[toToken]}
                    onChange={(e) => setToToken(e.target.value)}
                    className="min-w-[180px]"
                    classNames={{
                      trigger: "bg-black/30 border-gray-800 h-[56px]",
                      value: "text-white",
                      popover: "bg-gray-900 border border-gray-800",
                    }}
                    renderValue={(items) => {
                      const token = tokens.find(t => t.symbol === items[0]?.key);
                      return token ? (
                        <div className="flex items-center gap-2">
                          <img 
                            src={token.icon} 
                            alt={token.symbol}
                            className="w-6 h-6"
                          />
                          <div>
                            <div className="font-medium">{token.symbol}</div>
                            <div className="text-xs text-gray-400">{token.name}</div>
                          </div>
                        </div>
                      ) : null;
                    }}
                  >
                    {tokens.map((token) => (
                      <SelectItem 
                        key={token.symbol} 
                        value={token.symbol}
                        className="data-[selected=true]:bg-eco-green/20"
                        startContent={
                          <img 
                            src={token.icon} 
                            alt={token.symbol}
                            className="w-6 h-6"
                          />
                        }
                      >
                        <div className="flex flex-col">
                          <span className="font-medium">{token.symbol}</span>
                          <span className="text-xs text-gray-400">{token.name}</span>
                        </div>
                      </SelectItem>
                    ))}
                  </Select>
                </div>
              </div>

              {fromAmount && toAmount && (
                <div className="bg-black/30 p-4 rounded-lg text-sm space-y-2">
                  <div className="flex justify-between text-gray-400">
                    <span>Exchange Rate</span>
                    <div className="flex items-center gap-2">
                      <span>1 {fromToken} = {getExchangeRate(fromToken, toToken)} {toToken}</span>
                      <Button
                        isIconOnly
                        variant="light"
                        className="text-gray-400 hover:text-white w-6 h-6 min-w-0"
                      >
                        <RefreshCw className="w-4 h-4" />
                      </Button>
                    </div>
                  </div>
                  <div className="flex justify-between text-gray-400">
                    <span>Slippage Tolerance</span>
                    <span>{slippage}%</span>
                  </div>
                </div>
              )}
            </div>
          )}
        </ModalBody>
        <ModalFooter className="flex flex-col gap-4">
          {!showSettings && (
            <>
              <Button
                className="w-full bg-eco-green text-white font-semibold"
                size="lg"
                isLoading={isLoading}
                spinner={<CircularProgress size="sm" color="white" />}
                onClick={handleTrade}
                startContent={!isLoading && <Wallet className="w-4 h-4" />}
                isDisabled={!fromAmount || !toAmount}
              >
                {isLoading ? "Swapping..." : "Swap Tokens"}
              </Button>
              <div className="text-center text-sm text-gray-400">
                Trading activity on GreenBond platform is facilitated through Concordex.{' '}
                <Link
                  href="https://concordex.io"
                  target="_blank"
                  rel="noopener noreferrer"
                  className="text-eco-green hover:text-eco-green/80 inline-flex items-center gap-1"
                >
                  Visit Concordex <ExternalLink className="w-3 h-3" />
                </Link>
                {' '}for full secondary market experience.
              </div>
            </>
          )}
        </ModalFooter>
      </ModalContent>
    </Modal>
  );
}