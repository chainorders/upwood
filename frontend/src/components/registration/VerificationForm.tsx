import React from 'react';
import { Button, Input } from '@nextui-org/react';
import { Mail, Send } from 'lucide-react';

interface VerificationFormProps {
  email: string;
  verificationCode: string[];
  inputRefs: React.RefObject<HTMLInputElement>[];
  onVerificationCodeChange: (index: number, value: string) => void;
  onVerificationKeyDown: (index: number, e: React.KeyboardEvent) => void;
  onVerify: () => void;
  onResend: () => void;
}

export function VerificationForm({
  email,
  verificationCode,
  inputRefs,
  onVerificationCodeChange,
  onVerificationKeyDown,
  onVerify,
  onResend
}: VerificationFormProps) {
  return (
    <div className="flex flex-col items-center text-center">
      <div className="w-16 h-16 rounded-full bg-eco-green flex items-center justify-center mb-4">
        <Mail className="w-8 h-8" />
      </div>
      
      <h3 className="text-2xl font-bold mb-2 bg-gradient-to-r from-green-400 to-emerald-600 bg-clip-text text-transparent">
        Verify Your Email
      </h3>
      <p className="text-sm text-gray-400 mb-8">
        We've sent a verification code to {email}
      </p>

      <div className="flex gap-2 mb-6">
        {verificationCode.map((digit, index) => (
          <Input
            key={index}
            ref={inputRefs[index]}
            type="text"
            maxLength={1}
            value={digit}
            onChange={(e) => onVerificationCodeChange(index, e.target.value)}
            onKeyDown={(e) => onVerificationKeyDown(index, e)}
            classNames={{
              base: "group",
              input: "text-center text-xl font-mono text-white bg-transparent",
              inputWrapper: [
                "h-14 w-12 bg-black/30 border-gray-800",
                "transition-all duration-300",
                "data-[hover=true]:bg-black/50",
                "data-[hover=true]:border-eco-green/50",
                "data-[hover=true]:shadow-[0_0_15px_rgba(58,90,64,0.3)]"
              ].join(" ")
            }}
          />
        ))}
      </div>

      <Button
        className="w-full bg-eco-green text-white mb-4"
        onClick={onVerify}
        isDisabled={verificationCode.some(digit => !digit)}
      >
        Verify Code
      </Button>

      <div className="flex items-center gap-2 text-sm">
        <span className="text-gray-400">Didn't receive the code?</span>
        <Button
          variant="light"
          className="text-eco-green p-0"
          onClick={onResend}
          startContent={<Send className="w-4 h-4" />}
        >
          Resend
        </Button>
      </div>
    </div>
  );
}