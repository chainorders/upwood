import React, { useState } from 'react';
import { Modal, ModalContent, ModalHeader, ModalBody, ModalFooter, Button, Checkbox, Card } from "@nextui-org/react";
import { motion } from 'framer-motion';
import { FileText, Download, CheckCircle2 } from 'lucide-react';

interface SignAgreementsModalProps {
  isOpen: boolean;
  onClose: () => void;
  onComplete: () => void;
  project: {
    title: string;
  };
}

interface Agreement {
  id: string;
  title: string;
  description: string;
  signed: boolean;
}

export function SignAgreementsModal({ isOpen, onClose, onComplete, project }: SignAgreementsModalProps) {
  const [agreements, setAgreements] = useState<Agreement[]>([
    {
      id: 'investment',
      title: 'Investment Agreement',
      description: 'Terms and conditions for investing in forest-backed tokens',
      signed: false
    },
    {
      id: 'token',
      title: 'Token Purchase Agreement',
      description: 'Legal framework for the purchase and ownership of tEUGB tokens',
      signed: false
    }
  ]);

  const handleDownload = (agreementId: string) => {
    // In a real app, this would download the agreement PDF
    console.log(`Downloading agreement: ${agreementId}`);
  };

  const handleSign = (agreementId: string) => {
    setAgreements(prev => prev.map(agreement => 
      agreement.id === agreementId 
        ? { ...agreement, signed: true }
        : agreement
    ));
  };

  const allSigned = agreements.every(agreement => agreement.signed);

  return (
    <Modal 
      isOpen={isOpen} 
      onClose={onClose}
      className="dark"
      size="2xl"
      scrollBehavior="inside"
    >
      <ModalContent>
        <ModalHeader className="flex flex-col gap-1">
          <h3 className="text-xl font-bold text-white">Sign Investment Agreements</h3>
          <p className="text-sm text-gray-400">
            Please review and sign the following agreements to complete your investment in {project.title}
          </p>
        </ModalHeader>
        <ModalBody>
          <div className="space-y-4">
            {agreements.map((agreement) => (
              <motion.div
                key={agreement.id}
                initial={{ opacity: 0, y: 20 }}
                animate={{ opacity: 1, y: 0 }}
              >
                <Card className="bg-gray-900/50 border-gray-800">
                  <div className="p-6">
                    <div className="flex items-start gap-4">
                      <div className="p-3 rounded-lg bg-eco-green/10">
                        <FileText className="w-6 h-6 text-eco-green" />
                      </div>
                      <div className="flex-1">
                        <div className="flex items-start justify-between mb-2">
                          <div>
                            <h4 className="text-lg font-bold text-white">{agreement.title}</h4>
                            <p className="text-sm text-gray-400">{agreement.description}</p>
                          </div>
                          {agreement.signed && (
                            <CheckCircle2 className="w-5 h-5 text-eco-green" />
                          )}
                        </div>
                        <div className="flex items-center gap-4 mt-4">
                          <Button
                            size="sm"
                            variant="flat"
                            className="bg-gray-800 text-white"
                            startContent={<Download className="w-4 h-4" />}
                            onClick={() => handleDownload(agreement.id)}
                          >
                            Download PDF
                          </Button>
                          <Checkbox
                            isSelected={agreement.signed}
                            onValueChange={() => handleSign(agreement.id)}
                            color="success"
                          >
                            <span className="text-sm text-gray-400">
                              I have read and agree to the terms
                            </span>
                          </Checkbox>
                        </div>
                      </div>
                    </div>
                  </div>
                </Card>
              </motion.div>
            ))}
          </div>
        </ModalBody>
        <ModalFooter>
          <Button
            className="bg-eco-green text-white w-full"
            onPress={onComplete}
            isDisabled={!allSigned}
            size="lg"
          >
            Complete Investment
          </Button>
        </ModalFooter>
      </ModalContent>
    </Modal>
  );
}