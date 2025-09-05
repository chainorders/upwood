import React from 'react';
import { Button, Select, SelectItem } from '@nextui-org/react';
import { Coins, ArrowLeft, ArrowRight } from 'lucide-react';

interface IncomeInfo {
  sourceOfWealth: string;
  annualIncome: string;
  netWorth: string;
  annualTransactions: string;
}

interface IncomeDetailsFormProps {
  formData: IncomeInfo;
  onInputChange: (field: keyof IncomeInfo) => (e: React.ChangeEvent<HTMLInputElement>) => void;
  onSubmit: (e: React.FormEvent) => void;
  onBack: () => void;
}

const wealthSources = [
  { label: "Employment Income", value: "employment" },
  { label: "Business Income", value: "business" },
  { label: "Investment Returns", value: "investment" },
  { label: "Inheritance", value: "inheritance" },
  { label: "Real Estate", value: "real-estate" },
  { label: "Pension", value: "pension" },
  { label: "Savings", value: "savings" },
  { label: "Other", value: "other" }
];

const annualIncomeRanges = [
  { label: "Less than €25,000", value: "0-25000" },
  { label: "€25,000 - €50,000", value: "25000-50000" },
  { label: "€50,000 - €100,000", value: "50000-100000" },
  { label: "€100,000 - €250,000", value: "100000-250000" },
  { label: "€250,000 - €500,000", value: "250000-500000" },
  { label: "€500,000 - €1,000,000", value: "500000-1000000" },
  { label: "More than €1,000,000", value: "1000000+" }
];

const netWorthRanges = [
  { label: "Less than €50,000", value: "0-50000" },
  { label: "€50,000 - €100,000", value: "50000-100000" },
  { label: "€100,000 - €250,000", value: "100000-250000" },
  { label: "€250,000 - €500,000", value: "250000-500000" },
  { label: "€500,000 - €1,000,000", value: "500000-1000000" },
  { label: "€1,000,000 - €5,000,000", value: "1000000-5000000" },
  { label: "More than €5,000,000", value: "5000000+" }
];

const annualTransactionRanges = [
  { label: "Less than €10,000", value: "0-10000" },
  { label: "€10,000 - €50,000", value: "10000-50000" },
  { label: "€50,000 - €100,000", value: "50000-100000" },
  { label: "€100,000 - €500,000", value: "100000-500000" },
  { label: "€500,000 - €1,000,000", value: "500000-1000000" },
  { label: "More than €1,000,000", value: "1000000+" }
];

export function IncomeDetailsForm({ formData, onInputChange, onSubmit, onBack }: IncomeDetailsFormProps) {
  return (
    <form onSubmit={onSubmit} className="space-y-6">
      <div className="flex flex-col items-center text-center mb-6">
        <div className="w-16 h-16 rounded-full bg-eco-green flex items-center justify-center mb-4">
          <Coins className="w-8 h-8" />
        </div>
        <h3 className="text-2xl font-bold mb-2 bg-gradient-to-r from-green-400 to-emerald-600 bg-clip-text text-transparent">
          Income Details
        </h3>
        <p className="text-sm text-gray-400">
          Please provide information about your financial status
        </p>
      </div>

      <div className="space-y-6">
        <Select
          label="Source of Wealth"
          placeholder="Select your primary source of wealth"
          selectedKeys={formData.sourceOfWealth ? [formData.sourceOfWealth] : []}
          onSelectionChange={(keys) => {
            const value = Array.from(keys)[0]?.toString() || '';
            onInputChange('sourceOfWealth')(value);
          }}
          classNames={{
            base: "group",
            trigger: "bg-black/30 border-gray-800 h-[56px]",
            value: "text-white",
            popover: "bg-gray-900 border border-gray-800"
          }}
        >
          {wealthSources.map((source) => (
            <SelectItem key={source.value} value={source.value}>
              {source.label}
            </SelectItem>
          ))}
        </Select>

        <Select
          label="Annual Income"
          placeholder="Select your annual income range"
          selectedKeys={formData.annualIncome ? [formData.annualIncome] : []}
          onSelectionChange={(keys) => {
            const value = Array.from(keys)[0]?.toString() || '';
            onInputChange('annualIncome')(value);
          }}
          classNames={{
            base: "group",
            trigger: "bg-black/30 border-gray-800 h-[56px]",
            value: "text-white",
            popover: "bg-gray-900 border border-gray-800"
          }}
        >
          {annualIncomeRanges.map((range) => (
            <SelectItem key={range.value} value={range.value}>
              {range.label}
            </SelectItem>
          ))}
        </Select>

        <Select
          label="Net Worth"
          placeholder="Select your total net worth range"
          selectedKeys={formData.netWorth ? [formData.netWorth] : []}
          onSelectionChange={(keys) => {
            const value = Array.from(keys)[0]?.toString() || '';
            onInputChange('netWorth')(value);
          }}
          classNames={{
            base: "group",
            trigger: "bg-black/30 border-gray-800 h-[56px]",
            value: "text-white",
            popover: "bg-gray-900 border border-gray-800"
          }}
        >
          {netWorthRanges.map((range) => (
            <SelectItem key={range.value} value={range.value}>
              {range.label}
            </SelectItem>
          ))}
        </Select>

        <Select
          label="Estimated Annual Transactions"
          placeholder="Select your estimated annual transaction range"
          selectedKeys={formData.annualTransactions ? [formData.annualTransactions] : []}
          onSelectionChange={(keys) => {
            const value = Array.from(keys)[0]?.toString() || '';
            onInputChange('annualTransactions')(value);
          }}
          classNames={{
            base: "group",
            trigger: "bg-black/30 border-gray-800 h-[56px]",
            value: "text-white",
            popover: "bg-gray-900 border border-gray-800"
          }}
        >
          {annualTransactionRanges.map((range) => (
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
          isDisabled={!formData.sourceOfWealth || !formData.annualIncome || !formData.netWorth || !formData.annualTransactions}
        >
          Continue
        </Button>
      </div>
    </form>
  );
}