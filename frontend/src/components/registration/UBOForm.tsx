import React, { useState } from 'react';
import { Button, Input, Select, SelectItem, Card } from '@nextui-org/react';
import { Users, User, Globe, Calendar, MapPin, ArrowLeft, ArrowRight, Plus, Trash2 } from 'lucide-react';
import { countries } from './countries';
import { motion, AnimatePresence } from 'framer-motion';

interface UBOInfo {
  firstName: string;
  lastName: string;
  nationality: string;
  dateOfBirth: string;
  address: string;
}

interface UBOFormProps {
  onSubmit: (e: React.FormEvent) => void;
  onBack: () => void;
}

export function UBOForm({ onSubmit, onBack }: UBOFormProps) {
  const [uboList, setUboList] = useState<UBOInfo[]>([
    {
      firstName: '',
      lastName: '',
      nationality: '',
      dateOfBirth: '',
      address: ''
    }
  ]);

  const handleInputChange = (index: number, field: keyof UBOInfo) => (
    e: React.ChangeEvent<HTMLInputElement>
  ) => {
    const newUboList = [...uboList];
    newUboList[index] = {
      ...newUboList[index],
      [field]: e.target.value
    };
    setUboList(newUboList);
  };

  const handleAddUBO = () => {
    setUboList([
      ...uboList,
      {
        firstName: '',
        lastName: '',
        nationality: '',
        dateOfBirth: '',
        address: ''
      }
    ]);
  };

  const handleRemoveUBO = (index: number) => {
    if (uboList.length === 1) return;
    setUboList(uboList.filter((_, i) => i !== index));
  };

  const isFormValid = uboList.every(ubo => 
    ubo.firstName && 
    ubo.lastName && 
    ubo.nationality && 
    ubo.dateOfBirth && 
    ubo.address
  );

  return (
    <form onSubmit={onSubmit} className="space-y-6">
      <div className="flex flex-col items-center text-center mb-6">
        <div className="w-16 h-16 rounded-full bg-eco-green flex items-center justify-center mb-4">
          <Users className="w-8 h-8" />
        </div>
        <h3 className="text-2xl font-bold mb-2 bg-gradient-to-r from-green-400 to-emerald-600 bg-clip-text text-transparent">
          Ultimate Beneficial Owners
        </h3>
        <p className="text-sm text-gray-400">
          List individuals that own more than 25% of the company
        </p>
      </div>

      <div className="space-y-6">
        <AnimatePresence>
          {uboList.map((ubo, index) => (
            <motion.div
              key={index}
              initial={{ opacity: 0, y: 20 }}
              animate={{ opacity: 1, y: 0 }}
              exit={{ opacity: 0, y: -20 }}
              transition={{ duration: 0.3 }}
            >
              <Card className="bg-gray-900/50 border-gray-800">
                <div className="p-6">
                  <div className="flex items-center justify-between mb-4">
                    <h4 className="text-lg font-semibold text-white">
                      Beneficial Owner {index + 1}
                    </h4>
                    {uboList.length > 1 && (
                      <Button
                        isIconOnly
                        variant="light"
                        className="text-gray-400 hover:text-red-500"
                        onClick={() => handleRemoveUBO(index)}
                      >
                        <Trash2 className="w-4 h-4" />
                      </Button>
                    )}
                  </div>

                  <div className="space-y-4">
                    <div className="grid grid-cols-1 md:grid-cols-2 gap-4">
                      <Input
                        type="text"
                        label="First Name"
                        placeholder="Enter first name"
                        value={ubo.firstName}
                        onChange={handleInputChange(index, 'firstName')}
                        startContent={<User className="w-4 h-4 text-gray-400" />}
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
                        label="Last Name"
                        placeholder="Enter last name"
                        value={ubo.lastName}
                        onChange={handleInputChange(index, 'lastName')}
                        startContent={<User className="w-4 h-4 text-gray-400" />}
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

                    <div className="grid grid-cols-1 md:grid-cols-2 gap-4">
                      <Select
                        label="Nationality"
                        placeholder="Select nationality"
                        value={ubo.nationality}
                        onChange={handleInputChange(index, 'nationality')}
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
                        label="Date of Birth"
                        placeholder="Select date of birth"
                        value={ubo.dateOfBirth}
                        onChange={handleInputChange(index, 'dateOfBirth')}
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
                    </div>

                    <Input
                      type="text"
                      label="Address of Residence"
                      placeholder="Enter full address"
                      value={ubo.address}
                      onChange={handleInputChange(index, 'address')}
                      startContent={<MapPin className="w-4 h-4 text-gray-400" />}
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
                </div>
              </Card>
            </motion.div>
          ))}
        </AnimatePresence>

        <Button
          className="w-full bg-gray-800 text-white"
          onClick={handleAddUBO}
          startContent={<Plus className="w-4 h-4" />}
        >
          Add Another Beneficial Owner
        </Button>
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
          isDisabled={!isFormValid}
        >
          Continue
        </Button>
      </div>
    </form>
  );
}