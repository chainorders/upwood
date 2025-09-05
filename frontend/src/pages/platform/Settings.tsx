import React, { useState } from 'react';
import { Card, Button, Switch, Tabs, Tab, Select, SelectItem, Input } from "@nextui-org/react";
import { motion } from 'framer-motion';
import { 
  Bell, Shield, Globe, Moon, Sun, Wallet, 
  Mail, Smartphone, Key, Lock, Eye, EyeOff,
  Languages, DollarSign, Bitcoin, AlertTriangle
} from 'lucide-react';

export default function Settings() {
  const [selectedTab, setSelectedTab] = useState("preferences");
  const [showApiKey, setShowApiKey] = useState(false);
  
  const [settings, setSettings] = useState({
    theme: "dark",
    language: "en",
    currency: "EUR",
    cryptoCurrency: "ETH",
    notifications: {
      email: true,
      push: true,
      investment: true,
      security: true,
      marketing: false
    },
    security: {
      twoFactor: true,
      biometric: false,
      loginNotifications: true,
      deviceHistory: true
    }
  });

  const languages = [
    { label: "English", value: "en" },
    { label: "Deutsch", value: "de" },
    { label: "Español", value: "es" },
    { label: "Français", value: "fr" }
  ];

  const currencies = [
    { label: "Euro (EUR)", value: "EUR" },
    { label: "US Dollar (USD)", value: "USD" },
    { label: "British Pound (GBP)", value: "GBP" }
  ];

  const cryptoCurrencies = [
    { label: "Ethereum (ETH)", value: "ETH" },
    { label: "Bitcoin (BTC)", value: "BTC" },
    { label: "USD Coin (USDC)", value: "USDC" }
  ];

  const apiKey = "sk_test_51NcyJ9K8z...";

  return (
    <div className="p-4 lg:p-6">
      <div className="mb-8">
        <h1 className="text-2xl lg:text-3xl font-bold text-white mb-4">Settings</h1>
        <p className="text-sm lg:text-base text-gray-400">
          Customize your platform experience and security preferences
        </p>
      </div>

      <Tabs 
        selectedKey={selectedTab}
        onSelectionChange={(key) => setSelectedTab(key.toString())}
        color="success"
        variant="bordered"
        classNames={{
          tabList: "bg-gray-900/50 p-0 border border-gray-800 rounded-lg",
          cursor: "bg-eco-green",
          tab: "text-gray-400 h-10 lg:h-12 text-sm lg:text-base",
          tabContent: "group-data-[selected=true]:text-white"
        }}
      >
        {/* Preferences Tab */}
        <Tab
          key="preferences"
          title={
            <div className="flex items-center gap-2">
              <Globe className="w-4 h-4" />
              <span>Preferences</span>
            </div>
          }
        >
          <div className="mt-6">
            <motion.div
              initial={{ opacity: 0 }}
              animate={{ opacity: 1 }}
              transition={{ duration: 0.4 }}
            >
              <Card className="bg-gray-900/50 border-gray-800">
                <div className="p-6">
                  <div className="space-y-6">
                    {/* Theme Selection */}
                    <div>
                      <h3 className="text-lg font-bold text-white mb-4">Appearance</h3>
                      <div className="flex items-center justify-between">
                        <div>
                          <div className="font-medium text-white">Dark Mode</div>
                          <div className="text-sm text-gray-400">Toggle between light and dark themes</div>
                        </div>
                        <div className="flex items-center gap-2">
                          <Sun className="w-4 h-4 text-gray-400" />
                          <Switch 
                            isSelected={settings.theme === "dark"}
                            onValueChange={(value) => setSettings(prev => ({ ...prev, theme: value ? "dark" : "light" }))}
                            color="success"
                          />
                          <Moon className="w-4 h-4 text-gray-400" />
                        </div>
                      </div>
                    </div>

                    {/* Language Selection */}
                    <div>
                      <h3 className="text-lg font-bold text-white mb-4">Language & Region</h3>
                      <div className="grid grid-cols-1 lg:grid-cols-2 gap-4 lg:gap-6">
                        <div>
                          <label className="block text-sm text-gray-400 mb-2">Language</label>
                          <Select
                            defaultSelectedKeys={[settings.language]}
                            startContent={<Languages className="w-4 h-4 text-gray-400" />}
                            classNames={{
                              trigger: "bg-black/30 border-gray-800",
                              value: "text-white"
                            }}
                          >
                            {languages.map((lang) => (
                              <SelectItem key={lang.value} value={lang.value}>
                                {lang.label}
                              </SelectItem>
                            ))}
                          </Select>
                        </div>
                        <div>
                          <label className="block text-sm text-gray-400 mb-2">Currency</label>
                          <Select
                            defaultSelectedKeys={[settings.currency]}
                            startContent={<DollarSign className="w-4 h-4 text-gray-400" />}
                            classNames={{
                              trigger: "bg-black/30 border-gray-800",
                              value: "text-white"
                            }}
                          >
                            {currencies.map((currency) => (
                              <SelectItem key={currency.value} value={currency.value}>
                                {currency.label}
                              </SelectItem>
                            ))}
                          </Select>
                        </div>
                      </div>
                    </div>

                    {/* Cryptocurrency Settings */}
                    <div>
                      <h3 className="text-lg font-bold text-white mb-4">Cryptocurrency</h3>
                      <div>
                        <label className="block text-sm text-gray-400 mb-2">Default Trading Currency</label>
                        <Select
                          defaultSelectedKeys={[settings.cryptoCurrency]}
                          startContent={<Bitcoin className="w-4 h-4 text-gray-400" />}
                          classNames={{
                            trigger: "bg-black/30 border-gray-800",
                            value: "text-white"
                          }}
                        >
                          {cryptoCurrencies.map((crypto) => (
                            <SelectItem key={crypto.value} value={crypto.value}>
                              {crypto.label}
                            </SelectItem>
                          ))}
                        </Select>
                      </div>
                    </div>
                  </div>
                </div>
              </Card>
            </motion.div>
          </div>
        </Tab>

        {/* Notifications Tab */}
        <Tab
          key="notifications"
          title={
            <div className="flex items-center gap-2">
              <Bell className="w-4 h-4" />
              <span>Notifications</span>
            </div>
          }
        >
          <div className="mt-6">
            <motion.div
              initial={{ opacity: 0 }}
              animate={{ opacity: 1 }}
              transition={{ duration: 0.4 }}
            >
              <Card className="bg-gray-900/50 border-gray-800">
                <div className="p-6">
                  <div className="space-y-6">
                    {/* Email Notifications */}
                    <div>
                      <h3 className="text-lg font-bold text-white mb-4">Email Notifications</h3>
                      <div className="space-y-6">
                        <div className="flex justify-between items-center">
                          <div className="flex-1 pr-4">
                            <div className="font-medium text-white">Investment Updates</div>
                            <div className="text-sm text-gray-400">Receive updates about your investments</div>
                          </div>
                          <Switch 
                            isSelected={settings.notifications.investment}
                            onValueChange={(value) => setSettings(prev => ({
                              ...prev,
                              notifications: { ...prev.notifications, investment: value }
                            }))}
                            color="success"
                          />
                        </div>
                        <div className="flex justify-between items-center">
                          <div className="flex-1 pr-4">
                            <div className="font-medium text-white">Security Alerts</div>
                            <div className="text-sm text-gray-400">Get notified about security events</div>
                          </div>
                          <Switch 
                            isSelected={settings.notifications.security}
                            onValueChange={(value) => setSettings(prev => ({
                              ...prev,
                              notifications: { ...prev.notifications, security: value }
                            }))}
                            color="success"
                          />
                        </div>
                        <div className="flex justify-between items-center">
                          <div className="flex-1 pr-4">
                            <div className="font-medium text-white">Marketing</div>
                            <div className="text-sm text-gray-400">Receive news and special offers</div>
                          </div>
                          <Switch 
                            isSelected={settings.notifications.marketing}
                            onValueChange={(value) => setSettings(prev => ({
                              ...prev,
                              notifications: { ...prev.notifications, marketing: value }
                            }))}
                            color="success"
                          />
                        </div>
                      </div>
                    </div>

                    {/* Push Notifications */}
                    <div className="pt-6 border-t border-gray-800">
                      <h3 className="text-lg font-bold text-white mb-4">Push Notifications</h3>
                      <div className="space-y-6">
                        <div className="flex justify-between items-center">
                          <div className="flex-1 pr-4">
                            <div className="font-medium text-white">Enable Push Notifications</div>
                            <div className="text-sm text-gray-400">Receive notifications on your device</div>
                          </div>
                          <Switch 
                            isSelected={settings.notifications.push}
                            onValueChange={(value) => setSettings(prev => ({
                              ...prev,
                              notifications: { ...prev.notifications, push: value }
                            }))}
                            color="success"
                          />
                        </div>
                      </div>
                    </div>
                  </div>
                </div>
              </Card>
            </motion.div>
          </div>
        </Tab>

        {/* Security Tab */}
        <Tab
          key="security"
          title={
            <div className="flex items-center gap-2">
              <Shield className="w-4 h-4" />
              <span>Security</span>
            </div>
          }
        >
          <div className="mt-6">
            <motion.div
              initial={{ opacity: 0 }}
              animate={{ opacity: 1 }}
              transition={{ duration: 0.4 }}
            >
              <Card className="bg-gray-900/50 border-gray-800">
                <div className="p-6">
                  <div className="space-y-6">
                    {/* Two-Factor Authentication */}
                    <div>
                      <h3 className="text-lg font-bold text-white mb-4">Two-Factor Authentication</h3>
                      <div className="space-y-6">
                        <div className="flex justify-between items-center">
                          <div className="flex-1 pr-4">
                            <div className="font-medium text-white">Enable 2FA</div>
                            <div className="text-sm text-gray-400">Add an extra layer of security</div>
                          </div>
                          <Switch 
                            isSelected={settings.security.twoFactor}
                            onValueChange={(value) => setSettings(prev => ({
                              ...prev,
                              security: { ...prev.security, twoFactor: value }
                            }))}
                            color="success"
                          />
                        </div>
                      </div>
                    </div>

                    {/* API Key Management */}
                    <div className="pt-6 border-t border-gray-800">
                      <h3 className="text-lg font-bold text-white mb-4">API Access</h3>
                      <div className="space-y-4">
                        <div>
                          <label className="block text-sm text-gray-400 mb-2">API Key</label>
                          <div className="flex flex-col sm:flex-row gap-2">
                            <Input
                              type={showApiKey ? "text" : "password"}
                              value={apiKey}
                              readOnly
                              className="flex-1"
                              startContent={<Key className="w-4 h-4 text-gray-400" />}
                              endContent={
                                <Button
                                  isIconOnly
                                  variant="light"
                                  onClick={() => setShowApiKey(!showApiKey)}
                                >
                                  {showApiKey ? 
                                    <EyeOff className="w-4 h-4 text-gray-400" /> : 
                                    <Eye className="w-4 h-4 text-gray-400" />
                                  }
                                </Button>
                              }
                              classNames={{
                                input: "text-white",
                                inputWrapper: "bg-black/30 border-gray-800"
                              }}
                            />
                            <Button
                              className="bg-eco-green text-white"
                              size="sm"
                            >
                              Generate New
                            </Button>
                          </div>
                        </div>
                      </div>
                    </div>

                    {/* Session Management */}
                    <div className="pt-6 border-t border-gray-800">
                      <h3 className="text-lg font-bold text-white mb-4">Session Management</h3>
                      <Button
                        className="bg-red-500/10 text-red-500 hover:bg-red-500/20"
                        variant="flat"
                        startContent={<AlertTriangle className="w-4 h-4" />}
                      >
                        Sign Out All Devices
                      </Button>
                    </div>
                  </div>
                </div>
              </Card>
            </motion.div>
          </div>
        </Tab>
      </Tabs>
    </div>
  );
}