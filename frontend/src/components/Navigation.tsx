import React, { useState } from 'react';
import { Button, Modal, ModalContent, ModalBody } from '@nextui-org/react';
import { Menu, X } from 'lucide-react';
import { Link, useLocation } from 'react-router-dom';
import { motion, AnimatePresence } from 'framer-motion';

interface NavigationProps {
  onLaunchApp?: () => void;
}

export function Navigation({ onLaunchApp }: NavigationProps) {
  const location = useLocation();
  const [isMobileMenuOpen, setIsMobileMenuOpen] = useState(false);
  
  const navItems = [
    { name: 'Home', path: '/' },
    { name: 'About Us', path: '/about' },
    { name: 'Learn', path: '/learn' },
    { name: 'Contact', path: '/contact' },
  ];

  return (
    <>
      <nav className="max-w-7xl mx-auto px-4 py-6">
        <div className="flex justify-between items-center">
          <div className="w-[200px]">
            <Link to="/" className="flex items-center">
              <img 
                src="https://www.upwood.io/images/upwood.png" 
                alt="Upwood Logo" 
                className="h-8 ml-auto"
              />
            </Link>
          </div>
          
          {/* Desktop Navigation */}
          <div className="hidden md:flex items-center gap-8">
            {navItems.map((item) => (
              <Link
                key={item.path}
                to={item.path}
                className={`text-sm font-medium transition-colors hover:text-eco-green ${
                  location.pathname === item.path ? 'text-eco-green' : 'text-gray-400'
                }`}
              >
                {item.name}
              </Link>
            ))}
          </div>

          <div className="flex items-center gap-4">
            <Button 
              className="hidden md:flex font-semibold bg-[#3a5a40] text-white hover:bg-[#3a5a40]/90"
              size="lg"
              onClick={onLaunchApp}
            >
              Launch App
            </Button>
            
            {/* Mobile Menu Button */}
            <Button
              isIconOnly
              variant="light"
              className="md:hidden"
              onClick={() => setIsMobileMenuOpen(true)}
            >
              <Menu className="w-6 h-6 text-white" />
            </Button>
          </div>
        </div>
      </nav>

      {/* Mobile Menu */}
      <Modal
        isOpen={isMobileMenuOpen}
        onClose={() => setIsMobileMenuOpen(false)}
        hideCloseButton
        className="dark"
        placement="top"
        classNames={{
          wrapper: "h-screen",
          base: "h-screen m-0 max-w-full rounded-none bg-black",
        }}
      >
        <ModalContent>
          {() => (
            <ModalBody className="p-0">
              <div className="min-h-screen bg-black">
                {/* Mobile Menu Header */}
                <div className="flex justify-between items-center p-4 border-b border-gray-800">
                  <div className="w-[180px]">
                    <img 
                      src="https://www.upwood.io/images/upwood.png" 
                      alt="Upwood Logo" 
                      className="h-8 ml-auto"
                    />
                  </div>
                  <Button
                    isIconOnly
                    variant="light"
                    onClick={() => setIsMobileMenuOpen(false)}
                  >
                    <X className="w-6 h-6 text-white" />
                  </Button>
                </div>

                {/* Mobile Menu Links */}
                <div className="p-4">
                  <div className="space-y-4">
                    {navItems.map((item) => (
                      <motion.div
                        key={item.path}
                        initial={{ opacity: 0, x: 20 }}
                        animate={{ opacity: 1, x: 0 }}
                        transition={{ duration: 0.2 }}
                      >
                        <Link
                          to={item.path}
                          className={`block py-3 text-lg font-medium transition-colors ${
                            location.pathname === item.path 
                              ? 'text-eco-green' 
                              : 'text-gray-400 hover:text-white'
                          }`}
                          onClick={() => setIsMobileMenuOpen(false)}
                        >
                          {item.name}
                        </Link>
                      </motion.div>
                    ))}
                  </div>

                  <motion.div
                    initial={{ opacity: 0, y: 20 }}
                    animate={{ opacity: 1, y: 0 }}
                    transition={{ duration: 0.2, delay: 0.2 }}
                    className="mt-8"
                  >
                    <Button 
                      className="w-full font-semibold bg-[#3a5a40] text-white hover:bg-[#3a5a40]/90"
                      size="lg"
                      onClick={() => {
                        onLaunchApp?.();
                        setIsMobileMenuOpen(false);
                      }}
                    >
                      Launch App
                    </Button>
                  </motion.div>
                </div>
              </div>
            </ModalBody>
          )}
        </ModalContent>
      </Modal>
    </>
  );
}