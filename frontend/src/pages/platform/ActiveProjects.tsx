import React, { useState } from 'react';
import { Card, Button, Progress, Chip } from "@nextui-org/react";
import { motion } from 'framer-motion';
import { TreePine, Globe, Coins, ArrowUpRight } from 'lucide-react';
import { ProjectDetailsDrawer } from '../../components/platform/ProjectDetailsDrawer';
import { TokenSwapModal } from '../../components/platform/TokenSwapModal';

const projects = [
  {
    id: 1,
    title: "EU Taxonomy aligned Green Bonds",
    location: "Latvia, EU",
    description: "Green Bonds backed up by diversified portfolio of forest properties yielding from sustainable forestry and carbon credits.",
    raised: 970000,
    goal: 1000000,
    roi: "5%+variable",
    term: "10 years",
    type: "Green Bond",
    status: "Active",
    image: "https://images.pexels.com/photos/32965051/pexels-photo-32965051.jpeg?_gl=1*34iq8n*_ga*ODk3OTA0Mzg1LjE3NDM0NDIxMjY.*_ga_8JE65Q40S6*czE3NTIyNTk0NDAkbzUkZzEkdDE3NTIyNTk1MjEkajQyJGwwJGgw",
    carbonCredits: "5,325 tons",
    area: "250 hectares",
    tokenSymbol: "tEUGB"
  },
  {
    id: 2,
    title: "Forest bonds",
    location: "Latvia, EU",
    description: "Forest asset backed securities yielding from sustainable forestry operations.",
    raised: 550000,
    goal: 1000000,
    roi: "7%+variable",
    term: "7 years",
    type: "Sustainability linked bond",
    status: "Active",
    image: "https://images.pexels.com/photos/1008739/pexels-photo-1008739.jpeg?_gl=1*17toqmm*_ga*ODk3OTA0Mzg1LjE3NDM0NDIxMjY.*_ga_8JE65Q40S6*czE3NTIyNTk0NDAkbzUkZzEkdDE3NTIyNTk2MzAkajEyJGwwJGgw",
    carbonCredits: "4,473 tons",
    area: "312 hectares",
    tokenSymbol: "tEUGB"
  }
];

export default function ActiveProjects() {
  const [selectedProject, setSelectedProject] = useState<typeof projects[0] | null>(null);
  const [isInvestModalOpen, setIsInvestModalOpen] = useState(false);

  const handleInvest = (project: typeof projects[0]) => {
    setSelectedProject(project);
    setIsInvestModalOpen(true);
  };

  return (
    <div className="p-4 lg:p-6">
      <div className="mb-8">
        <h1 className="text-2xl lg:text-3xl font-bold text-white mb-4">Active Projects</h1>
        <p className="text-sm lg:text-base text-gray-400">
          Explore and invest in our carefully selected forest projects
        </p>
      </div>

      <div className="grid grid-cols-1 lg:grid-cols-2 gap-6">
        {projects.map((project, index) => (
          <motion.div
            key={project.id}
            initial={{ opacity: 0, y: 20 }}
            animate={{ opacity: 1, y: 0 }}
            transition={{ duration: 0.6, delay: index * 0.1 }}
          >
            <Card className="bg-gray-900/50 border-gray-800">
              <div className="aspect-video relative overflow-hidden">
                <img
                  src={project.image}
                  alt={project.title}
                  className="w-full h-full object-cover"
                />
                <div className="absolute inset-0 bg-gradient-to-t from-black/60 to-transparent" />
                <Chip 
                  className="absolute top-4 right-4 bg-eco-green text-white"
                  size="sm"
                  startContent={<TreePine className="w-4 h-4" />}
                >
                  {project.type}
                </Chip>
              </div>

              <div className="p-6">
                <div className="flex items-start justify-between mb-4">
                  <div>
                    <h3 className="text-lg lg:text-xl font-bold text-white mb-2">{project.title}</h3>
                    <div className="flex items-center text-gray-400 text-sm">
                      <Globe className="w-4 h-4 mr-1" />
                      {project.location}
                    </div>
                  </div>
                  <Chip color="success" variant="flat">
                    {project.status}
                  </Chip>
                </div>

                <p className="text-sm lg:text-base text-gray-400 mb-6">{project.description}</p>

                <div className="grid grid-cols-1 sm:grid-cols-2 gap-4 mb-6">
                  <div className="bg-black/30 p-4 rounded-lg">
                    <div className="text-sm text-gray-400 mb-1">Annual Yield</div>
                    <div className="text-lg lg:text-xl font-bold text-eco-green">{project.roi}</div>
                  </div>
                  <div className="bg-black/30 p-4 rounded-lg">
                    <div className="text-sm text-gray-400 mb-1">Maturity Period</div>
                    <div className="text-lg lg:text-xl font-bold text-white">{project.term}</div>
                  </div>
                  <div className="bg-black/30 p-4 rounded-lg">
                    <div className="text-sm text-gray-400 mb-1">Carbon Credits</div>
                    <div className="text-lg lg:text-xl font-bold text-white">{project.carbonCredits}</div>
                  </div>
                  <div className="bg-black/30 p-4 rounded-lg">
                    <div className="text-sm text-gray-400 mb-1">Total Area</div>
                    <div className="text-lg lg:text-xl font-bold text-white">{project.area}</div>
                  </div>
                </div>

                <div className="mb-6">
                  <div className="flex justify-between text-sm mb-2">
                    <span className="text-gray-400">Bond series funded</span>
                    <span className="text-white font-medium">
                      €{project.raised.toLocaleString()} / €{project.goal.toLocaleString()}
                    </span>
                  </div>
                  <Progress 
                    value={(project.raised / project.goal) * 100}
                    className="h-2"
                    color="success"
                  />
                </div>

                <div className="flex flex-col sm:flex-row gap-3">
                  <Button
                    className="flex-1 bg-eco-green text-white font-semibold"
                    endContent={<Coins className="w-4 h-4" />}
                    onClick={() => handleInvest(project)}
                  >
                    Invest Now
                  </Button>
                  <Button
                    className="bg-gray-800 text-white"
                    variant="flat"
                    endContent={<ArrowUpRight className="w-4 h-4" />}
                    onClick={() => setSelectedProject(project)}
                  >
                    Details
                  </Button>
                </div>
              </div>
            </Card>
          </motion.div>
        ))}
      </div>

      <ProjectDetailsDrawer
        isOpen={selectedProject !== null && !isInvestModalOpen}
        onClose={() => setSelectedProject(null)}
        project={selectedProject!}
      />

      {selectedProject && (
        <TokenSwapModal
          isOpen={isInvestModalOpen}
          onClose={() => setIsInvestModalOpen(false)}
          project={selectedProject}
        />
      )}
    </div>
  );
}