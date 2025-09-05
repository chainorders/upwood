import React, { useState } from 'react';
import { Modal, ModalContent, ModalHeader, ModalBody, ModalFooter, Button, Input, Avatar } from "@nextui-org/react";
import { Send, Paperclip } from 'lucide-react';

interface Message {
  id: number;
  text: string;
  sender: 'user' | 'support';
  timestamp: Date;
}

interface SupportChatProps {
  isOpen: boolean;
  onClose: () => void;
}

export function SupportChat({ isOpen, onClose }: SupportChatProps) {
  const [message, setMessage] = useState('');
  const [messages, setMessages] = useState<Message[]>([
    {
      id: 1,
      text: "Hello! How can I help you today?",
      sender: 'support',
      timestamp: new Date()
    }
  ]);

  const handleSend = () => {
    if (!message.trim()) return;

    // Add user message
    const userMessage: Message = {
      id: messages.length + 1,
      text: message,
      sender: 'user',
      timestamp: new Date()
    };

    setMessages(prev => [...prev, userMessage]);
    setMessage('');

    // Simulate support response
    setTimeout(() => {
      const supportMessage: Message = {
        id: messages.length + 2,
        text: "Thank you for your message. Our support team will get back to you shortly.",
        sender: 'support',
        timestamp: new Date()
      };
      setMessages(prev => [...prev, supportMessage]);
    }, 1000);
  };

  const handleFileUpload = () => {
    // Create a hidden file input
    const fileInput = document.createElement('input');
    fileInput.type = 'file';
    fileInput.accept = '.pdf,.doc,.docx,.txt,.jpg,.jpeg,.png';
    fileInput.style.display = 'none';

    // Handle file selection
    fileInput.onchange = (e) => {
      const file = (e.target as HTMLInputElement).files?.[0];
      if (file) {
        // In a real app, this would handle file upload
        console.log('Selected file:', file.name);
        
        // Add a message about the file
        const fileMessage: Message = {
          id: messages.length + 1,
          text: `Attached file: ${file.name}`,
          sender: 'user',
          timestamp: new Date()
        };
        setMessages(prev => [...prev, fileMessage]);
      }
    };

    // Trigger file selection
    document.body.appendChild(fileInput);
    fileInput.click();
    document.body.removeChild(fileInput);
  };

  return (
    <Modal 
      isOpen={isOpen} 
      onClose={onClose}
      className="dark"
      size="2xl"
      scrollBehavior="inside"
    >
      <ModalContent>
        <ModalHeader className="flex flex-col gap-1">
          <h3 className="text-xl font-bold text-eco-green">Support Chat</h3>
          <p className="text-sm text-gray-400 font-normal">
            We typically reply within a few minutes
          </p>
        </ModalHeader>
        <ModalBody>
          <div className="space-y-4">
            {messages.map((msg) => (
              <div
                key={msg.id}
                className={`flex gap-3 ${msg.sender === 'user' ? 'flex-row-reverse' : ''}`}
              >
                <Avatar
                  className={msg.sender === 'support' ? 'bg-eco-green text-white' : 'bg-gray-600'}
                  name={msg.sender === 'support' ? 'Support' : 'You'}
                />
                <div
                  className={`
                    max-w-[80%] rounded-lg p-3
                    ${msg.sender === 'user' 
                      ? 'bg-eco-green text-white' 
                      : 'bg-gray-800 text-white'
                    }
                  `}
                >
                  <p>{msg.text}</p>
                  <div className={`
                    text-xs mt-1
                    ${msg.sender === 'user' ? 'text-white/70' : 'text-gray-400'}
                  `}>
                    {msg.timestamp.toLocaleTimeString()}
                  </div>
                </div>
              </div>
            ))}
          </div>
        </ModalBody>
        <ModalFooter>
          <div className="flex gap-2 w-full">
            <Button
              isIconOnly
              className="bg-gray-800 text-white"
              onClick={handleFileUpload}
            >
              <Paperclip className="w-4 h-4" />
            </Button>
            <Input
              value={message}
              onChange={(e) => setMessage(e.target.value)}
              placeholder="Type your message..."
              onKeyPress={(e) => e.key === 'Enter' && handleSend()}
              classNames={{
                input: "text-white",
                inputWrapper: "bg-black/30 border-gray-800"
              }}
            />
            <Button
              className="bg-eco-green text-white"
              isIconOnly
              onClick={handleSend}
            >
              <Send className="w-4 h-4" />
            </Button>
          </div>
        </ModalFooter>
      </ModalContent>
    </Modal>
  );
}