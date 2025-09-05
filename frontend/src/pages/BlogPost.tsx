import React, { useState } from 'react';
import { useParams, useNavigate } from 'react-router-dom';
import { Navigation } from '../components/Navigation';
import { Button, Card, Chip, Popover, PopoverTrigger, PopoverContent, Input } from "@nextui-org/react";
import { motion } from 'framer-motion';
import { ArrowLeft, Calendar, Clock, Share2, ArrowRight, Copy, CheckCheck } from 'lucide-react';
import { Footer } from '../components/Footer';

// This would typically come from an API or CMS
const blogPosts = {
  "upwood-launches-platform": {
    title: "Upwood partners with Concordium to bring green bonds on-chain",
    excerpt: "Upwood has partnered with Concordium blockchain to revolutionize impact investments.",
    content: `
      <p class="lead text-xl text-gray-300 mb-8">Upwood, a pioneering Northern European-based platform specializing in forest asset tokenization, has partnered with Concordium blockchain to revolutionize sustainable investments. This collaboration enhances the transparency, accessibility and security of tokenized forest assets and carbon credits, marking a significant advancement in green finance.</p>

      <h2 class="text-3xl font-bold text-eco-green mt-12 mb-6">Institutional grade Blockchain</h2>
      <p class="text-gray-300 leading-relaxed mb-6">Upwood strategically chose Concordium for its unique advantages in Real-World Asset tokenization. Concordium's built-in ID layer provides essential regulatory compliance while maintaining privacy—crucial features for issuance of regulated financial instruments on-chain. This identity verification at the protocol level creates trust without compromising security, a fundamental requirement for institutional adoption.</p>

      <img 
        src="https://images.unsplash.com/photo-1542273917363-3b1817f69a2d?auto=format&fit=crop&q=80" 
        alt="Sustainable Forest Management" 
        class="my-8 rounded-xl w-full"
      />

      <h2 class="text-3xl font-bold text-eco-green mt-12 mb-6">Performance metrics</h2>
      <p class="text-gray-300 leading-relaxed mb-6">Concordium's impressive performance metrics—2-second finality, 2000 transactions per second, and minimal transaction costs—provide the efficiency needed for Upwood's tokenized forest investment platform, where users can invest in forest-backed tokens with minimal costs associated with blockchain transactions.</p>
      <ul class="list-disc list-inside space-y-2 text-gray-300 mb-8">
        <li>Blockchain-based asset tokenization for transparent ownership records</li>
        <li>Forest monitoring using satellite and drone imagery</li>
        <li>Embedding environmental data in token metadata though geospatial models</li>
        <li>Smart contracts for automated dividend distribution</li>
      </ul>

      <div class="bg-gray-900/50 border border-gray-800 rounded-xl p-6 my-8">
        <h3 class="text-xl font-bold mb-4 text-white">Investment Performance Metrics</h3>
        <div class="relative" style="height: 300px;">
          <!-- Placeholder for Chart.js or similar -->
          <img 
            src="https://images.unsplash.com/photo-1551288049-bebda4e38f71?auto=format&fit=crop&q=80"
            alt="Investment Growth Chart"
            class="absolute inset-0 w-full h-full object-cover rounded-lg"
          />
        </div>
        <p class="text-sm text-gray-400 mt-4">
          Historical performance of tokenized forest investments vs. traditional forestry investments (2020-2024)
        </p>
      </div>

      <h2 class="text-3xl font-bold text-eco-green mt-12 mb-6">Sustainability at its core</h2>
      <p class="text-gray-300 leading-relaxed mb-6">Environmental alignment was another decisive factor. Concordium's Proof-of-Stake consensus significantly reduces energy consumption, perfectly complementing Upwood's mission to use environmentally friendly and energy efficient technologies.</p>

      <img 
        src="https://images.unsplash.com/photo-1473448912268-2022ce9509d8?auto=format&fit=crop&q=80" 
        alt="Forest Conservation" 
        class="my-8 rounded-xl w-full"
      />

      <h2 class="text-3xl font-bold text-eco-green mt-12 mb-6">Final remarks</h2>
      <p class="text-gray-300 leading-relaxed mb-6">Upwood and Concordium partnership democratizes access to ESG Impact investments through forest backed green bond issuance on blockchain. Investors can securely invest in stable and profitable green bonds backed up real world forests, carbon credit sales, and while being able to verify environmental impact in on-chain through Upwood's geospatial verification mechanism, all secured by Concordium's verifiable blockchain infrastructure.</p>
      
      <div class="bg-[#1D4B3A] p-8 rounded-xl my-12">
        <h3 class="text-2xl font-bold text-white mb-4">Key Takeaways</h3>
        <ul class="space-y-4">
          <li class="flex items-start gap-3">
            <span class="w-6 h-6 rounded-full bg-eco-green/20 flex items-center justify-center flex-shrink-0 mt-1">1</span>
            <p class="text-gray-200">Concordium's ID layer ensures regulatory compliance for tokenized forest assets</p>
          </li>
          <li class="flex items-start gap-3">
            <span class="w-6 h-6 rounded-full bg-eco-green/20 flex items-center justify-center flex-shrink-0 mt-1">2</span>
            <p class="text-gray-200">The partnership democratizes access to sustainable investments through Green Bond tokens</p>
          </li>
          <li class="flex items-start gap-3">
            <span class="w-6 h-6 rounded-full bg-eco-green/20 flex items-center justify-center flex-shrink-0 mt-1">3</span>
            <p class="text-gray-200">Upwood & Concordium share strong focus on environmental impact and sustainability</p>
          </li>
          <li class="flex items-start gap-3">
            <span class="w-6 h-6 rounded-full bg-eco-green/20 flex items-center justify-center flex-shrink-0 mt-1">4</span>
            <p class="text-gray-200">Transparent tracking combats greenwashing in ESG investments</p>
          </li>
        </ul>
      </div>
    `,
    date: "2024-03-15",
    readTime: "5 min",
    category: "news",
    image: "https://images.unsplash.com/photo-1542601906990-b4d3fb778b09?auto=format&fit=crop&q=80",
    tags: ["Launch", "Technology", "Investment", "Sustainability"]
  }
};

export default function BlogPost() {
  const { slug } = useParams();
  const navigate = useNavigate();
  const [copied, setCopied] = useState(false);
  const post = blogPosts[slug as keyof typeof blogPosts];

  if (!post) {
    return (
      <div className="min-h-screen bg-black text-white py-20">
        <div className="max-w-4xl mx-auto px-4 text-center">
          <h1 className="text-4xl font-bold text-eco-green mb-4">Post Not Found</h1>
          <Button 
            className="bg-eco-green text-white"
            onClick={() => navigate('/learn')}
            startContent={<ArrowLeft className="w-4 h-4" />}
          >
            Back to Blog
          </Button>
        </div>
      </div>
    );
  }

  const handleCopyLink = () => {
    const url = window.location.href;
    navigator.clipboard.writeText(url);
    setCopied(true);
    setTimeout(() => setCopied(false), 2000);
  };

  return (
    <div className="min-h-screen bg-black text-white">
      <Navigation onLaunchApp={() => navigate('/')} />
      
      {/* Hero Section */}
      <div className="relative h-[60vh] overflow-hidden">
        <div className="absolute inset-0">
          <img 
            src={post.image} 
            alt={post.title}
            className="w-full h-full object-cover"
          />
          <div className="absolute inset-0 bg-gradient-to-b from-black/60 via-black/40 to-black" />
        </div>
        <div className="absolute inset-0 flex items-center">
          <div className="max-w-4xl mx-auto px-4">
            <motion.div
              initial={{ opacity: 0, y: 20 }}
              animate={{ opacity: 1, y: 0 }}
              transition={{ duration: 0.6 }}
            >
              <Button
                className="mb-6 bg-black/30 text-white backdrop-blur-sm border border-white/10"
                size="sm"
                startContent={<ArrowLeft className="w-4 h-4" />}
                onClick={() => navigate('/learn')}
              >
                Back to Blog
              </Button>
              <h1 className="text-5xl font-bold mb-6">{post.title}</h1>
              <div className="flex flex-wrap items-center gap-4 text-sm text-gray-300">
                <div className="flex items-center gap-2">
                  <Calendar className="w-4 h-4" />
                  {new Date(post.date).toLocaleDateString()}
                </div>
                <div className="flex items-center gap-2">
                  <Clock className="w-4 h-4" />
                  {post.readTime} read
                </div>
                <Chip
                  className="bg-eco-green text-white"
                  size="sm"
                >
                  {post.category}
                </Chip>
              </div>
            </motion.div>
          </div>
        </div>
      </div>

      {/* Content Section */}
      <div className="max-w-4xl mx-auto px-4 py-12">
        <div className="grid grid-cols-1 lg:grid-cols-[1fr_200px] gap-8">
          <motion.div
            initial={{ opacity: 0, y: 20 }}
            animate={{ opacity: 1, y: 0 }}
            transition={{ duration: 0.6, delay: 0.2 }}
          >
            {/* Article Content */}
            <article className="prose prose-invert prose-green max-w-none">
              <div dangerouslySetInnerHTML={{ __html: post.content }} />
            </article>

            {/* Tags */}
            <div className="mt-8 pt-8 border-t border-gray-800">
              <div className="flex flex-wrap gap-2">
                {post.tags.map((tag) => (
                  <Chip
                    key={tag}
                    className="bg-gray-800 text-gray-300"
                    size="sm"
                  >
                    {tag}
                  </Chip>
                ))}
              </div>
            </div>
          </motion.div>

          {/* Sidebar */}
          <motion.div
            initial={{ opacity: 0, x: 20 }}
            animate={{ opacity: 1, x: 0 }}
            transition={{ duration: 0.6, delay: 0.4 }}
            className="space-y-4"
          >
            <Popover placement="bottom" showArrow offset={10}>
              <PopoverTrigger>
                <Button
                  className="w-full bg-eco-green text-white"
                  startContent={<Share2 className="w-4 h-4" />}
                >
                  Share Article
                </Button>
              </PopoverTrigger>
              <PopoverContent className="w-[300px]">
                <div className="px-1 py-2">
                  <div className="text-small font-bold mb-2">Share this article</div>
                  <div className="flex gap-2">
                    <Input
                      value={window.location.href}
                      readOnly
                      classNames={{
                        input: "text-small",
                        inputWrapper: "bg-default-100"
                      }}
                    />
                    <Button
                      isIconOnly
                      className={copied ? "bg-success text-white" : "bg-default"}
                      onClick={handleCopyLink}
                    >
                      {copied ? <CheckCheck className="w-4 h-4" /> : <Copy className="w-4 h-4" />}
                    </Button>
                  </div>
                </div>
              </PopoverContent>
            </Popover>
          </motion.div>
        </div>

        {/* CTA Section */}
        <div className="mt-16">
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