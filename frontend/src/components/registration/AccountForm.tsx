import React from 'react';
import { Button, Input, Radio, RadioGroup, Checkbox, Link } from '@nextui-org/react';
import { User, Building2, Mail, Lock, ArrowLeft, ArrowRight, UserCheck } from 'lucide-react';
import { useNavigate } from 'react-router-dom';

interface FormData {
  email: string;
  password: string;
  confirmPassword: string;
  accountType: string;
  termsAccepted: boolean;
}

interface AccountFormProps {
  formData: FormData;
  onInputChange: (field: keyof FormData) => (e: React.ChangeEvent<HTMLInputElement>) => void;
  onSubmit: (e: React.FormEvent) => void;
  onBack: () => void;
}

export function AccountForm({ formData, onInputChange, onSubmit, onBack }: AccountFormProps) {
  const navigate = useNavigate();

  return (
    <form onSubmit={onSubmit} className="space-y-6">
      <div className="flex flex-col items-center text-center mb-6">
        <div className="w-16 h-16 rounded-full bg-eco-green flex items-center justify-center mb-4">
          <UserCheck className="w-8 h-8" />
        </div>
        <h3 className="text-2xl font-bold mb-2 bg-gradient-to-r from-green-400 to-emerald-600 bg-clip-text text-transparent">
          Create Your Account
        </h3>
        <p className="text-sm text-gray-400">
          Enter your details to begin the registration process
        </p>
      </div>

      <div className="space-y-4">
        <Input
          type="email"
          label="Email address"
          placeholder="Enter your email"
          value={formData.email}
          onChange={onInputChange('email')}
          startContent={<Mail className="w-4 h-4 text-gray-400 group-data-[hover=true]:text-eco-green transition-colors" />}
          classNames={{
            base: "group",
            label: "text-gray-400",
            input: [
              "bg-transparent",
              "text-white",
              "placeholder:text-gray-500",
              "group-data-[has-value=true]:text-white"
            ].join(" "),
            inputWrapper: [
              "bg-black/30",
              "border-gray-800",
              "group-data-[focus=true]:bg-black/40",
              "group-data-[hover=true]:bg-black/40",
              "!cursor-text",
              "group-data-[focus=true]:border-eco-green/50",
              "group-data-[focus=true]:ring-eco-green/20",
              "group-data-[hover=true]:border-eco-green/50",
              "group-data-[hover=true]:ring-eco-green/20"
            ].join(" ")
          }}
        />

        <Input
          type="password"
          label="Password"
          placeholder="Create a password"
          value={formData.password}
          onChange={onInputChange('password')}
          startContent={<Lock className="w-4 h-4 text-gray-400 group-data-[hover=true]:text-eco-green transition-colors" />}
          classNames={{
            base: "group",
            label: "text-gray-400",
            input: [
              "bg-transparent",
              "text-white",
              "placeholder:text-gray-500",
              "group-data-[has-value=true]:text-white"
            ].join(" "),
            inputWrapper: [
              "bg-black/30",
              "border-gray-800",
              "group-data-[focus=true]:bg-black/40",
              "group-data-[hover=true]:bg-black/40",
              "!cursor-text",
              "group-data-[focus=true]:border-eco-green/50",
              "group-data-[focus=true]:ring-eco-green/20",
              "group-data-[hover=true]:border-eco-green/50",
              "group-data-[hover=true]:ring-eco-green/20"
            ].join(" ")
          }}
        />

        <Input
          type="password"
          label="Confirm Password"
          placeholder="Confirm your password"
          value={formData.confirmPassword}
          onChange={onInputChange('confirmPassword')}
          startContent={<Lock className="w-4 h-4 text-gray-400 group-data-[hover=true]:text-eco-green transition-colors" />}
          classNames={{
            base: "group",
            label: "text-gray-400",
            input: [
              "bg-transparent",
              "text-white",
              "placeholder:text-gray-500",
              "group-data-[has-value=true]:text-white"
            ].join(" "),
            inputWrapper: [
              "bg-black/30",
              "border-gray-800",
              "group-data-[focus=true]:bg-black/40",
              "group-data-[hover=true]:bg-black/40",
              "!cursor-text",
              "group-data-[focus=true]:border-eco-green/50",
              "group-data-[focus=true]:ring-eco-green/20",
              "group-data-[hover=true]:border-eco-green/50",
              "group-data-[hover=true]:ring-eco-green/20"
            ].join(" ")
          }}
        />

        <div className="pt-4">
          <RadioGroup
            label="Account Type"
            value={formData.accountType}
            onValueChange={(value) => onInputChange('accountType')({ target: { value } } as React.ChangeEvent<HTMLInputElement>)}
          >
            <div className="grid grid-cols-1 md:grid-cols-2 gap-4 pt-2">
              <Radio
                value="individual"
                classNames={{
                  base: "bg-black/30 border border-gray-800 rounded-lg p-4 cursor-pointer transition-all duration-300 hover:border-eco-green/50 hover:shadow-[0_0_15px_rgba(58,90,64,0.3)]",
                }}
              >
                <div className="flex items-center gap-2">
                  <User className="w-4 h-4 text-eco-green" />
                  <div>
                    <p className="font-medium text-white">Individual</p>
                    <p className="text-xs text-gray-400">Personal investment account</p>
                  </div>
                </div>
              </Radio>
              <Radio
                value="legal"
                classNames={{
                  base: "bg-black/30 border border-gray-800 rounded-lg p-4 cursor-pointer transition-all duration-300 hover:border-eco-green/50 hover:shadow-[0_0_15px_rgba(58,90,64,0.3)]",
                }}
              >
                <div className="flex items-center gap-2">
                  <Building2 className="w-4 h-4 text-eco-green" />
                  <div>
                    <p className="font-medium text-white">Legal Entity</p>
                    <p className="text-xs text-gray-400">Corporate investment account</p>
                  </div>
                </div>
              </Radio>
            </div>
          </RadioGroup>
        </div>

        <div className="pt-4">
          <Checkbox
            isSelected={formData.termsAccepted}
            onValueChange={(value) => onInputChange('termsAccepted')({ target: { value } } as React.ChangeEvent<HTMLInputElement>)}
            classNames={{
              label: "text-sm text-gray-400"
            }}
          >
            I agree to the{' '}
            <Link
              href="/terms"
              className="text-eco-green hover:underline"
              onClick={(e) => {
                e.preventDefault();
                navigate('/terms');
              }}
            >
              Terms & Conditions
            </Link>
            {' '}and{' '}
            <Link
              href="/privacy"
              className="text-eco-green hover:underline"
              onClick={(e) => {
                e.preventDefault();
                navigate('/privacy');
              }}
            >
              Privacy Policy
            </Link>
          </Checkbox>
        </div>
      </div>

      <div className="flex gap-3 mt-6">
        <Button
          className="flex-1 bg-gray-800 text-white"
          onClick={onBack}
          startContent={<ArrowLeft className="w-4 h-4" />}
        >
          Back
        </Button>
        <Button
          type="submit"
          className="flex-1 bg-eco-green text-white"
          endContent={<ArrowRight className="w-4 h-4" />}
          isDisabled={!formData.termsAccepted}
        >
          Continue
        </Button>
      </div>
    </form>
  );
}