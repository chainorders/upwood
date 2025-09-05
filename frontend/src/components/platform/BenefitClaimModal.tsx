import React, { useState } from 'react';
import { Modal, ModalContent, ModalHeader, ModalBody, ModalFooter, Button, Input } from "@nextui-org/react";
import { motion } from 'framer-motion';
import { Copy, ExternalLink, Gift } from 'lucide-react';

interface BenefitClaimModalProps {
  isOpen: boolean;
  onClose: () => void;
  benefit: {
    title: string;
    partner: string;
    discount: string;
    validUntil: string;
  } | null;
}

export function BenefitClaimModal({ isOpen, onClose, benefit }: BenefitClaimModalProps) {
  const [copied, setCopied] = useState(false);
  const voucherCode = "GREENBOND-2024-XYZ";
  const partnerUrl = "https://partner-website.com/redeem";

  const handleCopyCode = () => {
    navigator.clipboard.writeText(voucherCode);
    setCopied(true);
    setTimeout(() => setCopied(false), 2000);
  };

  if (!benefit) return null;

  return (
    <Modal 
      isOpen={isOpen} 
      onClose={onClose}
      className="dark"
      size="lg"
    >
      <ModalContent>
        <ModalHeader className="flex flex-col gap-1">
          <div className="flex items-center gap-3">
            <div className="p-2 rounded-lg bg-eco-green/20">
              <Gift className="w-6 h-6 text-eco-green" />
            </div>
            <div>
              <h3 className="text-xl font-bold text-white">Claim Your Benefit</h3>
              <p className="text-sm text-gray-400">
                Use the code below to redeem your benefit
              </p>
            </div>
          </div>
        </ModalHeader>
        <ModalBody>
          <motion.div
            initial={{ opacity: 0, y: 20 }}
            animate={{ opacity: 1, y: 0 }}
            transition={{ duration: 0.4 }}
            className="space-y-6"
          >
            <div className="bg-gray-900/50 p-4 rounded-lg border border-gray-800">
              <div className="text-sm text-gray-400 mb-2">Benefit Details</div>
              <h4 className="text-lg font-bold text-white mb-1">{benefit.title}</h4>
              <p className="text-sm text-gray-400">Partner: {benefit.partner}</p>
              <div className="mt-2 text-sm">
                <span className="text-eco-green font-medium">{benefit.discount}</span>
                <span className="text-gray-500 mx-2">•</span>
                <span className="text-gray-500">
                  Valid until {new Date(benefit.validUntil).toLocaleDateString()}
                </span>
              </div>
            </div>

            <div>
              <label className="block text-sm text-gray-400 mb-2">Your Voucher Code</label>
              <div className="flex gap-2">
                <Input
                  value={voucherCode}
                  readOnly
                  className="flex-1 font-mono"
                  classNames={{
                    input: "text-white",
                    inputWrapper: "bg-black/30 border-gray-800"
                  }}
                />
                <Button
                  className={copied ? "bg-green-500 text-white" : "bg-eco-green text-white"}
                  onClick={handleCopyCode}
                  startContent={<Copy className="w-4 h-4" />}
                >
                  {copied ? "Copied!" : "Copy"}
                </Button>
              </div>
            </div>

            <div className="bg-black/30 p-4 rounded-lg">
              <div className="flex items-start gap-3 text-sm text-gray-400">
                <div className="p-2 rounded-lg bg-gray-800/50">
                  <ExternalLink className="w-4 h-4" />
                </div>
                <div>
                  <p className="mb-2">
                    Visit the partner website and enter your voucher code during checkout:
                  </p>
                  <a 
                    href={partnerUrl}
                    target="_blank"
                    rel="noopener noreferrer"
                    className="text-eco-green hover:underline"
                  >
                    {partnerUrl}
                  </a>
                </div>
              </div>
            </div>
          </motion.div>
        </ModalBody>
        <ModalFooter>
          <Button
            className="w-full bg-gray-800 text-white"
            size="lg"
            onClick={onClose}
          >
            Close
          </Button>
        </ModalFooter>
      </ModalContent>
    </Modal>
  );
}