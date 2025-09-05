import React from 'react';
import { Button, Input, Select, SelectItem } from '@nextui-org/react';
import { Building2, Globe, Calendar, FileText, ArrowLeft, ArrowRight } from 'lucide-react';
import { countries } from './countries';

interface CompanyInfo {
  companyName: string;
  placeOfIncorporation: string;
  dateOfEstablishment: string;
  registrationNumber: string;
}

interface CompanyInformationFormProps {
  formData: CompanyInfo;
  onInputChange: (field: keyof CompanyInfo) => (e: React.ChangeEvent<HTMLInputElement>) => void;
  onSubmit: (e: React.FormEvent) => void;
  onBack: () => void;
}

export function CompanyInformationForm({ formData, onInputChange, onSubmit, onBack }: CompanyInformationFormProps) {
  return (
    <form onSubmit={onSubmit} className="space-y-6">
      <div className="flex flex-col items-center text-center mb-6">
        <div className="w-16 h-16 rounded-full bg-eco-green flex items-center justify-center mb-4">
          <Building2 className="w-8 h-8" />
        </div>
        <h3 className="text-2xl font-bold mb-2 bg-gradient-to-r from-green-400 to-emerald-600 bg-clip-text text-transparent">
          Company Information
        </h3>
        <p className="text-sm text-gray-400">
          Please provide your company details
        </p>
      </div>

      <div className="space-y-4">
        <Input
          type="text"
          label="Company Name"
          placeholder="Enter company name"
          value={formData.companyName}
          onChange={onInputChange('companyName')}
          startContent={<Building2 className="w-4 h-4 text-gray-400" />}
          classNames={{
            base: "group",
            input: "text-white bg-transparent",
            inputWrapper: [
              "bg-black/30 border-gray-800",
              "transition-all duration-300",
              "data-[hover=true]:bg-black/50",
              "data-[hover=true]:border-eco-green/50",
              "data-[hover=true]:shadow-[0_0_15px_rgba(58,90,64,0.3)]"
            ].join(" ")
          }}
        />

        <Select
          label="Place of Incorporation"
          placeholder="Select country of incorporation"
          selectedKeys={formData.placeOfIncorporation ? [formData.placeOfIncorporation] : []}
          onSelectionChange={(keys) => {
            const value = Array.from(keys)[0]?.toString() || '';
            onInputChange('placeOfIncorporation')(value);
          }}
          startContent={<Globe className="w-4 h-4 text-gray-400" />}
          classNames={{
            base: "group",
            trigger: "bg-black/30 border-gray-800 h-[56px]",
            value: "text-white",
            popover: "bg-gray-900 border border-gray-800"
          }}
        >
          {countries.map((country) => (
            <SelectItem key={country.value} value={country.value}>
              {country.label}
            </SelectItem>
          ))}
        </Select>

        <Input
          type="date"
          label="Date of Establishment"
          placeholder="Select date of establishment"
          value={formData.dateOfEstablishment}
          onChange={onInputChange('dateOfEstablishment')}
          startContent={<Calendar className="w-4 h-4 text-gray-400" />}
          classNames={{
            base: "group",
            input: "text-white bg-transparent",
            inputWrapper: [
              "bg-black/30 border-gray-800",
              "transition-all duration-300",
              "data-[hover=true]:bg-black/50",
              "data-[hover=true]:border-eco-green/50",
              "data-[hover=true]:shadow-[0_0_15px_rgba(58,90,64,0.3)]"
            ].join(" ")
          }}
        />

        <Input
          type="text"
          label="Company Registration Number"
          placeholder="Enter registration number"
          value={formData.registrationNumber}
          onChange={onInputChange('registrationNumber')}
          startContent={<FileText className="w-4 h-4 text-gray-400" />}
          classNames={{
            base: "group",
            input: "text-white bg-transparent",
            inputWrapper: [
              "bg-black/30 border-gray-800",
              "transition-all duration-300",
              "data-[hover=true]:bg-black/50",
              "data-[hover=true]:border-eco-green/50",
              "data-[hover=true]:shadow-[0_0_15px_rgba(58,90,64,0.3)]"
            ].join(" ")
          }}
        />
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
          isDisabled={!formData.companyName || !formData.placeOfIncorporation || !formData.dateOfEstablishment || !formData.registrationNumber}
        >
          Continue
        </Button>
      </div>
    </form>
  );
}