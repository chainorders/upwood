import React, { useState } from 'react';
import { Card, Button, Chip } from "@nextui-org/react";
import { motion } from 'framer-motion';
import { FileText, Download, Eye, Calendar, FileCheck, FileWarning } from 'lucide-react';
import { DocumentView } from '../../components/platform/DocumentView';

const contracts = [
  {
    id: 1,
    title: "Baltic Pine Forest Investment Agreement",
    projectName: "Baltic Pine Forest",
    signedDate: "2024-01-15",
    type: "Investment Contract",
    format: "PDF",
    status: "Active",
    size: "2.4 MB",
    category: "contract",
    url: "https://example.com/documents/contract.pdf"
  },
  {
    id: 2,
    title: "Nordic Spruce Estate Token Purchase",
    projectName: "Nordic Spruce Estate",
    signedDate: "2024-02-01",
    type: "Token Agreement",
    format: "EDOC",
    status: "Active",
    size: "1.8 MB",
    category: "contract",
    url: "https://example.com/documents/token-agreement.pdf"
  },
  {
    id: 3,
    title: "Carbon Credits Trading Agreement",
    projectName: "Baltic Pine Forest",
    signedDate: "2024-01-15",
    type: "Trading Agreement",
    format: "PDF",
    status: "Active",
    size: "1.2 MB",
    category: "contract",
    url: "https://example.com/documents/trading-agreement.pdf"
  }
];

const offeringDocuments = [
  {
    id: 4,
    title: "Baltic Pine Forest Investment Memorandum",
    projectName: "Baltic Pine Forest",
    date: "2024-01-10",
    type: "Investment Memorandum",
    format: "PDF",
    status: "Current",
    size: "3.5 MB",
    category: "offering",
    url: "https://example.com/documents/memorandum.pdf"
  },
  {
    id: 5,
    title: "Nordic Spruce Estate Prospectus",
    projectName: "Nordic Spruce Estate",
    date: "2024-01-25",
    type: "Prospectus",
    format: "PDF",
    status: "Current",
    size: "4.2 MB",
    category: "offering",
    url: "https://example.com/documents/prospectus.pdf"
  },
  {
    id: 6,
    title: "Carbon Credits Trading Terms",
    projectName: "Baltic Pine Forest",
    date: "2024-01-12",
    type: "Terms & Conditions",
    format: "PDF",
    status: "Current",
    size: "1.8 MB",
    category: "offering",
    url: "https://example.com/documents/terms.pdf"
  }
];

export default function Contracts() {
  const [selectedCategory, setSelectedCategory] = useState<'contract' | 'offering'>('contract');
  const [selectedDocument, setSelectedDocument] = useState<(typeof contracts[0] & { date: string }) | null>(null);

  const handleDownload = (documentId: number) => {
    // In a real application, this would trigger a download
    console.log(`Downloading document ${documentId}`);
  };

  const handleView = (document: typeof contracts[0] | typeof offeringDocuments[0]) => {
    setSelectedDocument({
      ...document,
      date: 'signedDate' in document ? document.signedDate : document.date
    });
  };

  const displayDocuments = selectedCategory === 'contract' ? contracts : offeringDocuments;

  return (
    <div className="p-4 lg:p-6">
      <div className="mb-8">
        <h1 className="text-2xl lg:text-3xl font-bold text-white mb-4">Documents</h1>
        <p className="text-sm lg:text-base text-gray-400 mb-6">
          View and download your contracts and offering documents
        </p>

        <div className="flex flex-col sm:flex-row gap-3">
          <Button
            className={`${
              selectedCategory === 'contract'
                ? 'bg-eco-green text-white'
                : 'bg-gray-900/50 text-gray-400 hover:text-white'
            } transition-colors`}
            startContent={<FileCheck className="w-4 h-4" />}
            onClick={() => setSelectedCategory('contract')}
          >
            My Contracts
          </Button>
          <Button
            className={`${
              selectedCategory === 'offering'
                ? 'bg-eco-green text-white'
                : 'bg-gray-900/50 text-gray-400 hover:text-white'
            } transition-colors`}
            startContent={<FileWarning className="w-4 h-4" />}
            onClick={() => setSelectedCategory('offering')}
          >
            Offering Documents
          </Button>
        </div>
      </div>

      <div className="grid grid-cols-1 gap-6">
        {displayDocuments.map((document, index) => (
          <motion.div
            key={document.id}
            initial={{ opacity: 0, y: 20 }}
            animate={{ opacity: 1, y: 0 }}
            transition={{ duration: 0.6, delay: index * 0.1 }}
          >
            <Card className="bg-gray-900/50 border-gray-800">
              <div className="p-4 lg:p-6">
                <div className="flex items-start justify-between mb-6">
                  <div className="flex items-start gap-3 lg:gap-4 flex-1 min-w-0">
                    <div className="p-2 lg:p-3 rounded-lg bg-eco-green/10 flex-shrink-0">
                      <FileText className="w-5 h-5 lg:w-6 lg:h-6 text-eco-green" />
                    </div>
                    <div className="min-w-0">
                      <h3 className="text-lg lg:text-xl font-bold text-white mb-2 break-words">
                        {document.title}
                      </h3>
                      <div className="flex flex-col sm:flex-row sm:items-center gap-2 sm:gap-4 text-sm text-gray-400">
                        <div className="flex items-center gap-1">
                          <Calendar className="w-4 h-4" />
                          {new Date('signedDate' in document ? document.signedDate : document.date).toLocaleDateString()}
                        </div>
                        <div className="break-words">Project: {document.projectName}</div>
                      </div>
                    </div>
                  </div>
                  <div className="flex flex-col sm:flex-row items-end sm:items-center gap-2 flex-shrink-0">
                    <Chip
                      className="bg-eco-green/10 text-eco-green"
                      size="sm"
                    >
                      {document.format}
                    </Chip>
                    <Chip
                      color="success"
                      variant="flat"
                      size="sm"
                    >
                      {document.status}
                    </Chip>
                  </div>
                </div>

                <div className="flex flex-col sm:flex-row sm:items-center justify-between gap-4">
                  <div className="flex flex-col sm:flex-row sm:items-center gap-2 sm:gap-4">
                    <div className="text-sm text-gray-400">
                      Type: {document.type}
                    </div>
                    <div className="text-sm text-gray-400">
                      Size: {document.size}
                    </div>
                  </div>
                  <div className="flex flex-col sm:flex-row gap-3">
                    <Button
                      className="bg-gray-800 text-white"
                      variant="flat"
                      startContent={<Eye className="w-4 h-4" />}
                      onClick={() => handleView(document)}
                    >
                      View
                    </Button>
                    <Button
                      className="bg-eco-green text-white"
                      startContent={<Download className="w-4 h-4" />}
                      onClick={() => handleDownload(document.id)}
                    >
                      Download
                    </Button>
                  </div>
                </div>
              </div>
            </Card>
          </motion.div>
        ))}
      </div>

      <DocumentView
        isOpen={selectedDocument !== null}
        onClose={() => setSelectedDocument(null)}
        document={selectedDocument}
      />
    </div>
  );
}