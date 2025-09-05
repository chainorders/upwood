import React, { useRef } from 'react';
import { Button, Card, Input, Slider, Accordion, AccordionItem } from '@nextui-org/react';
import { ArrowRight, Calculator, Coins, Trees, Banknote, Leaf, Trees as Tree, ChevronDown } from 'lucide-react';
import { motion } from 'framer-motion';
import { Navigation } from '../components/Navigation';

export function HomePage({ onRequestInvitation, onLaunchApp }: {
  onRequestInvitation: () => void;
  onLaunchApp: () => void;
}) {
  const [investment, setInvestment] = React.useState(1000);
  const [years, setYears] = React.useState(5);
  const [selectedTokenType, setSelectedTokenType] = React.useState(0);
  const annualReturn = 0.08;
  
  const learnMoreSectionRef = useRef<HTMLDivElement>(null);

  const scrollToLearnMore = () => {
    learnMoreSectionRef.current?.scrollIntoView({ behavior: 'smooth' });
  };

  const handleInvestmentChange = (e: React.ChangeEvent<HTMLInputElement>) => {
    const value = e.target.value;
    // Only update if value is empty or a valid number
    if (value === '' || /^\d+$/.test(value)) {
      setInvestment(value === '' ? 0 : parseInt(value));
    }
  };

  const tokenTypes = [
    {
      name: "Green Bond Token",
      description: "World's first tokenized Green Bond backed up by real world forests and carbon credits, providing stable, profitable and sustainable impact investment opportunity.",
      icon: Tree,
      stats: [
        { label: "Token symbol", value: "tEUGB" },
        { label: "Yield", value: "6% + variable" }
      ]
    },
    {
      name: "Carbon Credit Token",
      description: "Tokenized carbon credits from verified forest projects. Carbon credit tokens ensure full transparency over its lifecycle and full information on relevant data.",
      icon: Coins,
      stats: [
        { label: "Token symbol", value: "tCO2" },
        { label: "CO₂ Offset goal", value: "1 giga ton" }
      ]
    },
    {
      name: "NFT Tree Token",
      description: "Own a unique digital collectible linked to real trees. Each NFT is linked to a specific forest area, allowing you use it for digital collection or aditional benefits.",
      icon: Leaf,
      stats: []
    }
  ];

  const calculateReturns = () => {
    const futureValue = investment * Math.pow(1 + annualReturn, years);
    const totalReturns = futureValue - investment;

    return {
      futureValue: futureValue.toFixed(2),
      totalReturns: totalReturns.toFixed(2)
    };
  };

  const returns = calculateReturns();
  const selectedToken = tokenTypes[selectedTokenType];

  return (
    <>
      <Navigation onLaunchApp={onLaunchApp} />
      
      {/* Hero Section */}
      <div className="relative overflow-hidden">
        <div className="absolute inset-0 bg-gradient-radial from-forest-green/20 via-black to-black" />
        <div className="max-w-7xl mx-auto px-4 py-12 sm:py-20 relative">
          <div className="max-w-6xl mx-auto">
            <div className="grid md:grid-cols-2 gap-8 md:gap-12 items-center">
              {/* Left Column - Hero Content */}
              <div className="text-center md:text-left">
                <h1 className="text-4xl sm:text-5xl md:text-6xl font-bold mb-4 sm:mb-6 bg-gradient-to-r from-green-400 to-emerald-600 bg-clip-text text-transparent leading-tight">
                  Invest in Earth's Future with Green Bond Tokens
                </h1>
                <p className="text-lg sm:text-xl text-gray-400 mb-6 sm:mb-8">
                  Secure, transparent, and impactful investments backed by forests and carbon credits
                </p>
                <div className="flex flex-col sm:flex-row gap-4 justify-center md:justify-start">
                  <Button 
                    size="lg" 
                    className="w-full sm:w-auto font-semibold bg-[#3a5a40] text-white hover:bg-[#3a5a40]/90"
                    onClick={onRequestInvitation}
                  >
                    Request Invitation <ArrowRight className="ml-2" />
                  </Button>
                  <Button 
                    size="lg" 
                    variant="bordered" 
                    color="success"
                    className="w-full sm:w-auto"
                    onClick={scrollToLearnMore}
                  >
                    Learn More
                  </Button>
                </div>
              </div>

              {/* Right Column - Calculator */}
              <Card className="p-4 sm:p-6 bg-gray-900/50 border-1 border-gray-800 mt-8 md:mt-0">
                <div className="flex items-center gap-2 mb-4 sm:mb-6">
                  <Calculator className="w-5 h-5 sm:w-6 sm:h-6 text-eco-green" />
                  <h3 className="text-lg sm:text-xl font-bold">Green Bond Calculator</h3>
                </div>
                
                <div className="space-y-4 sm:space-y-6">
                  <div>
                    <label className="block text-sm text-gray-400 mb-2">
                      Investment Amount (€)
                    </label>
                    <Input
                      type="text"
                      value={investment || ''}
                      onChange={handleInvestmentChange}
                      min="100"
                      startContent={
                        <div className="pointer-events-none flex items-center">
                          <span className="text-default-400 text-small">€</span>
                        </div>
                      }
                      classNames={{
                        input: "text-white",
                        inputWrapper: "bg-black/30 border-gray-800"
                      }}
                    />
                  </div>

                  <div>
                    <label className="block text-sm text-gray-400 mb-2">
                      Investment Period: {years} years
                    </label>
                    <Slider 
                      size="sm"
                      step={1}
                      maxValue={10}
                      minValue={1}
                      value={years}
                      onChange={(value) => setYears(Number(value))}
                      className="max-w-md"
                      color="success"
                    />
                  </div>

                  <div className="bg-gray-800/50 p-4 rounded-lg space-y-3">
                    <div className="flex justify-between">
                      <span className="text-gray-400">Future Value:</span>
                      <span className="font-bold text-eco-green">€{returns.futureValue}</span>
                    </div>
                    <div className="flex justify-between">
                      <span className="text-gray-400">Total Returns:</span>
                      <span className="font-bold text-eco-green">€{returns.totalReturns}</span>
                    </div>
                  </div>

                  <p className="text-xs text-gray-500">
                    *Estimated returns based on 8% annual yield. Past performance does not guarantee future results.
                  </p>
                </div>
              </Card>
            </div>
          </div>
        </div>
      </div>

      {/* Text Sections */}
      <div className="relative overflow-hidden" ref={learnMoreSectionRef}>
        <div className="max-w-7xl mx-auto px-4 py-12 sm:py-20">
          <div className="max-w-6xl mx-auto space-y-12 sm:space-y-20">
            <div className="space-y-4 sm:space-y-6">
              <motion.h2 
                initial={{ opacity: 0, y: 50 }}
                whileInView={{ opacity: 1, y: 0 }}
                viewport={{ once: true }}
                transition={{ duration: 0.6 }}
                className="text-3xl sm:text-4xl font-bold text-eco-green text-center md:text-left"
              >
                Humanity Needs to Act Now
              </motion.h2>
              <motion.p 
                initial={{ opacity: 0, y: 30 }}
                whileInView={{ opacity: 1, y: 0 }}
                viewport={{ once: true }}
                transition={{ duration: 0.6, delay: 0.2 }}
                className="text-lg sm:text-xl text-gray-400 leading-relaxed max-w-4xl text-center md:text-left"
              >
                The urgency of addressing climate change has led humanity to seek for more sustainable and ethical investment opportunities. Forests, known as Earth's lungs, offer a compelling solution.
              </motion.p>
            </div>

            <div className="space-y-4 sm:space-y-6">
              <motion.h2 
                initial={{ opacity: 0, y: 50 }}
                whileInView={{ opacity: 1, y: 0 }}
                viewport={{ once: true }}
                transition={{ duration: 0.6 }}
                className="text-3xl sm:text-4xl font-bold text-eco-green text-center md:text-left"
              >
                We Want to Save the Planet
              </motion.h2>
              <motion.p 
                initial={{ opacity: 0, y: 30 }}
                whileInView={{ opacity: 1, y: 0 }}
                viewport={{ once: true }}
                transition={{ duration: 0.6, delay: 0.2 }}
                className="text-lg sm:text-xl text-gray-400 leading-relaxed max-w-4xl text-center md:text-left"
              >
                We plant the trees and take care of the land! It isn't merely a financial venture; it's a commitment to ecological regeneration. Our mission is to increase and maintain healthy forest areas while offering stable and profitable investment in green bond tokens.
              </motion.p>
            </div>
          </div>
        </div>
      </div>

      {/* How it Works Section */}
      <div className="relative overflow-hidden">
        <div className="max-w-7xl mx-auto px-4 py-12 sm:py-20">
          <div className="max-w-6xl mx-auto">
            <motion.h2 
              initial={{ opacity: 0, y: 50 }}
              whileInView={{ opacity: 1, y: 0 }}
              viewport={{ once: true }}
              transition={{ duration: 0.6 }}
              className="text-3xl sm:text-4xl font-bold text-eco-green text-center mb-8 sm:mb-16"
            >
              How it Works
            </motion.h2>
            <div className="grid sm:grid-cols-2 md:grid-cols-3 gap-6 sm:gap-8">
              <motion.div
                initial={{ opacity: 0, y: 30 }}
                whileInView={{ opacity: 1, y: 0 }}
                viewport={{ once: true }}
                transition={{ duration: 0.6, delay: 0.2 }}
              >
                <Card className="p-6 sm:p-8 bg-gray-900/50 border-1 border-gray-800 h-full">
                  <div className="flex flex-col items-center text-center">
                    <Coins className="w-12 h-12 sm:w-16 sm:h-16 text-eco-green mb-4 sm:mb-6" />
                    <h3 className="text-lg sm:text-xl font-bold mb-3 sm:mb-4">Investment</h3>
                    <p className="text-gray-400">
                      Investors purchase green bond tokens to finance afforestation projects. Tokens can be traded or held to receive returns.
                    </p>
                  </div>
                </Card>
              </motion.div>

              <motion.div
                initial={{ opacity: 0, y: 30 }}
                whileInView={{ opacity: 1, y: 0 }}
                viewport={{ once: true }}
                transition={{ duration: 0.6, delay: 0.4 }}
              >
                <Card className="p-6 sm:p-8 bg-gray-900/50 border-1 border-gray-800 h-full">
                  <div className="flex flex-col items-center text-center">
                    <Trees className="w-12 h-12 sm:w-16 sm:h-16 text-eco-green mb-4 sm:mb-6" />
                    <h3 className="text-lg sm:text-xl font-bold mb-3 sm:mb-4">Forest Management</h3>
                    <p className="text-gray-400">
                      Upwood together with partners plant trees and maintain the forest throughout offering period. Each forest is verified with geospatial models to verify investment impact.
                    </p>
                  </div>
                </Card>
              </motion.div>

              <motion.div
                initial={{ opacity: 0, y: 30 }}
                whileInView={{ opacity: 1, y: 0 }}
                viewport={{ once: true }}
                transition={{ duration: 0.6, delay: 0.6 }}
                className="sm:col-span-2 md:col-span-1"
              >
                <Card className="p-6 sm:p-8 bg-gray-900/50 border-1 border-gray-800 h-full">
                  <div className="flex flex-col items-center text-center">
                    <Banknote className="w-12 h-12 sm:w-16 sm:h-16 text-eco-green mb-4 sm:mb-6" />
                    <h3 className="text-lg sm:text-xl font-bold mb-3 sm:mb-4">Returns</h3>
                    <p className="text-gray-400">
                      Investors receive bond interest payments from forestry operations and carbon credit sales. At the end of maturity period investors receive bond face value.
                    </p>
                  </div>
                </Card>
              </motion.div>
            </div>
          </div>
        </div>
      </div>

      {/* Token Info section */}
      <div className="relative overflow-hidden">
        <div className="max-w-7xl mx-auto px-4 py-12 sm:py-20">
          <div className="max-w-6xl mx-auto">
            <motion.div
              key={selectedToken.name}
              initial={{ opacity: 0, y: 20 }}
              animate={{ opacity: 1, y: 0 }}
              exit={{ opacity: 0, y: -20 }}
              transition={{ duration: 0.5 }}
              className="grid md:grid-cols-3 gap-8 sm:gap-12 items-center mb-8"
            >
              {/* Token Graphics */}
              <div className="relative">
                <div className="aspect-square rounded-full bg-gradient-to-br from-forest-green/30 to-eco-green/30 p-8 flex items-center justify-center">
                  <div className="absolute inset-0 rounded-full border-2 border-eco-green/20 animate-pulse"></div>
                  {React.createElement(selectedToken.icon, { className: "w-20 h-20 sm:w-32 sm:h-32 text-eco-green" })}
                </div>
              </div>

              {/* Token Information */}
              <div className="md:col-span-2 space-y-4 sm:space-y-6 text-center md:text-left">
                <h2 className="text-3xl sm:text-4xl font-bold text-eco-green">
                  {selectedToken.name}
                </h2>
                <p className="text-lg sm:text-xl text-gray-400 leading-relaxed">
                  {selectedToken.description}
                </p>
                <div className="flex flex-wrap gap-6 sm:gap-8 justify-center md:justify-start mt-6 sm:mt-8">
                  {selectedToken.stats.map((stat, index) => (
                    <div key={index} className="flex flex-col">
                      <span className="text-2xl sm:text-3xl font-bold text-eco-green">{stat.value}</span>
                      <span className="text-sm text-gray-400">{stat.label}</span>
                    </div>
                  ))}
                </div>
              </div>
            </motion.div>

            {/* Token Type Slider */}
            <div className="flex flex-col items-center">
              <Slider 
                size="sm"
                step={1}
                maxValue={2}
                minValue={0}
                value={selectedTokenType}
                onChange={(value) => setSelectedTokenType(Number(value))}
                className="max-w-[200px]"
                color="success"
                marks={[
                  { value: 0, label: "Bond" },
                  { value: 1, label: "Carbon" },
                  { value: 2, label: "NFT" }
                ]}
              />
            </div>
          </div>
        </div>
      </div>

      {/* Impact Investment App Section */}
      <div className="relative overflow-hidden">
        <div className="max-w-7xl mx-auto px-4 py-12 sm:py-20">
          <div className="max-w-6xl mx-auto">
            <motion.div
              initial={{ opacity: 0, y: 30 }}
              whileInView={{ opacity: 1, y: 0 }}
              viewport={{ once: true }}
              transition={{ duration: 0.6 }}
              className="text-center mb-8 sm:mb-12"
            >
              <h2 className="text-3xl sm:text-4xl font-bold text-eco-green mb-4 sm:mb-6">
                Our Impact Investment App
              </h2>
              <p className="text-lg sm:text-xl text-gray-400 max-w-3xl mx-auto">
                We make investments in forests as simple as investing in regular stocks, bonds and cryptocurrencies.
              </p>
            </motion.div>

            <motion.div
              initial={{ opacity: 0, scale: 0.95 }}
              whileInView={{ opacity: 1, scale: 1 }}
              viewport={{ once: true }}
              transition={{ duration: 0.6, delay: 0.2 }}
              className="aspect-video max-w-4xl mx-auto rounded-xl overflow-hidden shadow-lg mb-20 sm:mb-32"
            >
              <iframe 
                width="100%" 
                height="100%" 
                src="https://www.youtube-nocookie.com/embed/AOiFkh_HUv8" 
                title="YouTube video player" 
                frameBorder="0" 
                allow="accelerometer; autoplay; clipboard-write; encrypted-media; gyroscope; picture-in-picture" 
                allowFullScreen
              ></iframe>
            </motion.div>

            {/* Partners Section */}
            <motion.div
              initial={{ opacity: 0, y: 30 }}
              whileInView={{ opacity: 1, y: 0 }}
              viewport={{ once: true }}
              transition={{ duration: 0.6 }}
              className="text-center"
            >
              <h2 className="text-3xl sm:text-4xl font-bold text-eco-green mb-8 sm:mb-16">
                Upwood Partners
              </h2>
              <div className="grid grid-cols-2 md:grid-cols-4 gap-4 items-center max-w-4xl mx-auto">
                {[1, 2, 3, 4].map((index) => (
                  <div key={index} className="flex items-center justify-center">
                    <img
                      src="https://cdn.prod.website-files.com/62ac7e64a826043da22f394e/62aef8da450de1a05b104469_seier-capital-logo.svg"
                      alt={`Partner ${index}`}
                      className="max-h-12 w-auto opacity-70 hover:opacity-100 transition-opacity duration-300 [filter:grayscale(100%)_brightness(0.8)]"
                    />
                  </div>
                ))}
              </div>
            </motion.div>

            {/* Press Section */}
            <motion.div
              initial={{ opacity: 0, y: 30 }}
              whileInView={{ opacity: 1, y: 0 }}
              viewport={{ once: true }}
              transition={{ duration: 0.6 }}
              className="text-center mt-20 sm:mt-32"
            >
              <h2 className="text-3xl sm:text-4xl font-bold text-eco-green mb-8 sm:mb-16">
                Read about Upwood in Press
              </h2>
              <div className="grid grid-cols-1 md:grid-cols-3 gap-4 items-center max-w-4xl mx-auto">
                {[1, 2, 3].map((index) => (
                  <div key={index} className="flex items-center justify-center">
                    <img
                      src="https://cointelegraph.com/icons/logo/en.svg"
                      alt={`Press ${index}`}
                      className="max-h-12 w-auto opacity-70 hover:opacity-100 transition-opacity duration-300 [filter:grayscale(100%)_brightness(0.8)]"
                    />
                  </div>
                ))}
              </div>
            </motion.div>

            {/* FAQ Section */}
            <motion.div
              initial={{ opacity: 0, y: 30 }}
              whileInView={{ opacity: 1, y: 0 }}
              viewport={{ once: true }}
              transition={{ duration: 0.6 }}
              className="mt-20 sm:mt-32"
            >
              <h2 className="text-3xl sm:text-4xl font-bold text-eco-green text-center mb-8 sm:mb-16">
                Frequently Asked Questions
              </h2>
              <div className="max-w-4xl mx-auto">
                <Accordion 
                  variant="shadow"
                  className="bg-gray-900/50"
                >
                  {/* General FAQ */}
                  <AccordionItem
                    key="general"
                    aria-label="General"
                    title="General"
                    indicator={<ChevronDown className="text-eco-green" />}
                    classNames={{
                      title: "text-xl font-semibold text-eco-green",
                      content: "text-gray-300"
                    }}
                  >
                    <Accordion>
                      <AccordionItem
                        key="what-is-greenbond"
                        title="What is Upwood?"
                        className="bg-transparent"
                      >
                        Upwood is impact investment platform which allows to invest in real world forests through impact bonds. Upwood utilize blockchain technology and conduct geospatial verification thereby ensuring transparent life cycle of impact investments.
                      </AccordionItem>
                      <AccordionItem
                        key="how-it-works"
                        title="How does it work?"
                        className="bg-transparent"
                      >
                        We issue sustainability linked bonds on blockchain. Each token represents a bond backed up by real world forests, providing both environmental and financial returns.
                      </AccordionItem>
                      <AccordionItem
                        key="minimum-investment"
                        title="What is the minimum investment?"
                        className="bg-transparent"
                      >
                        The minimum investment starts at €100, making sustainable forest investment accessible to a wider range of investors.
                      </AccordionItem>
                    </Accordion>
                  </AccordionItem>

                  {/* Legal FAQ */}
                  <AccordionItem
                    key="legal"
                    aria-label="Legal"
                    title="Legal"
                    indicator={<ChevronDown className="text-eco-green" />}
                    classNames={{
                      title: "text-xl font-semibold text-eco-green",
                      content: "text-gray-300"
                    }}
                  >
                    <Accordion>
                      <AccordionItem
                        key="regulatory-compliance"
                        title="What regulatory framework applies?"
                        className="bg-transparent"
                      >
                        Our bond tokens are MFID II compliant and regulated financial instruments with approved Prospectus.
                      </AccordionItem>
                      <AccordionItem
                        key="investor-protection"
                        title="How are investors protected?"
                        className="bg-transparent"
                      >
                        Upwood in its core have mission to ensure full transparency and eliminate greenwashing, all data about the asssets and operations are avaialble for investors and all operations are overseen by licensed financial institutions and auditors.
                      </AccordionItem>
                      <AccordionItem
                        key="token-rights"
                        title="What rights do token holders have?"
                        className="bg-transparent"
                      >
                        Token holders receive regular interest payments and can claim face value at the maturity period.
                      </AccordionItem>
                    </Accordion>
                  </AccordionItem>

                  {/* Investment Process FAQ */}
                  <AccordionItem
                    key="investment"
                    aria-label="Investment Process"
                    title="Investment Process"
                    indicator={<ChevronDown className="text-eco-green" />}
                    classNames={{
                      title: "text-xl font-semibold text-eco-green",
                      content: "text-gray-300"
                    }}
                  >
                    <Accordion>
                      <AccordionItem
                        key="how-to-invest"
                        title="How can I invest?"
                        className="bg-transparent"
                      >
                        You can invest through our platform after completing the KYC/KYB process. We accept both traditional currency and cryptocurrency payments.
                      </AccordionItem>
                      <AccordionItem
                        key="returns"
                        title="What returns can I expect?"
                        className="bg-transparent"
                      >
                        Historical returns from forest investments range from 6-10% annually, combining both timber value appreciation, land value increase and carbon credit revenues.
                      </AccordionItem>
                      <AccordionItem
                        key="exit-options"
                        title="What are the exit options?"
                        className="bg-transparent"
                      >
                        Tokens can be traded on secondary markets or held until maturity, typically 5-10 years.
                      </AccordionItem>
                    </Accordion>
                  </AccordionItem>

                  {/* Geospatial Verification FAQ */}
                  <AccordionItem
                    key="geospatial"
                    aria-label="Geospatial Verification"
                    title="Geospatial Verification"
                    indicator={<ChevronDown className="text-eco-green" />}
                    classNames={{
                      title: "text-xl font-semibold text-eco-green",
                      content: "text-gray-300"
                    }}
                  >
                    <Accordion>
                      <AccordionItem
                        key="monitoring"
                        title="How are forests monitored?"
                        className="bg-transparent"
                      >
                        We use satellite imagery, drone imagery and AI-powered analysis to monitor forest health, growth, and carbon sequestration.
                      </AccordionItem>
                      <AccordionItem
                        key="data-accuracy"
                        title="How accurate is the data?"
                        className="bg-transparent"
                      >
                        Our geospatial monitoring system provides currently 75%+ accuracy in measuring forest metrics, verified by independent auditors. We are continuosly updating our models to ensure higher accuracy
                      </AccordionItem>
                      <AccordionItem
                        key="verification-process"
                        title="What is the verification process?"
                        className="bg-transparent"
                      >
                        Forest data is continuously collected via satellites, drone imagery and verified anually by independent forestry experts and environmental auditors.
                      </AccordionItem>
                    </Accordion>
                  </AccordionItem>
                </Accordion>
              </div>
            </motion.div>
          </div>
        </div>
      </div>

      {/* CTA Section */}
      <div className="container mx-auto px-4 py-12 sm:py-20">
        <Card className="p-6 sm:p-12 bg-[#1D4B3A] border-1 border-gray-800">
          <div className="max-w-3xl mx-auto text-center">
            <h2 className="text-3xl sm:text-4xl font-bold mb-4 sm:mb-6 text-white">Time for Real Climate Action</h2>
            <p className="text-white/90 mb-6 sm:mb-8 text-base sm:text-lg leading-relaxed">
              Join community of like minded investors to fund and earn from stable, profitable long term investments in forests that are backed up by dividends from sustainable forestry operations, forestland value increase and carbon credits.
            </p>
            <div className="flex justify-center">
              <Button 
                size="lg"
                className="w-full sm:w-auto font-semibold bg-[#3a5a40] text-white hover:bg-[#3a5a40]/90"
                onClick={onRequestInvitation}
              >
                Request Invitation <ArrowRight className="ml-2" />
              </Button>
            </div>
          </div>
        </Card>
      </div>
    </>
  );
}