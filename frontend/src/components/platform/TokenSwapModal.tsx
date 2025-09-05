import React, { useState, useEffect } from 'react';
import { Modal, ModalContent, ModalHeader, ModalBody, ModalFooter, Button, Input, Select, SelectItem, CircularProgress } from "@nextui-org/react";
import { motion } from 'framer-motion';
import { Wallet, ArrowDown, Info, CheckCircle2, Loader2 } from 'lucide-react';
import { SignAgreementsModal } from './SignAgreementsModal';

interface TokenSwapModalProps {
  isOpen: boolean;
  onClose: () => void;
  project: {
    title: string;
    tokenSymbol?: string;
  };
}

const chains = [
  { 
    id: "concordium", 
    name: "Concordium", 
    logo: "https://assets.concordium.com/concordium.svg",
    disabled: false
  },
  { 
    id: "polygon", 
    name: "Polygon", 
    logo: "https://cryptologos.cc/logos/polygon-matic-logo.svg",
    disabled: true
  },
  { 
    id: "base", 
    name: "Base", 
    logo: "https://raw.githubusercontent.com/base-org/brand-kit/main/logo/base-logo.svg",
    disabled: true
  }
];

export function TokenSwapModal({ isOpen, onClose, project }: TokenSwapModalProps) {
  const [amount, setAmount] = useState<string>('');
  const [selectedChain, setSelectedChain] = useState<string>("concordium");
  const [tokenAmount, setTokenAmount] = useState<number>(0);
  const [showSignAgreements, setShowSignAgreements] = useState(false);
  const [showPending, setShowPending] = useState(false);
  const [showSuccess, setShowSuccess] = useState(false);
  const [transactionProgress, setTransactionProgress] = useState(0);

  // Calculate token amount based on EUR input (1 tEUGB = 100€)
  useEffect(() => {
    const eurAmount = parseFloat(amount) || 0;
    setTokenAmount(eurAmount / 100);
  }, [amount]);

  const handleInvest = () => {
    setShowSignAgreements(true);
  };

  const handleAgreementsSigned = () => {
    setShowSignAgreements(false);
    setShowPending(true);

    // Simulate transaction progress
    let progress = 0;
    const interval = setInterval(() => {
      progress += Math.random() * 8 + 2; // Random progress between 2-10%
      setTransactionProgress(progress);
      
      if (progress >= 100) {
        setTransactionProgress(100); // Ensure it ends at exactly 100%
        clearInterval(interval);
        setShowPending(false);
        setShowSuccess(true);
        
        // Reset after success message
        setTimeout(() => {
          setShowSuccess(false);
          setAmount('');
          onClose();
        }, 3000);
      }
    }, 200); // Slightly slower for more realistic feel
  };

  if (showSuccess) {
    return (
      <Modal 
        isOpen={isOpen} 
        onClose={onClose}
        className="dark"
        size="lg"
      >
        <ModalContent>
          <ModalBody>
            <motion.div 
              initial={{ scale: 0.5, opacity: 0 }}
              animate={{ scale: 1, opacity: 1 }}
              className="py-12 text-center"
            >
              <CheckCircle2 className="w-16 h-16 text-eco-green mx-auto mb-6" />
              <h3 className="text-2xl font-bold text-white mb-2">Investment Successful!</h3>
              <p className="text-gray-400">
                Your investment in {project.title} has been confirmed
              </p>
            </motion.div>
          </ModalBody>
        </ModalContent>
      </Modal>
    );
  }

  if (showPending) {
    return (
      <Modal 
        isOpen={isOpen} 
        onClose={onClose}
        className="dark"
        size="lg"
      >
        <ModalContent>
          <ModalBody>
            <motion.div 
              initial={{ opacity: 0 }}
              animate={{ opacity: 1 }}
              transition={{ duration: 0.5 }}
              className="py-12 text-center"
            >
              <motion.div 
                className="relative mx-auto w-20 h-20 mb-6"
                animate={{ 
                  scale: [1, 1.05, 1],
                  rotate: [0, 180, 360]
                }}
                transition={{ 
                  scale: { duration: 2, repeat: Infinity, ease: "easeInOut" },
                  rotate: { duration: 3, repeat: Infinity, ease: "linear" }
                }}
              >
                <CircularProgress
                  value={transactionProgress}
                  className="w-20 h-20"
                  strokeWidth={3}
                  color="success"
                  classNames={{
                    svg: "drop-shadow-lg",
                    track: "stroke-gray-800/50",
                    indicator: "stroke-eco-green drop-shadow-sm"
                  }}
                />
                <motion.div
                  className="absolute top-1/2 left-1/2 transform -translate-x-1/2 -translate-y-1/2"
                  animate={{ rotate: 360 }}
                  transition={{ duration: 1.5, repeat: Infinity, ease: "linear" }}
                >
                  <Loader2 className="w-8 h-8 text-eco-green" />
                </motion.div>
                <motion.div
                  className="absolute inset-0 rounded-full border-2 border-eco-green/20"
                  animate={{ 
                    scale: [1, 1.2, 1],
                    opacity: [0.3, 0.1, 0.3]
                  }}
                  transition={{ duration: 2, repeat: Infinity, ease: "easeInOut" }}
                />
              </motion.div>
              <motion.h3 
                className="text-2xl font-bold text-white mb-2"
                animate={{ opacity: [1, 0.7, 1] }}
                transition={{ duration: 1.5, repeat: Infinity, ease: "easeInOut" }}
              >
                Transaction in Progress
              </motion.h3>
              <p className="text-gray-400 mb-4">
                Please wait while we process your investment
              </p>
              <motion.div
                initial={{ opacity: 0, y: 10 }}
                animate={{ opacity: 1, y: 0 }}
                transition={{ delay: 0.5 }}
              >
                <div className="flex items-center justify-center gap-2 text-sm text-gray-500 mb-2">
                  <span>Progress: {Math.round(transactionProgress)}%</span>
                </div>
                <p className="text-xs text-gray-600">
                  Estimated time remaining: {Math.max(1, Math.ceil((100 - transactionProgress) / 25))} seconds
                </p>
              </motion.div>
            </motion.div>
          </ModalBody>
        </ModalContent>
      </Modal>
    );
  }

  return (
    <>
      <Modal 
        isOpen={isOpen && !showSignAgreements} 
        onClose={onClose}
        className="dark"
        size="lg"
      >
        <ModalContent>
          <ModalHeader className="flex flex-col gap-1">
            <h3 className="text-xl font-bold text-white">Invest in {project.title}</h3>
            <p className="text-sm text-gray-400">
              Purchase tEUGB tokens backed by real-world forest assets
            </p>
          </ModalHeader>
          <ModalBody>
            <div className="space-y-6">
              {/* Amount Input */}
              <div className="space-y-4">
                <div className="bg-gray-900/50 p-4 rounded-lg border border-gray-800">
                  <label className="block text-sm text-gray-400 mb-2">
                    Investment Amount (EUR)
                  </label>
                  <Input
                    type="number"
                    value={amount}
                    onChange={(e) => setAmount(e.target.value)}
                    placeholder="Enter amount in EUR"
                    min="100"
                    startContent={
                      <div className="pointer-events-none flex items-center">
                        <span className="text-default-400 text-small">€</span>
                      </div>
                    }
                    classNames={{
                      input: "text-white",
                      inputWrapper: "bg-black/30 border-gray-800"
                    }}
                  />
                </div>

                <div className="flex justify-center">
                  <div className="bg-gray-900 rounded-full p-2">
                    <ArrowDown className="w-5 h-5 text-gray-400" />
                  </div>
                </div>

                <div className="bg-gray-900/50 p-4 rounded-lg border border-gray-800">
                  <label className="block text-sm text-gray-400 mb-2">
                    Token Amount (tEUGB)
                  </label>
                  <div className="bg-black/30 border border-gray-800 rounded-lg px-4 py-3 text-white">
                    {tokenAmount.toFixed(2)} tEUGB
                  </div>
                  <div className="mt-2 flex items-start gap-2 text-xs text-gray-500">
                    <Info className="w-4 h-4 flex-shrink-0" />
                    <p>1 tEUGB token = €100. Tokens represent your share in the forest project.</p>
                  </div>
                </div>
              </div>

              {/* Chain Selection */}
              <div>
                <label className="block text-sm text-gray-400 mb-2">
                  Select Chain
                </label>
                <Select
                  selectedKeys={[selectedChain]}
                  onChange={(e) => setSelectedChain(e.target.value)}
                  classNames={{
                    trigger: "bg-black/30 border-gray-800",
                    value: "text-white"
                  }}
                >
                  {chains.map((chain) => (
                    <SelectItem 
                      key={chain.id} 
                      value={chain.id}
                      textValue={chain.name}
                      className={chain.disabled ? "opacity-50" : ""}
                    >
                      <div className="flex items-center gap-2">
                        <img 
                          src={chain.logo} 
                          alt={chain.name} 
                          className="w-5 h-5"
                        />
                        <span>{chain.name}</span>
                        {chain.disabled && (
                          <span className="text-xs text-gray-500">(Coming Soon)</span>
                        )}
                      </div>
                    </SelectItem>
                  ))}
                </Select>
              </div>
            </div>
          </ModalBody>
          <ModalFooter>
            <Button
              className="bg-eco-green text-white w-full"
              onPress={handleInvest}
              startContent={<Wallet className="w-4 h-4" />}
              isDisabled={!amount || parseFloat(amount) < 100}
              size="lg"
            >
              Invest
            </Button>
          </ModalFooter>
        </ModalContent>
      </Modal>

      <SignAgreementsModal
        isOpen={showSignAgreements}
        onClose={() => setShowSignAgreements(false)}
        onComplete={handleAgreementsSigned}
        project={project}
      />
    </>
  );
}