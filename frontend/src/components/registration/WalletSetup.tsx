import React from 'react';
import { Button } from '@nextui-org/react';
import { Wallet, Plus, Link as LinkIcon } from 'lucide-react';

export function WalletSetup() {
  return (
    <div className="flex flex-col items-center text-center">
      <div className="w-16 h-16 rounded-full bg-eco-green flex items-center justify-center mb-4">
        <Wallet className="w-8 h-8" />
      </div>
      
      <h3 className="text-2xl font-bold mb-2 bg-gradient-to-r from-green-400 to-emerald-600 bg-clip-text text-transparent">
        Last step and you are good to go
      </h3>
      <p className="text-sm text-gray-400 mb-8">
        Let's create or connect your digital wallet
      </p>

      <div className="space-y-4 w-full">
        <Button
          className="w-full bg-eco-green text-white h-14 text-lg font-medium transition-all duration-300 hover:shadow-[0_0_20px_rgba(58,90,64,0.5)]"
          startContent={<Plus className="w-5 h-5" />}
        >
          Create New Wallet
        </Button>
        
        <Button
          className="w-full bg-gray-800 text-white h-14 text-lg font-medium transition-all duration-300 hover:shadow-[0_0_20px_rgba(58,90,64,0.5)]"
          startContent={<LinkIcon className="w-5 h-5" />}
        >
          Connect Existing Wallet
        </Button>
      </div>
    </div>
  );
}