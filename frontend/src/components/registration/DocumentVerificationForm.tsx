import React, { useState } from 'react';
import { Button, Card } from '@nextui-org/react';
import { FileText, ArrowLeft, ArrowRight, Upload, CheckCircle2, AlertCircle, Info, X } from 'lucide-react';
import { motion, AnimatePresence } from 'framer-motion';

interface DocumentVerificationFormProps {
  onSubmit: (e: React.FormEvent) => void;
  onBack: () => void;
  accountType: 'individual' | 'legal';
}

type DocumentType = 'passport' | 'national-id' | 'residence-permit' | 'extract' | 'articles' | 'structure' | 'funds' | 'wealth' | 'ubo-id';

interface DocumentUpload {
  id: string;
  file: File;
  status: 'uploading' | 'success' | 'error';
  error?: string;
}

interface DocumentTypeInfo {
  type: DocumentType;
  title: string;
  description: string;
  tooltip: string;
  maxFiles?: number;
}

export function DocumentVerificationForm({ onSubmit, onBack, accountType }: DocumentVerificationFormProps) {
  const [selectedDocument, setSelectedDocument] = useState<DocumentType | null>(null);
  const [documentUploads, setDocumentUploads] = useState<Record<DocumentType, DocumentUpload[]>>({});

  const handleFileSelect = (e: React.ChangeEvent<HTMLInputElement>) => {
    if (!selectedDocument) return;

    const files = Array.from(e.target.files || []);
    const currentUploads = documentUploads[selectedDocument] || [];
    const documentType = documentTypes.find(d => d.type === selectedDocument);
    const maxFiles = documentType?.maxFiles || 1;

    // Check if we've reached the maximum number of files
    if (currentUploads.length + files.length > maxFiles) {
      alert(`You can only upload up to ${maxFiles} file${maxFiles > 1 ? 's' : ''} for this document type`);
      return;
    }

    const newUploads = files.map(file => {
      // Check file type
      const allowedTypes = ['image/jpeg', 'image/png', 'application/pdf'];
      if (!allowedTypes.includes(file.type)) {
        return {
          id: crypto.randomUUID(),
          file,
          status: 'error' as const,
          error: 'Please upload a JPG, PNG, or PDF file'
        };
      }

      // Check file size (max 5MB)
      if (file.size > 5 * 1024 * 1024) {
        return {
          id: crypto.randomUUID(),
          file,
          status: 'error' as const,
          error: 'File size must be less than 5MB'
        };
      }

      return {
        id: crypto.randomUUID(),
        file,
        status: 'success' as const
      };
    });

    setDocumentUploads(prev => ({
      ...prev,
      [selectedDocument]: [...currentUploads, ...newUploads]
    }));
  };

  const handleRemoveFile = (documentType: DocumentType, uploadId: string) => {
    setDocumentUploads(prev => ({
      ...prev,
      [documentType]: prev[documentType]?.filter(upload => upload.id !== uploadId) || []
    }));
  };

  const individualDocumentTypes: DocumentTypeInfo[] = [
    {
      type: 'passport',
      title: 'Passport',
      description: 'Upload your valid passport',
      tooltip: 'Must be a valid, non-expired passport. All pages with information must be clearly visible.',
      maxFiles: 3
    },
    {
      type: 'national-id',
      title: 'National ID Card',
      description: 'Upload your national ID card (front and back)',
      tooltip: 'Must be a valid government-issued ID card. Both front and back must be clearly visible.',
      maxFiles: 2
    },
    {
      type: 'residence-permit',
      title: 'Residence Permit',
      description: 'Upload your residence permit if applicable',
      tooltip: 'Must be a valid residence permit showing your right to reside in the country.',
      maxFiles: 2
    }
  ];

  const legalEntityDocumentTypes: DocumentTypeInfo[] = [
    {
      type: 'extract',
      title: 'Extract of Commercial Register',
      description: 'Upload a recent extract from the commercial register',
      tooltip: 'Official document proving company registration and basic information. Must be issued within last 3 months.',
      maxFiles: 2
    },
    {
      type: 'articles',
      title: 'Articles of Association',
      description: 'Upload your company\'s articles of association',
      tooltip: 'Legal document defining company rules, purpose, and structure. Must be the latest approved version.',
      maxFiles: 3
    },
    {
      type: 'structure',
      title: 'Company Structure Chart',
      description: 'Upload your company\'s organizational structure',
      tooltip: 'Visual representation showing ownership structure, subsidiaries, and relationships between entities.',
      maxFiles: 2
    },
    {
      type: 'funds',
      title: 'Source of Funds',
      description: 'Document proving the origin of investment funds',
      tooltip: 'Bank statements, investment records, or other documents showing how the investment funds were obtained.',
      maxFiles: 5
    },
    {
      type: 'wealth',
      title: 'Source of Wealth',
      description: 'Document proving overall wealth origin',
      tooltip: 'Documentation showing how your overall wealth was generated (business ownership, inheritance, investments, etc.).',
      maxFiles: 5
    },
    {
      type: 'ubo-id',
      title: 'UBO Identification',
      description: 'Upload ID documents for all listed UBOs',
      tooltip: 'Valid government-issued ID (passport or national ID card) for each ultimate beneficial owner listed.',
      maxFiles: 10
    }
  ];

  const documentTypes = accountType === 'individual' ? individualDocumentTypes : legalEntityDocumentTypes;

  const isFormValid = documentTypes.some(docType => {
    const uploads = documentUploads[docType.type] || [];
    return uploads.length > 0 && uploads.every(upload => upload.status === 'success');
  });

  return (
    <form onSubmit={onSubmit} className="space-y-6">
      <div className="flex flex-col items-center text-center mb-6">
        <div className="w-16 h-16 rounded-full bg-eco-green flex items-center justify-center mb-4">
          <FileText className="w-8 h-8" />
        </div>
        <h3 className="text-2xl font-bold mb-2 bg-gradient-to-r from-green-400 to-emerald-600 bg-clip-text text-transparent">
          Document Verification
        </h3>
        <p className="text-sm text-gray-400">
          {accountType === 'individual' 
            ? 'Please upload a valid identification document'
            : 'Please upload the following required documents'
          }
        </p>
      </div>

      <div className="space-y-4">
        {/* Document Type Selection */}
        <div className="grid grid-cols-1 md:grid-cols-2 gap-4">
          {documentTypes.map((doc) => (
            <Card
              key={doc.type}
              isPressable
              isHoverable
              className={`border ${
                selectedDocument === doc.type 
                  ? 'border-eco-green bg-eco-green/10' 
                  : 'border-gray-800 bg-black/30'
              }`}
              onPress={() => setSelectedDocument(doc.type)}
            >
              <div className="p-4">
                <div className="flex items-center justify-between mb-1">
                  <h4 className="font-medium text-white">{doc.title}</h4>
                  <div className="group relative">
                    <Info className="w-4 h-4 text-gray-400 cursor-help" />
                    <div className="absolute right-0 top-6 w-64 p-2 bg-gray-900 border border-gray-800 rounded-lg text-xs text-gray-400 opacity-0 invisible group-hover:opacity-100 group-hover:visible transition-all duration-200 z-50">
                      {doc.tooltip}
                    </div>
                  </div>
                </div>
                <p className="text-xs text-gray-400">{doc.description}</p>
                {documentUploads[doc.type]?.length > 0 && (
                  <div className="mt-3 space-y-2">
                    {documentUploads[doc.type].map((upload) => (
                      <div
                        key={upload.id}
                        className={`flex items-center justify-between p-2 rounded-lg ${
                          upload.status === 'error' ? 'bg-red-500/10' : 'bg-eco-green/10'
                        }`}
                      >
                        <div className="flex items-center gap-2">
                          <FileText className={`w-4 h-4 ${
                            upload.status === 'error' ? 'text-red-500' : 'text-eco-green'
                          }`} />
                          <span className="text-xs text-white truncate max-w-[150px]">
                            {upload.file.name}
                          </span>
                        </div>
                        <Button
                          isIconOnly
                          variant="light"
                          size="sm"
                          className="text-gray-400 hover:text-white"
                          onClick={() => handleRemoveFile(doc.type, upload.id)}
                        >
                          <X className="w-4 h-4" />
                        </Button>
                      </div>
                    ))}
                  </div>
                )}
              </div>
            </Card>
          ))}
        </div>

        {/* Upload Section */}
        {selectedDocument && (
          <div className="mt-6">
            <div className="border-2 border-dashed border-gray-800 rounded-lg p-8 text-center">
              <input
                type="file"
                id="document-upload"
                className="hidden"
                accept=".jpg,.jpeg,.png,.pdf"
                onChange={handleFileSelect}
                multiple
              />
              
              <div>
                <Upload className="w-12 h-12 text-gray-400 mx-auto mb-4" />
                <p className="text-gray-400 mb-2">
                  Drag and drop your document{documentTypes.find(d => d.type === selectedDocument)?.maxFiles! > 1 ? 's' : ''} here or
                </p>
                <Button
                  as="label"
                  htmlFor="document-upload"
                  className="bg-eco-green text-white"
                >
                  Browse Files
                </Button>
                <p className="text-xs text-gray-500 mt-4">
                  Supported formats: JPG, PNG, PDF (max 5MB per file)
                </p>
                {documentTypes.find(d => d.type === selectedDocument)?.maxFiles! > 1 && (
                  <p className="text-xs text-gray-500 mt-2">
                    You can upload up to {documentTypes.find(d => d.type === selectedDocument)?.maxFiles} files
                  </p>
                )}
              </div>
            </div>
          </div>
        )}
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