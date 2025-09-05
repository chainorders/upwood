import React from 'react';
import { Modal, ModalContent, ModalHeader, ModalBody, Button, Tabs, Tab, Progress, Chip } from "@nextui-org/react";
import { motion } from 'framer-motion';
import { Globe, Coins, FileText, BarChart3, Map, Download } from 'lucide-react';

interface ProjectDetailsDrawerProps {
  isOpen: boolean;
  onClose: () => void;
  project: {
    id: number;
    title: string;
    location: string;
    description: string;
    raised: number;
    goal: number;
    roi: string;
    term: string;
    type: string;
    status: string;
    image: string;
    carbonCredits: string;
    area: string;
  };
}

export function ProjectDetailsDrawer({ isOpen, onClose, project }: ProjectDetailsDrawerProps) {
  return (
    <Modal 
      isOpen={isOpen} 
      onClose={onClose}
      className="dark"
      size="3xl"
      placement="right"
      scrollBehavior="inside"
    >
      <ModalContent className="h-screen">
        {() => (
          <>
            <ModalHeader className="flex flex-col gap-1">
              <h3 className="text-2xl font-bold text-white">{project.title}</h3>
              <div className="flex items-center gap-2 text-gray-400">
                <Globe className="w-4 h-4" />
                {project.location}
              </div>
            </ModalHeader>
            <ModalBody className="overflow-auto">
              <Tabs 
                color="success"
                variant="bordered"
                classNames={{
                  tabList: "bg-gray-900/50 p-0 border border-gray-800 rounded-lg",
                  cursor: "bg-eco-green",
                  tab: "text-gray-400 h-12",
                  tabContent: "group-data-[selected=true]:text-white"
                }}
              >
                {/* Market Information */}
                <Tab
                  key="market"
                  title={
                    <div className="flex items-center gap-2">
                      <BarChart3 className="w-4 h-4" />
                      <span>Market Information</span>
                    </div>
                  }
                >
                  <div className="py-4 space-y-6">
                    <div>
                      <h4 className="text-lg font-bold text-white mb-4">Market Analysis</h4>
                      <div className="space-y-4">
                        <div className="bg-black/30 p-4 rounded-lg">
                          <h5 className="font-medium text-white mb-2">Timber Market Trends</h5>
                          <p className="text-gray-400">
                            The timber market in the Baltic region has shown consistent growth over the past decade, with prices increasing by an average of 5-7% annually. The demand for sustainable timber products continues to rise, driven by environmental regulations and consumer preferences.
                          </p>
                        </div>
                        <div className="bg-black/30 p-4 rounded-lg">
                          <h5 className="font-medium text-white mb-2">Carbon Credit Market</h5>
                          <p className="text-gray-400">
                            The voluntary carbon market has experienced significant growth, with prices for forest-based carbon credits increasing by over 140% in the last year. Projections indicate continued strong demand as companies strive to meet their net-zero commitments.
                          </p>
                        </div>
                      </div>
                    </div>

                    <div>
                      <h4 className="text-lg font-bold text-white mb-4">Competitive Analysis</h4>
                      <div className="bg-black/30 p-4 rounded-lg">
                        <div className="space-y-4">
                          <div>
                            <div className="flex justify-between text-sm mb-2">
                              <span className="text-gray-400">Market Position</span>
                              <span className="text-white font-medium">Strong</span>
                            </div>
                            <Progress 
                              value={85}
                              className="h-2"
                              color="success"
                            />
                          </div>
                          <div>
                            <div className="flex justify-between text-sm mb-2">
                              <span className="text-gray-400">Growth Potential</span>
                              <span className="text-white font-medium">Very High</span>
                            </div>
                            <Progress 
                              value={90}
                              className="h-2"
                              color="success"
                            />
                          </div>
                          <div>
                            <div className="flex justify-between text-sm mb-2">
                              <span className="text-gray-400">Risk Level</span>
                              <span className="text-white font-medium">Low</span>
                            </div>
                            <Progress 
                              value={25}
                              className="h-2"
                              color="success"
                            />
                          </div>
                        </div>
                      </div>
                    </div>
                  </div>
                </Tab>

                {/* Project Financials */}
                <Tab
                  key="financials"
                  title={
                    <div className="flex items-center gap-2">
                      <Coins className="w-4 h-4" />
                      <span>Project Financials</span>
                    </div>
                  }
                >
                  <div className="py-4 space-y-6">
                    <div>
                      <h4 className="text-lg font-bold text-white mb-4">Investment Overview</h4>
                      <div className="grid grid-cols-1 md:grid-cols-2 gap-4">
                        <div className="bg-black/30 p-4 rounded-lg">
                          <div className="text-sm text-gray-400 mb-1">Total Investment Goal</div>
                          <div className="text-xl font-bold text-white">€{project.goal.toLocaleString()}</div>
                          <div className="mt-4">
                            <div className="flex justify-between text-sm mb-2">
                              <span className="text-gray-400">Progress</span>
                              <span className="text-white font-medium">
                                {((project.raised / project.goal) * 100).toFixed(1)}%
                              </span>
                            </div>
                            <Progress 
                              value={(project.raised / project.goal) * 100}
                              className="h-2"
                              color="success"
                            />
                          </div>
                        </div>
                        <div className="bg-black/30 p-4 rounded-lg">
                          <div className="text-sm text-gray-400 mb-1">Token Price</div>
                          <div className="text-xl font-bold text-white">€100</div>
                          <div className="mt-2 text-sm text-gray-400">
                            Minimum Investment: €1,000
                          </div>
                        </div>
                      </div>
                    </div>

                    <div>
                      <h4 className="text-lg font-bold text-white mb-4">Revenue Streams</h4>
                      <div className="space-y-4">
                        <div className="bg-black/30 p-4 rounded-lg">
                          <h5 className="font-medium text-white mb-2">Timber Revenue</h5>
                          <p className="text-gray-400 mb-4">
                            Projected annual timber revenue based on sustainable harvesting practices and current market prices.
                          </p>
                          <div className="flex justify-between text-sm">
                            <span className="text-gray-400">Estimated Annual Yield</span>
                            <span className="text-eco-green font-medium">€150,000</span>
                          </div>
                        </div>
                        <div className="bg-black/30 p-4 rounded-lg">
                          <h5 className="font-medium text-white mb-2">Carbon Credits</h5>
                          <p className="text-gray-400 mb-4">
                            Additional revenue from carbon credit sales in the voluntary carbon market.
                          </p>
                          <div className="flex justify-between text-sm">
                            <span className="text-gray-400">Annual Carbon Revenue</span>
                            <span className="text-eco-green font-medium">€75,000</span>
                          </div>
                        </div>
                      </div>
                    </div>
                  </div>
                </Tab>

                {/* Prospectus */}
                <Tab
                  key="prospectus"
                  title={
                    <div className="flex items-center gap-2">
                      <FileText className="w-4 h-4" />
                      <span>Prospectus</span>
                    </div>
                  }
                >
                  <div className="py-4 space-y-6">
                    <div className="bg-black/30 p-6 rounded-lg">
                      <div className="flex items-center justify-between mb-6">
                        <div>
                          <h4 className="text-lg font-bold text-white mb-1">Project Prospectus</h4>
                          <p className="text-sm text-gray-400">
                            Detailed information about the investment opportunity
                          </p>
                        </div>
                        <Button
                          className="bg-eco-green text-white"
                          startContent={<Download className="w-4 h-4" />}
                        >
                          Download PDF
                        </Button>
                      </div>
                      <div className="space-y-4">
                        <div>
                          <h5 className="font-medium text-white mb-2">Key Documents</h5>
                          <div className="space-y-2">
                            <div className="flex items-center justify-between p-3 bg-black/30 rounded-lg">
                              <div className="flex items-center gap-3">
                                <FileText className="w-4 h-4 text-gray-400" />
                                <span className="text-white">Investment Memorandum</span>
                              </div>
                              <Button
                                size="sm"
                                variant="flat"
                                className="bg-gray-800 text-white"
                              >
                                View
                              </Button>
                            </div>
                            <div className="flex items-center justify-between p-3 bg-black/30 rounded-lg">
                              <div className="flex items-center gap-3">
                                <FileText className="w-4 h-4 text-gray-400" />
                                <span className="text-white">Financial Projections</span>
                              </div>
                              <Button
                                size="sm"
                                variant="flat"
                                className="bg-gray-800 text-white"
                              >
                                View
                              </Button>
                            </div>
                            <div className="flex items-center justify-between p-3 bg-black/30 rounded-lg">
                              <div className="flex items-center gap-3">
                                <FileText className="w-4 h-4 text-gray-400" />
                                <span className="text-white">Risk Assessment</span>
                              </div>
                              <Button
                                size="sm"
                                variant="flat"
                                className="bg-gray-800 text-white"
                              >
                                View
                              </Button>
                            </div>
                          </div>
                        </div>
                      </div>
                    </div>
                  </div>
                </Tab>

                {/* Geospatial Information */}
                <Tab
                  key="geospatial"
                  title={
                    <div className="flex items-center gap-2">
                      <Map className="w-4 h-4" />
                      <span>Geospatial Info</span>
                    </div>
                  }
                >
                  <div className="py-4 space-y-6">
                    <div>
                      <h4 className="text-lg font-bold text-white mb-4">Location & Mapping</h4>
                      <div className="aspect-video bg-black/30 rounded-lg overflow-hidden mb-4">
                        {/* Placeholder for map */}
                        <div className="w-full h-full flex items-center justify-center text-gray-400">
                          Interactive Map Coming Soon
                        </div>
                      </div>
                      <div className="grid grid-cols-2 gap-4">
                        <div className="bg-black/30 p-4 rounded-lg">
                          <div className="text-sm text-gray-400 mb-1">Coordinates</div>
                          <div className="text-white">56.9496° N, 24.1052° E</div>
                        </div>
                        <div className="bg-black/30 p-4 rounded-lg">
                          <div className="text-sm text-gray-400 mb-1">Total Area</div>
                          <div className="text-white">{project.area}</div>
                        </div>
                      </div>
                    </div>

                    <div>
                      <h4 className="text-lg font-bold text-white mb-4">Forest Composition</h4>
                      <div className="space-y-4">
                        <div className="bg-black/30 p-4 rounded-lg">
                          <h5 className="font-medium text-white mb-2">Species Distribution</h5>
                          <div className="space-y-3">
                            <div>
                              <div className="flex justify-between text-sm mb-1">
                                <span className="text-gray-400">Pine</span>
                                <span className="text-white">60%</span>
                              </div>
                              <Progress 
                                value={60}
                                className="h-2"
                                color="success"
                              />
                            </div>
                            <div>
                              <div className="flex justify-between text-sm mb-1">
                                <span className="text-gray-400">Spruce</span>
                                <span className="text-white">30%</span>
                              </div>
                              <Progress 
                                value={30}
                                className="h-2"
                                color="success"
                              />
                            </div>
                            <div>
                              <div className="flex justify-between text-sm mb-1">
                                <span className="text-gray-400">Birch</span>
                                <span className="text-white">10%</span>
                              </div>
                              <Progress 
                                value={10}
                                className="h-2"
                                color="success"
                              />
                            </div>
                          </div>
                        </div>
                      </div>
                    </div>

                    <div>
                      <h4 className="text-lg font-bold text-white mb-4">Environmental Impact</h4>
                      <div className="grid grid-cols-2 gap-4">
                        <div className="bg-black/30 p-4 rounded-lg">
                          <div className="text-sm text-gray-400 mb-1">Carbon Sequestration</div>
                          <div className="text-white font-bold mb-2">{project.carbonCredits}</div>
                          <div className="text-sm text-gray-400">Annual CO₂ absorption</div>
                        </div>
                        <div className="bg-black/30 p-4 rounded-lg">
                          <div className="text-sm text-gray-400 mb-1">Biodiversity Score</div>
                          <div className="text-white font-bold mb-2">8.5/10</div>
                          <div className="text-sm text-gray-400">Based on habitat assessment</div>
                        </div>
                      </div>
                    </div>
                  </div>
                </Tab>
              </Tabs>
            </ModalBody>
          </>
        )}
      </ModalContent>
    </Modal>
  );
}