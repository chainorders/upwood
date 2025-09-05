import React from 'react';
import { Button, Input, Select, SelectItem } from '@nextui-org/react';
import { User, Globe, MapPin, ArrowLeft, ArrowRight } from 'lucide-react';
import { countries } from './countries';

interface PersonalInfo {
  firstName: string;
  lastName: string;
  nationality: string;
  address: string;
}

interface PersonalInfoFormProps {
  formData: PersonalInfo;
  onInputChange: (field: keyof PersonalInfo) => (e: React.ChangeEvent<HTMLInputElement>) => void;
  onSubmit: (e: React.FormEvent) => void;
  onBack: () => void;
}

export function PersonalInfoForm({ formData, onInputChange, onSubmit, onBack }: PersonalInfoFormProps) {
  return (
    <form onSubmit={onSubmit} className="space-y-6">
      <div className="flex flex-col items-center text-center mb-6">
        <div className="w-16 h-16 rounded-full bg-eco-green flex items-center justify-center mb-4">
          <User className="w-8 h-8" />
        </div>
        <h3 className="text-2xl font-bold mb-2 bg-gradient-to-r from-green-400 to-emerald-600 bg-clip-text text-transparent">
          Personal Information
        </h3>
        <p className="text-sm text-gray-400">
          Please provide your personal details
        </p>
      </div>

      <div className="space-y-4">
        <div className="grid grid-cols-1 md:grid-cols-2 gap-4">
          <Input
            type="text"
            label="First Name"
            placeholder="Enter your first name"
            value={formData.firstName}
            onChange={onInputChange('firstName')}
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
            type="text"
            label="Last Name"
            placeholder="Enter your last name"
            value={formData.lastName}
            onChange={onInputChange('lastName')}
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
        </div>

        <Select
          label="Nationality"
          placeholder="Select your nationality"
          selectedKeys={formData.nationality ? [formData.nationality] : []}
          onSelectionChange={(keys) => {
            const value = Array.from(keys)[0]?.toString() || '';
            onInputChange('nationality')(value);
          }}
          startContent={<Globe className="w-4 h-4 text-gray-400" />}
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

        <Input
          type="text"
          label="Address of Residence"
          placeholder="Enter your full address"
          value={formData.address}
          onChange={onInputChange('address')}
          startContent={<MapPin className="w-4 h-4 text-gray-400" />}
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
          isDisabled={!formData.firstName || !formData.lastName || !formData.nationality || !formData.address}
        >
          Continue
        </Button>
      </div>
    </form>
  );
}