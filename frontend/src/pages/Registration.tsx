import React, { useState } from 'react';
import { motion, AnimatePresence } from 'framer-motion';
import { Card, Button } from '@nextui-org/react';
import { Navigation } from '../components/Navigation';
import { Footer } from '../components/Footer';
import { useNavigate } from 'react-router-dom';

// Import registration components
import { RegistrationSteps } from '../components/registration/RegistrationSteps';
import { AccountForm } from '../components/registration/AccountForm';
import { PersonalInfoForm } from '../components/registration/PersonalInfoForm';
import { CompanyRepresentativeForm } from '../components/registration/CompanyRepresentativeForm';
import { CompanyInformationForm } from '../components/registration/CompanyInformationForm';
import { UBOForm } from '../components/registration/UBOForm';
import { ClientTypeForm } from '../components/registration/ClientTypeForm';
import { EmploymentDetailsForm } from '../components/registration/EmploymentDetailsForm';
import { IncomeDetailsForm } from '../components/registration/IncomeDetailsForm';
import { DocumentVerificationForm } from '../components/registration/DocumentVerificationForm';
import { VerificationForm } from '../components/registration/VerificationForm';
import { IdentityVerification } from '../components/registration/IdentityVerification';
import { WalletSetup } from '../components/registration/WalletSetup';
import { RegulatoryStatusForm } from '../components/registration/RegulatoryStatusForm';
import { TransactionDetailsForm } from '../components/registration/TransactionDetailsForm';

export default function Registration() {
  const navigate = useNavigate();
  const [activeStep, setActiveStep] = useState(0);
  const [showRegistrationForm, setShowRegistrationForm] = useState(false);
  const [showPersonalInfo, setShowPersonalInfo] = useState(false);
  const [showCompanyRepresentative, setShowCompanyRepresentative] = useState(false);
  const [showCompanyInformation, setShowCompanyInformation] = useState(false);
  const [showUBO, setShowUBO] = useState(false);
  const [showClientType, setShowClientType] = useState(false);
  const [showRegulatoryStatus, setShowRegulatoryStatus] = useState(false);
  const [showTransactionDetails, setShowTransactionDetails] = useState(false);
  const [showEmploymentDetails, setShowEmploymentDetails] = useState(false);
  const [showIncomeDetails, setShowIncomeDetails] = useState(false);
  const [showDocumentVerification, setShowDocumentVerification] = useState(false);
  const [showVerification, setShowVerification] = useState(false);
  const [showIdentityVerification, setShowIdentityVerification] = useState(false);
  const [showWalletSetup, setShowWalletSetup] = useState(false);
  const [verificationCode, setVerificationCode] = useState(['', '', '', '', '', '']);
  const [formData, setFormData] = useState({
    email: '',
    password: '',
    confirmPassword: '',
    accountType: 'individual',
    termsAccepted: false,
    firstName: '',
    lastName: '',
    nationality: '',
    address: '',
    companyName: '',
    placeOfIncorporation: '',
    dateOfEstablishment: '',
    registrationNumber: '',
    industry: '',
    organizationType: '',
    occupation: '',
    profession: '',
    sourceOfWealth: '',
    annualIncome: '',
    netWorth: '',
    annualTransactions: '',
    isFinanciallySupervised: '',
    financialAuthorityName: '',
    financialAuthorityCountry: '',
    isListedOnExchange: '',
    stockExchangeName: '',
    stockExchangeCountry: '',
    anticipatedAnnualAmount: ''
  });

  const inputRefs = Array(6).fill(0).map(() => React.createRef<HTMLInputElement>());

  const handleNext = () => {
    if (activeStep < 2) {
      setActiveStep(prev => prev + 1);
    }
  };

  const handlePrevious = () => {
    if (activeStep > 0) {
      setActiveStep(prev => prev - 1);
    }
  };

  const handleStartRegistration = () => {
    setShowRegistrationForm(true);
  };

  const handleSubmitRegistration = (e: React.FormEvent) => {
    e.preventDefault();
    if (!formData.termsAccepted) {
      return;
    }
    if (formData.accountType === 'individual') {
      setShowPersonalInfo(true);
    } else {
      setShowCompanyRepresentative(true);
    }
  };

  const handleSubmitPersonalInfo = (e: React.FormEvent) => {
    e.preventDefault();
    setShowEmploymentDetails(true);
  };

  const handleSubmitCompanyRepresentative = (e: React.FormEvent) => {
    e.preventDefault();
    setShowCompanyInformation(true);
  };

  const handleSubmitCompanyInformation = (e: React.FormEvent) => {
    e.preventDefault();
    setShowUBO(true);
  };

  const handleSubmitUBO = (e: React.FormEvent) => {
    e.preventDefault();
    setShowClientType(true);
  };

  const handleSubmitClientType = (e: React.FormEvent) => {
    e.preventDefault();
    setShowRegulatoryStatus(true);
  };

  const handleSubmitRegulatoryStatus = (e: React.FormEvent) => {
    e.preventDefault();
    setShowTransactionDetails(true);
  };

  const handleSubmitTransactionDetails = (e: React.FormEvent) => {
    e.preventDefault();
    setShowDocumentVerification(true);
  };

  const handleSubmitEmploymentDetails = (e: React.FormEvent) => {
    e.preventDefault();
    setShowIncomeDetails(true);
  };

  const handleSubmitIncomeDetails = (e: React.FormEvent) => {
    e.preventDefault();
    setShowDocumentVerification(true);
  };

  const handleSubmitDocumentVerification = (e: React.FormEvent) => {
    e.preventDefault();
    setShowVerification(true);
  };

  const handleInputChange = (field: keyof typeof formData) => (
    e: React.ChangeEvent<HTMLInputElement> | { target: { value: string } } | string
  ) => {
    let value: string;

    if (typeof e === 'string') {
      // Handle direct string value (from Select)
      value = e;
    } else if ('target' in e) {
      // Handle event from Input
      value = e.target.value;
    } else {
      // Fallback
      value = '';
    }

    setFormData(prev => ({
      ...prev,
      [field]: value
    }));
  };

  const handleVerificationCodeChange = (index: number, value: string) => {
    if (value && !/^\d+$/.test(value)) return;

    const newCode = [...verificationCode];
    newCode[index] = value;
    setVerificationCode(newCode);

    if (value && index < 5) {
      inputRefs[index + 1].current?.focus();
    }
  };

  const handleVerificationKeyDown = (index: number, e: React.KeyboardEvent) => {
    if (e.key === 'Backspace' && !verificationCode[index] && index > 0) {
      inputRefs[index - 1].current?.focus();
    }
  };

  const handleVerifyCode = () => {
    setShowVerification(false);
    setShowIdentityVerification(true);
  };

  const handleResendCode = () => {
    console.log('Resending code to:', formData.email);
  };

  const handleContinueWeb = () => {
    setShowIdentityVerification(false);
    setShowWalletSetup(true);
  };

  const renderContent = () => {
    if (showWalletSetup) {
      return <WalletSetup />;
    }

    if (showIdentityVerification) {
      return <IdentityVerification onContinueWeb={handleContinueWeb} />;
    }

    if (showVerification) {
      return (
        <VerificationForm
          email={formData.email}
          verificationCode={verificationCode}
          inputRefs={inputRefs}
          onVerificationCodeChange={handleVerificationCodeChange}
          onVerificationKeyDown={handleVerificationKeyDown}
          onVerify={handleVerifyCode}
          onResend={handleResendCode}
        />
      );
    }

    if (showDocumentVerification) {
      return (
        <DocumentVerificationForm
          onSubmit={handleSubmitDocumentVerification}
          onBack={() => setShowDocumentVerification(false)}
          accountType={formData.accountType as 'individual' | 'legal'}
        />
      );
    }

    if (showIncomeDetails) {
      return (
        <IncomeDetailsForm
          formData={formData}
          onInputChange={handleInputChange}
          onSubmit={handleSubmitIncomeDetails}
          onBack={() => setShowIncomeDetails(false)}
        />
      );
    }

    if (showEmploymentDetails) {
      return (
        <EmploymentDetailsForm
          formData={formData}
          onInputChange={handleInputChange}
          onSubmit={handleSubmitEmploymentDetails}
          onBack={() => {
            setShowEmploymentDetails(false);
            if (formData.accountType === 'individual') {
              setShowPersonalInfo(true);
            } else {
              setShowTransactionDetails(true);
            }
          }}
        />
      );
    }

    if (showTransactionDetails) {
      return (
        <TransactionDetailsForm
          formData={formData}
          onInputChange={handleInputChange}
          onSubmit={handleSubmitTransactionDetails}
          onBack={() => {
            setShowTransactionDetails(false);
            setShowRegulatoryStatus(true);
          }}
        />
      );
    }

    if (showRegulatoryStatus) {
      return (
        <RegulatoryStatusForm
          formData={formData}
          onInputChange={handleInputChange}
          onSubmit={handleSubmitRegulatoryStatus}
          onBack={() => {
            setShowRegulatoryStatus(false);
            setShowClientType(true);
          }}
        />
      );
    }

    if (showClientType) {
      return (
        <ClientTypeForm
          formData={formData}
          onInputChange={handleInputChange}
          onSubmit={handleSubmitClientType}
          onBack={() => {
            setShowClientType(false);
            setShowUBO(true);
          }}
        />
      );
    }

    if (showUBO) {
      return (
        <UBOForm
          onSubmit={handleSubmitUBO}
          onBack={() => {
            setShowUBO(false);
            setShowCompanyInformation(true);
          }}
        />
      );
    }

    if (showCompanyInformation) {
      return (
        <CompanyInformationForm
          formData={formData}
          onInputChange={handleInputChange}
          onSubmit={handleSubmitCompanyInformation}
          onBack={() => {
            setShowCompanyInformation(false);
            setShowCompanyRepresentative(true);
          }}
        />
      );
    }

    if (showCompanyRepresentative) {
      return (
        <CompanyRepresentativeForm
          formData={formData}
          onInputChange={handleInputChange}
          onSubmit={handleSubmitCompanyRepresentative}
          onBack={() => setShowCompanyRepresentative(false)}
        />
      );
    }

    if (showPersonalInfo) {
      return (
        <PersonalInfoForm
          formData={formData}
          onInputChange={handleInputChange}
          onSubmit={handleSubmitPersonalInfo}
          onBack={() => setShowPersonalInfo(false)}
        />
      );
    }

    if (showRegistrationForm) {
      return (
        <AccountForm
          formData={formData}
          onInputChange={handleInputChange}
          onSubmit={handleSubmitRegistration}
          onBack={() => setShowRegistrationForm(false)}
        />
      );
    }

    return (
      <RegistrationSteps
        activeStep={activeStep}
        onNext={handleNext}
        onPrevious={handlePrevious}
        onStartRegistration={handleStartRegistration}
      />
    );
  };

  return (
    <div className="min-h-screen bg-black text-white">
      <Navigation onLaunchApp={() => navigate('/')} />
      
      <div className="max-w-7xl mx-auto px-4 py-12">
        <div className="text-center max-w-3xl mx-auto mb-8">
          <motion.h1 
            initial={{ opacity: 0, y: 20 }}
            animate={{ opacity: 1, y: 0 }}
            transition={{ duration: 0.6 }}
            className="text-4xl font-bold mb-4 bg-gradient-to-r from-green-400 to-emerald-600 bg-clip-text text-transparent"
          >
            Join Upwood Platform
          </motion.h1>
          <motion.p
            initial={{ opacity: 0, y: 20 }}
            animate={{ opacity: 1, y: 0 }}
            transition={{ duration: 0.6, delay: 0.2 }}
            className="text-lg text-gray-400"
          >
            Follow these simple steps to start investing in sustainable forestry
          </motion.p>
        </div>

        <div className="max-w-2xl mx-auto">
          <div className="relative">
            <AnimatePresence mode="wait">
              <motion.div
                key={showWalletSetup ? 'wallet' : (showIdentityVerification ? 'identity' : (showVerification ? 'verification' : (showDocumentVerification ? 'document' : (showIncomeDetails ? 'income' : (showEmploymentDetails ? 'employment' : (showCompanyInformation ? 'company-info' : (showCompanyRepresentative ? 'company' : (showPersonalInfo ? 'personal' : (showRegistrationForm ? 'form' : `step-${activeStep}`)))))))))}
                initial={{ opacity: 0, x: 20 }}
                animate={{ opacity: 1, x: 0 }}
                exit={{ opacity: 0, x: -20 }}
                transition={{ duration: 0.3 }}
              >
                <Card className="p-6 bg-gray-900/50 border-2 border-eco-green">
                  {renderContent()}
                </Card>
              </motion.div>
            </AnimatePresence>
          </div>

          <motion.div
            initial={{ opacity: 0, y: 20 }}
            animate={{ opacity: 1, y: 0 }}
            transition={{ duration: 0.6, delay: 0.4 }}
            className="text-center mt-8"
          >
            <p className="text-sm text-gray-400 mb-4">
              Need help with registration? Contact our support team
            </p>
            <Button
              className="bg-gray-800 text-white"
              onClick={() => navigate('/contact')}
            >
              Contact Support
            </Button>
          </motion.div>
        </div>
      </div>

      <Footer />
    </div>
  );
}