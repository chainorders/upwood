import React from 'react';
import { Button, Select, SelectItem } from '@nextui-org/react';
import { Coins, ArrowLeft, ArrowRight } from 'lucide-react';

interface TransactionDetailsInfo {
  anticipatedAnnualAmount: string;
}

interface TransactionDetailsFormProps {
  formData: TransactionDetailsInfo;
  onInputChange: (field: keyof TransactionDetailsInfo) => (e: React.ChangeEvent<HTMLInputElement>) => void;
  onSubmit: (e: React.FormEvent) => void;
  onBack: () => void;
}

const transactionRanges = [
  { label: "Less than €100,000", value: "0-100000" },
  { label: "€100,000 - €500,000", value: "100000-500000" },
  { label: "€500,000 - €1,000,000", value: "500000-1000000" },
  { label: "€1,000,000 - €5,000,000", value: "1000000-5000000" },
  { label: "€5,000,000 - €10,000,000", value: "5000000-10000000" },
  { label: "€10,000,000 - €50,000,000", value: "10000000-50000000" },
  { label: "More than €50,000,000", value: "50000000+" }
];

export function TransactionDetailsForm({ formData, onInputChange, onSubmit, onBack }: TransactionDetailsFormProps) {
  return (
    <form onSubmit={onSubmit} className="space-y-6">
      <div className="flex flex-col items-center text-center mb-6">
        <div className="w-16 h-16 rounded-full bg-eco-green flex items-center justify-center mb-4">
          <Coins className="w-8 h-8" />
        </div>
        <h3 className="text-2xl font-bold mb-2 bg-gradient-to-r from-green-400 to-emerald-600 bg-clip-text text-transparent">
          Transaction Details
        </h3>
        <p className="text-sm text-gray-400">
          Please provide your anticipated annual transaction amount
        </p>
      </div>

      <div className="space-y-6">
        <Select
          label="Anticipated Annual Transaction Amount"
          placeholder="Select anticipated amount range"
          selectedKeys={formData.anticipatedAnnualAmount ? [formData.anticipatedAnnualAmount] : []}
          onSelectionChange={(keys) => {
            const value = Array.from(keys)[0]?.toString() || '';
            onInputChange('anticipatedAnnualAmount')(value);
          }}
          classNames={{
            base: "group",
            trigger: "bg-black/30 border-gray-800 h-[56px]",
            value: "text-white",
            popover: "bg-gray-900 border border-gray-800"
          }}
        >
          {transactionRanges.map((range) => (
            <SelectItem key={range.value} value={range.value}>
              {range.label}
            </SelectItem>
          ))}
        </Select>
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
          isDisabled={!formData.anticipatedAnnualAmount}
        >
          Continue
        </Button>
      </div>
    </form>
  );
}