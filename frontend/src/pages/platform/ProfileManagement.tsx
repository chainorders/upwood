import React, { useState } from 'react';
import { Card, Button, Input, Tabs, Tab, Chip, Switch, Table, TableHeader, TableColumn, TableBody, TableRow, TableCell } from "@nextui-org/react";
import { motion } from 'framer-motion';
import { User, Building2, Users, Copy, Mail, Phone, Globe, FileText, Share2, Coins, UserPlus, Trash2, Calendar } from 'lucide-react';
import { SendLegalEntityMemberModal } from '../../components/platform/SendLegalEntityMemberModal';

interface LegalEntityMember {
  id: string;
  name: string;
  email: string;
  role: string;
  status: string;
  joinedAt: string;
}

export default function ProfileManagement() {
  const [selectedTab, setSelectedTab] = useState("personal");
  const [notifications, setNotifications] = useState({
    email: true,
    investment: true,
    marketing: false
  });
  const [showInviteModal, setShowInviteModal] = useState(false);
  const [members, setMembers] = useState<LegalEntityMember[]>([
    {
      id: "1",
      name: "John Doe",
      email: "john.doe@example.com",
      role: "Administrator",
      status: "Active",
      joinedAt: "2024-01-15"
    },
    {
      id: "2",
      name: "Sarah Smith",
      email: "sarah.smith@example.com",
      role: "Member",
      status: "Active",
      joinedAt: "2024-02-01"
    },
    {
      id: "3",
      name: "Mike Johnson",
      email: "mike.j@example.com",
      role: "Member",
      status: "Pending",
      joinedAt: "2024-03-15"
    }
  ]);

  const userInfo = {
    name: "John Doe",
    email: "john.doe@example.com",
    phone: "+1 234 567 8900",
    country: "United States",
    verificationStatus: "Verified",
    kycStatus: "Completed"
  };

  const legalEntity = {
    companyName: "Green Investments LLC",
    registrationNumber: "LL12345678",
    vatNumber: "VAT123456789",
    country: "United States",
    status: "Pending Verification"
  };

  const referralInfo = {
    code: "JOHN123",
    totalReferrals: 12,
    activeReferrals: 8,
    totalEarnings: "€2,450",
    pendingEarnings: "€350",
    recentReferrals: [
      {
        id: 1,
        user: "Alice Smith",
        date: "2024-03-10",
        status: "Active",
        earnings: "€200"
      },
      {
        id: 2,
        user: "Bob Johnson",
        date: "2024-03-08",
        status: "Pending",
        earnings: "€150"
      }
    ]
  };

  const copyToClipboard = (text: string) => {
    navigator.clipboard.writeText(text);
  };

  const handleRemoveMember = (memberId: string) => {
    setMembers(prev => prev.filter(member => member.id !== memberId));
  };

  return (
    <div className="p-4 lg:p-6">
      <div className="mb-8">
        <h1 className="text-2xl lg:text-3xl font-bold text-white mb-4">Profile Settings</h1>
        <p className="text-sm lg:text-base text-gray-400">
          Manage your personal information, legal entity, and referral settings
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
        <Tab
          key="personal"
          title={
            <div className="flex items-center gap-2">
              <User className="w-4 h-4" />
              <span>Personal Info</span>
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
                  <div className="grid grid-cols-1 lg:grid-cols-2 gap-4 lg:gap-6">
                    <div>
                      <label className="block text-sm text-gray-400 mb-2">Full Name</label>
                      <Input
                        value={userInfo.name}
                        classNames={{
                          input: "text-white",
                          inputWrapper: "bg-black/30 border-gray-800"
                        }}
                      />
                    </div>
                    <div>
                      <label className="block text-sm text-gray-400 mb-2">Email</label>
                      <Input
                        value={userInfo.email}
                        type="email"
                        startContent={<Mail className="w-4 h-4 text-gray-400" />}
                        classNames={{
                          input: "text-white",
                          inputWrapper: "bg-black/30 border-gray-800"
                        }}
                      />
                    </div>
                    <div>
                      <label className="block text-sm text-gray-400 mb-2">Phone</label>
                      <Input
                        value={userInfo.phone}
                        startContent={<Phone className="w-4 h-4 text-gray-400" />}
                        classNames={{
                          input: "text-white",
                          inputWrapper: "bg-black/30 border-gray-800"
                        }}
                      />
                    </div>
                    <div>
                      <label className="block text-sm text-gray-400 mb-2">Country</label>
                      <Input
                        value={userInfo.country}
                        startContent={<Globe className="w-4 h-4 text-gray-400" />}
                        classNames={{
                          input: "text-white",
                          inputWrapper: "bg-black/30 border-gray-800"
                        }}
                      />
                    </div>
                  </div>

                  <div className="mt-6 pt-6 border-t border-gray-800">
                    <h3 className="text-lg font-bold text-white mb-4">Verification Status</h3>
                    <div className="flex gap-4">
                      <Chip
                        className="bg-eco-green/10 text-eco-green"
                        size="sm"
                      >
                        Identity: {userInfo.verificationStatus}
                      </Chip>
                      <Chip
                        className="bg-eco-green/10 text-eco-green"
                        size="sm"
                      >
                        KYC: {userInfo.kycStatus}
                      </Chip>
                    </div>
                  </div>

                  <div className="mt-6 pt-6 border-t border-gray-800">
                    <h3 className="text-lg font-bold text-white mb-4">Notification Settings</h3>
                    <div className="space-y-6">
                      <div className="flex justify-between items-center">
                        <div className="flex-1 pr-4">
                          <div className="font-medium text-white">Email Notifications</div>
                          <div className="text-sm text-gray-400">Receive important updates via email</div>
                        </div>
                        <Switch 
                          isSelected={notifications.email}
                          onValueChange={(value) => setNotifications(prev => ({ ...prev, email: value }))}
                          color="success"
                        />
                      </div>
                      <div className="flex justify-between items-center">
                        <div className="flex-1 pr-4">
                          <div className="font-medium text-white">Investment Updates</div>
                          <div className="text-sm text-gray-400">Get notified about your investments</div>
                        </div>
                        <Switch 
                          isSelected={notifications.investment}
                          onValueChange={(value) => setNotifications(prev => ({ ...prev, investment: value }))}
                          color="success"
                        />
                      </div>
                      <div className="flex justify-between items-center">
                        <div className="flex-1 pr-4">
                          <div className="font-medium text-white">Marketing Communications</div>
                          <div className="text-sm text-gray-400">Receive news and special offers</div>
                        </div>
                        <Switch 
                          isSelected={notifications.marketing}
                          onValueChange={(value) => setNotifications(prev => ({ ...prev, marketing: value }))}
                          color="success"
                        />
                      </div>
                    </div>
                  </div>

                  <div className="mt-6 flex justify-end">
                    <Button
                      className="bg-eco-green text-white"
                    >
                      Save Changes
                    </Button>
                  </div>
                </div>
              </Card>
            </motion.div>
          </div>
        </Tab>

        <Tab
          key="legal"
          title={
            <div className="flex items-center gap-2">
              <Building2 className="w-4 h-4" />
              <span>Legal Entity</span>
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
                  <div className="flex items-center justify-between mb-6">
                    <h3 className="text-lg font-bold text-white">Legal Entity Information</h3>
                    <Chip
                      className="bg-amber-500/10 text-amber-500"
                      size="sm"
                    >
                      {legalEntity.status}
                    </Chip>
                  </div>

                  <div className="grid grid-cols-1 lg:grid-cols-2 gap-4 lg:gap-6">
                    <div>
                      <label className="block text-sm text-gray-400 mb-2">Company Name</label>
                      <Input
                        value={legalEntity.companyName}
                        startContent={<Building2 className="w-4 h-4 text-gray-400" />}
                        classNames={{
                          input: "text-white",
                          inputWrapper: "bg-black/30 border-gray-800"
                        }}
                      />
                    </div>
                    <div>
                      <label className="block text-sm text-gray-400 mb-2">Registration Number</label>
                      <Input
                        value={legalEntity.registrationNumber}
                        startContent={<FileText className="w-4 h-4 text-gray-400" />}
                        classNames={{
                          input: "text-white",
                          inputWrapper: "bg-black/30 border-gray-800"
                        }}
                      />
                    </div>
                    <div>
                      <label className="block text-sm text-gray-400 mb-2">VAT Number</label>
                      <Input
                        value={legalEntity.vatNumber}
                        classNames={{
                          input: "text-white",
                          inputWrapper: "bg-black/30 border-gray-800"
                        }}
                      />
                    </div>
                    <div>
                      <label className="block text-sm text-gray-400 mb-2">Country</label>
                      <Input
                        value={legalEntity.country}
                        startContent={<Globe className="w-4 h-4 text-gray-400" />}
                        classNames={{
                          input: "text-white",
                          inputWrapper: "bg-black/30 border-gray-800"
                        }}
                      />
                    </div>
                  </div>

                  <div className="mt-8 pt-8 border-t border-gray-800">
                    <div className="flex items-center justify-between mb-6">
                      <div>
                        <h3 className="text-base lg:text-lg font-bold text-white">Legal Entity Members</h3>
                        <p className="text-xs lg:text-sm text-gray-400">Manage members of your legal entity</p>
                      </div>
                      <Button
                        className="bg-eco-green text-white"
                        size="sm"
                        startContent={<UserPlus className="w-4 h-4" />}
                        onClick={() => setShowInviteModal(true)}
                      >
                        <span className="hidden sm:inline">Add Member</span>
                        <span className="sm:hidden">Add</span>
                      </Button>
                    </div>

                    <Card className="border-none bg-black/40">
                      <Table
                        aria-label="Legal entity members"
                        shadow="none"
                        isCompact
                        classNames={{
                          base: "max-h-[420px]",
                          table: "min-h-[420px]",
                          thead: "[&>tr]:first:shadow-none",
                          th: [
                            "bg-transparent",
                            "text-gray-400",
                            "border-b",
                            "border-gray-800",
                            "font-medium",
                            "text-xs",
                            "tracking-wider",
                            "h-12"
                          ].join(" "),
                          td: [
                            "text-white",
                            "group-data-[first=true]:first:rounded-l-lg",
                            "group-data-[first=true]:last:rounded-r-lg",
                            "border-b",
                            "border-gray-800/50",
                            "h-[72px]"
                          ].join(" "),
                          tr: [
                            "transition-colors",
                            "hover:bg-gray-800/40",
                            "[&>td:first-child]:pl-6",
                            "[&>td:last-child]:pr-6"
                          ].join(" "),
                          wrapper: "p-0 rounded-lg border border-gray-800/50 bg-black/20",
                          tbody: "before:content-[''] before:block before:h-3",
                        }}
                      >
                        <TableHeader>
                          <TableColumn>NAME</TableColumn>
                          <TableColumn>EMAIL</TableColumn>
                          <TableColumn className="hidden sm:table-cell">ROLE</TableColumn>
                          <TableColumn>STATUS</TableColumn>
                          <TableColumn className="hidden lg:table-cell">JOINED</TableColumn>
                          <TableColumn align="center">ACTIONS</TableColumn>
                        </TableHeader>
                        <TableBody emptyContent="No members found">
                          {members.map((member) => (
                            <TableRow key={member.id} className="group">
                              <TableCell>
                                <div className="flex items-center gap-2 lg:gap-3">
                                  <div className="w-10 h-10 rounded-full bg-gray-800/80 flex items-center justify-center">
                                    <User className="w-5 h-5 text-gray-400" />
                                  </div>
                                  <div>
                                    <span className="font-medium text-sm lg:text-base">{member.name}</span>
                                    <div className="sm:hidden text-xs text-gray-400">{member.role}</div>
                                  </div>
                                </div>
                              </TableCell>
                              <TableCell>
                                <div className="flex items-center gap-2 text-sm">
                                  <Mail className="w-4 h-4 text-gray-400" />
                                  <span className="truncate">{member.email}</span>
                                </div>
                              </TableCell>
                              <TableCell className="hidden sm:table-cell">
                                <Chip
                                  className={member.role === "Administrator" 
                                    ? "bg-eco-green/10 text-eco-green border-eco-green/20"
                                    : "bg-blue-500/10 text-blue-500 border-blue-500/20"
                                  }
                                  variant="bordered"
                                  size="sm"
                                >
                                  {member.role}
                                </Chip>
                              </TableCell>
                              <TableCell>
                                <Chip
                                  className={member.status === "Active"
                                    ? "bg-green-500/10 text-green-500 border-green-500/20"
                                    : "bg-amber-500/10 text-amber-500 border-amber-500/20"
                                  }
                                  variant="bordered"
                                  size="sm"
                                  startContent={
                                    <div className={`w-1.5 h-1.5 rounded-full ${
                                      member.status === "Active" ? "bg-green-500" : "bg-amber-500"
                                    }`} 
                                    />
                                  }
                                >
                                  {member.status}
                                </Chip>
                              </TableCell>
                              <TableCell className="hidden lg:table-cell">
                                <div className="flex items-center gap-2 text-gray-400">
                                  <Calendar className="w-4 h-4" />
                                  {new Date(member.joinedAt).toLocaleDateString()}
                                </div>
                              </TableCell>
                              <TableCell>
                                <div className="flex justify-center">
                                  <Button
                                    isIconOnly
                                    variant="light"
                                    className="text-danger hover:text-danger-400 w-8 h-8"
                                    onClick={() => handleRemoveMember(member.id)}
                                  >
                                    <Trash2 className="w-4 h-4" />
                                  </Button>
                                </div>
                              </TableCell>
                            </TableRow>
                          ))}
                        </TableBody>
                      </Table>
                    </Card>
                  </div>

                  <div className="mt-6 flex flex-col sm:flex-row justify-end gap-3">
                    <Button
                      className="bg-gray-800 text-white"
                      variant="flat"
                    >
                      Cancel
                    </Button>
                    <Button
                      className="bg-eco-green text-white"
                    >
                      Save Changes
                    </Button>
                  </div>
                </div>
              </Card>
            </motion.div>
          </div>
        </Tab>

        <Tab
          key="referral"
          title={
            <div className="flex items-center gap-2">
              <Users className="w-4 h-4" />
              <span>Referral Program</span>
            </div>
          }
        >
          <div className="mt-6">
            <motion.div
              initial={{ opacity: 0 }}
              animate={{ opacity: 1 }}
              transition={{ duration: 0.4 }}
            >
              <div className="grid grid-cols-1 lg:grid-cols-3 gap-6">
                <div className="lg:col-span-2">
                  <Card className="bg-gray-900/50 border-gray-800">
                    <div className="p-4 lg:p-6">
                      <h3 className="text-lg font-bold text-white mb-6">Your Referral Link</h3>
                      
                      <div className="space-y-6">
                        <div>
                          <label className="block text-sm text-gray-400 mb-2">Share this link</label>
                          <div className="flex flex-col sm:flex-row gap-2">
                            <Input
                              value={`https://greenbond.io/ref/${referralInfo.code}`}
                              readOnly
                              className="flex-1"
                              classNames={{
                                input: "text-white",
                                inputWrapper: "bg-black/30 border-gray-800"
                              }}
                            />
                            <div className="flex gap-2">
                              <Button
                                className="bg-gray-800 text-white flex-1 sm:flex-none"
                                variant="flat"
                                startContent={<Copy className="w-4 h-4" />}
                                onClick={() => copyToClipboard(`https://greenbond.io/ref/${referralInfo.code}`)}
                              >
                                Copy
                              </Button>
                              <Button
                                className="bg-eco-green text-white flex-1 sm:flex-none"
                                startContent={<Share2 className="w-4 h-4" />}
                              >
                                Share
                              </Button>
                            </div>
                          </div>
                        </div>

                        <div className="grid grid-cols-1 sm:grid-cols-2 gap-4">
                          <div className="bg-black/30 p-4 rounded-lg">
                            <div className="text-sm text-gray-400 mb-1">Total Referrals</div>
                            <div className="text-xl font-bold text-white">{referralInfo.totalReferrals}</div>
                          </div>
                          <div className="bg-black/30 p-4 rounded-lg">
                            <div className="text-sm text-gray-400 mb-1">Active Referrals</div>
                            <div className="text-xl font-bold text-eco-green">{referralInfo.activeReferrals}</div>
                          </div>
                        </div>

                        <div className="grid grid-cols-1 sm:grid-cols-2 gap-4">
                          <div className="bg-black/30 p-4 rounded-lg">
                            <div className="text-sm text-gray-400 mb-1">Total Earnings</div>
                            <div className="text-xl font-bold text-white">{referralInfo.totalEarnings}</div>
                          </div>
                          <div className="bg-black/30 p-4 rounded-lg">
                            <div className="text-sm text-gray-400 mb-1">Pending Earnings</div>
                            <div className="text-xl font-bold text-eco-green">{referralInfo.pendingEarnings}</div>
                          </div>
                        </div>
                      </div>
                    </div>
                  </Card>
                </div>

                <div>
                  <Card className="bg-gray-900/50 border-gray-800">
                    <div className="p-4 lg:p-6">
                      <h3 className="text-lg font-bold text-white mb-4">Recent Referrals</h3>
                      <div className="space-y-4">
                        {referralInfo.recentReferrals.map((referral) => (
                          <div
                            key={referral.id}
                            className="p-4 bg-black/30 rounded-lg"
                          >
                            <div className="flex justify-between items-start mb-2">
                              <div>
                                <div className="font-medium text-white">{referral.user}</div>
                                <div className="text-sm text-gray-400">
                                  {new Date(referral.date).toLocaleDateString()}
                                </div>
                              </div>
                              <Chip
                                size="sm"
                                className={referral.status === "Active" 
                                  ? "bg-eco-green/10 text-eco-green"
                                  : "bg-amber-500/10 text-amber-500"
                                }
                              >
                                {referral.status}
                              </Chip>
                            </div>
                            <div className="flex items-center gap-2 text-sm">
                              <Coins className="w-4 h-4 text-eco-green" />
                              <span className="text-eco-green font-medium">{referral.earnings}</span>
                            </div>
                          </div>
                        ))}
                      </div>
                    </div>
                  </Card>
                </div>
              </div>
            </motion.div>
          </div>
        </Tab>
      </Tabs>

      <SendLegalEntityMemberModal
        isOpen={showInviteModal}
        onClose={() => setShowInviteModal(false)}
      />
    </div>
  );
}