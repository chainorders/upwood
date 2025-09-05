import React from 'react';
import { Outlet, NavLink, useNavigate } from 'react-router-dom';
import { Button } from '@nextui-org/react';
import { TreePine, LayoutGrid, Wallet, Settings, LogOut, FileText, User, MessageSquare, Bell, Leaf } from 'lucide-react';
import { SupportChat } from './SupportChat';
import { NotificationBell } from './NotificationBell';

export function PlatformLayout() {
  const navigate = useNavigate();
  const [isSupportOpen, setIsSupportOpen] = React.useState(false);
  const [isMobileMenuOpen, setIsMobileMenuOpen] = React.useState(false);
  
  const navigation = [
    { name: 'Active Projects', path: '/platform/projects', icon: LayoutGrid },
    { name: 'My Assets', path: '/platform/assets', icon: Wallet },
    { name: 'Contracts', path: '/platform/contracts', icon: FileText },
    { name: 'Wallet', path: '/platform/wallet', icon: Wallet },
    { name: 'Profile', path: '/platform/profile', icon: User },
  ];

  return (
    <div className="min-h-screen bg-black">
      {/* Sidebar */}
      <div className={`fixed inset-y-0 left-0 w-64 bg-gray-900 border-r border-gray-800 transform transition-transform duration-300 ease-in-out z-50 ${
        isMobileMenuOpen ? 'translate-x-0' : '-translate-x-full'
      } lg:translate-x-0`}>
        <div className="flex flex-col h-full">
          {/* Logo */}
          <div className="p-6">
            <div className="flex items-center gap-2">
              <img 
                src="https://www.upwood.io/images/upwood.png" 
                alt="Upwood Logo" 
                className="h-8"
              />
            </div>
          </div>

          {/* Navigation */}
          <nav className="flex-1 px-4 space-y-2">
            {navigation.map((item) => (
              <NavLink
                key={item.name}
                to={item.path}
                className={({ isActive }) => `
                  flex items-center gap-3 px-4 py-3 rounded-lg
                  ${isActive 
                    ? 'bg-eco-green text-white' 
                    : 'text-gray-400 hover:bg-gray-800 hover:text-white'
                  }
                  transition-colors
                `}
              >
                <item.icon className="w-5 h-5" />
                {item.name}
              </NavLink>
            ))}
          </nav>

          {/* Bottom Actions */}
          <div className="p-4 border-t border-gray-800 space-y-2">
            <NotificationBell />
            <Button
              className="w-full justify-start text-gray-400 hover:text-white"
              variant="light"
              startContent={<MessageSquare className="w-5 h-5" />}
              onClick={() => setIsSupportOpen(true)}
            >
              Support
            </Button>
            <Button
              className="w-full justify-start text-gray-400 hover:text-white"
              variant="light"
              startContent={<Settings className="w-5 h-5" />}
              onClick={() => navigate('/platform/settings')}
            >
              Settings
            </Button>
            <Button
              className="w-full justify-start text-gray-400 hover:text-white"
              variant="light"
              startContent={<LogOut className="w-5 h-5" />}
              onClick={() => navigate('/')}
            >
              Disconnect
            </Button>
          </div>
        </div>
      </div>

      {/* Mobile Menu Overlay */}
      {isMobileMenuOpen && (
        <div 
          className="fixed inset-0 bg-black bg-opacity-50 z-40 lg:hidden"
          onClick={() => setIsMobileMenuOpen(false)}
        />
      )}

      {/* Mobile Header */}
      <div className="lg:hidden fixed top-0 left-0 right-0 bg-gray-900 border-b border-gray-800 z-30">
        <div className="flex items-center justify-between p-4">
          <button
            onClick={() => setIsMobileMenuOpen(true)}
            className="text-gray-400 hover:text-white"
          >
            <svg className="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M4 6h16M4 12h16M4 18h16" />
            </svg>
          </button>
          <img 
            src="https://www.upwood.io/images/upwood.png" 
            alt="Upwood Logo" 
            className="h-6"
          />
          <div className="w-6" /> {/* Spacer for centering */}
        </div>
      </div>

      {/* Main Content */}
      <div className="lg:ml-64 pt-16 lg:pt-0">
        <Outlet />
      </div>

      {/* Support Chat Modal */}
      <SupportChat isOpen={isSupportOpen} onClose={() => setIsSupportOpen(false)} />
    </div>
  );
}