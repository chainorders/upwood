import React from 'react';
import { Navigation } from '../components/Navigation';
import { useNavigate } from 'react-router-dom';
import { motion } from 'framer-motion';
import { TreePine, Sprout, Globe, Cpu, ArrowRight } from 'lucide-react';
import { Button, Card } from '@nextui-org/react';

export default function About() {
  const navigate = useNavigate();

  const teamMembers = [
    {
      title: "Chief Executive Officer",
      name: "Lauris Borodovskis",
      image: "https://www.upwood.io/_next/image?url=https%3A%2F%2Fa.storyblok.com%2Ff%2F284116%2F200x200%2F8f4b86ebfe%2Funtitled-design-2.png&w=640&q=100"
    },
    {
      title: "Chief Legal Officer",
      name: "Volodymyr Havrylyuk-Yensen",
      image: "https://cdn.prod.website-files.com/649da2af183e2280a8bd1953/649da2af183e2280a8bd1a53_1674132529654.jpeg"
    },
    {
      title: "Chief Financial Officer",
      name: "Armands Rudzitis",
      image: "https://www.upwood.io/_next/image?url=https%3A%2F%2Fa.storyblok.com%2Ff%2F284116%2F500x500%2F2781a487a2%2Farmands.png&w=640&q=100"
    },
    {
      title: "Chief Sustainability Officer",
      name: "Maxym Semak",
      image: "https://www.upwood.io/_next/image?url=https%3A%2F%2Fa.storyblok.com%2Ff%2F284116%2F200x200%2F7f0986cf13%2Funtitled-design-4.png&w=640&q=100"
    },
    {
      title: "Lead Blockchain Developer",
      name: "Parv Sharma",
      image: "https://images.unsplash.com/photo-1506794778202-cad84cf45f1d?auto=format&fit=crop&q=80"
    },
    {
      title: "Lead Financial Analyst",
      name: "Stuart Bell",
      image: "https://images.unsplash.com/photo-1534528741775-53994a69daeb?auto=format&fit=crop&q=80"
    }
  ];

  const advisors = [
    {
      title: "Tokenization Advisor",
      name: "Claus Skaaning",
      image: "https://www.blockleaders.io/.image/t_share/MjAzNjk2NTY4Mzc1NzE1NjE2/claus-skaaning-ceo.png"
    },
    {
      title: "Investment Advisor",
      name: "Andriy Zinchuk",
      image: "https://schedule.sxswsydney.com/_next/image?url=https%3A%2F%2Fdxqy0f0etgpjs.cloudfront.net%2Fb53ed282-56f3-4419-bde4-da537c7aade7.jpeg&w=1080&q=75"
    },
    {
      title: "Sales Advisor",
      name: "Aruuven Ramen",
      image: "https://images.unsplash.com/photo-1507003211169-0a1dd7228f2d?auto=format&fit=crop&q=80"
    },
    {
      title: "Legal Advisor",
      name: "Sam Thyroff-Kohl",
      image: "https://www.upwood.io/_next/image?url=https%3A%2F%2Fa.storyblok.com%2Ff%2F284116%2F200x200%2F5d304c0dd9%2Funtitled-design-5.png&w=640&q=100"
    }
  ];

  const partners = [
    {
      name: "Forest Capital",
      logo: "https://images.unsplash.com/photo-1560179707-f14e90ef3623?auto=format&fit=crop&q=80",
      description: "Leading investment firm specializing in sustainable forestry assets"
    },
    {
      name: "Green Tech Solutions",
      logo: "https://images.unsplash.com/photo-1486406146926-c627a92ad1ab?auto=format&fit=crop&q=80",
      description: "Innovative technology provider for environmental monitoring"
    },
    {
      name: "Sustainable Growth",
      logo: "https://images.unsplash.com/photo-1497366811353-6870744d04b2?auto=format&fit=crop&q=80",
      description: "Global leader in carbon credit certification and trading"
    },
    {
      name: "EcoFinance Group",
      logo: "https://images.unsplash.com/photo-1554469384-e58fac16e23a?auto=format&fit=crop&q=80",
      description: "Financial institution focused on green investments"
    }
  ];

  return (
    <div className="min-h-screen bg-black text-white">
      <Navigation onLaunchApp={() => navigate('/')} />

      {/* Hero Section */}
      <div className="relative overflow-hidden">
        <div className="absolute inset-0 bg-gradient-radial from-forest-green/20 via-black to-black" />
        <div className="max-w-7xl mx-auto px-4 py-20 relative">
          <div className="max-w-6xl mx-auto">
            <div className="grid md:grid-cols-2 gap-12 items-center">
              {/* Left Column - Hero Content */}
              <div className="text-left">
                <motion.h1 
                  initial={{ opacity: 0, y: 20 }}
                  animate={{ opacity: 1, y: 0 }}
                  transition={{ duration: 0.6 }}
                  className="text-6xl font-bold mb-6 bg-gradient-to-r from-green-400 to-emerald-600 bg-clip-text text-transparent"
                >
                  Our Mission is to plant more trees
                </motion.h1>
                <motion.p 
                  initial={{ opacity: 0, y: 20 }}
                  animate={{ opacity: 1, y: 0 }}
                  transition={{ duration: 0.6, delay: 0.2 }}
                  className="text-xl text-gray-400 mb-8"
                >
                  At Upwood, we're revolutionising the way we think about our finances by prioritising sustainable capital growth. We came together to provide humanity with accessible forest investments
                </motion.p>
              </div>

              {/* Right Column - Image */}
              <motion.div
                initial={{ opacity: 0, scale: 0.95 }}
                animate={{ opacity: 1, scale: 1 }}
                transition={{ duration: 0.6, delay: 0.3 }}
                className="relative aspect-square rounded-2xl overflow-hidden"
              >
                <div className="absolute inset-0 bg-gradient-to-t from-black/60 to-transparent z-10" />
                <img
                  src="https://images.pexels.com/photos/1423600/pexels-photo-1423600.jpeg?auto=compress&cs=tinysrgb&w=1260&h=750&dpr=1"
                  alt="Forest landscape"
                  className="w-full h-full object-cover"
                />
              </motion.div>
            </div>
          </div>
        </div>
      </div>

      {/* The Upwood Story Section */}
      <div className="relative py-24">
        <div className="max-w-3xl mx-auto px-4">
          <motion.div
            initial={{ opacity: 0, y: 20 }}
            whileInView={{ opacity: 1, y: 0 }}
            viewport={{ once: true }}
            transition={{ duration: 0.6 }}
            className="text-center mb-16"
          >
            <h2 className="text-4xl font-bold text-eco-green mb-4">The Upwood Story</h2>
            <div className="w-24 h-1 bg-eco-green mx-auto rounded-full"></div>
          </motion.div>

          <div className="space-y-20">
            <motion.div
              initial={{ opacity: 0, y: 20 }}
              whileInView={{ opacity: 1, y: 0 }}
              viewport={{ once: true }}
              transition={{ duration: 0.6 }}
              className="text-center"
            >
              <div className="flex items-center justify-center gap-4 mb-6">
                <TreePine className="w-8 h-8 text-eco-green" />
                <h3 className="text-2xl font-bold text-eco-green">We plant trees for living</h3>
              </div>
              <p className="text-gray-400 leading-relaxed">
                Upwood team consists of diverse team of professionals who came together to combine knowledge and experience for simple reason - plant the trees and revolutionize the way we plant the trees and finance forest plantations.
              </p>
            </motion.div>

            <motion.div
              initial={{ opacity: 0, y: 20 }}
              whileInView={{ opacity: 1, y: 0 }}
              viewport={{ once: true }}
              transition={{ duration: 0.6 }}
              className="text-center"
            >
              <div className="flex items-center justify-center gap-4 mb-6">
                <Sprout className="w-8 h-8 text-eco-green" />
                <h3 className="text-2xl font-bold text-eco-green">When you plant the tree, you do it for the next generation</h3>
              </div>
              <p className="text-gray-400 leading-relaxed">
                We understand that the forest we plant today will take long time until it's maturity, however, as foresters say "when you plant the tree, you do it for the next generation". Forestry is lifestyle, it's your choice to be part of something that will remain long after your life and it is in our hands to build the future we want to see.
              </p>
            </motion.div>

            <motion.div
              initial={{ opacity: 0, y: 20 }}
              whileInView={{ opacity: 1, y: 0 }}
              viewport={{ once: true }}
              transition={{ duration: 0.6 }}
              className="text-center"
            >
              <div className="flex items-center justify-center gap-4 mb-6">
                <Globe className="w-8 h-8 text-eco-green" />
                <h3 className="text-2xl font-bold text-eco-green">Founded in the heart of Northern European forests</h3>
              </div>
              <p className="text-gray-400 leading-relaxed">
                Our story starts at 2023 when Upwood founders came together to build green venture with purpose to empower humanity with accessible forest investments. Upwood is located in European Union country Latvia. More than half of Latvian area is covered in forests being one of the greenest EU states. Forestry industry in Latvia is highly advanced with skilled workforce, clear regulations and growing prices of forest and agricultural land.
              </p>
            </motion.div>

            <motion.div
              initial={{ opacity: 0, y: 20 }}
              whileInView={{ opacity: 1, y: 0 }}
              viewport={{ once: true }}
              transition={{ duration: 0.6 }}
              className="text-center"
            >
              <div className="flex items-center justify-center gap-4 mb-6">
                <Cpu className="w-8 h-8 text-eco-green" />
                <h3 className="text-2xl font-bold text-eco-green">New generation forest company</h3>
              </div>
              <p className="text-gray-400 leading-relaxed">
                We are revolutionizing forest industry with latest technologies and practical applications of Blockchain, AI, Geospatial verification and digital twins. We see that intelligent software and hardware applications can be implemented in forest management cycle to improve results and achieve maximum environmental and financial results. We are building technologically advanced forest management future, for the humanity, for the planet.
              </p>
            </motion.div>
          </div>
        </div>
      </div>

      {/* Upwood Team Section */}
      <div className="relative py-12">
        <div className="max-w-7xl mx-auto px-4">
          <motion.div
            initial={{ opacity: 0, y: 20 }}
            whileInView={{ opacity: 1, y: 0 }}
            viewport={{ once: true }}
            transition={{ duration: 0.6 }}
            className="text-center mb-16"
          >
            <h2 className="text-4xl font-bold text-eco-green mb-4">Upwood Team</h2>
            <div className="w-24 h-1 bg-eco-green mx-auto rounded-full"></div>
            <p className="mt-6 text-xl text-gray-400 max-w-2xl mx-auto">
              Meet the passionate individuals driving innovation in sustainable forestry
            </p>
          </motion.div>

          <div className="flex flex-wrap justify-center gap-8">
            {teamMembers.slice(0, 4).map((member, index) => (
              <motion.div
                key={member.name}
                initial={{ opacity: 0, y: 20 }}
                whileInView={{ opacity: 1, y: 0 }}
                viewport={{ once: true }}
                transition={{ duration: 0.6, delay: index * 0.1 }}
                className="w-full md:w-[calc(50%-1rem)] lg:w-[calc(25%-1.5rem)] bg-black/50 rounded-lg overflow-hidden border border-gray-800 group hover:border-eco-green/50 transition-colors"
              >
                <div className="aspect-square overflow-hidden">
                  <img
                    src={member.image}
                    alt={member.name}
                    className="w-full h-full object-cover group-hover:scale-105 transition-transform duration-500"
                  />
                </div>
                <div className="p-6">
                  <h3 className="text-eco-green font-semibold mb-1">{member.title}</h3>
                  <p className="text-xl font-bold text-white">{member.name}</p>
                </div>
              </motion.div>
            ))}
          </div>

          <div className="flex flex-wrap justify-center gap-8 mt-8">
            {teamMembers.slice(4).map((member, index) => (
              <motion.div
                key={member.name}
                initial={{ opacity: 0, y: 20 }}
                whileInView={{ opacity: 1, y: 0 }}
                viewport={{ once: true }}
                transition={{ duration: 0.6, delay: index * 0.1 }}
                className="w-full md:w-[calc(50%-1rem)] lg:w-[calc(25%-1.5rem)] bg-black/50 rounded-lg overflow-hidden border border-gray-800 group hover:border-eco-green/50 transition-colors"
              >
                <div className="aspect-square overflow-hidden">
                  <img
                    src={member.image}
                    alt={member.name}
                    className="w-full h-full object-cover group-hover:scale-105 transition-transform duration-500"
                  />
                </div>
                <div className="p-6">
                  <h3 className="text-eco-green font-semibold mb-1">{member.title}</h3>
                  <p className="text-xl font-bold text-white">{member.name}</p>
                </div>
              </motion.div>
            ))}
          </div>
        </div>
      </div>

      {/* Upwood Advisors Section */}
      <div className="relative py-12">
        <div className="max-w-7xl mx-auto px-4">
          <motion.div
            initial={{ opacity: 0, y: 20 }}
            whileInView={{ opacity: 1, y: 0 }}
            viewport={{ once: true }}
            transition={{ duration: 0.6 }}
            className="text-center mb-16"
          >
            <h2 className="text-4xl font-bold text-eco-green mb-4">Upwood Advisors</h2>
            <div className="w-24 h-1 bg-eco-green mx-auto rounded-full"></div>
            <p className="mt-6 text-xl text-gray-400 max-w-2xl mx-auto">
              Expert advisors guiding our vision for sustainable forest investment
            </p>
          </motion.div>

          <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-8">
            {advisors.map((advisor, index) => (
              <motion.div
                key={advisor.name}
                initial={{ opacity: 0, y: 20 }}
                whileInView={{ opacity: 1, y: 0 }}
                viewport={{ once: true }}
                transition={{ duration: 0.6, delay: index * 0.1 }}
                className="bg-black/50 rounded-lg overflow-hidden border border-gray-800 group hover:border-eco-green/50 transition-colors"
              >
                <div className="aspect-square overflow-hidden">
                  <img
                    src={advisor.image}
                    alt={advisor.name}
                    className="w-full h-full object-cover group-hover:scale-105 transition-transform duration-500"
                  />
                </div>
                <div className="p-6">
                  <h3 className="text-eco-green font-semibold mb-1">{advisor.title}</h3>
                  <p className="text-xl font-bold text-white">{advisor.name}</p>
                </div>
              </motion.div>
            ))}
          </div>
        </div>
      </div>

      {/* Upwood Partners Section */}
      <div className="relative py-12">
        <div className="max-w-7xl mx-auto px-4">
          <motion.div
            initial={{ opacity: 0, y: 20 }}
            whileInView={{ opacity: 1, y: 0 }}
            viewport={{ once: true }}
            transition={{ duration: 0.6 }}
            className="text-center mb-16"
          >
            <h2 className="text-4xl font-bold text-eco-green mb-4">Upwood Partners</h2>
            <div className="w-24 h-1 bg-eco-green mx-auto rounded-full"></div>
          </motion.div>

          <div className="grid grid-cols-2 md:grid-cols-4 gap-4 items-center max-w-4xl mx-auto">
            {[1, 2, 3, 4].map((index) => (
              <motion.div
                key={index}
                initial={{ opacity: 0, y: 20 }}
                whileInView={{ opacity: 1, y: 0 }}
                viewport={{ once: true }}
                transition={{ duration: 0.6, delay: index * 0.1 }}
                className="flex items-center justify-center"
              >
                <img
                  src="https://cdn.prod.website-files.com/62ac7e64a826043da22f394e/62aef8da450de1a05b104469_seier-capital-logo.svg"
                  alt={`Partner ${index}`}
                  className="max-h-12 w-auto opacity-70 hover:opacity-100 transition-opacity duration-300 [filter:grayscale(100%)_brightness(0.8)]"
                />
              </motion.div>
            ))}
          </div>
        </div>
      </div>

      {/* Press Section */}
      <div className="relative py-12 bg-black/30">
        <div className="max-w-7xl mx-auto px-4">
          <motion.div
            initial={{ opacity: 0, y: 20 }}
            whileInView={{ opacity: 1, y: 0 }}
            viewport={{ once: true }}
            transition={{ duration: 0.6 }}
            className="text-center mb-16"
          >
            <h2 className="text-4xl font-bold text-eco-green mb-4">Read about Upwood in Press</h2>
            <div className="w-24 h-1 bg-eco-green mx-auto rounded-full"></div>
          </motion.div>

          <div className="grid grid-cols-1 md:grid-cols-3 gap-4 items-center max-w-4xl mx-auto">
            {[1, 2, 3].map((index) => (
              <motion.div
                key={index}
                initial={{ opacity: 0, y: 20 }}
                whileInView={{ opacity: 1, y: 0 }}
                viewport={{ once: true }}
                transition={{ duration: 0.6, delay: index * 0.1 }}
                className="flex items-center justify-center"
              >
                <img
                  src="https://cointelegraph.com/icons/logo/en.svg"
                  alt={`Press ${index}`}
                  className="max-h-12 w-auto opacity-70 hover:opacity-100 transition-opacity duration-300 [filter:grayscale(100%)_brightness(0.8)]"
                />
              </motion.div>
            ))}
          </div>
        </div>
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
  );
}