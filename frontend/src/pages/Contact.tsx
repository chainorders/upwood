import React, { useState } from 'react';
import { Navigation } from '../components/Navigation';
import { useNavigate } from 'react-router-dom';
import { motion, AnimatePresence } from 'framer-motion';
import { Input, Textarea, Button, Card } from "@nextui-org/react";
import { Send, ArrowRight, CheckCircle2 } from 'lucide-react';
import { Footer } from '../components/Footer';

export default function Contact() {
  const navigate = useNavigate();
  const [formData, setFormData] = useState({
    name: '',
    email: '',
    subject: '',
    message: ''
  });
  const [isSubmitting, setIsSubmitting] = useState(false);
  const [showSuccess, setShowSuccess] = useState(false);

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();
    setIsSubmitting(true);
    
    // Simulate API call
    await new Promise(resolve => setTimeout(resolve, 1000));
    
    // Show success message
    setShowSuccess(true);
    
    // Reset form
    setFormData({
      name: '',
      email: '',
      subject: '',
      message: ''
    });
    setIsSubmitting(false);
    
    // Hide success message after 5 seconds
    setTimeout(() => {
      setShowSuccess(false);
    }, 4000);
  };

  const handleChange = (e: React.ChangeEvent<HTMLInputElement | HTMLTextAreaElement>) => {
    const { name, value } = e.target;
    setFormData(prev => ({
      ...prev,
      [name]: value
    }));
  };

  return (
    <div className="min-h-screen bg-black text-white">
      <Navigation onLaunchApp={() => navigate('/')} />
      
      {/* Hero Section */}
      <div className="relative overflow-hidden">
        <div className="absolute inset-0 bg-gradient-radial from-forest-green/20 via-black to-black" />
        <div className="max-w-7xl mx-auto px-4 py-20 relative">
          <motion.div
            initial={{ opacity: 0, y: 20 }}
            animate={{ opacity: 1, y: 0 }}
            transition={{ duration: 0.6 }}
            className="text-center max-w-3xl mx-auto"
          >
            <h1 className="text-5xl font-bold mb-6 bg-gradient-to-r from-green-400 to-emerald-600 bg-clip-text text-transparent">
              Get in Touch
            </h1>
            <p className="text-xl text-gray-400">
              Have questions about forest investments? We're here to help!
            </p>
          </motion.div>
        </div>
      </div>

      {/* Contact Form Section */}
      <div className="max-w-7xl mx-auto px-4 pb-12">
        <div className="max-w-2xl mx-auto">
          <motion.div
            initial={{ opacity: 0, y: 20 }}
            animate={{ opacity: 1, y: 0 }}
            transition={{ duration: 0.6, delay: 0.2 }}
          >
            <Card className="p-6 bg-gray-900/50 border-gray-800">
              <form onSubmit={handleSubmit} className="space-y-6">
                <div className="grid grid-cols-1 md:grid-cols-2 gap-4">
                  <Input
                    type="text"
                    label="Name"
                    name="name"
                    value={formData.name}
                    onChange={handleChange}
                    variant="bordered"
                    isRequired
                    classNames={{
                      input: "text-white",
                      inputWrapper: "border-gray-600",
                    }}
                  />
                  <Input
                    type="email"
                    label="Email"
                    name="email"
                    value={formData.email}
                    onChange={handleChange}
                    variant="bordered"
                    isRequired
                    classNames={{
                      input: "text-white",
                      inputWrapper: "border-gray-600",
                    }}
                  />
                </div>
                
                <Input
                  type="text"
                  label="Subject"
                  name="subject"
                  value={formData.subject}
                  onChange={handleChange}
                  variant="bordered"
                  isRequired
                  classNames={{
                    input: "text-white",
                    inputWrapper: "border-gray-600",
                  }}
                />
                
                <Textarea
                  label="Message"
                  name="message"
                  value={formData.message}
                  onChange={handleChange}
                  variant="bordered"
                  minRows={4}
                  isRequired
                  classNames={{
                    input: "text-white",
                    inputWrapper: "border-gray-600",
                  }}
                />
                
                <Button
                  type="submit"
                  className="w-full bg-eco-green text-white font-semibold hover:bg-eco-green/90"
                  size="lg"
                  isLoading={isSubmitting}
                  startContent={!isSubmitting && <Send className="w-4 h-4" />}
                >
                  Send Message
                </Button>
              </form>
            </Card>
          </motion.div>
        </div>

        {/* Success Message */}
        <AnimatePresence>
          {showSuccess && (
            <motion.div
              initial={{ opacity: 0, y: 50 }}
              animate={{ opacity: 1, y: 0 }}
              exit={{ opacity: 0, y: -50 }}
              className="fixed bottom-8 right-8 bg-eco-green/90 backdrop-blur-md text-white p-4 rounded-lg shadow-lg flex items-center gap-4 z-[9999]"
            >
              <CheckCircle2 className="w-6 h-6" />
              <p>Thank you for contacting us! We will get back to you soon.</p>
            </motion.div>
          )}
        </AnimatePresence>

        {/* CTA Section */}
        <div className="container mx-auto px-4 pt-12 pb-8">
          <Card className="p-12 bg-[#1D4B3A] border-1 border-gray-800">
            <div className="max-w-3xl mx-auto text-center">
              <motion.h2 
                initial={{ opacity: 0, y: 20 }}
                whileInView={{ opacity: 1, y: 0 }}
                viewport={{ once: true }}
                transition={{ duration: 0.6 }}
                className="text-4xl font-bold mb-6 text-white"
              >
                Time for Real Climate Action
              </motion.h2>
              <motion.p 
                initial={{ opacity: 0, y: 20 }}
                whileInView={{ opacity: 1, y: 0 }}
                viewport={{ once: true }}
                transition={{ duration: 0.6, delay: 0.2 }}
                className="text-white/90 mb-8 text-lg leading-relaxed"
              >
                Join community of like minded investors to fund and earn from stable, profitable long term investments in forests that are backed up by dividends from sustainable forestry operations, forestland value increase and carbon credits.
              </motion.p>
              <motion.div 
                initial={{ opacity: 0, y: 20 }}
                whileInView={{ opacity: 1, y: 0 }}
                viewport={{ once: true }}
                transition={{ duration: 0.6, delay: 0.4 }}
                className="flex justify-center"
              >
                <Button 
                  size="lg"
                  className="font-semibold bg-[#3a5a40] text-white hover:bg-[#3a5a40]/90"
                  onClick={() => navigate('/')}
                >
                  Request Invitation <ArrowRight className="ml-2" />
                </Button>
              </motion.div>
            </div>
          </Card>
        </div>
      </div>

      <Footer />
    </div>
  );
}