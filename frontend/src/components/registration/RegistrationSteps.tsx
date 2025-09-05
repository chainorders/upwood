import React from 'react';
import { UserCheck, ShieldCheck, FileCheck, Fingerprint, Key, Lock, Landmark, Coins, CheckCircle2, TreePine, Wallet, ArrowLeft, ArrowRight } from 'lucide-react';
import { Button } from '@nextui-org/react';
import { motion } from 'framer-motion';

interface Step {
  title: string;
  description: string;
  icon: React.ElementType;
  details: {
    icon: React.ElementType;
    title: string;
    description: string;
  }[];
}

interface RegistrationStepsProps {
  activeStep: number;
  onNext: () => void;
  onPrevious: () => void;
  onStartRegistration: () => void;
}

export const steps: Step[] = [
  {
    title: "Identity Verification (KYC/KYB)",
    description: "Complete identity verification steps to onboard Upwood platform",
    icon: UserCheck,
    details: [
      {
        icon: ShieldCheck,
        title: "Submit Documents",
        description: "Provide government-issued ID and proof of address"
      },
      {
        icon: FileCheck,
        title: "Verification Check",
        description: "Your documents will be reviewed"
      },
      {
        icon: Fingerprint,
        title: "Biometric Verification",
        description: "Complete a quick facial recognition check"
      }
    ]
  },
  {
    title: "Digital Wallet Setup",
    description: "Set up your digital wallet to manage your investments",
    icon: Wallet,
    details: [
      {
        icon: Key,
        title: "Create Wallet",
        description: "Set up a new digital wallet or connect an existing one"
      },
      {
        icon: Lock,
        title: "Secure Access",
        description: "Enable two-factor authentication for added security"
      },
      {
        icon: Landmark,
        title: "Link Bank Account",
        description: "Connect your bank account for seamless transactions"
      }
    ]
  },
  {
    title: "Platform Access",
    description: "Start investing in sustainable forestry projects",
    icon: TreePine,
    details: [
      {
        icon: Coins,
        title: "Investment Ready",
        description: "Browse and invest in verified forest projects"
      },
      {
        icon: CheckCircle2,
        title: "Full Access",
        description: "Get complete access to all platform features"
      },
      {
        icon: TreePine,
        title: "Start Investing",
        description: "Make your first investment in sustainable forestry"
      }
    ]
  }
];

export function RegistrationSteps({ activeStep, onNext, onPrevious, onStartRegistration }: RegistrationStepsProps) {
  return (
    <div className="flex flex-col items-center text-center">
      <div className="w-16 h-16 rounded-full bg-eco-green flex items-center justify-center mb-4">
        {React.createElement(steps[activeStep].icon, { className: "w-8 h-8" })}
      </div>
      
      <h3 className="text-2xl font-bold mb-2 bg-gradient-to-r from-green-400 to-emerald-600 bg-clip-text text-transparent">
        {steps[activeStep].title}
      </h3>
      <p className="text-sm text-gray-400 mb-6">{steps[activeStep].description}</p>

      <div className="space-y-3 w-full">
        {steps[activeStep].details.map((detail, index) => (
          <div 
            key={index}
            className="p-3 rounded-lg bg-eco-green/10 transition-all duration-300 hover:bg-eco-green/20 hover:shadow-[0_0_15px_rgba(58,90,64,0.5)] group cursor-pointer"
          >
            <div className="flex items-start gap-3">
              <div className="p-1.5 rounded-lg bg-eco-green/20 text-eco-green transition-colors group-hover:bg-eco-green/30">
                <detail.icon className="w-4 h-4" />
              </div>
              <div className="text-left flex-1 min-w-0">
                <h4 className="text-white font-semibold text-sm mb-0.5 transition-colors group-hover:text-eco-green">{detail.title}</h4>
                <p className="text-xs text-gray-400">{detail.description}</p>
              </div>
            </div>
          </div>
        ))}
      </div>

      <div className="flex gap-3 mt-6 w-full">
        {activeStep > 0 && (
          <Button
            className="flex-1 bg-gray-800 text-white"
            onClick={onPrevious}
            startContent={<ArrowLeft className="w-4 h-4" />}
          >
            Previous
          </Button>
        )}
        
        {activeStep < steps.length - 1 ? (
          <Button
            className="flex-1 bg-eco-green text-white"
            onClick={onNext}
            endContent={<ArrowRight className="w-4 h-4" />}
          >
            Next
          </Button>
        ) : (
          <Button
            className="flex-1 bg-eco-green text-white"
            onClick={onStartRegistration}
            endContent={<ArrowRight className="w-4 h-4" />}
          >
            Start Registration
          </Button>
        )}
      </div>
    </div>
  );
}