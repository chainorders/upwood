import React from 'react';
import { Button } from '@nextui-org/react';
import { UserCheck, Smartphone } from 'lucide-react';

interface IdentityVerificationProps {
  onContinueWeb: () => void;
}

export function IdentityVerification({ onContinueWeb }: IdentityVerificationProps) {
  return (
    <div className="flex flex-col items-center text-center">
      <div className="w-16 h-16 rounded-full bg-eco-green flex items-center justify-center mb-4">
        <UserCheck className="w-8 h-8" />
      </div>
      
      <h3 className="text-2xl font-bold mb-2 bg-gradient-to-r from-green-400 to-emerald-600 bg-clip-text text-transparent">
        Verify Your Identity
      </h3>
      <p className="text-sm text-gray-400 mb-8">
        Scan QR code to continue verification on mobile
      </p>

      <div className="bg-white p-8 rounded-lg mb-8">
        <img 
          src="https://api.qrserver.com/v1/create-qr-code/?size=200x200&data=https://verify.upwood.io/123456"
          alt="Verification QR Code"
          className="w-48 h-48"
        />
      </div>

      <Button
        className="w-full bg-eco-green text-white mb-4"
        endContent={<Smartphone className="w-4 h-4" />}
        onClick={onContinueWeb}
      >
        Continue on Web
      </Button>
    </div>
  );
}