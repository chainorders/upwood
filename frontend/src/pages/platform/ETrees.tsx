import React, { useState } from 'react';
import { Card, Button, Chip, Progress } from "@nextui-org/react";
import { motion } from 'framer-motion';
import { TreePine, Calendar, Gift, ArrowRight, Info, Trophy, Star, Lock, Sparkles, Crown } from 'lucide-react';
import { BenefitClaimModal } from '../../components/platform/BenefitClaimModal';

interface TreeNFT {
  id: string;
  tokenId: string;
  species: string;
  location: string;
  image: string;
  project: string;
  mintedAt: string;
}

interface NFTBenefit {
  id: string;
  title: string;
  partner: string;
  discount: string;
  requiredNFTs: number;
  validUntil: string;
  category: 'software' | 'travel' | 'services' | 'education' | 'retail' | 'wellness' | 'lifestyle' | 'fashion';
  xp: number;
}

interface Achievement {
  id: string;
  title: string;
  description: string;
  icon: React.ReactNode;
  requiredNFTs: number;
  xp: number;
  unlocked: boolean;
}

export default function ETrees() {
  const [selectedPage, setSelectedPage] = useState<'collection' | 'benefits'>('collection');
  const [showAchievement, setShowAchievement] = useState(false);
  const [lastUnlockedAchievement, setLastUnlockedAchievement] = useState<Achievement | null>(null);
  const [selectedCategory, setSelectedCategory] = useState<string | null>(null);
  const [selectedBenefit, setSelectedBenefit] = useState<NFTBenefit | null>(null);

  const treeNFTs: TreeNFT[] = [
    {
      id: "1",
      tokenId: "#0001",
      species: "Scots Pine",
      location: "Baltic Pine Forest",
      image: "https://images.unsplash.com/photo-1503785640985-f62e3aeee448?auto=format&fit=crop&q=80",
      project: "Baltic Pine Forest",
      mintedAt: "2024-01-15"
    },
    {
      id: "2",
      tokenId: "#0002",
      species: "Norway Spruce",
      location: "Nordic Spruce Estate",
      image: "https://images.unsplash.com/photo-1587583484084-8f9f2b9e9b3c?auto=format&fit=crop&q=80",
      project: "Nordic Spruce Estate",
      mintedAt: "2024-02-01"
    },
    {
      id: "3",
      tokenId: "#0003",
      species: "Silver Birch",
      location: "Baltic Pine Forest",
      image: "https://images.unsplash.com/photo-1542273917363-3b1817f69a2d?auto=format&fit=crop&q=80",
      project: "Baltic Pine Forest",
      mintedAt: "2024-02-15"
    },
    {
      id: "4",
      tokenId: "#0004",
      species: "Ancient Pine",
      location: "Baltic Pine Forest",
      image: "https://images.unsplash.com/photo-1473448912268-2022ce9509d8?auto=format&fit=crop&q=80",
      project: "Baltic Pine Forest",
      mintedAt: "2024-03-01"
    }
  ];

  const userLevel = Math.floor(treeNFTs.length / 2) + 1;
  const xpToNextLevel = (userLevel * 1000) - (treeNFTs.length * 500);
  const totalXPForLevel = userLevel * 1000;
  const currentXP = totalXPForLevel - xpToNextLevel;

  const achievements: Achievement[] = [
    {
      id: "1",
      title: "Forest Pioneer",
      description: "Mint your first E-tree NFT",
      icon: <TreePine className="w-6 h-6" />,
      requiredNFTs: 1,
      xp: 500,
      unlocked: treeNFTs.length >= 1
    },
    {
      id: "2",
      title: "Growing Forest",
      description: "Own 3 E-tree NFTs",
      icon: <Sparkles className="w-6 h-6" />,
      requiredNFTs: 3,
      xp: 1000,
      unlocked: treeNFTs.length >= 3
    },
    {
      id: "3",
      title: "Forest Guardian",
      description: "Own 5 E-tree NFTs",
      icon: <Crown className="w-6 h-6" />,
      requiredNFTs: 5,
      xp: 2000,
      unlocked: treeNFTs.length >= 5
    }
  ];

  const categories = [
    { id: 'travel', name: 'Travel', icon: '✈️' },
    { id: 'lifestyle', name: 'Lifestyle', icon: '🌿' },
    { id: 'software', name: 'Software Tools', icon: '💻' },
    { id: 'fashion', name: 'Sustainable Fashion', icon: '👕' }
  ];

  const nftBenefits: NFTBenefit[] = [
    {
      id: "1",
      title: "50% Off Forest Management Pro",
      partner: "ForestTech Solutions",
      discount: "50% OFF",
      requiredNFTs: 2,
      validUntil: "2024-12-31",
      category: 'software',
      xp: 500
    },
    {
      id: "2",
      title: "Free GIS Software License",
      partner: "GeoForest",
      discount: "100% OFF",
      requiredNFTs: 5,
      validUntil: "2024-12-31",
      category: 'software',
      xp: 1000
    },
    {
      id: "3",
      title: "€200 Flight Voucher",
      partner: "EcoTravel",
      discount: "€200",
      requiredNFTs: 5,
      validUntil: "2024-12-31",
      category: 'travel',
      xp: 1000
    },
    {
      id: "4",
      title: "25% Off Eco-Lodges",
      partner: "GreenStay",
      discount: "25% OFF",
      requiredNFTs: 3,
      validUntil: "2024-12-31",
      category: 'travel',
      xp: 750
    },
    {
      id: "5",
      title: "30% Off Yoga Retreats",
      partner: "EcoWellness",
      discount: "30% OFF",
      requiredNFTs: 3,
      validUntil: "2024-12-31",
      category: 'lifestyle',
      xp: 750
    },
    {
      id: "6",
      title: "Free Meditation App",
      partner: "MindfulEarth",
      discount: "100% OFF",
      requiredNFTs: 1,
      validUntil: "2024-12-31",
      category: 'lifestyle',
      xp: 250
    },
    {
      id: "7",
      title: "40% Off Eco-Fashion",
      partner: "GreenWardrobe",
      discount: "40% OFF",
      requiredNFTs: 2,
      validUntil: "2024-12-31",
      category: 'fashion',
      xp: 500
    },
    {
      id: "8",
      title: "Free Sustainable Design Course",
      partner: "EcoFashionAcademy",
      discount: "100% OFF",
      requiredNFTs: 4,
      validUntil: "2024-12-31",
      category: 'fashion',
      xp: 800
    },
    {
      id: "9",
      title: "20% Off Organic Clothing",
      partner: "EarthWear",
      discount: "20% OFF",
      requiredNFTs: 3,
      validUntil: "2024-12-31",
      category: 'fashion',
      xp: 750
    },
    {
      id: "10",
      title: "Cloud Storage for Forest Data",
      partner: "GreenCloud",
      discount: "100% OFF",
      requiredNFTs: 1,
      validUntil: "2024-12-31",
      category: 'software',
      xp: 250
    }
  ];

  const handleClaimBenefit = (benefit: NFTBenefit) => {
    if (treeNFTs.length >= benefit.requiredNFTs) {
      setSelectedBenefit(benefit);
      
      const newAchievements = achievements.filter(a => 
        !a.unlocked && treeNFTs.length >= a.requiredNFTs
      );
      
      if (newAchievements.length > 0) {
        setLastUnlockedAchievement(newAchievements[0]);
        setShowAchievement(true);
        setTimeout(() => setShowAchievement(false), 3000);
      }
    }
  };

  const renderCollection = () => (
    <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
      {treeNFTs.map((nft, index) => (
        <motion.div
          key={nft.id}
          initial={{ opacity: 0, y: 20 }}
          animate={{ opacity: 1, y: 0 }}
          transition={{ duration: 0.6, delay: index * 0.1 }}
        >
          <Card className="bg-gray-900/50 border-gray-800 hover:border-eco-green/50 transition-colors">
            <div className="aspect-[4/3] relative overflow-hidden">
              <img
                src={nft.image}
                alt={nft.species}
                className="w-full h-full object-cover"
              />
              <div className="absolute inset-0 bg-gradient-to-t from-black/60 to-transparent" />
              <div className="absolute top-4 left-4">
                <Chip
                  className="bg-black/40 backdrop-blur-sm text-white border-white/20"
                  size="sm"
                >
                  {nft.tokenId}
                </Chip>
              </div>
            </div>

            <div className="p-4">
              <div className="flex items-center gap-2 mb-2">
                <TreePine className="w-5 h-5 text-eco-green" />
                <h3 className="text-lg font-bold text-white">{nft.species}</h3>
              </div>

              <div className="text-sm text-gray-400 mb-4">
                <div className="flex items-center gap-2">
                  <Calendar className="w-4 h-4" />
                  <span>Minted: {new Date(nft.mintedAt).toLocaleDateString()}</span>
                </div>
                <div className="mt-1">Project: {nft.project}</div>
              </div>

              <Button
                className="w-full bg-eco-green text-white"
              >
                View Details
              </Button>
            </div>
          </Card>
        </motion.div>
      ))}
    </div>
  );

  const renderBenefits = () => (
    <div className="space-y-8">
      <div className="bg-gray-900/50 p-6 rounded-lg border border-gray-800">
        <div className="flex items-center justify-between mb-4">
          <div className="flex items-center gap-3">
            <div className="w-12 h-12 rounded-full bg-eco-green/20 flex items-center justify-center">
              <Trophy className="w-6 h-6 text-eco-green" />
            </div>
            <div>
              <h3 className="text-xl font-bold text-white">Level {userLevel}</h3>
              <p className="text-sm text-gray-400">Forest Guardian</p>
            </div>
          </div>
          <div className="text-right">
            <p className="text-sm text-gray-400">XP to Next Level</p>
            <p className="text-lg font-bold text-eco-green">{xpToNextLevel} XP</p>
          </div>
        </div>
        <Progress 
          value={(currentXP / totalXPForLevel) * 100}
          className="h-2"
          color="success"
        />
        <div className="flex justify-between mt-2 text-sm text-gray-500">
          <span>{currentXP} XP</span>
          <span>{totalXPForLevel} XP</span>
        </div>
      </div>

      <div className="bg-gray-900/50 p-6 rounded-lg border border-gray-800">
        <h3 className="text-lg font-bold text-white mb-4 flex items-center gap-2">
          <Star className="w-5 h-5 text-eco-green" />
          Achievements
        </h3>
        <div className="grid grid-cols-1 md:grid-cols-3 gap-4">
          {achievements.map((achievement) => (
            <Card
              key={achievement.id}
              className={`bg-black/30 border ${
                achievement.unlocked ? 'border-eco-green' : 'border-gray-800'
              }`}
            >
              <div className="p-4">
                <div className="flex items-center gap-3 mb-3">
                  <div className={`w-10 h-10 rounded-full flex items-center justify-center ${
                    achievement.unlocked ? 'bg-eco-green/20 text-eco-green' : 'bg-gray-800/50 text-gray-600'
                  }`}>
                    {achievement.unlocked ? achievement.icon : <Lock className="w-5 h-5" />}
                  </div>
                  <div>
                    <h4 className="font-medium text-white">{achievement.title}</h4>
                    <p className="text-sm text-gray-400">{achievement.description}</p>
                  </div>
                </div>
                <div className="flex justify-between items-center text-sm">
                  <span className="text-gray-500">{achievement.xp} XP</span>
                  <Chip
                    className={achievement.unlocked 
                      ? "bg-eco-green/10 text-eco-green"
                      : "bg-gray-800 text-gray-400"
                    }
                    size="sm"
                  >
                    {achievement.unlocked ? "Unlocked" : `${treeNFTs.length}/${achievement.requiredNFTs} NFTs`}
                  </Chip>
                </div>
              </div>
            </Card>
          ))}
        </div>
      </div>

      <div className="flex flex-wrap gap-3">
        <Button
          className={`${
            selectedCategory === null
              ? 'bg-eco-green text-white'
              : 'bg-gray-900/50 text-gray-400 hover:text-white'
          } transition-colors`}
          onClick={() => setSelectedCategory(null)}
        >
          All Benefits
        </Button>
        {categories.map((category) => (
          <Button
            key={category.id}
            className={`${
              selectedCategory === category.id
                ? 'bg-eco-green text-white'
                : 'bg-gray-900/50 text-gray-400 hover:text-white'
            } transition-colors`}
            onClick={() => setSelectedCategory(category.id)}
          >
            <span className="mr-2">{category.icon}</span>
            {category.name}
          </Button>
        ))}
      </div>

      <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4">
        {nftBenefits
          .filter(benefit => !selectedCategory || benefit.category === selectedCategory)
          .map((benefit, index) => (
            <motion.div
              key={benefit.id}
              initial={{ opacity: 0, y: 20 }}
              animate={{ opacity: 1, y: 0 }}
              transition={{ duration: 0.6, delay: index * 0.1 }}
            >
              <Card className={`bg-gray-900/50 ${
                treeNFTs.length >= benefit.requiredNFTs 
                  ? 'border-eco-green/50' 
                  : 'border-gray-800'
              }`}>
                <div className="p-4">
                  <div className="flex items-center justify-between mb-3">
                    <Chip
                      className="bg-eco-green text-white"
                      size="sm"
                    >
                      {benefit.discount}
                    </Chip>
                    <div className="flex items-center gap-2">
                      <Chip
                        className={treeNFTs.length >= benefit.requiredNFTs 
                          ? "bg-eco-green/10 text-eco-green" 
                          : "bg-red-500/10 text-red-500"
                        }
                        size="sm"
                      >
                        {treeNFTs.length}/{benefit.requiredNFTs} NFTs
                      </Chip>
                      <Chip
                        className="bg-purple-500/10 text-purple-500"
                        size="sm"
                      >
                        +{benefit.xp} XP
                      </Chip>
                    </div>
                  </div>

                  <h3 className="text-white font-semibold mb-1">{benefit.title}</h3>
                  <p className="text-sm text-gray-400 mb-3">Partner: {benefit.partner}</p>

                  <div className="flex items-center justify-between text-xs text-gray-500 mb-3">
                    <span>Valid until: {new Date(benefit.validUntil).toLocaleDateString()}</span>
                  </div>

                  <Button
                    className={`w-full ${
                      treeNFTs.length >= benefit.requiredNFTs 
                        ? 'bg-eco-green text-white' 
                        : 'bg-gray-800 text-gray-400'
                    }`}
                    size="sm"
                    isDisabled={treeNFTs.length < benefit.requiredNFTs}
                    endContent={<ArrowRight className="w-4 h-4" />}
                    onClick={() => handleClaimBenefit(benefit)}
                  >
                    {treeNFTs.length >= benefit.requiredNFTs ? "Claim" : "Locked"}
                  </Button>
                </div>
              </Card>
            </motion.div>
          ))}
      </div>

      {showAchievement && lastUnlockedAchievement && (
        <motion.div
          initial={{ opacity: 0, y: 50 }}
          animate={{ opacity: 1, y: 0 }}
          exit={{ opacity: 0, y: -50 }}
          className="fixed bottom-8 right-8 bg-eco-green text-white p-4 rounded-lg shadow-lg flex items-center gap-4"
        >
          <div className="w-12 h-12 rounded-full bg-white/20 flex items-center justify-center">
            {lastUnlockedAchievement.icon}
          </div>
          <div>
            <h4 className="font-bold">Achievement Unlocked!</h4>
            <p className="text-sm">{lastUnlockedAchievement.title}</p>
          </div>
        </motion.div>
      )}
    </div>
  );

  return (
    <div className="p-6">
      <div className="mb-8">
        <h1 className="text-3xl font-bold text-white mb-4">E-trees Collection</h1>
        <p className="text-gray-400 mb-8">
          Your digital collection of unique forest inventory NFTs
        </p>

        <div className="flex gap-3">
          <Button
            className={`${
              selectedPage === 'collection'
                ? 'bg-eco-green text-white'
                : 'bg-gray-900/50 text-gray-400 hover:text-white'
            } transition-colors`}
            startContent={<TreePine className="w-4 h-4" />}
            onClick={() => setSelectedPage('collection')}
          >
            My Collection
          </Button>
          <Button
            className={`${
              selectedPage === 'benefits'
                ? 'bg-eco-green text-white'
                : 'bg-gray-900/50 text-gray-400 hover:text-white'
            } transition-colors`}
            startContent={<Gift className="w-4 h-4" />}
            onClick={() => setSelectedPage('benefits')}
          >
            NFT Benefits
          </Button>
        </div>
      </div>

      {selectedPage === 'collection' ? renderCollection() : renderBenefits()}

      <BenefitClaimModal
        isOpen={selectedBenefit !== null}
        onClose={() => setSelectedBenefit(null)}
        benefit={selectedBenefit}
      />
    </div>
  );
}