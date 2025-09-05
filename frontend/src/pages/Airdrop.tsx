import React, { useState } from 'react';
import { motion, AnimatePresence } from 'framer-motion';
import { Card, Button, Chip } from "@nextui-org/react";
import { Navigation } from '../components/Navigation';
import { Footer } from '../components/Footer';
import { useNavigate } from 'react-router-dom';
import { TreePine, Gift, Leaf, Trophy, Plane, Car, Hotel, Wallet, ArrowRight, ShoppingBag, Headphones } from 'lucide-react';

interface Benefit {
  icon: React.ElementType;
  title: string;
  description: string;
  partner: string;
  discount: string;
}

export default function Airdrop() {
  const navigate = useNavigate();
  const [showBenefits, setShowBenefits] = useState(false);

  const benefits: Benefit[] = [
    {
      icon: Plane,
      title: "Flight Discounts",
      description: "Get exclusive discounts on flight bookings worldwide",
      partner: "Booking.com",
      discount: "Up to 15% off"
    },
    {
      icon: Car,
      title: "Car Rental Deals",
      description: "Special rates on car rentals globally",
      partner: "Sixt",
      discount: "20% off"
    },
    {
      icon: Hotel,
      title: "Hotel Stays",
      description: "Premium discounts on hotel bookings",
      partner: "Booking.com",
      discount: "Up to 25% off"
    },
    {
      icon: ShoppingBag,
      title: "Sustainable Fashion",
      description: "Discounts on eco-friendly fashion brands",
      partner: "Various Brands",
      discount: "Up to 20% off"
    },
    {
      icon: Headphones,
      title: "Digital Services",
      description: "Premium subscriptions to digital services",
      partner: "Multiple Partners",
      discount: "Up to 6 months free"
    }
  ];

  const renderWelcomeFrame = () => (
    <motion.div
      initial={{ opacity: 0, y: 20 }}
      animate={{ opacity: 1, y: 0 }}
      exit={{ opacity: 0, y: -20 }}
      transition={{ duration: 0.6 }}
    >
      <Card className="p-6 bg-gray-900/50 border-2 border-eco-green">
        <div className="flex flex-col items-center text-center">
          <motion.div
            initial={{ scale: 0 }}
            animate={{ scale: 1 }}
            transition={{ duration: 0.6, delay: 0.2 }}
            className="relative mb-8"
          >
            <div className="w-24 h-24 rounded-full bg-eco-green/20 flex items-center justify-center">
              <TreePine className="w-12 h-12 text-eco-green" />
            </div>
            <div className="absolute -top-2 -right-2 w-10 h-10 rounded-full bg-eco-green flex items-center justify-center animate-bounce">
              <Gift className="w-5 h-5 text-white" />
            </div>
          </motion.div>

          <motion.h1
            initial={{ opacity: 0, y: 20 }}
            animate={{ opacity: 1, y: 0 }}
            transition={{ duration: 0.6, delay: 0.3 }}
            className="text-4xl font-bold mb-4 bg-gradient-to-r from-green-400 to-emerald-600 bg-clip-text text-transparent"
          >
            E-tree Airdrop
          </motion.h1>

          <motion.p
            initial={{ opacity: 0, y: 20 }}
            animate={{ opacity: 1, y: 0 }}
            transition={{ duration: 0.6, delay: 0.4 }}
            className="text-xl text-gray-300 mb-8"
          >
            Welcome to Upwood's exclusive NFT airdrop campaign
          </motion.p>

          <motion.div
            initial={{ opacity: 0, y: 20 }}
            animate={{ opacity: 1, y: 0 }}
            transition={{ duration: 0.6, delay: 0.5 }}
            className="grid grid-cols-1 md:grid-cols-2 gap-4 w-full mb-8"
          >
            <div className="bg-black/30 p-6 rounded-lg border border-gray-800 flex flex-col items-center">
              <div className="w-16 h-16 rounded-full bg-eco-green/20 flex items-center justify-center mb-4">
                <Leaf className="w-8 h-8 text-eco-green" />
              </div>
              <h3 className="text-lg font-semibold mb-2 text-white">E-tree NFTs</h3>
              <p className="text-sm text-gray-400">
                Unique digital collectibles linked to real forest areas
              </p>
            </div>

            <div className="bg-black/30 p-6 rounded-lg border border-gray-800 flex flex-col items-center">
              <div className="w-16 h-16 rounded-full bg-eco-green/20 flex items-center justify-center mb-4">
                <Trophy className="w-8 h-8 text-eco-green" />
              </div>
              <h3 className="text-lg font-semibold mb-2 text-white">Exclusive Benefits</h3>
              <p className="text-sm text-gray-400">
                Access special perks and rewards with your NFTs
              </p>
            </div>
          </motion.div>

          <motion.div
            initial={{ opacity: 0, y: 20 }}
            animate={{ opacity: 1, y: 0 }}
            transition={{ duration: 0.6, delay: 0.6 }}
            className="w-full"
          >
            <Button
              className="w-full bg-eco-green text-white font-semibold h-12 text-lg"
              onClick={() => setShowBenefits(true)}
            >
              Join Airdrop
            </Button>
            <p className="text-sm text-gray-500 mt-4">
              Limited time offer • First batch 400 NFTs
            </p>
          </motion.div>
        </div>
      </Card>
    </motion.div>
  );

  const renderBenefitsFrame = () => (
    <motion.div
      initial={{ opacity: 0, y: 20 }}
      animate={{ opacity: 1, y: 0 }}
      exit={{ opacity: 0, y: -20 }}
      transition={{ duration: 0.6 }}
    >
      <Card className="p-6 bg-gray-900/50 border-2 border-eco-green">
        <div className="flex flex-col items-center text-center">
          <motion.div
            initial={{ scale: 0 }}
            animate={{ scale: 1 }}
            transition={{ duration: 0.6, delay: 0.2 }}
            className="mb-6"
          >
            <div className="w-16 h-16 rounded-full bg-eco-green/20 flex items-center justify-center">
              <Gift className="w-8 h-8 text-eco-green" />
            </div>
          </motion.div>

          <motion.h2
            initial={{ opacity: 0, y: 20 }}
            animate={{ opacity: 1, y: 0 }}
            transition={{ duration: 0.6, delay: 0.3 }}
            className="text-2xl font-bold mb-2 text-white"
          >
            NFT Benefits
          </motion.h2>

          <motion.p
            initial={{ opacity: 0, y: 20 }}
            animate={{ opacity: 1, y: 0 }}
            transition={{ duration: 0.6, delay: 0.4 }}
            className="text-gray-400 mb-6"
          >
            Unlock 50+ exclusive partner rewards and pre-sale access
          </motion.p>

          <motion.div
            initial={{ opacity: 0, y: 20 }}
            animate={{ opacity: 1, y: 0 }}
            transition={{ duration: 0.6, delay: 0.5 }}
            className="w-full mb-6"
          >
            <div className="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 gap-3">
              {benefits.map((benefit, index) => (
                <motion.div
                  key={benefit.title}
                  initial={{ opacity: 0, y: 20 }}
                  animate={{ opacity: 1, y: 0 }}
                  transition={{ duration: 0.6, delay: 0.2 + (index * 0.1) }}
                >
                  <Card className="bg-black/30 border border-gray-800 hover:border-eco-green/50 transition-all duration-300">
                    <div className="p-3">
                      <div className="flex items-start gap-3">
                        <div className="w-10 h-10 rounded-full bg-eco-green/10 flex items-center justify-center flex-shrink-0">
                          <benefit.icon className="w-5 h-5 text-eco-green" />
                        </div>
                        <div className="text-left flex-1">
                          <h3 className="font-semibold text-white text-sm mb-1">{benefit.title}</h3>
                          <div className="flex items-center justify-between">
                            <span className="text-xs text-gray-500">{benefit.partner}</span>
                            <Chip size="sm" className="bg-eco-green/10 text-eco-green text-xs">
                              {benefit.discount}
                            </Chip>
                          </div>
                        </div>
                      </div>
                    </div>
                  </Card>
                </motion.div>
              ))}
            </div>
          </motion.div>

          <motion.div
            initial={{ opacity: 0, y: 20 }}
            animate={{ opacity: 1, y: 0 }}
            transition={{ duration: 0.6, delay: 0.8 }}
            className="w-full space-y-4"
          >
            <Card className="bg-[#1D4B3A] border-none p-4">
              <div className="flex items-center gap-3">
                <div className="w-10 h-10 rounded-full bg-eco-green/20 flex items-center justify-center">
                  <Wallet className="w-5 h-5 text-eco-green" />
                </div>
                <div className="text-left">
                  <h3 className="font-semibold text-white text-sm">Pre-sale Access</h3>
                  <p className="text-xs text-gray-300">
                    Get exclusive access to Upwood bond token pre-sale
                  </p>
                </div>
              </div>
            </Card>

            <Button
              className="w-full bg-eco-green text-white font-semibold h-11"
              onClick={() => navigate('/registration')}
              endContent={<ArrowRight className="w-4 h-4" />}
            >
              Continue to Registration
            </Button>
            
            <p className="text-xs text-gray-500">
              Complete registration to claim your E-tree NFT and unlock all benefits
            </p>
          </motion.div>
        </div>
      </Card>
    </motion.div>
  );

  return (
    <div className="min-h-screen bg-black text-white">
      <Navigation onLaunchApp={() => navigate('/')} />
      
      <div className="max-w-7xl mx-auto px-4 py-12">
        <div className="max-w-4xl mx-auto">
          <AnimatePresence mode="wait">
            {!showBenefits ? renderWelcomeFrame() : renderBenefitsFrame()}
          </AnimatePresence>
        </div>
      </div>

      <Footer />
    </div>
  );
}