import React from 'react';
import { Modal, ModalContent, ModalHeader, ModalBody, ModalFooter, Button, Chip } from "@nextui-org/react";
import { FileText, Download, Calendar } from 'lucide-react';

interface DocumentViewProps {
  isOpen: boolean;
  onClose: () => void;
  document: {
    id: number;
    title: string;
    projectName: string;
    date: string;
    type: string;
    format: string;
    status: string;
    size: string;
    content?: string;
    url?: string;
  } | null;
}

export function DocumentView({ isOpen, onClose, document }: DocumentViewProps) {
  if (!document) {
    return null;
  }

  const handleDownload = () => {
    // In a real app, this would trigger the document download
    console.log('Downloading document:', document.id);
  };

  return (
    <Modal
      isOpen={isOpen}
      onClose={onClose}
      className="dark"
      size="4xl"
      scrollBehavior="inside"
    >
      <ModalContent>
        <ModalHeader className="flex flex-col gap-1">
          <div className="flex items-center justify-between">
            <div className="flex items-center gap-4">
              <div className="p-3 rounded-lg bg-eco-green/10">
                <FileText className="w-6 h-6 text-eco-green" />
              </div>
              <div>
                <h3 className="text-xl font-bold text-white">{document.title}</h3>
                <div className="flex items-center gap-4 text-sm text-gray-400">
                  <div className="flex items-center gap-2">
                    <Calendar className="w-4 h-4" />
                    {new Date(document.date).toLocaleDateString()}
                  </div>
                  <div>Project: {document.projectName}</div>
                </div>
              </div>
            </div>
            <div className="flex items-center gap-2">
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
        </ModalHeader>
        <ModalBody>
          <div className="min-h-[600px] bg-black/30 rounded-lg p-6">
            <iframe
              src="https://www.w3.org/WAI/ER/tests/xhtml/testfiles/resources/pdf/dummy.pdf"
              className="w-full h-full min-h-[600px] rounded-lg"
              title={document.title}
            />
          </div>
        </ModalBody>
        <ModalFooter>
          <div className="flex justify-between items-center w-full">
            <div className="text-sm text-gray-400">
              Size: {document.size}
            </div>
            <Button
              className="bg-eco-green text-white"
              startContent={<Download className="w-4 h-4" />}
              onClick={handleDownload}
            >
              Download
            </Button>
          </div>
        </ModalFooter>
      </ModalContent>
    </Modal>
  );
}