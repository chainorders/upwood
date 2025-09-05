import React from 'react';
import { Button, Select, SelectItem } from '@nextui-org/react';
import { Briefcase, ArrowLeft, ArrowRight } from 'lucide-react';

interface EmploymentInfo {
  occupation: string;
  profession: string;
}

interface EmploymentDetailsFormProps {
  formData: EmploymentInfo;
  onInputChange: (field: keyof EmploymentInfo) => (e: React.ChangeEvent<HTMLInputElement>) => void;
  onSubmit: (e: React.FormEvent) => void;
  onBack: () => void;
}

const occupationTypes = [
  { label: "Self Employed", value: "self-employed" },
  { label: "Employed", value: "employed" },
  { label: "Part-time", value: "part-time" },
  { label: "Pensioner", value: "pensioner" },
  { label: "Student", value: "student" }
];

const professions = [
  { label: "Accountant", value: "accountant" },
  { label: "Architect", value: "architect" },
  { label: "Artist", value: "artist" },
  { label: "Business Owner", value: "business-owner" },
  { label: "Consultant", value: "consultant" },
  { label: "Doctor", value: "doctor" },
  { label: "Engineer", value: "engineer" },
  { label: "Financial Analyst", value: "financial-analyst" },
  { label: "Graphic Designer", value: "graphic-designer" },
  { label: "IT Professional", value: "it-professional" },
  { label: "Lawyer", value: "lawyer" },
  { label: "Manager", value: "manager" },
  { label: "Marketing Professional", value: "marketing" },
  { label: "Nurse", value: "nurse" },
  { label: "Professor", value: "professor" },
  { label: "Real Estate Agent", value: "real-estate-agent" },
  { label: "Researcher", value: "researcher" },
  { label: "Sales Professional", value: "sales" },
  { label: "Student", value: "student" },
  { label: "Teacher", value: "teacher" },
  { label: "Other", value: "other" }
];

export function EmploymentDetailsForm({ formData, onInputChange, onSubmit, onBack }: EmploymentDetailsFormProps) {
  return (
    <form onSubmit={onSubmit} className="space-y-6">
      <div className="flex flex-col items-center text-center mb-6">
        <div className="w-16 h-16 rounded-full bg-eco-green flex items-center justify-center mb-4">
          <Briefcase className="w-8 h-8" />
        </div>
        <h3 className="text-2xl font-bold mb-2 bg-gradient-to-r from-green-400 to-emerald-600 bg-clip-text text-transparent">
          Employment Details
        </h3>
        <p className="text-sm text-gray-400">
          Tell us about your employment status
        </p>
      </div>

      <div className="space-y-6">
        <Select
          label="Occupation"
          placeholder="Select your occupation"
          selectedKeys={formData.occupation ? [formData.occupation] : []}
          onSelectionChange={(keys) => {
            const value = Array.from(keys)[0]?.toString() || '';
            onInputChange('occupation')(value);
          }}
          startContent={<Briefcase className="w-4 h-4 text-gray-400" />}
          classNames={{
            base: "group",
            trigger: "bg-black/30 border-gray-800 h-[56px]",
            value: "text-white",
            popover: "bg-gray-900 border border-gray-800"
          }}
        >
          {occupationTypes.map((type) => (
            <SelectItem key={type.value} value={type.value}>
              {type.label}
            </SelectItem>
          ))}
        </Select>

        <Select
          label="Profession"
          placeholder="Select your profession"
          selectedKeys={formData.profession ? [formData.profession] : []}
          onSelectionChange={(keys) => {
            const value = Array.from(keys)[0]?.toString() || '';
            onInputChange('profession')(value);
          }}
          startContent={<Briefcase className="w-4 h-4 text-gray-400" />}
          classNames={{
            base: "group",
            trigger: "bg-black/30 border-gray-800 h-[56px]",
            value: "text-white",
            popover: "bg-gray-900 border border-gray-800"
          }}
        >
          {professions.map((profession) => (
            <SelectItem key={profession.value} value={profession.value}>
              {profession.label}
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
          isDisabled={!formData.occupation || !formData.profession}
        >
          Continue
        </Button>
      </div>
    </form>
  );
}