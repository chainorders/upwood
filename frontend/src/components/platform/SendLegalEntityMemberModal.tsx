import React, { useState } from 'react';
import { Modal, ModalContent, ModalHeader, ModalBody, ModalFooter, Button, Input } from "@nextui-org/react";
import { Mail, Send } from 'lucide-react';

interface SendLegalEntityMemberModalProps {
  isOpen: boolean;
  onClose: () => void;
}

export function SendLegalEntityMemberModal({ isOpen, onClose }: SendLegalEntityMemberModalProps) {
  const [email, setEmail] = useState('');
  const [isLoading, setIsLoading] = useState(false);

  const handleSendInvitation = async () => {
    if (!email) return;

    setIsLoading(true);
    // Simulate API call
    await new Promise(resolve => setTimeout(resolve, 1000));
    setIsLoading(false);
    setEmail('');
    onClose();
  };

  return (
    <Modal 
      isOpen={isOpen} 
      onClose={onClose}
      className="dark"
    >
      <ModalContent>
        <ModalHeader className="flex flex-col gap-1">
          <h3 className="text-xl font-bold text-white">Add Legal Entity Member</h3>
          <p className="text-sm text-gray-400">
            Send an invitation to add a new member to your legal entity
          </p>
        </ModalHeader>
        <ModalBody>
          <div className="space-y-4">
            <Input
              type="email"
              label="Email address"
              placeholder="Enter member's email"
              value={email}
              onChange={(e) => setEmail(e.target.value)}
              startContent={<Mail className="w-4 h-4 text-gray-400" />}
              classNames={{
                input: "text-white",
                inputWrapper: "bg-black/30 border-gray-800"
              }}
            />
          </div>
        </ModalBody>
        <ModalFooter>
          <Button
            className="bg-eco-green text-white w-full"
            size="lg"
            isLoading={isLoading}
            onClick={handleSendInvitation}
            startContent={!isLoading && <Send className="w-4 h-4" />}
          >
            Send Invitation
          </Button>
        </ModalFooter>
      </ModalContent>
    </Modal>
  );
}