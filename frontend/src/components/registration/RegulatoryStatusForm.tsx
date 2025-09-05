import React, { useState } from 'react';
import { Button, Select, SelectItem, Input, RadioGroup, Radio } from '@nextui-org/react';
import { Shield, ArrowLeft, ArrowRight } from 'lucide-react';
import { countries } from './countries';

interface RegulatoryStatusInfo {
  isFinanciallySupervised: string;
  financialAuthorityName?: string;
  financialAuthorityCountry?: string;
  isListedOnExchange: string;
  stockExchangeName?: string;
  stockExchangeCountry?: string;
}

interface RegulatoryStatusFormProps {
  formData: RegulatoryStatusInfo;
  onInputChange: (field: keyof RegulatoryStatusInfo) => (e: React.ChangeEvent<HTMLInputElement>) => void;
  onSubmit: (e: React.FormEvent) => void;
  onBack: () => void;
}

export function RegulatoryStatusForm({ formData, onInputChange, onSubmit, onBack }: RegulatoryStatusFormProps) {
  const isFinanciallySupervised = formData.isFinanciallySupervised === 'yes';
  const isListedOnExchange = formData.isListedOnExchange === 'yes';

  return (
    <form onSubmit={onSubmit} className="space-y-6">
      <div className="flex flex-col items-center text-center mb-6">
        <div className="w-16 h-16 rounded-full bg-eco-green flex items-center justify-center mb-4">
          <Shield className="w-8 h-8" />
        </div>
        <h3 className="text-2xl font-bold mb-2 bg-gradient-to-r from-green-400 to-emerald-600 bg-clip-text text-transparent">
          Regulatory Status
        </h3>
        <p className="text-sm text-gray-400">
          Please provide information about your company's regulatory status
        </p>
      </div>

      <div className="space-y-6">
        {/* Financial Authority Supervision */}
        <div className="space-y-4">
          <RadioGroup
            label="Is your company supervised by a Financial Authority?"
            value={formData.isFinanciallySupervised}
            onValueChange={(value) => onInputChange('isFinanciallySupervised')(value)}
            classNames={{
              label: "text-gray-400",
              wrapper: "gap-4"
            }}
          >
            <Radio 
              value="yes"
              classNames={{
                label: "text-white"
              }}
            >
              Yes
            </Radio>
            <Radio 
              value="no"
              classNames={{
                label: "text-white"
              }}
            >
              No
            </Radio>
          </RadioGroup>

          {isFinanciallySupervised && (
            <div className="space-y-4 pt-4">
              <Input
                type="text"
                label="Name of Financial Authority"
                placeholder="Enter authority name"
                value={formData.financialAuthorityName}
                onChange={onInputChange('financialAuthorityName')}
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

              <Select
                label="Country of Financial Authority"
                placeholder="Select country"
                selectedKeys={formData.financialAuthorityCountry ? [formData.financialAuthorityCountry] : []}
                onSelectionChange={(keys) => {
                  const value = Array.from(keys)[0]?.toString() || '';
                  onInputChange('financialAuthorityCountry')(value);
                }}
                classNames={{
                  base: "group",
                  label: "text-gray-400",
                  value: "text-white",
                  trigger: [
                    "bg-black/30",
                    "border-gray-800",
                    "data-[hover=true]:bg-black/40",
                    "group-data-[focus=true]:border-eco-green/50",
                    "group-data-[focus=true]:ring-eco-green/20",
                    "group-data-[hover=true]:border-eco-green/50",
                    "group-data-[hover=true]:ring-eco-green/20"
                  ].join(" "),
                  popover: "bg-gray-900 border border-gray-800"
                }}
              >
                {countries.map((country) => (
                  <SelectItem key={country.value} value={country.value}>
                    {country.label}
                  </SelectItem>
                ))}
              </Select>
            </div>
          )}
        </div>

        {/* Stock Exchange Listing */}
        <div className="space-y-4">
          <RadioGroup
            label="Is your company listed on a stock exchange?"
            value={formData.isListedOnExchange}
            onValueChange={(value) => onInputChange('isListedOnExchange')(value)}
            classNames={{
              label: "text-gray-400",
              wrapper: "gap-4"
            }}
          >
            <Radio 
              value="yes"
              classNames={{
                label: "text-white"
              }}
            >
              Yes
            </Radio>
            <Radio 
              value="no"
              classNames={{
                label: "text-white"
              }}
            >
              No
            </Radio>
          </RadioGroup>

          {isListedOnExchange && (
            <div className="space-y-4 pt-4">
              <Input
                type="text"
                label="Name of Stock Exchange"
                placeholder="Enter stock exchange name"
                value={formData.stockExchangeName}
                onChange={onInputChange('stockExchangeName')}
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

              <Select
                label="Country of Stock Exchange"
                placeholder="Select country"
                selectedKeys={formData.stockExchangeCountry ? [formData.stockExchangeCountry] : []}
                onSelectionChange={(keys) => {
                  const value = Array.from(keys)[0]?.toString() || '';
                  onInputChange('stockExchangeCountry')(value);
                }}
                classNames={{
                  base: "group",
                  label: "text-gray-400",
                  value: "text-white",
                  trigger: [
                    "bg-black/30",
                    "border-gray-800",
                    "data-[hover=true]:bg-black/40",
                    "group-data-[focus=true]:border-eco-green/50",
                    "group-data-[focus=true]:ring-eco-green/20",
                    "group-data-[hover=true]:border-eco-green/50",
                    "group-data-[hover=true]:ring-eco-green/20"
                  ].join(" "),
                  popover: "bg-gray-900 border border-gray-800"
                }}
              >
                {countries.map((country) => (
                  <SelectItem key={country.value} value={country.value}>
                    {country.label}
                  </SelectItem>
                ))}
              </Select>
            </div>
          )}
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
          isDisabled={
            !formData.isFinanciallySupervised || !formData.isListedOnExchange ||
            (isFinanciallySupervised && (!formData.financialAuthorityName || !formData.financialAuthorityCountry)) ||
            (isListedOnExchange && (!formData.stockExchangeName || !formData.stockExchangeCountry))
          }
        >
          Continue
        </Button>
      </div>
    </form>
  );
}