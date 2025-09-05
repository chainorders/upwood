import React, { useState } from 'react';
import { Modal, ModalContent, ModalHeader, ModalBody, ModalFooter, Button, Input, Checkbox } from "@nextui-org/react";
import { Link, useNavigate } from 'react-router-dom';

interface InvitationModalProps {
  isOpen: boolean;
  onClose: () => void;
}

export function InvitationModal({ isOpen, onClose }: InvitationModalProps) {
  const navigate = useNavigate();
  const [email, setEmail] = useState('');
  const [investment, setInvestment] = useState('');
  const [agreed, setAgreed] = useState(false);
  const [error, setError] = useState('');

  const handleSubmit = () => {
    if (!email || !investment || !agreed) {
      setError('Please fill in all fields and accept the terms');
      return;
    }

    // For now, just close the modal
    onClose();
  };

  const handleTermsClick = (e: React.MouseEvent) => {
    e.preventDefault();
    onClose();
    navigate('/terms');
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
          <h3 className="text-xl font-bold text-eco-green">Request Invitation</h3>
        </ModalHeader>
        <ModalBody>
          <div className="space-y-6">
            <Input
              type="email"
              label="Email address"
              placeholder="Enter your email"
              value={email}
              onChange={(e) => setEmail(e.target.value)}
              variant="bordered"
              classNames={{
                input: "text-white",
                inputWrapper: "border-gray-600",
              }}
            />
            <Input
              type="number"
              label="Planned investment amount (EUR)"
              placeholder="Enter amount"
              value={investment}
              onChange={(e) => setInvestment(e.target.value)}
              variant="bordered"
              classNames={{
                input: "text-white",
                inputWrapper: "border-gray-600",
              }}
              startContent={
                <div className="pointer-events-none flex items-center">
                  <span className="text-default-400 text-small">€</span>
                </div>
              }
            />
            <div className="flex items-start gap-2">
              <Checkbox
                isSelected={agreed}
                onValueChange={setAgreed}
                className="text-sm"
              />
              <span className="text-gray-400 text-sm">
                By expressing your interest to join Upwood platform you agree to{' '}
                <a 
                  href="/terms" 
                  className="text-eco-green hover:underline"
                  onClick={handleTermsClick}
                >
                  Terms of Use
                </a>
                {' '}and be contacted for marketing purposes
              </span>
            </div>
            {error && (
              <p className="text-red-500 text-sm">{error}</p>
            )}
          </div>
        </ModalBody>
        <ModalFooter>
          <Button
            className="bg-eco-green text-white w-full"
            onClick={handleSubmit}
          >
            Join Waitlist
          </Button>
        </ModalFooter>
      </ModalContent>
    </Modal>
  );
}