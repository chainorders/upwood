import React, { useState } from 'react';
import { Modal, ModalContent, ModalHeader, ModalBody, ModalFooter, Button } from "@nextui-org/react";
import { Wallet, AlertCircle } from 'lucide-react';
import { useNavigate } from 'react-router-dom';

interface LaunchAppModalProps {
  isOpen: boolean;
  onClose: () => void;
  onRequestInvitation: () => void;
}

export function LaunchAppModal({ isOpen, onClose, onRequestInvitation }: LaunchAppModalProps) {
  const navigate = useNavigate();
  const [error, setError] = useState('');

  const handleConnectWallet = () => {
    // For demo purposes, directly navigate to the platform
    onClose();
    navigate('/platform/projects');
  };

  const handleRequestInvitation = () => {
    onClose();
    onRequestInvitation();
  };

  return (
    <Modal 
      isOpen={isOpen} 
      onClose={onClose}
      className="dark"
      size="lg"
      classNames={{
        base: "bg-gray-900 border border-gray-800",
        header: "border-b border-gray-800",
        body: "py-6",
        footer: "border-t border-gray-800",
      }}
    >
      <ModalContent>
        <ModalHeader className="flex flex-col gap-1">
          <h3 className="text-2xl font-bold text-eco-green">Launch Upwood App</h3>
          <p className="text-sm text-gray-400 font-normal">
            Connect your wallet or request an invitation to access the platform
          </p>
        </ModalHeader>
        <ModalBody>
          <div className="space-y-6">
            <div className="space-y-4">
              <Button
                className="w-full bg-eco-green hover:bg-eco-green/90 text-white h-12 font-semibold"
                //onClick={handleConnectWallet}
                startContent={<Wallet className="w-5 h-5" />}
              >
                Connect Wallet (coming soon)
              </Button>

              {error && (
                <div className="flex items-start gap-3 p-4 bg-red-500/10 rounded-lg border border-red-500/20">
                  <AlertCircle className="w-5 h-5 text-red-500 flex-shrink-0 mt-0.5" />
                  <p className="text-sm text-red-400">{error}</p>
                </div>
              )}

              <div className="relative py-4">
                <div className="absolute inset-0 flex items-center">
                  <div className="w-full border-t border-gray-800"></div>
                </div>
                <div className="relative flex justify-center">
                  <span className="px-4 bg-gray-900 text-sm text-gray-500">or</span>
                </div>
              </div>

              <Button
                className="w-full bg-eco-green/10 hover:bg-eco-green/20 text-eco-green h-12 font-semibold"
                variant="flat"
                onClick={handleRequestInvitation}
              >
                Request Invitation
              </Button>
            </div>
          </div>
        </ModalBody>
        <ModalFooter>
          <Button
            className="w-full text-gray-400 hover:text-white h-11"
            variant="light"
            onPress={onClose}
          >
            Close
          </Button>
        </ModalFooter>
      </ModalContent>
    </Modal>
  );
}