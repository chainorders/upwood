import React, { useEffect, useState } from 'react';
import { motion } from 'framer-motion';

// Generate random circles with different sizes and positions
const generateCircles = (count: number) => {
  return Array.from({ length: count }, (_, i) => ({
    id: i,
    size: Math.random() * 20 + 10, // Random between 10-30px
    x: Math.random() * 100,
    y: Math.random() * 100,
    duration: Math.random() * 15 + 20, // Longer duration for smoother movement
    delay: Math.random() * -20,
  }));
};

export function FloatingCircles() {
  const [circles] = useState(() => generateCircles(40));
  const [windowSize, setWindowSize] = useState({
    width: window.innerWidth,
    height: window.innerHeight,
  });

  useEffect(() => {
    const handleResize = () => {
      setWindowSize({
        width: window.innerWidth,
        height: window.innerHeight,
      });
    };

    window.addEventListener('resize', handleResize);
    return () => window.removeEventListener('resize', handleResize);
  }, []);

  return (
    <div className="fixed inset-0 pointer-events-none overflow-hidden">
      {circles.map((circle) => {
        const baseX = circle.x * (windowSize.width * 0.01);
        const baseY = circle.y * (windowSize.height * 0.01);
        
        // Create random movement paths
        const pathX = Array.from({ length: 5 }, () => 
          baseX + (Math.random() * 2 - 1) * (windowSize.width * 0.05)
        );
        const pathY = Array.from({ length: 5 }, () => 
          baseY + (Math.random() * 2 - 1) * (windowSize.height * 0.05)
        );

        return (
          <motion.div
            key={circle.id}
            className="absolute rounded-full bg-[#4a7a50]"
            style={{
              width: circle.size,
              height: circle.size,
              opacity: 0.25,
              backdropFilter: 'blur(2px)',
            }}
            initial={{
              x: baseX,
              y: baseY,
            }}
            animate={{
              x: pathX,
              y: pathY,
            }}
            transition={{
              duration: circle.duration,
              repeat: Infinity,
              repeatType: "reverse",
              delay: circle.delay,
              ease: "easeInOut",
            }}
          />
        );
      })}
    </div>
  );
}