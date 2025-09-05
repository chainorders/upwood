import React, { useState } from 'react';
import { Navigation } from '../components/Navigation';
import { useNavigate } from 'react-router-dom';
import { motion } from 'framer-motion';
import { Card, Button, Chip } from "@nextui-org/react";
import { Calendar, Clock, Newspaper, Cpu, Coins, Leaf, ArrowRight } from 'lucide-react';

export default function Learn() {
  const navigate = useNavigate();
  const [selectedCategory, setSelectedCategory] = useState<string | null>(null);

  const categories = [
    { id: 'news', name: 'Upwood News', icon: Newspaper, color: 'bg-blue-500' },
    { id: 'technology', name: 'Technology', icon: Cpu, color: 'bg-purple-500' },
    { id: 'investments', name: 'Investments', icon: Coins, color: 'bg-amber-500' },
    { id: 'sustainability', name: 'Sustainability', icon: Leaf, color: 'bg-green-500' }
  ];

  const articles = [
    {
      id: 1,
      title: "Upwood partners with Concordium to bring Green Bonds on-chain",
      excerpt: "Introducing a new way to invest in sustainable forestry with blockchain technology and real-world assets.",
      category: "news",
      readTime: "5 min",
      date: "2024-03-15",
      image: "https://images.unsplash.com/photo-1542601906990-b4d3fb778b09?auto=format&fit=crop&q=80"
    },
    {
      id: 2,
      title: "The Role of AI in Modern Forest Management",
      excerpt: "How artificial intelligence is revolutionizing the way we monitor and manage forest resources.",
      category: "technology",
      readTime: "8 min",
      date: "2024-03-14",
      image: "https://images.unsplash.com/photo-1473448912268-2022ce9509d8?auto=format&fit=crop&q=80"
    },
    {
      id: 3,
      title: "Understanding Green Bond Investment Returns",
      excerpt: "A comprehensive guide to calculating and maximizing returns from forest investments.",
      category: "investments",
      readTime: "10 min",
      date: "2024-03-13",
      image: "https://images.unsplash.com/photo-1502920514313-52581002a659?auto=format&fit=crop&q=80"
    },
    {
      id: 4,
      title: "Carbon Credits: A New Asset Class",
      excerpt: "How forest-based carbon credits are emerging as a valuable investment opportunity.",
      category: "investments",
      readTime: "7 min",
      date: "2024-03-12",
      image: "https://images.unsplash.com/photo-1511497584788-876760111969?auto=format&fit=crop&q=80"
    },
    {
      id: 5,
      title: "Sustainable Forestry Practices in 2024",
      excerpt: "Latest developments in sustainable forest management and their environmental impact.",
      category: "sustainability",
      readTime: "6 min",
      date: "2024-03-11",
      image: "https://images.unsplash.com/photo-1448375240586-882707db888b?auto=format&fit=crop&q=80"
    },
    {
      id: 6,
      title: "Blockchain Technology in Forest Verification",
      excerpt: "How blockchain ensures transparency and traceability in forest investments.",
      category: "technology",
      readTime: "9 min",
      date: "2024-03-10",
      image: "https://images.unsplash.com/photo-1542601906990-b4d3fb778b09?auto=format&fit=crop&q=80"
    }
  ];

  const filteredArticles = articles.filter(article => {
    return !selectedCategory || article.category === selectedCategory;
  });

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
              Learn About Green Investment
            </h1>
            <p className="text-xl text-gray-400 mb-4">
              Discover the latest insights about sustainable forestry, technology, and investment opportunities
            </p>
          </motion.div>
        </div>
      </div>

      {/* Categories */}
      <div className="max-w-7xl mx-auto px-4">
        <div className="flex justify-center gap-3 flex-wrap mb-12">
          {categories.map((category) => (
            <Button
              key={category.id}
              className={`${
                selectedCategory === category.id 
                  ? 'bg-eco-green text-white'
                  : 'bg-gray-900/50 text-gray-400 hover:text-white'
              } transition-colors`}
              startContent={<category.icon className="w-4 h-4" />}
              onClick={() => setSelectedCategory(
                selectedCategory === category.id ? null : category.id
              )}
            >
              {category.name}
            </Button>
          ))}
        </div>

        {/* Articles Grid */}
        <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-8 mb-20">
          {filteredArticles.map((article, index) => {
            const CategoryIcon = categories.find(c => c.id === article.category)?.icon || Newspaper;
            
            return (
              <motion.div
                key={article.id}
                initial={{ opacity: 0, y: 20 }}
                animate={{ opacity: 1, y: 0 }}
                transition={{ duration: 0.6, delay: index * 0.1 }}
              >
                <Card 
                  className="bg-gray-900/50 border-gray-800 hover:border-eco-green/50 transition-colors cursor-pointer group"
                  isPressable
                  onClick={() => navigate(`/blog/upwood-launches-platform`)}
                >
                  <div className="aspect-video relative overflow-hidden">
                    <img
                      src={article.image}
                      alt={article.title}
                      className="w-full h-full object-cover group-hover:scale-105 transition-transform duration-500"
                    />
                    <div className="absolute inset-0 bg-gradient-to-t from-black/60 to-transparent" />
                  </div>
                  <div className="p-6">
                    <div className="flex items-center gap-2 mb-4">
                      <Chip
                        startContent={<CategoryIcon className="w-4 h-4" />}
                        variant="flat"
                        color="success"
                        className="bg-eco-green/10 text-eco-green"
                      >
                        {categories.find(c => c.id === article.category)?.name}
                      </Chip>
                    </div>
                    <h3 className="text-xl font-bold mb-3 group-hover:text-eco-green transition-colors">
                      {article.title}
                    </h3>
                    <p className="text-gray-400 mb-4">
                      {article.excerpt}
                    </p>
                    <div className="flex items-center justify-between text-sm text-gray-500">
                      <div className="flex items-center gap-2">
                        <Calendar className="w-4 h-4" />
                        <span>{new Date(article.date).toLocaleDateString()}</span>
                      </div>
                      <div className="flex items-center gap-2">
                        <Clock className="w-4 h-4" />
                        <span>{article.readTime} read</span>
                      </div>
                    </div>
                  </div>
                </Card>
              </motion.div>
            );
          })}
        </div>

        {/* CTA Section */}
        <div className="container mx-auto px-4 py-20">
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
    </div>
  );
}