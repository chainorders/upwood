import React from 'react';
import { Button, Select, SelectItem } from '@nextui-org/react';
import { Building2, ArrowLeft, ArrowRight } from 'lucide-react';

interface ClientTypeInfo {
  industry: string;
  organizationType: string;
}

interface ClientTypeFormProps {
  formData: ClientTypeInfo;
  onInputChange: (field: keyof ClientTypeInfo) => (e: React.ChangeEvent<HTMLInputElement>) => void;
  onSubmit: (e: React.FormEvent) => void;
  onBack: () => void;
}

const industries = [
  { label: "Agriculture & Forestry", value: "agriculture" },
  { label: "Banking & Financial Services", value: "banking" },
  { label: "Construction & Real Estate", value: "construction" },
  { label: "Consumer Goods", value: "consumer-goods" },
  { label: "Education", value: "education" },
  { label: "Energy & Utilities", value: "energy" },
  { label: "Healthcare", value: "healthcare" },
  { label: "Information Technology", value: "it" },
  { label: "Insurance", value: "insurance" },
  { label: "Manufacturing", value: "manufacturing" },
  { label: "Media & Entertainment", value: "media" },
  { label: "Mining & Metals", value: "mining" },
  { label: "Professional Services", value: "professional-services" },
  { label: "Public Sector", value: "public-sector" },
  { label: "Retail & Wholesale", value: "retail" },
  { label: "Technology", value: "technology" },
  { label: "Telecommunications", value: "telecommunications" },
  { label: "Transportation & Logistics", value: "transportation" },
  { label: "Travel & Hospitality", value: "travel" },
  { label: "Other", value: "other" }
];

const organizationTypes = [
  { label: "Private Limited Company", value: "private-limited" },
  { label: "Public Limited Company", value: "public-limited" },
  { label: "Partnership", value: "partnership" },
  { label: "Limited Liability Partnership", value: "llp" },
  { label: "Sole Proprietorship", value: "sole-proprietorship" },
  { label: "Trust", value: "trust" },
  { label: "Foundation", value: "foundation" },
  { label: "Non-Profit Organization", value: "non-profit" },
  { label: "Government Entity", value: "government" },
  { label: "Cooperative", value: "cooperative" },
  { label: "Investment Fund", value: "investment-fund" },
  { label: "Holding Company", value: "holding-company" },
  { label: "Branch Office", value: "branch-office" },
  { label: "Representative Office", value: "representative-office" },
  { label: "Other", value: "other" }
];

export function ClientTypeForm({ formData, onInputChange, onSubmit, onBack }: ClientTypeFormProps) {
  return (
    <form onSubmit={onSubmit} className="space-y-6">
      <div className="flex flex-col items-center text-center mb-6">
        <div className="w-16 h-16 rounded-full bg-eco-green flex items-center justify-center mb-4">
          <Building2 className="w-8 h-8" />
        </div>
        <h3 className="text-2xl font-bold mb-2 bg-gradient-to-r from-green-400 to-emerald-600 bg-clip-text text-transparent">
          Client Type
        </h3>
        <p className="text-sm text-gray-400">
          Please specify your organization's industry and type
        </p>
      </div>

      <div className="space-y-6">
        <Select
          label="Industry/Sector"
          placeholder="Select your industry"
          selectedKeys={formData.industry ? [formData.industry] : []}
          onSelectionChange={(keys) => {
            const value = Array.from(keys)[0]?.toString() || '';
            onInputChange('industry')(value);
          }}
          classNames={{
            base: "group",
            trigger: "bg-black/30 border-gray-800 h-[56px]",
            value: "text-white",
            popover: "bg-gray-900 border border-gray-800"
          }}
        >
          {industries.map((industry) => (
            <SelectItem key={industry.value} value={industry.value}>
              {industry.label}
            </SelectItem>
          ))}
        </Select>

        <Select
          label="Type of Organization"
          placeholder="Select organization type"
          selectedKeys={formData.organizationType ? [formData.organizationType] : []}
          onSelectionChange={(keys) => {
            const value = Array.from(keys)[0]?.toString() || '';
            onInputChange('organizationType')(value);
          }}
          classNames={{
            base: "group",
            trigger: "bg-black/30 border-gray-800 h-[56px]",
            value: "text-white",
            popover: "bg-gray-900 border border-gray-800"
          }}
        >
          {organizationTypes.map((type) => (
            <SelectItem key={type.value} value={type.value}>
              {type.label}
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
          isDisabled={!formData.industry || !formData.organizationType}
        >
          Continue
        </Button>
      </div>
    </form>
  );
}